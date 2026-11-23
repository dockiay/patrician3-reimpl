//! Generado por `reimpl/tools/ui_rt_grp.py cy` con `tools/lift.py` (no editar a mano): grupo `cy`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_CY: [(u32, super::core_table::CoreFn); 72] = [
    (0x4d7340, f_004d7340),
    (0x4d7870, f_004d7870),
    (0x4d7ac0, f_004d7ac0),
    (0x4daf10, f_004daf10),
    (0x4f3610, f_004f3610),
    (0x4f3630, f_004f3630),
    (0x4f3640, f_004f3640),
    (0x4f3650, f_004f3650),
    (0x4f5450, f_004f5450),
    (0x4f5a20, f_004f5a20),
    (0x4f5d80, f_004f5d80),
    (0x4f6470, f_004f6470),
    (0x4f64b0, f_004f64b0),
    (0x4f69b0, f_004f69b0),
    (0x4f8a50, f_004f8a50),
    (0x4f8c70, f_004f8c70),
    (0x4f9170, f_004f9170),
    (0x4f91c0, f_004f91c0),
    (0x4f9220, f_004f9220),
    (0x4f99b0, f_004f99b0),
    (0x4fbba0, f_004fbba0),
    (0x4fd0d0, f_004fd0d0),
    (0x4fd5e0, f_004fd5e0),
    (0x4fda10, f_004fda10),
    (0x4ff2d0, f_004ff2d0),
    (0x4ff780, f_004ff780),
    (0x4ffc20, f_004ffc20),
    (0x500ec0, f_00500ec0),
    (0x500f50, f_00500f50),
    (0x50cfb0, f_0050cfb0),
    (0x50ffb0, f_0050ffb0),
    (0x51ac00, f_0051ac00),
    (0x51d5f0, f_0051d5f0),
    (0x51d680, f_0051d680),
    (0x51d6f0, f_0051d6f0),
    (0x51e270, f_0051e270),
    (0x5226c0, f_005226c0),
    (0x523d70, f_00523d70),
    (0x5241e0, f_005241e0),
    (0x524780, f_00524780),
    (0x524c30, f_00524c30),
    (0x524cd0, f_00524cd0),
    (0x524e20, f_00524e20),
    (0x524f00, f_00524f00),
    (0x525220, f_00525220),
    (0x525390, f_00525390),
    (0x5259d0, f_005259d0),
    (0x525ca0, f_00525ca0),
    (0x525d10, f_00525d10),
    (0x526b00, f_00526b00),
    (0x529660, f_00529660),
    (0x529e20, f_00529e20),
    (0x52a150, f_0052a150),
    (0x52de10, f_0052de10),
    (0x52de40, f_0052de40),
    (0x52ea80, f_0052ea80),
    (0x52ec50, f_0052ec50),
    (0x52ee90, f_0052ee90),
    (0x5312e0, f_005312e0),
    (0x532420, f_00532420),
    (0x532c50, f_00532c50),
    (0x5333c0, f_005333c0),
    (0x533cf0, f_00533cf0),
    (0x533d10, f_00533d10),
    (0x533e80, f_00533e80),
    (0x534090, f_00534090),
    (0x5340d0, f_005340d0),
    (0x534f10, f_00534f10),
    (0x5353a0, f_005353a0),
    (0x5492d0, f_005492d0),
    (0x612ee0, f_00612ee0),
    (0x524b70, f_00524b70),
];
pub fn f_004d7340(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d7340;
    loop {
        match pc {
            0x4d7340 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            c.push(m,c.get(7,32));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4d73b9; continue; }
            pc = 0x4d7358; continue;
            }
            0x4d7358 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4d736e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xa),16));
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(0,32); let r = c.add(a,0xau32,32); c.set(0,32,r); }
            { let a = c.get(1,16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x4d73b9; continue; }
            pc = 0x4d737e; continue;
            }
            0x4d737e => {
            c.set(2,8,0x44u32);
            pc = 0x4d7380; continue;
            }
            0x4d7380 => {
            c.set(3,32,c.get(7,32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4d73b9; continue; }
            pc = 0x4d7387; continue;
            }
            0x4d7387 => {
            c.set(3,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x4),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4d73ad; continue; }
            pc = 0x4d7399; continue;
            }
            0x4d7399 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = rm(m,c.r[1].wrapping_add(0x5),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4d73ad; continue; }
            pc = 0x4d73a2; continue;
            }
            0x4d73a2 => {
            c.set(3,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = rm(m,c.r[1].wrapping_add(0x8),16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x4d73c4; continue; }
            pc = 0x4d73ad; continue;
            }
            0x4d73ad => {
            c.set(0,32,c.r[1].wrapping_add(0x6));
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x6),16));
            { let a = c.get(1,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x4d7380; continue; }
            pc = 0x4d73b9; continue;
            }
            0x4d73b9 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,16); let r = c.or(a,0xffffu32,16); c.set(0,16,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4d73c4 => {
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x6),16));
            c.set(2,16,rm(m,c.r[0],16));
            { let a = c.r[0]; wm(m,a,16,c.get(7,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.r[1].wrapping_add(0x6); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,16,c.get(2,16)); }
            c.set(6,16,rm(m,c.r[1].wrapping_add(0xc),16));
            c.push(m, 0x4d73e3);
            { let args = []; c.r[0] = host.call(m, 0x51aab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,c.get(6,16));
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

pub fn f_004d7870(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d7870;
    loop {
        match pc {
            0x4d7870 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,16,rm(m,c.r[1],16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x4d78c0; continue; }
            pc = 0x4d7886; continue;
            }
            0x4d7886 => {
            c.set(5,32,rm(m,c.r[7],32));
            pc = 0x4d7888; continue;
            }
            0x4d7888 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x4),8); c.sub(a,0x71u32,8); }
            if c.cond(5) { pc = 0x4d78b4; continue; }
            pc = 0x4d7898; continue;
            }
            0x4d7898 => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),32),32); }
            if c.cond(5) { pc = 0x4d78b4; continue; }
            pc = 0x4d78a7; continue;
            }
            0x4d78a7 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[6].wrapping_add(0xa),8));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4d78c9; continue; }
            pc = 0x4d78b4; continue;
            }
            0x4d78b4 => {
            c.set(1,32,c.r[0].wrapping_add(0x6));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x4d7888; continue; }
            pc = 0x4d78c0; continue;
            }
            0x4d78c0 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4d78c9 => {
            c.set(3,16,rm(m,c.r[1],16));
            c.set(6,32,c.get(3,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[5]).wrapping_add(0x6),16));
            { let a = c.r[1]; wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            c.push(m, 0x4d78e8);
            { let args = []; c.r[0] = host.call(m, 0x51aab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7],32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x4),16));
            c.set(0,8,0x1u32);
            { let a = c.r[6].wrapping_add(c.r[1]).wrapping_add(0x6); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,16,c.get(3,16)); }
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

pub fn f_004d7ac0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d7ac0;
    loop {
        match pc {
            0x4d7ac0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x6),16));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(13) { pc = 0x4d7af8; continue; }
            pc = 0x4d7ad0; continue;
            }
            0x4d7ad0 => {
            c.set(6,32,rm(m,c.r[1],32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4d7ad6; continue;
            }
            0x4d7ad6 => {
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[6]));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4),8));
            { let a = c.get(0,8); c.sub(a,0x5fu32,8); }
            if c.cond(5) { pc = 0x4d7aee; continue; }
            pc = 0x4d7ae4; continue;
            }
            0x4d7ae4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4d7aff; continue; }
            pc = 0x4d7aee; continue;
            }
            0x4d7aee => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x6),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4d7ad6; continue; }
            pc = 0x4d7af8; continue;
            }
            0x4d7af8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4d7aff => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004daf10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4daf10;
    loop {
        match pc {
            0x4daf10 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4daf2b; continue; }
            pc = 0x4daf24; continue;
            }
            0x4daf24 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4daf2b => {
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x4daf50; continue; }
            pc = 0x4daf49; continue;
            }
            0x4daf49 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4daf50 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4daf5c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f220, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4daf91; continue; }
            pc = 0x4daf64; continue;
            }
            0x4daf64 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0x66666667u32);
            c.set(1,32,c.r[2].wrapping_add(c.r[2]).wrapping_add(0x5));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,0x64u32);
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(7,32,c.get(2,32));
            c.set(0,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4daf91 => {
            c.set(1,32,c.r[0].wrapping_mul(4).wrapping_add(0x5));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(7,32,c.get(2,32));
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(6,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m, 0x4dafc3);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4dafc9; continue; }
            pc = 0x4dafc7; continue;
            }
            0x4dafc7 => {
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            pc = 0x4dafc9; continue;
            }
            0x4dafc9 => {
            c.set(0,32,c.get(7,32));
            c.set(1,32,0x64u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3610(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3610;
    loop {
        match pc {
            0x4f3610 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x2c8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x10u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.sub(a,0x10u32,8); }
            c.set(1,8,u32::from(c.cond(4)));
            c.set(0,8,c.get(1,8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3630(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3630;
    loop {
        match pc {
            0x4f3630 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x762),16));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3640(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3640;
    loop {
        match pc {
            0x4f3640 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xc),16));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3650(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3650;
    loop {
        match pc {
            0x4f3650 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x10),8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f5450(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f5450;
    loop {
        match pc {
            0x4f5450 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4f54f5; continue; }
            pc = 0x4f5462; continue;
            }
            0x4f5462 => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[1].wrapping_add(0xe),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4f54f5; continue; }
            pc = 0x4f5476; continue;
            }
            0x4f5476 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4f547e; continue;
            }
            0x4f547e => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x39),8));
            c.set(6,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4f54c9; continue; }
            pc = 0x4f549b; continue;
            }
            0x4f549b => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x4f54c9; continue; }
            pc = 0x4f54a8; continue;
            }
            0x4f54a8 => {
            { let a = c.get(0,16); c.sub(a,0x2u32,16); }
            if c.cond(4) { pc = 0x4f54c9; continue; }
            pc = 0x4f54ae; continue;
            }
            0x4f54ae => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x54),32));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4f54de; continue; }
            pc = 0x4f54b8; continue;
            }
            0x4f54b8 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,0x0u32); }
            c.push(m, 0x4f54c9);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4f54c9; continue;
            }
            0x4f54c9 => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4f547e; continue; }
            pc = 0x4f54d7; continue;
            }
            0x4f54d7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4f54de => {
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[3]; wm(m,a,32,0x0u32); }
            c.push(m, 0x4f54f5);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4f54f5; continue;
            }
            0x4f54f5 => {
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

pub fn f_004f5a20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f5a20;
    loop {
        match pc {
            0x4f5a20 => {
            { let a = c.get(4,32); let r = c.sub(a,0x44u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4f5a4b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x4f5d72; continue; }
            pc = 0x4f5a57; continue;
            }
            0x4f5a57 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            pc = 0x4f5a63; continue;
            }
            0x4f5a5f => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x38),32));
            pc = 0x4f5a63; continue;
            }
            0x4f5a63 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x468),32));
            c.set(2,32,0x1u32);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x2c6),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.get(5,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4f5cd2; continue; }
            pc = 0x4f5a92; continue;
            }
            0x4f5a92 => {
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x2ce),16));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(7,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4f5bd0; continue; }
            pc = 0x4f5abd; continue;
            }
            0x4f5abd => {
            pc = 0x4f5ac3; continue;
            }
            0x4f5abf => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4f5ac3; continue;
            }
            0x4f5ac3 => {
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x4f5ad9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4dafe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4f5ae1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4f5bb5; continue; }
            pc = 0x4f5af2; continue;
            }
            0x4f5af2 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x7ac),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x80c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0]),8));
            c.set(1,32,0x6dd73cu32);
            { let a = c.get(0,8); let n = 0x7u32; let r = c.shr(a,n,8); c.set(0,8,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4f5b1f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4daf10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x2cc),16));
            c.set(0,16,rm(m,0x6de4a6u32,16));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4f5b75; continue; }
            pc = 0x4f5b3e; continue;
            }
            0x4f5b3e => {
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f5b49);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6),8));
            { let a = c.get(1,8); c.sub(a,rm(m,c.r[2].wrapping_add(0x3),8),8); }
            if c.cond(4) { pc = 0x4f5b69; continue; }
            pc = 0x4f5b55; continue;
            }
            0x4f5b55 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6de4a6u32,16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4f5b3e; continue; }
            pc = 0x4f5b67; continue;
            }
            0x4f5b67 => {
            pc = 0x4f5b75; continue;
            }
            0x4f5b69 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(5,16,rm(m,c.r[0].wrapping_add(0xc),16));
            pc = 0x4f5b75; continue;
            }
            0x4f5b75 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x20),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4f5b95; continue; }
            pc = 0x4f5b87; continue;
            }
            0x4f5b87 => {
            if c.cond(5) { pc = 0x4f5bb1; continue; }
            pc = 0x4f5b89; continue;
            }
            0x4f5b89 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4f5bb1; continue; }
            pc = 0x4f5b95; continue;
            }
            0x4f5b95 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            pc = 0x4f5bb1; continue;
            }
            0x4f5bb1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x4f5bb5; continue;
            }
            0x4f5bb5 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x4f5abf; continue; }
            pc = 0x4f5bcc; continue;
            }
            0x4f5bcc => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4f5bd0; continue;
            }
            0x4f5bd0 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x2d0),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4f5cd2; continue; }
            pc = 0x4f5bea; continue;
            }
            0x4f5bea => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x4f5c04);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4dafe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4f5c0c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4f5cbb; continue; }
            pc = 0x4f5c1b; continue;
            }
            0x4f5c1b => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x7ac),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x80c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0]),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,8); let n = 0x7u32; let r = c.shr(a,n,8); c.set(0,8,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x4f5c4d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4daf10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x3),8));
            c.set(0,32,0x55555556u32);
            { let a = c.get(1,32); let r = c.sub(a,0x15u32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(6,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x2de),16));
            c.set(0,16,rm(m,c.r[2].wrapping_mul(2).wrapping_add(0x672a18),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x20),32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x4f5c9f; continue; }
            pc = 0x4f5c91; continue;
            }
            0x4f5c91 => {
            if c.cond(5) { pc = 0x4f5cbb; continue; }
            pc = 0x4f5c93; continue;
            }
            0x4f5c93 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4f5cbb; continue; }
            pc = 0x4f5c9f; continue;
            }
            0x4f5c9f => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(1,8)); }
            pc = 0x4f5cbb; continue;
            }
            0x4f5cbb => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(3,16,rm(m,c.r[5].wrapping_add(0x4),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4f5bea; continue; }
            pc = 0x4f5cd2; continue;
            }
            0x4f5cd2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x2c8),16));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f5cea);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4f5a5f; continue; }
            pc = 0x4f5cf6; continue;
            }
            0x4f5cf6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(4) { pc = 0x4f5d72; continue; }
            pc = 0x4f5d00; continue;
            }
            0x4f5d00 => {
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m, 0x4f5d09);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,0x6de508u32,32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.push(m, 0x4f5d2e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x28u32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x4f5d68);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4f5d72 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f5d80(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f5d80;
    loop {
        match pc {
            0x4f5d80 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f5d91);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(5,32,c.get(0,32));
            { let a = rm(m,c.r[6].wrapping_add(0x8),16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x4f5fbf; continue; }
            pc = 0x4f5d9f; continue;
            }
            0x4f5d9f => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6dfc90u32);
            c.push(m, 0x4f5daa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54e1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6dd744u32,32));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd748u32,16),16); }
            if c.cond(3) { pc = 0x4f5e00; continue; }
            pc = 0x4f5db9; continue;
            }
            0x4f5db9 => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x4f5dca);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x6),16); c.sub(a,0x1u32,16); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            if c.cond(5) { pc = 0x4f5df7; continue; }
            pc = 0x4f5dd5; continue;
            }
            0x4f5dd5 => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x10),8));
            { let a = c.get(2,16); c.sub(a,c.get(5,16),16); }
            if c.cond(5) { pc = 0x4f5de5; continue; }
            pc = 0x4f5ddf; continue;
            }
            0x4f5ddf => {
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,8,0xffu32); }
            pc = 0x4f5df7; continue;
            }
            0x4f5de5 => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x11),8));
            { let a = c.get(2,16); c.sub(a,c.get(5,16),16); }
            if c.cond(5) { pc = 0x4f5df7; continue; }
            pc = 0x4f5def; continue;
            }
            0x4f5def => {
            { let a = c.r[0].wrapping_add(0x11); wm(m,a,8,0xffu32); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,8,0xffu32); }
            pc = 0x4f5df7; continue;
            }
            0x4f5df7 => {
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd748u32,16),16); }
            if c.cond(2) { pc = 0x4f5db9; continue; }
            pc = 0x4f5e00; continue;
            }
            0x4f5e00 => {
            { let a = c.r[6].wrapping_add(0x53c); wm(m,a,32,0xc3500u32); }
            { let a = c.r[6].wrapping_add(0x540); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x544); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x608); wm(m,a,32,c.get(3,32)); }
            c.set(0,8,rm(m,0x6de4b4u32,8));
            c.set(1,8,0x7u32);
            { let a = c.r[6].wrapping_add(0x546); wm(m,a,8,c.get(0,8)); }
            { let (lo,hi) = c.imul1(c.get(0,8),c.get(1,8),8); c.set(0,16,lo | (hi<<8)); }
            c.push(m,c.get(7,32));
            c.push(m,0x60u32);
            c.set(2,32,c.r[6].wrapping_add(0x548));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x547); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4f5e42);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x60u32);
            c.set(0,32,c.r[6].wrapping_add(0x5a8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f5e51);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4f5e5b);
            { let args = []; c.r[0] = host.call(m, 0x4f72c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x4d8));
            { let a = c.r[6].wrapping_add(0x464); wm(m,a,32,0x40000000u32); }
            c.push(m, 0x4f5e70);
            { let args = []; c.r[0] = host.call(m, 0x534890, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6]; wm(m,a,32,0xf4240u32); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(3,32));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4f5e8d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f68d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x8u32);
            c.set(2,32,c.r[6].wrapping_add(0x38));
            c.push(m,0x5u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x26); wm(m,a,8,0x0u32); }
            c.push(m, 0x4f5e9e);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(5,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd730u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f5eb3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d6780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6de4a4u32,32));
            c.set(1,32,rm(m,0x6ddb70u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,0x6ddb74u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4f5edf);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6ddb74u32,32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4f5ef5);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x4f5f39; continue; }
            pc = 0x4f5f02; continue;
            }
            0x4f5f02 => {
            c.set(0,32,c.get(3,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4f5f14);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6ddb74u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1d),8));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x4f5f2d; continue; }
            pc = 0x4f5f29; continue;
            }
            0x4f5f29 => {
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,8,0x1u32); }
            pc = 0x4f5f2d; continue;
            }
            0x4f5f2d => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x4f5f02; continue; }
            pc = 0x4f5f37; continue;
            }
            0x4f5f37 => {
            pc = 0x4f5f3d; continue;
            }
            0x4f5f39 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4f5f3d; continue;
            }
            0x4f5f3d => {
            c.set(0,32,rm(m,0x6dd888u32,32));
            c.set(2,32,rm(m,0x6de4a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.r[6].wrapping_add(0x1d); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[7]),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x4f5f88; continue; }
            pc = 0x4f5f68; continue;
            }
            0x4f5f68 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1d),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x1d); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x4f5f7d; continue; }
            pc = 0x4f5f79; continue;
            }
            0x4f5f79 => {
            { let a = c.r[6].wrapping_add(0x1d); wm(m,a,8,0x0u32); }
            pc = 0x4f5f7d; continue;
            }
            0x4f5f7d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[7]),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x4f5f68; continue; }
            pc = 0x4f5f88; continue;
            }
            0x4f5f88 => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6dfc90u32);
            c.push(m, 0x4f5f93);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54e170, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x19),8));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x6f1));
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x6f1),8));
            { let a = c.get(1,16); c.sub(a,c.get(5,16),16); }
            if c.cond(5) { pc = 0x4f5fbf; continue; }
            pc = 0x4f5fbc; continue;
            }
            0x4f5fbc => {
            { let a = c.r[0]; wm(m,a,8,0xffu32); }
            pc = 0x4f5fbf; continue;
            }
            0x4f5fbf => {
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

pub fn f_004f6470(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f6470;
    loop {
        match pc {
            0x4f6470 => {
            { let a = rm(m,c.r[1].wrapping_add(0x1a),8); c.and(a,0x1u32,8); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x30),8));
            if c.cond(4) { pc = 0x4f648f; continue; }
            pc = 0x4f6479; continue;
            }
            0x4f6479 => {
            { let a = c.get(0,8); let r = c.or(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.r[1].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x30),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f648e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4f648f => {
            { let a = c.get(0,8); let r = c.and(a,0x7fu32,8); c.set(0,8,r); }
            { let a = c.r[1].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x30),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f64a4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f64b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f64b0;
    loop {
        match pc {
            0x4f64b0 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x19),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4f64cb; continue; }
            pc = 0x4f64c2; continue;
            }
            0x4f64c2 => {
            c.set(6,32,0x1u32);
            { let a = c.get(6,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x4f64cd; continue;
            }
            0x4f64cb => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x4f64cd; continue;
            }
            0x4f64cd => {
            c.set(1,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4f6518; continue; }
            pc = 0x4f64e3; continue;
            }
            0x4f64e3 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f64ee);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x2c6),8));
            c.set(2,32,0x1u32);
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2c8),16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.or(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(2,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4f64e3; continue; }
            pc = 0x4f6518; continue;
            }
            0x4f6518 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.set(7,32,rm(m,0x6dd894u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4f655e; continue; }
            pc = 0x4f652e; continue;
            }
            0x4f652e => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            pc = 0x4f6535; continue;
            }
            0x4f6535 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(0x134),16); c.sub(a,0xcu32,16); }
            if c.cond(3) { pc = 0x4f6553; continue; }
            pc = 0x4f6547; continue;
            }
            0x4f6547 => {
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x39),8));
            c.set(0,32,0x1u32);
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.or(a,c.get(0,32),32); c.set(6,32,r); }
            pc = 0x4f6553; continue;
            }
            0x4f6553 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x4f6535; continue; }
            pc = 0x4f655d; continue;
            }
            0x4f655d => {
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x4f655e; continue;
            }
            0x4f655e => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f69b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f69b0;
    loop {
        match pc {
            0x4f69b0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x118u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(5,32,c.r[0].wrapping_add(0xfffee900));
            c.push(m, 0x4f69cf);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1c),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4f69df);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,16,rm(m,c.r[7].wrapping_add(0x34),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x31),8));
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = rm(m,0x6de4b0u32,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(1,8)); }
            if c.cond(6) { pc = 0x4f6bba; continue; }
            pc = 0x4f69fa; continue;
            }
            0x4f69fa => {
            c.set(2,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[7].wrapping_add(0x28),32),32); }
            if c.cond(6) { pc = 0x4f6bba; continue; }
            pc = 0x4f6a09; continue;
            }
            0x4f6a09 => {
            { let a = c.get(2,32); let r = c.add(a,0xfffee900u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[7].wrapping_add(0x2c),32),32); }
            if c.cond(2) { pc = 0x4f6bba; continue; }
            pc = 0x4f6a18; continue;
            }
            0x4f6a18 => {
            { let a = rm(m,c.r[0],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x4f6a2b; continue; }
            pc = 0x4f6a1d; continue;
            }
            0x4f6a1d => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4f6a1d; continue; }
            pc = 0x4f6a27; continue;
            }
            0x4f6a27 => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(1,8)); }
            pc = 0x4f6a2b; continue;
            }
            0x4f6a2b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,0x3u32);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x36),8));
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4f6bba; continue; }
            pc = 0x4f6a4c; continue;
            }
            0x4f6a4c => {
            c.set(1,32,rm(m,0x6ddb70u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f6a65);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6ddb70u32,32));
            c.set(1,32,0x80u32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x34));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f6a83);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0xbc));
            c.set(0,16,rm(m,0x6ddb72u32,16));
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4f6a9b);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,0x80u32);
            c.set(0,16,rm(m,0x6ddb72u32,16));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(c.r[0]).wrapping_add(0xcc));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f6aba);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,0x6dd892u32,16));
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            { let a = c.get(6,16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x4f6af4; continue; }
            pc = 0x4f6ac8; continue;
            }
            0x4f6ac8 => {
            c.set(1,32,rm(m,0x6dd7a0u32,32));
            pc = 0x4f6ace; continue;
            }
            0x4f6ace => {
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(2) { pc = 0x4f6bba; continue; }
            pc = 0x4f6ae2; continue;
            }
            0x4f6ae2 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(c.r[2]).wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.set(6,16,rm(m,c.r[6],16));
            { let a = c.get(6,16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x4f6ace; continue; }
            pc = 0x4f6af4; continue;
            }
            0x4f6af4 => {
            c.push(m, 0x4f6af9);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x24),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4f6b5a; continue; }
            pc = 0x4f6b10; continue;
            }
            0x4f6b10 => {
            c.set(1,32,rm(m,0x6ddb70u32,32));
            c.set(6,16,rm(m,0x6ddb72u32,16));
            pc = 0x4f6b1d; continue;
            }
            0x4f6b1d => {
            { let a = c.get(2,8); c.sub(a,0x7fu32,8); }
            if c.cond(6) { pc = 0x4f6b32; continue; }
            pc = 0x4f6b22; continue;
            }
            0x4f6b22 => {
            { let a = c.get(0,32); let r = c.and(a,0x7fu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(7,16,c.get(6,16));
            { let b = c.get(7,32); c.idiv32(b); }
            { let a = c.get(2,8); let r = c.sub(a,0x7fu32,8); c.set(2,8,r); }
            pc = 0x4f6b46; continue;
            }
            0x4f6b32 => {
            c.set(0,8,c.get(2,8));
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            pc = 0x4f6b46; continue;
            }
            0x4f6b46 => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x24),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x4f6b1d; continue; }
            pc = 0x4f6b5a; continue;
            }
            0x4f6b5a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x128),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x87u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x4f6b76);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.and(a,0x7fu32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4f6b81);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.and(a,0x7fu32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4f6b8c);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.and(a,0x7fu32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4f6b97);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(0,8); let r = c.and(a,0x7fu32,8); c.set(0,8,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4f6bac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x118u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4f6bba => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x118u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f8a50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f8a50;
    loop {
        match pc {
            0x4f8a50 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2d4),32));
            c.set(6,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(7,32); c.sub(a,0x1u32,32); }
            if c.cond(12) { pc = 0x4f8ae6; continue; }
            pc = 0x4f8a7b; continue;
            }
            0x4f8a7b => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4f8ac7; continue; }
            pc = 0x4f8a83; continue;
            }
            0x4f8a83 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x773),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x771),8));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x4f8ae6; continue; }
            pc = 0x4f8aa8; continue;
            }
            0x4f8aa8 => {
            c.fld(c.ld(m,c.r[5].wrapping_add(0x464),4));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.ld(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x11c),4); c.fop('+',v); }
            c.set(0,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x11c));
            { let v = c.pop(m); c.set(5,32,v); }
            c.fstp32(m,c.r[0]);
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            0x4f8ac7 => {
            c.fld(c.ld(m,c.r[5].wrapping_add(0x464),4));
            { let v = c.ld(m,0x672838u32,4); c.fop('*',v); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let v = c.ld(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x11c),4); c.fop('+',v); }
            c.set(0,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x11c));
            c.fstp32(m,c.r[0]);
            pc = 0x4f8ae6; continue;
            }
            0x4f8ae6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f8c70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f8c70;
    loop {
        match pc {
            0x4f8c70 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.fld(c.ld(m,c.r[1].wrapping_add(0x464),4));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let v = c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x11c),4); c.fop('+',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x11c));
            c.fstp32(m,c.r[0]);
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f9170(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f9170;
    loop {
        match pc {
            0x4f9170 => {
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x4f91b6; continue; }
            pc = 0x4f917a; continue;
            }
            0x4f917a => {
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x11c));
            pc = 0x4f9190; continue;
            }
            0x4f9190 => {
            c.set(6,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.sub(a,0x9f8u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0xcu32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2c8),8); c.and(a,0xc0u32,8); }
            if c.cond(4) { pc = 0x4f91b2; continue; }
            pc = 0x4f91a8; continue;
            }
            0x4f91a8 => {
            c.fld(c.ld(m,c.r[1],4));
            { let v = c.ld(m,0x66bc80u32,4); c.fop('-',v); }
            c.fstp32(m,c.r[1]);
            pc = 0x4f91b2; continue;
            }
            0x4f91b2 => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x4f9190; continue; }
            pc = 0x4f91b5; continue;
            }
            0x4f91b5 => {
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x4f91b6; continue;
            }
            0x4f91b6 => {
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f91c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f91c0;
    loop {
        match pc {
            0x4f91c0 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x4f9215; continue; }
            pc = 0x4f91cc; continue;
            }
            0x4f91cc => {
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            pc = 0x4f91d0; continue;
            }
            0x4f91d0 => {
            c.set(7,32,rm(m,0x6de508u32,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(8)).wrapping_add(0x2c8),8); c.and(a,0xc0u32,8); }
            if c.cond(4) { pc = 0x4f91fa; continue; }
            pc = 0x4f91e8; continue;
            }
            0x4f91e8 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x48));
            c.fld(c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),4));
            { let v = c.ld(m,0x672f28u32,4); c.fop('-',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.fstp32(m,c.r[0]);
            pc = 0x4f91fa; continue;
            }
            0x4f91fa => {
            c.set(6,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x4f91d0; continue; }
            pc = 0x4f9213; continue;
            }
            0x4f9213 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x4f9215; continue;
            }
            0x4f9215 => {
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f9220(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f9220;
    loop {
        match pc {
            0x4f9220 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); c.sub(a,0xf4240u32,32); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x48));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            if c.cond(2) { pc = 0x4f924a; continue; }
            pc = 0x4f923a; continue;
            }
            0x4f923a => {
            c.fld(c.ld(m,c.r[0],4));
            { let v = c.ld(m,0x66dcd8u32,4); c.fop('-',v); }
            c.fstp32(m,c.r[0]);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            0x4f924a => {
            { let a = c.r[4]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.fild64(m,c.r[4]);
            c.fstp32(m,c.r[4].wrapping_add(0xc));
            c.fld(c.ld(m,c.r[4].wrapping_add(0xc),4));
            { let v = c.ld(m,0x672f30u32,4); c.fop('*',v); }
            { let v = c.ld(m,c.r[0],4); c.fop('r',v); }
            c.fstp32(m,c.r[0]);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f99b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f99b0;
    loop {
        match pc {
            0x4f99b0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(6,32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            if c.cond(14) { pc = 0x4f9ac0; continue; }
            pc = 0x4f99d2; continue;
            }
            0x4f99d2 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x4f99d8);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4f99ef; continue; }
            pc = 0x4f99e3; continue;
            }
            0x4f99e3 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4f99ef => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x68),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x4),32),32); }
            c.set(2,32,c.get(0,32));
            if c.cond(13) { pc = 0x4f9a00; continue; }
            pc = 0x4f99fd; continue;
            }
            0x4f99fd => {
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            pc = 0x4f9a00; continue;
            }
            0x4f9a00 => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x8),32),32); }
            if c.cond(14) { pc = 0x4f9a69; continue; }
            pc = 0x4f9a05; continue;
            }
            0x4f9a05 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4f9a9c; continue; }
            pc = 0x4f9a18; continue;
            }
            0x4f9a18 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[3]));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(6) { pc = 0x4f9a2e; continue; }
            pc = 0x4f9a24; continue;
            }
            0x4f9a24 => {
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4f9a24; continue; }
            pc = 0x4f9a2e; continue;
            }
            0x4f9a2e => {
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x4f9a57; continue; }
            pc = 0x4f9a32; continue;
            }
            0x4f9a32 => {
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(0,32,0x20202020u32);
            c.set(1,32,c.get(7,32));
            c.set(7,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(3,32,c.get(1,32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.stos(m,32,true);
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.stos(m,8,true);
            c.set(0,32,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            pc = 0x4f9a57; continue;
            }
            0x4f9a57 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x4f9a18; continue; }
            pc = 0x4f9a67; continue;
            }
            0x4f9a67 => {
            pc = 0x4f9a94; continue;
            }
            0x4f9a69 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0x4),32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4f9a9c; continue; }
            pc = 0x4f9a70; continue;
            }
            0x4f9a70 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[3]));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x4f9a8a; continue; }
            pc = 0x4f9a7c; continue;
            }
            0x4f9a7c => {
            c.set(3,8,rm(m,c.r[6],8));
            { let a = c.r[2]; wm(m,a,8,c.get(3,8)); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x4f9a7c; continue; }
            pc = 0x4f9a86; continue;
            }
            0x4f9a86 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x4f9a8a; continue;
            }
            0x4f9a8a => {
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4f9a70; continue; }
            pc = 0x4f9a94; continue;
            }
            0x4f9a94 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4f9a9c; continue;
            }
            0x4f9a9c => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x4f9add; continue; }
            pc = 0x4f9aa7; continue;
            }
            0x4f9aa7 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(7,32,c.get(2,32));
            c.set(2,32,c.get(1,32));
            c.set(0,32,0x20202020u32);
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.stos(m,32,true);
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.stos(m,8,true);
            pc = 0x4f9add; continue;
            }
            0x4f9ac0 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.set(0,32,c.r[5].wrapping_add(0xd));
            c.set(1,32,0x4u32);
            pc = 0x4f9ad0; continue;
            }
            0x4f9ad0 => {
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,0xffu32); }
            { let a = c.r[0]; wm(m,a,8,0xffu32); }
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x4f9ad0; continue; }
            pc = 0x4f9add; continue;
            }
            0x4f9add => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f9af4; continue; }
            pc = 0x4f9ae4; continue;
            }
            0x4f9ae4 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4f9aea);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0x68); wm(m,a,32,0x0u32); }
            pc = 0x4f9af4; continue;
            }
            0x4f9af4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); let r = c.add(a,0x59u32,32); c.set(5,32,r); }
            c.set(0,32,0x26u32);
            pc = 0x4f9b09; continue;
            }
            0x4f9b09 => {
            { let a = c.get(5,32); let r = c.sub(a,0x2u32,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0xffffffff),8));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x4f9b19; continue; }
            pc = 0x4f9b15; continue;
            }
            0x4f9b15 => {
            { let a = c.r[5].wrapping_add(0xffffffff); wm(m,a,8,0xffu32); }
            pc = 0x4f9b19; continue;
            }
            0x4f9b19 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[5],8));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(15) { pc = 0x4f9b26; continue; }
            pc = 0x4f9b22; continue;
            }
            0x4f9b22 => {
            { let a = c.r[5]; wm(m,a,8,0xffu32); }
            pc = 0x4f9b26; continue;
            }
            0x4f9b26 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x4f9b09; continue; }
            pc = 0x4f9b29; continue;
            }
            0x4f9b29 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004fbba0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4fbba0;
    loop {
        match pc {
            0x4fbba0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x64u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x74),32));
            c.set(0,32,c.r[1].wrapping_add(0xfffffffe));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x78),32));
            c.set(2,32,c.r[0].wrapping_add(0xfffffffc));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[1].wrapping_add(0x2));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[0].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xfffffffbu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x2));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,c.r[1].wrapping_add(0xfffffffe));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[0].wrapping_add(0xfffffffc));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[1].wrapping_add(0x2));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[0].wrapping_add(0x4));
            { let a = c.get(1,32); let r = c.add(a,0xfffffffbu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.add(a,0xau32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,c.r[1].wrapping_add(0xfffffffe));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[0].wrapping_add(0xfffffffc));
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4fbc2b; continue;
            }
            0x4fbc2b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x30),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x50); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x20u32,32); }
            if c.cond(12) { pc = 0x4fbc2b; continue; }
            pc = 0x4fbc44; continue;
            }
            0x4fbc44 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x68),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(6,32)); }
            c.set(0,32,0xfffffffeu32);
            pc = 0x4fbc52; continue;
            }
            0x4fbc52 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x50),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2]),8));
            { let a = c.get(2,8); let r = c.and(a,0x30u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x30u32,8); }
            if c.cond(5) { pc = 0x4fbcb4; continue; }
            pc = 0x4fbc61; continue;
            }
            0x4fbc61 => {
            c.set(2,32,c.r[0].wrapping_add(0x1));
            { let a = c.get(2,32); let r = c.and(a,0x7u32,32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x50),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2]),8));
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0xbu32,8); }
            if c.cond(5) { pc = 0x4fbc8a; continue; }
            pc = 0x4fbc76; continue;
            }
            0x4fbc76 => {
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0x7u32,32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x50),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2]),8));
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0xbu32,8); }
            if c.cond(4) { pc = 0x4fbccf; continue; }
            pc = 0x4fbc8a; continue;
            }
            0x4fbc8a => {
            c.set(2,32,c.r[0].wrapping_add(0x3));
            { let a = c.get(2,32); let r = c.and(a,0x7u32,32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x50),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2]),8));
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0xbu32,8); }
            if c.cond(5) { pc = 0x4fbcb4; continue; }
            pc = 0x4fbc9f; continue;
            }
            0x4fbc9f => {
            c.set(2,32,c.r[0].wrapping_add(0xfffffffc));
            { let a = c.get(2,32); let r = c.and(a,0x7u32,32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x50),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2]),8));
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0xbu32,8); }
            if c.cond(4) { pc = 0x4fbccf; continue; }
            pc = 0x4fbcb4; continue;
            }
            0x4fbcb4 => {
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            if c.cond(12) { pc = 0x4fbc52; continue; }
            pc = 0x4fbcbf; continue;
            }
            0x4fbcbf => {
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(6,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x64u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4fbccf => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(6,32)); }
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,0x4u32); }
            pc = 0x4fbcea; continue;
            }
            0x4fbce6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4fbcea; continue;
            }
            0x4fbcea => {
            c.set(2,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            c.set(3,32,c.get(1,32));
            c.set(6,32,c.get(0,32));
            { let a = c.get(3,32); let r = c.and(a,0x1u32,32); c.set(3,32,r); }
            c.set(5,32,0x5u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x4fbcff; continue;
            }
            0x4fbcff => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[2]),8));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(3,8); let r = c.and(a,0x8bu32,8); c.set(3,8,r); }
            { let a = c.get(3,8); let r = c.or(a,0xbu32,8); c.set(3,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x4fbcff; continue; }
            pc = 0x4fbd22; continue;
            }
            0x4fbd22 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4fbce6; continue; }
            pc = 0x4fbd2d; continue;
            }
            0x4fbd2d => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x78),32));
            c.push(m,0x4u32);
            c.set(1,32,c.get(7,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x3c),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4fbd44);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4fbec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x50),16));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x64u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004fd0d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4fd0d0;
    loop {
        match pc {
            0x4fd0d0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x672f84),8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,8); c.sub(a,0x10u32,8); }
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            if c.cond(12) { pc = 0x4fd0f3; continue; }
            pc = 0x4fd0e7; continue;
            }
            0x4fd0e7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4fd0f3 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(7,32,r); }
            c.set(0,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,c.get(7,32));
            c.set(5,32,sx(rm(m,c.r[0].wrapping_add(0x672f64),8),8)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x672f74),8),8)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x4fd119);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4fd13b; continue; }
            pc = 0x4fd12f; continue;
            }
            0x4fd12f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4fd13b => {
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            c.push(m, 0x4fd14a);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x38u32,32); }
            if c.cond(7) { pc = 0x4fd366; continue; }
            pc = 0x4fd15a; continue;
            }
            0x4fd15a => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x4fd460),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x4fd414)); continue;
            }
            0x4fd169 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x4fd16f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4fd17e => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x2eu32,32); }
            if c.cond(5) { pc = 0x4fd18f; continue; }
            pc = 0x4fd187; continue;
            }
            0x4fd187 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x64),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x65),8));
            pc = 0x4fd195; continue;
            }
            0x4fd18f => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x66),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x67),8));
            pc = 0x4fd195; continue;
            }
            0x4fd195 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(7,32,c.get(1,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x4fd1e9; continue; }
            pc = 0x4fd1a8; continue;
            }
            0x4fd1a8 => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            pc = 0x4fd1ac; continue;
            }
            0x4fd1ac => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(1,32,c.get(7,32));
            if c.cond(6) { pc = 0x4fd1cf; continue; }
            pc = 0x4fd1b8; continue;
            }
            0x4fd1b8 => {
            c.set(3,32,c.get(5,32));
            pc = 0x4fd1ba; continue;
            }
            0x4fd1ba => {
            c.set(5,32,c.get(1,32));
            { let a = c.r[2]; wm(m,a,8,0x1u32); }
            { let a = c.get(5,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x4fd1ba; continue; }
            pc = 0x4fd1cb; continue;
            }
            0x4fd1cb => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4fd1cf; continue;
            }
            0x4fd1cf => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0x1u32,32); c.set(7,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffff));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x4fd1ac; continue; }
            pc = 0x4fd1e9; continue;
            }
            0x4fd1e9 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4fd1f5 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672f84),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(3,32,0x1u32);
            { let a = c.get(3,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.or(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x4fd218; continue; }
            pc = 0x4fd211; continue;
            }
            0x4fd211 => {
            { let a = c.get(3,8); let r = c.or(a,0x80u32,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd218; continue;
            }
            0x4fd218 => {
            c.set(5,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd223 => {
            c.set(5,32,0xeu32);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd23a => {
            c.set(5,32,0x1du32);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd251 => {
            c.set(5,32,0x19u32);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd268 => {
            c.set(5,32,0x1bu32);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd27f => {
            c.set(5,32,0x14u32);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd296 => {
            c.set(5,32,0x1au32);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd2ad => {
            c.set(5,32,0x16u32);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd2c4 => {
            c.set(5,32,0x17u32);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd2db => {
            c.set(5,32,0x15u32);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd2ef => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x672f84),8));
            pc = 0x4fd2f9; continue;
            }
            0x4fd2f9 => {
            c.set(5,32,0x1u32);
            { let a = c.get(5,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.or(a,0xa0u32,32); c.set(5,32,r); }
            c.set(3,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd312 => {
            c.set(5,32,0x18u32);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd326 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672f84),8));
            pc = 0x4fd2f9; continue;
            }
            0x4fd332 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x672f84),8));
            pc = 0x4fd2f9; continue;
            }
            0x4fd33e => {
            c.set(5,32,0xfu32);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd352 => {
            c.set(5,32,0x1cu32);
            c.set(3,32,0x3fu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fd36e; continue;
            }
            0x4fd366 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x4fd36e; continue;
            }
            0x4fd36e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x4fd405; continue; }
            pc = 0x4fd37d; continue;
            }
            0x4fd37d => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(2,8,rm(m,c.r[7],8));
            { let a = c.get(2,32); let r = c.and(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4fd3fa; continue; }
            pc = 0x4fd38c; continue;
            }
            0x4fd38c => {
            c.set(0,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let b = c.get(1,32); c.div32(b); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.and(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x4fd3fa; continue; }
            pc = 0x4fd3b0; continue;
            }
            0x4fd3b0 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x4fd3b4; continue;
            }
            0x4fd3b4 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); c.sub(a,0x0u32,32); }
            c.set(7,32,c.get(1,32));
            c.set(0,32,c.get(2,32));
            if c.cond(6) { pc = 0x4fd3dc; continue; }
            pc = 0x4fd3bf; continue;
            }
            0x4fd3bf => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4fd3c3; continue;
            }
            0x4fd3c3 => {
            c.set(5,32,c.get(0,32));
            { let a = c.r[7]; wm(m,a,8,0x1u32); }
            { let a = c.get(5,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x4fd3c3; continue; }
            pc = 0x4fd3d4; continue;
            }
            0x4fd3d4 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x4fd3dc; continue;
            }
            0x4fd3dc => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0x1u32,32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4fd3b4; continue; }
            pc = 0x4fd3f6; continue;
            }
            0x4fd3f6 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4fd3fa; continue;
            }
            0x4fd3fa => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x4fd37d; continue; }
            pc = 0x4fd405; continue;
            }
            0x4fd405 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
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

pub fn f_004fd5e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4fd5e0;
    loop {
        match pc {
            0x4fd5e0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x4),32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x4fd5f5);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            if c.cond(5) { pc = 0x4fd609; continue; }
            pc = 0x4fd602; continue;
            }
            0x4fd602 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4fd609 => {
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            c.push(m, 0x4fd618);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5f),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x5e),8));
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0x4u32); }
            c.set(7,32,c.get(0,32));
            c.push(m,c.get(5,32));
            pc = 0x4fd63d; continue;
            }
            0x4fd63d => {
            c.set(1,32,c.get(2,32));
            c.set(0,32,c.get(7,32));
            c.set(3,32,0x3u32);
            pc = 0x4fd646; continue;
            }
            0x4fd646 => {
            c.set(5,32,c.get(0,32));
            { let a = c.r[1]; wm(m,a,8,0x1u32); }
            { let a = c.get(5,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x4fd646; continue; }
            pc = 0x4fd657; continue;
            }
            0x4fd657 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0x1u32,32); c.set(7,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4fd63d; continue; }
            pc = 0x4fd671; continue;
            }
            0x4fd671 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(5,32,v); }
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

pub fn f_004fda10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4fda10;
    loop {
        match pc {
            0x4fda10 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4fda28);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5c),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x5d),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x4fdb00; continue; }
            pc = 0x4fda53; continue;
            }
            0x4fda53 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x68),32));
            pc = 0x4fda56; continue;
            }
            0x4fda56 => {
            c.set(2,8,rm(m,c.r[3].wrapping_add(c.r[1]).wrapping_add(0xffffffff),8));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(2,8); c.sub(a,0x91u32,8); }
            if c.cond(4) { pc = 0x4fda69; continue; }
            pc = 0x4fda60; continue;
            }
            0x4fda60 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x4fda56; continue; }
            pc = 0x4fda64; continue;
            }
            0x4fda64 => {
            pc = 0x4fdb00; continue;
            }
            0x4fda69 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(7,32,c.get(1,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[2]));
            c.set(5,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x2));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(5,16)); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0xfffffffe));
            c.set(0,32,c.r[7].wrapping_add(0xfffffffe));
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,16,c.get(1,16)); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[3]),8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fdab2; continue; }
            pc = 0x4fdaa6; continue;
            }
            0x4fdaa6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[0]; wm(m,a,16,c.get(5,16)); }
            pc = 0x4fdbc1; continue;
            }
            0x4fdab2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1e),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[3]),8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fdacb; continue; }
            pc = 0x4fdac1; continue;
            }
            0x4fdac1 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x1e),16));
            pc = 0x4fdbba; continue;
            }
            0x4fdacb => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[3]),8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fdae7; continue; }
            pc = 0x4fdadb; continue;
            }
            0x4fdadb => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[0]; wm(m,a,16,c.get(7,16)); }
            pc = 0x4fdbc1; continue;
            }
            0x4fdae7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x22),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[3]),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,8); c.sub(a,0x11u32,8); }
            if c.cond(4) { pc = 0x4fdbbe; continue; }
            pc = 0x4fdb00; continue;
            }
            0x4fdb00 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4fdc95; continue; }
            pc = 0x4fdb13; continue;
            }
            0x4fdb13 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x68),32));
            pc = 0x4fdb16; continue;
            }
            0x4fdb16 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(c.r[1]),8); c.sub(a,0x91u32,8); }
            if c.cond(4) { pc = 0x4fdb32; continue; }
            pc = 0x4fdb1d; continue;
            }
            0x4fdb1d => {
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4fdb16; continue; }
            pc = 0x4fdb21; continue;
            }
            0x4fdb21 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4fdb27);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4fdb32 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(7,32,c.get(1,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[2]));
            c.set(5,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x2));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(5,16)); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0xfffffffe));
            c.set(0,32,c.r[7].wrapping_add(0xfffffffe));
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,16,c.get(1,16)); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[3]),8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fdb78; continue; }
            pc = 0x4fdb6f; continue;
            }
            0x4fdb6f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[0]; wm(m,a,16,c.get(5,16)); }
            pc = 0x4fdbc1; continue;
            }
            0x4fdb78 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1e),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[3]),8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fdb8e; continue; }
            pc = 0x4fdb87; continue;
            }
            0x4fdb87 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x1e),16));
            pc = 0x4fdbba; continue;
            }
            0x4fdb8e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[3]),8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fdba7; continue; }
            pc = 0x4fdb9e; continue;
            }
            0x4fdb9e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[0]; wm(m,a,16,c.get(7,16)); }
            pc = 0x4fdbc1; continue;
            }
            0x4fdba7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x22),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[3]),8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x4fdc91; continue; }
            pc = 0x4fdbba; continue;
            }
            0x4fdbba => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4fdbbe; continue;
            }
            0x4fdbbe => {
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            pc = 0x4fdbc1; continue;
            }
            0x4fdbc1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x1u32); }
            pc = 0x4fdbcd; continue;
            }
            0x4fdbcd => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[1].wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,c.r[1].wrapping_add(0xffffffff));
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.and(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4fdc09; continue; }
            pc = 0x4fdbf2; continue;
            }
            0x4fdbf2 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(2,16); let r = c.add(a,c.get(1,16),16); c.set(2,16,r); }
            c.set(1,32,c.r[2].wrapping_add(0xffffffff));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,16,c.get(2,16)); }
            pc = 0x4fdc1b; continue;
            }
            0x4fdc09 => {
            { let a = c.get(1,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x8),16),16); c.set(1,16,r); }
            c.set(2,32,c.r[1].wrapping_add(0xffffffff));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,16,c.get(1,16)); }
            pc = 0x4fdc1b; continue;
            }
            0x4fdc1b => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x4fdc1d; continue;
            }
            0x4fdc1d => {
            c.set(7,16,rm(m,c.r[4].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x1c),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(2,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); c.sub(a,0x40u32,32); }
            if c.cond(3) { pc = 0x4fdc6a; continue; }
            pc = 0x4fdc41; continue;
            }
            0x4fdc41 => {
            { let a = c.get(1,32); c.sub(a,0x20u32,32); }
            if c.cond(3) { pc = 0x4fdc55; continue; }
            pc = 0x4fdc46; continue;
            }
            0x4fdc46 => {
            { let a = c.get(1,32); c.sub(a,0x5u32,32); }
            if c.cond(2) { pc = 0x4fdc55; continue; }
            pc = 0x4fdc4b; continue;
            }
            0x4fdc4b => {
            { let a = c.get(1,32); c.sub(a,0x11u32,32); }
            if c.cond(3) { pc = 0x4fdc55; continue; }
            pc = 0x4fdc50; continue;
            }
            0x4fdc50 => {
            { let a = c.get(1,32); c.sub(a,0xbu32,32); }
            if c.cond(5) { pc = 0x4fdc62; continue; }
            pc = 0x4fdc55; continue;
            }
            0x4fdc55 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(7,16)); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x4fdc62; continue;
            }
            0x4fdc62 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],8); let r = c.or(a,0x40u32,8); { let a = c.r[1]; wm(m,a,8,r); } }
            pc = 0x4fdc6a; continue;
            }
            0x4fdc6a => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x4u32,32); }
            if c.cond(2) { pc = 0x4fdc1d; continue; }
            pc = 0x4fdc70; continue;
            }
            0x4fdc70 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x4fdbcd; continue; }
            pc = 0x4fdc80; continue;
            }
            0x4fdc80 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4fdc86);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4fdc91 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4fdc95; continue;
            }
            0x4fdc95 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4fdc9b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
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

pub fn f_004ff2d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ff2d0;
    loop {
        match pc {
            0x4ff2d0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x60u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(6,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x2c6),8));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x15u32);
            c.set(7,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4ff2ff; continue;
            }
            0x4ff2ff => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x44); wm(m,a,16,0x0u32); }
            if c.cond(5) { pc = 0x4ff2ff; continue; }
            pc = 0x4ff309; continue;
            }
            0x4ff309 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0x2f0),16); c.sub(a,c.get(6,16),16); }
            { let a = c.r[5].wrapping_add(0x2d2); wm(m,a,16,c.get(6,16)); }
            if c.cond(6) { pc = 0x4ff32d; continue; }
            pc = 0x4ff31b; continue;
            }
            0x4ff31b => {
            { let a = c.r[5].wrapping_add(0x2ec); wm(m,a,16,c.get(6,16)); }
            { let a = c.r[5].wrapping_add(0x2ee); wm(m,a,16,0x32u32); }
            pc = 0x4ff33d; continue;
            }
            0x4ff32d => {
            { let a = c.r[5].wrapping_add(0x2ec); wm(m,a,16,0x32u32); }
            { let a = c.r[5].wrapping_add(0x2ee); wm(m,a,16,c.get(6,16)); }
            pc = 0x4ff33d; continue;
            }
            0x4ff33d => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x762),16));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(3,32,0xc8u32);
            if c.cond(3) { pc = 0x4ff38e; continue; }
            pc = 0x4ff358; continue;
            }
            0x4ff358 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4ff360);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2),8));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[5].wrapping_add(0x2c4),16),16); }
            if c.cond(5) { pc = 0x4ff37b; continue; }
            pc = 0x4ff36e; continue;
            }
            0x4ff36e => {
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x1eu32,8); }
            if c.cond(5) { pc = 0x4ff37b; continue; }
            pc = 0x4ff374; continue;
            }
            0x4ff374 => {
            { let a = rm(m,c.r[5].wrapping_add(0x2ee),16); let r = c.add(a,c.get(3,16),16); { let a = c.r[5].wrapping_add(0x2ee); wm(m,a,16,r); } }
            pc = 0x4ff37b; continue;
            }
            0x4ff37b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4ff358; continue; }
            pc = 0x4ff38e; continue;
            }
            0x4ff38e => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x2ce),16));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4ff3f6; continue; }
            pc = 0x4ff3a4; continue;
            }
            0x4ff3a4 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4ff3ac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,0x15u32,8); }
            if c.cond(3) { pc = 0x4ff3c5; continue; }
            pc = 0x4ff3b4; continue;
            }
            0x4ff3b4 => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x44),16); let r = c.inc(a,16); { let a = c.r[4].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x44); wm(m,a,16,r); } }
            c.set(1,32,c.r[4].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x44));
            pc = 0x4ff3dc; continue;
            }
            0x4ff3c5 => {
            { let a = c.get(1,8); c.sub(a,0x1eu32,8); }
            if c.cond(5) { pc = 0x4ff3dc; continue; }
            pc = 0x4ff3ca; continue;
            }
            0x4ff3ca => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(6) { pc = 0x4ff3d5; continue; }
            pc = 0x4ff3d1; continue;
            }
            0x4ff3d1 => {
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,8,0x0u32); }
            pc = 0x4ff3d5; continue;
            }
            0x4ff3d5 => {
            { let a = rm(m,c.r[5].wrapping_add(0x2ec),16); let r = c.add(a,c.get(3,16),16); { let a = c.r[5].wrapping_add(0x2ec); wm(m,a,16,r); } }
            pc = 0x4ff3dc; continue;
            }
            0x4ff3dc => {
            { let a = rm(m,c.r[5].wrapping_add(0x2d2),16); let r = c.inc(a,16); { let a = c.r[5].wrapping_add(0x2d2); wm(m,a,16,r); } }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4ff3a4; continue; }
            pc = 0x4ff3f6; continue;
            }
            0x4ff3f6 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,16,rm(m,c.r[5].wrapping_add(0x2ce),16));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4ff563; continue; }
            pc = 0x4ff410; continue;
            }
            0x4ff410 => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4ff418);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,0x67371cu32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,0x1u32);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3),8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(2,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4ff550; continue; }
            pc = 0x4ff43d; continue;
            }
            0x4ff43d => {
            { let a = rm(m,c.r[4].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x44),16); c.sub(a,0x0u32,16); }
            { let a = c.r[6].wrapping_add(0x7); wm(m,a,8,0xffu32); }
            if c.cond(14) { pc = 0x4ff550; continue; }
            pc = 0x4ff44d; continue;
            }
            0x4ff44d => {
            c.set(2,8,rm(m,c.r[6],8));
            c.set(0,8,c.get(2,8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,8); let r = c.add(a,0x6u32,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,8); let r = c.and(a,0x1u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.add(a,c.get(2,8),8); c.set(1,8,r); }
            { let a = c.get(2,8); let r = c.sub(a,0x3u32,8); c.set(2,8,r); }
            c.set(3,8,c.get(1,8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(3,8); let r = c.sub(a,0x2u32,8); c.set(3,8,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,8,c.get(3,8)); }
            c.set(3,8,c.get(0,8));
            { let a = c.get(3,8); let r = c.add(a,0x7u32,8); c.set(3,8,r); }
            { let a = c.get(2,8); let r = c.sub(a,0x2u32,8); c.set(2,8,r); }
            { let a = c.get(1,8); let r = c.add(a,0x3u32,8); c.set(1,8,r); }
            { let a = c.get(0,8); let r = c.sub(a,0x3u32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x23); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x21); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,0x4u32);
            pc = 0x4ff49e; continue;
            }
            0x4ff49e => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x14),8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x15),8));
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(3,32); let r = c.and(a,0x1u32,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x1d); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4ff4cb; continue; }
            pc = 0x4ff4b7; continue;
            }
            0x4ff4b7 => {
            c.set(3,8,c.get(1,8));
            { let a = c.get(3,8); let r = c.and(a,0x1u32,8); c.set(3,8,r); }
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.get(3,8); c.sub(a,0x1u32,8); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x1d); wm(m,a,8,c.get(1,8)); }
            if c.cond(5) { pc = 0x4ff4df; continue; }
            pc = 0x4ff4c7; continue;
            }
            0x4ff4c7 => {
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            pc = 0x4ff4db; continue;
            }
            0x4ff4cb => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x1d); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(1,8); let r = c.and(a,0x1u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4ff4df; continue; }
            pc = 0x4ff4d9; continue;
            }
            0x4ff4d9 => {
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            pc = 0x4ff4db; continue;
            }
            0x4ff4db => {
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            pc = 0x4ff4df; continue;
            }
            0x4ff4df => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(2) { pc = 0x4ff49e; continue; }
            pc = 0x4ff4e5; continue;
            }
            0x4ff4e5 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x776),16));
            c.set(2,32,c.get(3,32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4ff550; continue; }
            pc = 0x4ff4f8; continue;
            }
            0x4ff4f8 => {
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4ff500);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4ff53d; continue; }
            pc = 0x4ff512; continue;
            }
            0x4ff512 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x4ff514; continue;
            }
            0x4ff514 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(0,8); c.sub(a,rm(m,c.r[4].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x1d),8),8); }
            if c.cond(5) { pc = 0x4ff537; continue; }
            pc = 0x4ff51d; continue;
            }
            0x4ff51d => {
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,8); c.sub(a,rm(m,c.r[4].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x1c),8),8); }
            if c.cond(5) { pc = 0x4ff537; continue; }
            pc = 0x4ff525; continue;
            }
            0x4ff525 => {
            c.set(1,8,c.get(3,8));
            c.set(2,32,0xffffu32);
            { let a = c.get(1,8); let r = c.and(a,0x7fu32,8); c.set(1,8,r); }
            { let a = c.r[6].wrapping_add(0x7); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x14u32);
            pc = 0x4ff537; continue;
            }
            0x4ff537 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x14u32,32); }
            if c.cond(2) { pc = 0x4ff514; continue; }
            pc = 0x4ff53d; continue;
            }
            0x4ff53d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.get(1,32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x776),16));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4ff4f8; continue; }
            pc = 0x4ff550; continue;
            }
            0x4ff550 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4ff410; continue; }
            pc = 0x4ff561; continue;
            }
            0x4ff561 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x4ff563; continue;
            }
            0x4ff563 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2d0),16));
            { let a = c.r[5].wrapping_add(0x2d4); wm(m,a,16,c.get(6,16)); }
            { let a = c.r[5].wrapping_add(0x2dc); wm(m,a,16,c.get(6,16)); }
            { let a = c.r[5].wrapping_add(0x2da); wm(m,a,16,c.get(6,16)); }
            { let a = c.r[5].wrapping_add(0x2d8); wm(m,a,16,c.get(6,16)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4ff5ef; continue; }
            pc = 0x4ff595; continue;
            }
            0x4ff595 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4ff59d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x2c4),8));
            c.set(6,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6),8));
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(7) { pc = 0x4ff5b6; continue; }
            pc = 0x4ff5af; continue;
            }
            0x4ff5af => {
            { let a = rm(m,c.r[5].wrapping_add(0x2d4),16); let r = c.inc(a,16); { let a = c.r[5].wrapping_add(0x2d4); wm(m,a,16,r); } }
            pc = 0x4ff5b6; continue;
            }
            0x4ff5b6 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x55555556u32);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3),8));
            { let a = c.get(1,32); let r = c.sub(a,0x15u32,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_mul(2).wrapping_add(0x672a18),16));
            { let a = rm(m,c.r[5].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x2d8),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[5].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x2d8); wm(m,a,16,r); } }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4ff595; continue; }
            pc = 0x4ff5ef; continue;
            }
            0x4ff5ef => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2cc),16));
            c.set(7,32,c.r[5].wrapping_add(0x2cc));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4a6u32,16),16); }
            if c.cond(3) { pc = 0x4ff67c; continue; }
            pc = 0x4ff605; continue;
            }
            0x4ff605 => {
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            pc = 0x4ff608; continue;
            }
            0x4ff608 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ff618);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6),8));
            c.set(0,16,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x44),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(14) { pc = 0x4ff643; continue; }
            pc = 0x4ff629; continue;
            }
            0x4ff629 => {
            c.set(1,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4ff634);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d36e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x6),8));
            { let a = c.r[4].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x44); wm(m,a,16,c.get(3,16)); }
            pc = 0x4ff670; continue;
            }
            0x4ff643 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4ff653);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x50e380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.r[7]; wm(m,a,16,c.get(1,16)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,0xffffu32); }
            c.push(m, 0x4ff670);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4ff670; continue;
            }
            0x4ff670 => {
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4a6u32,16),16); }
            if c.cond(2) { pc = 0x4ff608; continue; }
            pc = 0x4ff67c; continue;
            }
            0x4ff67c => {
            c.set(3,32,0x15u32);
            pc = 0x4ff681; continue;
            }
            0x4ff681 => {
            c.set(5,16,rm(m,c.r[4].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x42),16));
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(5,16); c.and(a,c.get(5,16),16); }
            if c.cond(14) { pc = 0x4ff6d9; continue; }
            pc = 0x4ff68c; continue;
            }
            0x4ff68c => {
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ff696);
            { let args = []; c.r[0] = host.call(m, 0x5306b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,16,c.get(0,16));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m, 0x4ff6aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,sx(c.get(5,16),16)&0xffffffffu32);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(7,32,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2c6),8));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4ff6c5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4d3490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x2cc),16));
            c.set(0,32,c.r[5].wrapping_add(0x2cc));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[0]; wm(m,a,16,c.get(6,16)); }
            pc = 0x4ff6d9; continue;
            }
            0x4ff6d9 => {
            { let a = c.get(3,32); c.sub(a,0x1u32,32); }
            if c.cond(7) { pc = 0x4ff681; continue; }
            pc = 0x4ff6de; continue;
            }
            0x4ff6de => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x60u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004ff780(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ff780;
    loop {
        match pc {
            0x4ff780 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2c6),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2c4),16));
            c.set(3,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ff7bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2f2),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            c.set(5,32,0x18u32);
            if c.cond(3) { pc = 0x4ff815; continue; }
            pc = 0x4ff7d5; continue;
            }
            0x4ff7d5 => {
            c.set(2,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(6,32,c.r[7].wrapping_add(0x64));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xa),8));
            c.set(0,32,0x2fa0be83u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,8,0x32u32);
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,8); let r = c.sub(a,c.get(2,8),8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.add(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x4ff82c; continue;
            }
            0x4ff815 => {
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x64u32); }
            c.set(6,32,c.r[7].wrapping_add(0x64));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x4ff82c; continue;
            }
            0x4ff828 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4ff82c; continue;
            }
            0x4ff82c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2ec),32));
            { let a = c.get(6,32); let r = c.sub(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4ff8db; continue; }
            pc = 0x4ff83e; continue;
            }
            0x4ff83e => {
            c.set(1,32,rm(m,c.r[6],32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x350),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x4ffa15; continue; }
            pc = 0x4ff854; continue;
            }
            0x4ff854 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2c6),8));
            c.set(1,32,0x6de3d8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4ff869);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52ea80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.sub(a,rm(m,c.r[0].wrapping_add(c.r[6]),32),32); c.set(3,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4ffa15; continue; }
            pc = 0x4ff87a; continue;
            }
            0x4ff87a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4ff884; continue; }
            pc = 0x4ff882; continue;
            }
            0x4ff882 => {
            c.set(3,32,c.get(0,32));
            pc = 0x4ff884; continue;
            }
            0x4ff884 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(3,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2c6),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x4ff899);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x1u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[6]),32));
            c.push(m,c.get(5,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x420),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(0x420); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2c6),8));
            c.push(m,c.get(2,32));
            c.push(m, 0x4ff8d6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f8690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4ffa15; continue;
            }
            0x4ff8db => {
            if c.cond(13) { pc = 0x4ffa15; continue; }
            pc = 0x4ff8e1; continue;
            }
            0x4ff8e1 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x350),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x4ffa15; continue; }
            pc = 0x4ff8f7; continue;
            }
            0x4ff8f7 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[3].wrapping_add(c.r[6]),32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2c6),8));
            c.set(1,32,0x6de3d8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4ff913);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x52ec50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4ffa15; continue; }
            pc = 0x4ff91d; continue;
            }
            0x4ff91d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4ff927; continue; }
            pc = 0x4ff925; continue;
            }
            0x4ff925 => {
            c.set(3,32,c.get(0,32));
            pc = 0x4ff927; continue;
            }
            0x4ff927 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[6]),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x4ff934; continue; }
            pc = 0x4ff932; continue;
            }
            0x4ff932 => {
            c.set(3,32,c.get(0,32));
            pc = 0x4ff934; continue;
            }
            0x4ff934 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(3,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2c6),8));
            c.set(1,32,0x6de3d8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4ff949);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(1,32); c.sub(a,0x1000000u32,32); }
            if c.cond(12) { pc = 0x4ff96d; continue; }
            pc = 0x4ff958; continue;
            }
            0x4ff958 => {
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(0,32,r); }
            pc = 0x4ff980; continue;
            }
            0x4ff96d => {
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            pc = 0x4ff980; continue;
            }
            0x4ff980 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4ffa15; continue; }
            pc = 0x4ff990; continue;
            }
            0x4ff990 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x1u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[6]),32));
            c.push(m,c.get(5,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x30));
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            c.fstp64(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[6].wrapping_add(0x3b4),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x38),8); c.fop('*',v); }
            c.fstp64(m,c.r[4].wrapping_add(0x38));
            c.fild32(m,c.r[4].wrapping_add(0x24));
            c.fstp64(m,c.r[4].wrapping_add(0x30));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x30),8));
            { let v = c.ld(m,c.r[4].wrapping_add(0x38),8); c.fop('+',v); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.fstp64(m,c.r[4].wrapping_add(0x38));
            c.fild32(m,c.r[4].wrapping_add(0x30));
            c.fstp64(m,c.r[4].wrapping_add(0x30));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),8));
            { let v = c.ld(m,c.r[4].wrapping_add(0x30),8); c.fop('/',v); }
            c.fstp32(m,c.r[6].wrapping_add(0x3b4));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x420),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(0x420); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2c6),8));
            c.push(m,c.get(2,32));
            c.push(m, 0x4ffa15);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f8840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4ffa15; continue;
            }
            0x4ffa15 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(15) { pc = 0x4ff828; continue; }
            pc = 0x4ffa1d; continue;
            }
            0x4ffa1d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004ffc20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ffc20;
    loop {
        match pc {
            0x4ffc20 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2c4),16));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ffc3d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c6),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(5,32,c.r[6].wrapping_add(0x64));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,0x18u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x4ffc66; continue;
            }
            0x4ffc66 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x5c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(5,32); let r = c.sub(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x4ffd2e; continue; }
            pc = 0x4ffc7c; continue;
            }
            0x4ffc7c => {
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c6),8));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de3d8u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4ffc97);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fild32(m,c.r[4].wrapping_add(0x18));
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,c.r[2].wrapping_mul(4).wrapping_add(0x673a18),4); c.fop('*',v); }
            { let v = c.ld(m,0x6734f8u32,4); c.fop('*',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(11) { pc = 0x4ffd2a; continue; }
            pc = 0x4ffccd; continue;
            }
            0x4ffccd => {
            { let a = rm(m,c.r[3],32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4ffd2a; continue; }
            pc = 0x4ffcd1; continue;
            }
            0x4ffcd1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x420),32));
            c.push(m,0x1u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x420); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4b4),32));
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.r[3].wrapping_add(0x4b4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(2,32)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2c6),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4ffd0c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f8840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[5]),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4ffd22; continue; }
            pc = 0x4ffd17; continue;
            }
            0x4ffd17 => {
            c.set(7,32,c.get(0,32));
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,0x0u32); }
            pc = 0x4ffd27; continue;
            }
            0x4ffd22 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,c.get(0,32)); }
            pc = 0x4ffd27; continue;
            }
            0x4ffd27 => {
            { let a = rm(m,c.r[5],32); let r = c.add(a,c.get(7,32),32); { let a = c.r[5]; wm(m,a,32,r); } }
            pc = 0x4ffd2a; continue;
            }
            0x4ffd2a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4ffd2e; continue;
            }
            0x4ffd2e => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x4ffc66; continue; }
            pc = 0x4ffd36; continue;
            }
            0x4ffd36 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00500ec0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x500ec0;
    loop {
        match pc {
            0x500ec0 => {
            c.set(0,32,c.get(1,32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2f2),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            if c.cond(3) { pc = 0x500f07; continue; }
            pc = 0x500ed6; continue;
            }
            0x500ed6 => {
            c.set(2,32,0x1u32);
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x3b4),32));
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(6,32),32); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(4) { pc = 0x500f07; continue; }
            pc = 0x500ee9; continue;
            }
            0x500ee9 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x354),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x500f07; continue; }
            pc = 0x500ef4; continue;
            }
            0x500ef4 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(14)));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x500f07 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00500f50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x500f50;
    loop {
        match pc {
            0x500f50 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x2c6),8));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            { let a = c.get(0,8); c.sub(a,0x15u32,8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            if c.cond(3) { pc = 0x501355; continue; }
            pc = 0x500f7f; continue;
            }
            0x500f7f => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,0x4u32);
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x2ce),16));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0xffu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x500f99);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x5010bd; continue; }
            pc = 0x500fad; continue;
            }
            0x500fad => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x500fb5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3),8));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x5010ac; continue; }
            pc = 0x500fd0; continue;
            }
            0x500fd0 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            c.set(5,32,c.get(1,32));
            c.set(7,32,c.r[3].wrapping_add(0x7a4));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            pc = 0x500feb; continue;
            }
            0x500feb => {
            c.set(1,8,rm(m,c.r[6],8));
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x501029; continue; }
            pc = 0x500ff8; continue;
            }
            0x500ff8 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x501017; continue; }
            pc = 0x500ffb; continue;
            }
            0x500ffb => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            if c.cond(4) { pc = 0x501010; continue; }
            pc = 0x501001; continue;
            }
            0x501001 => {
            c.set(2,8,c.get(0,8));
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,c.get(1,8),8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.add(a,0x5u32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.sub(a,0x3u32,8); c.set(0,8,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x501033; continue;
            }
            0x501010 => {
            { let a = c.get(0,8); let r = c.sub(a,0x4u32,8); c.set(0,8,r); }
            { let a = c.get(1,8); let r = c.sub(a,0x2u32,8); c.set(1,8,r); }
            pc = 0x501031; continue;
            }
            0x501017 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,c.get(1,8),8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.sub(a,0x5u32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.add(a,0x2u32,8); c.set(0,8,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x501033; continue;
            }
            0x501029 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(0,8); let r = c.add(a,0x4u32,8); c.set(0,8,r); }
            { let a = c.get(1,8); let r = c.add(a,0x2u32,8); c.set(1,8,r); }
            pc = 0x501031; continue;
            }
            0x501031 => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x501033; continue;
            }
            0x501033 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x50103a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fcba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x501081; continue; }
            pc = 0x501043; continue;
            }
            0x501043 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x50106b; continue; }
            pc = 0x501054; continue;
            }
            0x501054 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(0,32,0x1u32);
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,0x4u32);
            pc = 0x501085; continue;
            }
            0x50106b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x4u32);
            pc = 0x501085; continue;
            }
            0x501081 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x501085; continue;
            }
            0x501085 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x4u32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            if c.cond(2) { pc = 0x500feb; continue; }
            pc = 0x501093; continue;
            }
            0x501093 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5010ac; continue; }
            pc = 0x50109f; continue;
            }
            0x50109f => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            pc = 0x5010ac; continue;
            }
            0x5010ac => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x500fad; continue; }
            pc = 0x5010bd; continue;
            }
            0x5010bd => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x50122f; continue; }
            pc = 0x5010c8; continue;
            }
            0x5010c8 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,16,rm(m,c.r[3].wrapping_add(0x768),16));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x50122f; continue; }
            pc = 0x5010e2; continue;
            }
            0x5010e2 => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5010ea);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3),8));
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(0,8); c.sub(a,0x36u32,8); }
            if c.cond(4) { pc = 0x50121e; continue; }
            pc = 0x5010fd; continue;
            }
            0x5010fd => {
            { let a = c.get(0,8); c.sub(a,0x38u32,8); }
            if c.cond(4) { pc = 0x5011a0; continue; }
            pc = 0x501105; continue;
            }
            0x501105 => {
            { let a = c.get(0,8); c.sub(a,0x35u32,8); }
            if c.cond(4) { pc = 0x5011a0; continue; }
            pc = 0x50110d; continue;
            }
            0x50110d => {
            { let a = c.get(0,8); c.sub(a,0x37u32,8); }
            if c.cond(4) { pc = 0x501119; continue; }
            pc = 0x501111; continue;
            }
            0x501111 => {
            { let a = c.get(0,8); c.sub(a,0x34u32,8); }
            if c.cond(5) { pc = 0x50121e; continue; }
            pc = 0x501119; continue;
            }
            0x501119 => {
            c.set(2,8,rm(m,c.r[6],8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(7,32,c.r[3].wrapping_add(0x7a4));
            { let a = c.get(2,8); let r = c.add(a,0x5u32,8); c.set(2,8,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x501130);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fcba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x50124c; continue; }
            pc = 0x50113a; continue;
            }
            0x50113a => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(0,8); let r = c.sub(a,0x8u32,8); c.set(0,8,r); }
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x50114a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fcba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x501268; continue; }
            pc = 0x501152; continue;
            }
            0x501152 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[6],8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,c.get(1,8),8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.sub(a,0x4u32,8); c.set(0,8,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x50116a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fcba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            if c.cond(5) { pc = 0x501288; continue; }
            pc = 0x501175; continue;
            }
            0x501175 => {
            c.set(3,8,rm(m,c.r[6],8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.get(1,8); let r = c.add(a,0x9u32,8); c.set(1,8,r); }
            { let a = c.get(0,8); let r = c.add(a,c.get(3,8),8); c.set(0,8,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x501189);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fcba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x50121a; continue; }
            pc = 0x501191; continue;
            }
            0x501191 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.get(1,8); let r = c.and(a,0x1u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.add(a,c.get(2,8),8); c.set(1,8,r); }
            pc = 0x5012f4; continue;
            }
            0x5011a0 => {
            c.set(0,8,rm(m,c.r[6],8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(7,32,c.r[3].wrapping_add(0x7a4));
            { let a = c.get(0,8); let r = c.add(a,0x3u32,8); c.set(0,8,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5011b6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fcba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5012ab; continue; }
            pc = 0x5011c0; continue;
            }
            0x5011c0 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(1,8); let r = c.sub(a,0x8u32,8); c.set(1,8,r); }
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5011d1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fcba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5012c7; continue; }
            pc = 0x5011d9; continue;
            }
            0x5011d9 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.get(1,8); let r = c.add(a,0x9u32,8); c.set(1,8,r); }
            c.push(m,c.get(1,32));
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); let r = c.add(a,c.get(1,8),8); c.set(0,8,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,8); let r = c.sub(a,0x2u32,8); c.set(0,8,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5011f2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fcba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5012e7; continue; }
            pc = 0x5011fa; continue;
            }
            0x5011fa => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[6],8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,c.get(1,8),8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.sub(a,0x6u32,8); c.set(0,8,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x501212);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fcba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x50130f; continue; }
            pc = 0x50121a; continue;
            }
            0x50121a => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x50121e; continue;
            }
            0x50121e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x5010e2; continue; }
            pc = 0x50122f; continue;
            }
            0x50122f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x501231; continue;
            }
            0x501231 => {
            { let a = rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x10),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x501337; continue; }
            pc = 0x50123c; continue;
            }
            0x50123c => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(2) { pc = 0x501231; continue; }
            pc = 0x501242; continue;
            }
            0x501242 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x50124c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,8); let r = c.add(a,0x5u32,8); c.set(1,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x501268 => {
            c.set(2,8,rm(m,c.r[6],8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,8); let r = c.sub(a,0x8u32,8); c.set(1,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x501288 => {
            c.set(2,8,rm(m,c.r[6],8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.add(a,c.get(2,8),8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.sub(a,0x4u32,8); c.set(0,8,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5012ab => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,8); let r = c.add(a,0x3u32,8); c.set(2,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5012c7 => {
            c.set(0,8,rm(m,c.r[6],8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,8); let r = c.sub(a,0x8u32,8); c.set(2,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5012e7 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(3,8,rm(m,c.r[6],8));
            { let a = c.get(1,8); let r = c.and(a,0x1u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.add(a,c.get(3,8),8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.sub(a,0x2u32,8); c.set(1,8,r); }
            pc = 0x5012f4; continue;
            }
            0x5012f4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,8); let r = c.add(a,0x9u32,8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x50130f => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(3,8,rm(m,c.r[6],8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,8); let r = c.and(a,0x1u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.add(a,c.get(3,8),8); c.set(2,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(2,8); let r = c.sub(a,0x6u32,8); c.set(2,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x501337 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x10),8));
            c.set(0,8,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x11),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x501355 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x2d0),16));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5013ae; continue; }
            pc = 0x50136b; continue;
            }
            0x50136b => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x501377; continue;
            }
            0x501377 => {
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x50137f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x30),8));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x50139b; continue; }
            pc = 0x50138a; continue;
            }
            0x50138a => {
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.r[5]; wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(2,32,c.get(6,32));
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[7]; wm(m,a,8,c.get(1,8)); }
            if c.cond(12) { pc = 0x5013ae; continue; }
            pc = 0x50139b; continue;
            }
            0x50139b => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x501377; continue; }
            pc = 0x5013ae; continue;
            }
            0x5013ae => {
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

pub fn f_0050cfb0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50cfb0;
    loop {
        match pc {
            0x50cfb0 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x50d032; continue; }
            pc = 0x50cfc5; continue;
            }
            0x50cfc5 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x50cfd0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xe),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xf4),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x50d032; continue; }
            pc = 0x50cfe5; continue;
            }
            0x50cfe5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(7,32,0xffffu32);
            pc = 0x50cff0; continue;
            }
            0x50cff0 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x50),16));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x50d00b; continue; }
            pc = 0x50d005; continue;
            }
            0x50d005 => {
            { let a = c.r[0].wrapping_add(0x50); wm(m,a,16,c.get(7,16)); }
            pc = 0x50d019; continue;
            }
            0x50d00b => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x52),16));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x50d019; continue; }
            pc = 0x50d015; continue;
            }
            0x50d015 => {
            { let a = c.r[0].wrapping_add(0x52); wm(m,a,16,c.get(7,16)); }
            pc = 0x50d019; continue;
            }
            0x50d019 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[3]).wrapping_add(0x4),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xf4),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x50cff0; continue; }
            pc = 0x50d030; continue;
            }
            0x50d030 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x50d032; continue;
            }
            0x50d032 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050ffb0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50ffb0;
    loop {
        match pc {
            0x50ffb0 => {
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xe),16));
            { let a = c.get(2,16); c.sub(a,0x1u32,16); }
            if c.cond(3) { pc = 0x50ffbd; continue; }
            pc = 0x50ffba; continue;
            }
            0x50ffba => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x50ffbc; continue;
            }
            0x50ffbc => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x50ffbd => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(2,16); c.sub(a,0x400u32,16); }
            if c.cond(3) { pc = 0x50ffbc; continue; }
            pc = 0x50ffc7; continue;
            }
            0x50ffc7 => {
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,32,0xau32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            p3_core::economy::seh_pending::native(&mut c, m, 0x63c610);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0051ac00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51ac00;
    loop {
        match pc {
            0x51ac00 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,0x8u32);
            c.set(2,8,rm(m,c.r[0],8));
            c.set(6,32,c.r[0].wrapping_add(0x8));
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.r[1].wrapping_add(0x1); wm(m,a,8,c.get(2,8)); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.r[1].wrapping_add(0x2); wm(m,a,16,c.get(2,16)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x4),8));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,8,c.get(2,8)); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x5),8));
            c.set(2,8,0xffu32);
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[1].wrapping_add(0xfffffffd); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[0].wrapping_add(0x5); wm(m,a,8,c.get(2,8)); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.r[1].wrapping_add(0xfffffffe); wm(m,a,16,c.get(2,16)); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0xffffu32); }
            c.push(m, 0x51ac49);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x8u32);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x51ac53);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0051d5f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51d5f0;
    loop {
        match pc {
            0x51d5f0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x998),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x51d66f; continue; }
            pc = 0x51d5ff; continue;
            }
            0x51d5ff => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            c.push(m,c.get(3,32));
            { let a = c.r[2].wrapping_add(c.r[1]).wrapping_add(0x998); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x2e4),32));
            { let a = rm(m,c.r[1].wrapping_add(0x844),16); let r = c.add(a,0xfffbu32,16); { let a = c.r[1].wrapping_add(0x844); wm(m,a,16,r); } }
            { let a = c.get(0,32); let r = c.add(a,0xfu32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[1].wrapping_add(0x2e4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(0x2d4); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,0x14u32);
            c.set(0,32,c.r[1].wrapping_add(0x2e0));
            c.set(7,32,0x3u32);
            pc = 0x51d639; continue;
            }
            0x51d639 => {
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x51d647; continue; }
            pc = 0x51d63f; continue;
            }
            0x51d63f => {
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            pc = 0x51d656; continue;
            }
            0x51d647 => {
            c.set(3,32,0x1u32);
            { let a = c.r[0]; wm(m,a,32,0x1u32); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            pc = 0x51d656; continue;
            }
            0x51d656 => {
            c.set(2,32,rm(m,c.r[0],32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x2d4),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.r[1].wrapping_add(0x2d4); wm(m,a,32,c.get(3,32)); }
            if c.cond(5) { pc = 0x51d639; continue; }
            pc = 0x51d66c; continue;
            }
            0x51d66c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x51d66f; continue;
            }
            0x51d66f => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0051d680(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51d680;
    loop {
        match pc {
            0x51d680 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x2e0),32));
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(0x2e0); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x2e4),32));
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            if c.cond(13) { pc = 0x51d6a8; continue; }
            pc = 0x51d69c; continue;
            }
            0x51d69c => {
            { let a = c.r[1].wrapping_add(0x2e4); wm(m,a,32,0x0u32); }
            pc = 0x51d6b1; continue;
            }
            0x51d6a8 => {
            { let a = c.get(0,32); let r = c.add(a,0xfffffff1u32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(0x2e4); wm(m,a,32,c.get(0,32)); }
            pc = 0x51d6b1; continue;
            }
            0x51d6b1 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x2dc),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x2d8),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x2e4),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(0x2d4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x998),8));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.r[0].wrapping_add(c.r[1]).wrapping_add(0x998); wm(m,a,8,c.get(2,8)); }
            { let a = rm(m,c.r[1].wrapping_add(0x844),16); let r = c.add(a,0x5u32,16); { let a = c.r[1].wrapping_add(0x844); wm(m,a,16,r); } }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0051d6f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51d6f0;
    loop {
        match pc {
            0x51d6f0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x2c8),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x776),16));
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x864),16));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shr(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0x1u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x51d7b9; continue; }
            pc = 0x51d729; continue;
            }
            0x51d729 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x51d731);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x51d742; continue; }
            pc = 0x51d738; continue;
            }
            0x51d738 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x51d742 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x762),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x51d7b9; continue; }
            pc = 0x51d758; continue;
            }
            0x51d758 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x51d760);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),32),32); }
            if c.cond(4) { pc = 0x51d7c6; continue; }
            pc = 0x51d766; continue;
            }
            0x51d766 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x51d7a2; continue; }
            pc = 0x51d76a; continue;
            }
            0x51d76a => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x51d794; continue; }
            pc = 0x51d76e; continue;
            }
            0x51d76e => {
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x31u32,8); }
            if c.cond(7) { pc = 0x51d7a6; continue; }
            pc = 0x51d774; continue;
            }
            0x51d774 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x7ac),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x80c),32));
            { let a = rm(m,c.r[2].wrapping_add(c.r[1]),8); c.sub(a,0x80u32,8); }
            if c.cond(2) { pc = 0x51d7a6; continue; }
            pc = 0x51d794; continue;
            }
            0x51d794 => {
            { let a = c.get(5,32); c.sub(a,0x5u32,32); }
            if c.cond(14) { pc = 0x51d79e; continue; }
            pc = 0x51d799; continue;
            }
            0x51d799 => {
            { let a = c.get(5,32); let r = c.sub(a,0x5u32,32); c.set(5,32,r); }
            pc = 0x51d7a6; continue;
            }
            0x51d79e => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x51d7a6; continue;
            }
            0x51d7a2 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); let r = c.dec(a,32); { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,r); } }
            pc = 0x51d7a6; continue;
            }
            0x51d7a6 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x51d758; continue; }
            pc = 0x51d7b9; continue;
            }
            0x51d7b9 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0x10000000u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x51d7c6 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x51d7d6; continue; }
            pc = 0x51d7ca; continue;
            }
            0x51d7ca => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x51d7d6 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x51d80b; continue; }
            pc = 0x51d7da; continue;
            }
            0x51d7da => {
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x30u32,8); }
            if c.cond(7) { pc = 0x51d800; continue; }
            pc = 0x51d7e0; continue;
            }
            0x51d7e0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x7ac),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x80c),32));
            { let a = rm(m,c.r[2].wrapping_add(c.r[1]),8); c.sub(a,0x80u32,8); }
            if c.cond(3) { pc = 0x51d80b; continue; }
            pc = 0x51d800; continue;
            }
            0x51d800 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x51d80b => {
            { let a = c.get(5,32); c.sub(a,0x5u32,32); }
            if c.cond(12) { pc = 0x51d842; continue; }
            pc = 0x51d810; continue;
            }
            0x51d810 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6),8));
            c.set(0,32,0x66666667u32);
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x51d837);
            { let args = []; c.r[0] = host.call(m, 0x52dff0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x51d842 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x6),8));
            c.set(0,32,c.r[2].wrapping_add(c.r[5]).wrapping_add(0xffffffff));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x51d85d);
            { let args = []; c.r[0] = host.call(m, 0x52dff0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
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

pub fn f_0051e270(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51e270;
    loop {
        match pc {
            0x51e270 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x51e281);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,0x6dd744u32,32));
            c.set(0,8,rm(m,0x6de52au32,8));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x51e371; continue; }
            pc = 0x51e29a; continue;
            }
            0x51e29a => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x766),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x51e371; continue; }
            pc = 0x51e2ae; continue;
            }
            0x51e2ae => {
            c.push(m,0x2u32);
            c.set(1,32,c.r[7].wrapping_add(0x7a4));
            c.push(m, 0x51e2bb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fcfe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x51e371; continue; }
            pc = 0x51e2c3; continue;
            }
            0x51e2c3 => {
            c.set(1,32,rm(m,0x6dd748u32,32));
            c.set(3,8,0x1u32);
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x51e30d; continue; }
            pc = 0x51e2d5; continue;
            }
            0x51e2d5 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x51e2e0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x6),16); c.sub(a,0x2eu32,16); }
            if c.cond(5) { pc = 0x51e2f9; continue; }
            pc = 0x51e2e7; continue;
            }
            0x51e2e7 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xc),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2c0),8));
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x51e2f9; continue; }
            pc = 0x51e2f4; continue;
            }
            0x51e2f4 => {
            { let a = rm(m,c.r[0].wrapping_add(0xd),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x51e371; continue; }
            pc = 0x51e2f9; continue;
            }
            0x51e2f9 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(0,32,rm(m,0x6dd748u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x51e2d5; continue; }
            pc = 0x51e30d; continue;
            }
            0x51e30d => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2c0),8));
            c.set(1,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[4].wrapping_add(0x1a); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x19); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,0x700u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd73cu32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x48u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xffu32); }
            c.push(m, 0x51e362);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d9fc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x51e371);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x51e371; continue;
            }
            0x51e371 => {
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

pub fn f_005226c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5226c0;
    loop {
        match pc {
            0x5226c0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x663232u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x5226e7);
            { let args = []; c.r[0] = host.call(m, 0x503d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x3c),16));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,0x36u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[7].wrapping_add(0x7a4));
            c.push(m, 0x52270b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fa1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x672afau32,8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x672afbu32,8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x672a82u32,8));
            c.set(1,8,rm(m,0x672a83u32,8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x672abeu32,8));
            c.set(1,8,rm(m,0x672abfu32,8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,0x4u32);
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x672b36u32,8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x672abfu32,8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x20));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x522781; continue;
            }
            0x522781 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xffffffe0),32));
            { let a = c.r[0].wrapping_add(0xfffffff0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x522781; continue; }
            pc = 0x52278f; continue;
            }
            0x52278f => {
            c.set(3,32,0x1u32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2c0),8));
            c.push(m,c.get(3,32));
            c.push(m,0x13u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x5227aa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fop('*',v); }
            c.push(m, 0x5227cd);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2c0),8));
            c.push(m,c.get(3,32));
            c.push(m,0x13u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x5227e6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fop('*',v); }
            c.push(m, 0x52280a);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2c0),8));
            c.push(m,0xcu32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x522828);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fop('*',v); }
            c.push(m, 0x522849);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.push(m,c.get(3,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2c0),8));
            c.push(m,0x11u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x522867);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fop('*',v); }
            c.push(m, 0x52288b);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,0x672baeu32,8));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,0x672bafu32,8));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x672beau32,8));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,0x672bebu32,8));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,16,0x36u32); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,0x88888889u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x6f1),8));
            c.set(0,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(2)));
            { let a = c.r[6].wrapping_add(0x3e); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.r[6].wrapping_add(0x42); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x522925);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x5229bf; continue; }
            pc = 0x522932; continue;
            }
            0x522932 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x5229b4; continue; }
            pc = 0x52293c; continue;
            }
            0x52293c => {
            c.set(6,32,c.get(5,32));
            pc = 0x52293e; continue;
            }
            0x52293e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x2),8); c.sub(a,0x36u32,8); }
            if c.cond(5) { pc = 0x52297f; continue; }
            pc = 0x52294b; continue;
            }
            0x52294b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(3,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),8));
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x1),8));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x3),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x2eu32);
            c.push(m, 0x522976);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5928a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(6,32); let r = c.add(a,0xcu32,32); c.set(6,32,r); }
            pc = 0x5229ab; continue;
            }
            0x52297f => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),8));
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x1),8));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,8,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x3),8));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,c.get(1,16)); }
            c.push(m, 0x5229a5);
            { let args = []; c.r[0] = host.call(m, 0x582770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x24u32,32); c.set(6,32,r); }
            pc = 0x5229ab; continue;
            }
            0x5229ab => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x52293e; continue; }
            pc = 0x5229b4; continue;
            }
            0x5229b4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(5,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.push(m,c.get(2,32));
            pc = 0x5229c3; continue;
            }
            0x5229bf => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            pc = 0x5229c3; continue;
            }
            0x5229c3 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x5229ce);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x593800, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5229df);
            { let args = []; c.r[0] = host.call(m, 0x503d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00523d70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x523d70;
    loop {
        match pc {
            0x523d70 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x768),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x776),16),16); }
            if c.cond(3) { pc = 0x524093; continue; }
            pc = 0x523d8d; continue;
            }
            0x523d8d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,0x672af8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x672a80u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x672abcu32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x672b34u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x523e06; continue;
            }
            0x523e06 => {
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x6),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(7) { pc = 0x523e1e; continue; }
            pc = 0x523e0d; continue;
            }
            0x523e0d => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = rm(m,c.r[2],8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x523e1e; continue; }
            pc = 0x523e15; continue;
            }
            0x523e15 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            { let a = rm(m,c.r[2].wrapping_add(0x1),8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x523e3d; continue; }
            pc = 0x523e1e; continue;
            }
            0x523e1e => {
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x776),16),16); }
            if c.cond(3) { pc = 0x524093; continue; }
            pc = 0x523e2f; continue;
            }
            0x523e2f => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            pc = 0x523e06; continue;
            }
            0x523e3d => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x34),8));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x523f64; continue; }
            pc = 0x523e50; continue;
            }
            0x523e50 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2c0),8));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x523e66);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x523f60; continue; }
            pc = 0x523e76; continue;
            }
            0x523e76 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x523ead; continue; }
            pc = 0x523e7a; continue;
            }
            0x523e7a => {
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m,0x13u32);
            c.set(7,32,c.get(5,32));
            c.push(m, 0x523e8a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x500a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(5,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x24));
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x24),4));
            { let v = c.ld(m,0x673a64u32,4); c.fop('*',v); }
            c.push(m, 0x523eab);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            pc = 0x523ead; continue;
            }
            0x523ead => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x523eee; continue; }
            pc = 0x523eb5; continue;
            }
            0x523eb5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0xbu32);
            c.set(5,32,c.get(3,32));
            c.push(m, 0x523ec7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x500a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x24));
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x24),4));
            { let v = c.ld(m,0x673a44u32,4); c.fop('*',v); }
            c.push(m, 0x523ee8);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            pc = 0x523eee; continue;
            }
            0x523eee => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x523f21; continue; }
            pc = 0x523ef6; continue;
            }
            0x523ef6 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,0xcu32);
            c.push(m, 0x523f06);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x500a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.fld(c.ld(m,0x673a48u32,4));
            { let v = c.ld(m,0x66bc6cu32,4); c.fop('*',v); }
            c.push(m, 0x523f17);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            pc = 0x523f21; continue;
            }
            0x523f21 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x523f64; continue; }
            pc = 0x523f29; continue;
            }
            0x523f29 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,0x11u32);
            c.set(3,32,c.get(0,32));
            c.push(m, 0x523f3b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x500a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(3,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x34));
            c.fstp32(m,c.r[4].wrapping_add(0x34));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x34),4));
            { let v = c.ld(m,0x673a5cu32,4); c.fop('*',v); }
            c.push(m, 0x523f5a);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            pc = 0x523f60; continue;
            }
            0x523f60 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x523f64; continue;
            }
            0x523f64 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x50),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            if c.cond(9) { pc = 0x523f75; continue; }
            pc = 0x523f6e; continue;
            }
            0x523f6e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            pc = 0x523f77; continue;
            }
            0x523f75 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x523f77; continue;
            }
            0x523f77 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            if c.cond(9) { pc = 0x523f84; continue; }
            pc = 0x523f81; continue;
            }
            0x523f81 => {
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x523f84; continue;
            }
            0x523f84 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x34),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            if c.cond(9) { pc = 0x523f95; continue; }
            pc = 0x523f92; continue;
            }
            0x523f92 => {
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            pc = 0x523f95; continue;
            }
            0x523f95 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x48),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            if c.cond(9) { pc = 0x523fa6; continue; }
            pc = 0x523fa3; continue;
            }
            0x523fa3 => {
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            pc = 0x523fa6; continue;
            }
            0x523fa6 => {
            c.set(0,32,rm(m,0x672bacu32,32));
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de3d8u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            c.push(m,c.get(0,32));
            c.push(m,0x13u32);
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.push(m, 0x523fcf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0xbu32);
            c.set(1,32,0x6de3d8u32);
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.push(m, 0x523fed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            c.push(m,c.get(1,32));
            c.push(m,0xcu32);
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x524009);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            c.set(1,32,0x6de3d8u32);
            c.push(m,c.get(0,32));
            c.push(m,0x11u32);
            c.push(m, 0x524025);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.push(m,0x6u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x524031);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x529f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(3,8); let r = c.add(a,0xfdu32,8); c.set(3,8,r); }
            { let a = c.r[0].wrapping_add(0x3); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x524089; continue; }
            pc = 0x524048; continue;
            }
            0x524048 => {
            c.set(7,32,rm(m,c.r[1].wrapping_add(0xc324),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x524089; continue; }
            pc = 0x52405a; continue;
            }
            0x52405a => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0xffu32);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x7),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.or(a,0x1ffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x2fu32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x524089);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x590680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x524089; continue;
            }
            0x524089 => {
            { let a = c.r[6].wrapping_add(0x7a4); wm(m,a,32,0x1u32); }
            pc = 0x524093; continue;
            }
            0x524093 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005241e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5241e0;
    loop {
        match pc {
            0x5241e0 => {
            { let a = c.get(4,32); let r = c.sub(a,0xccu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[1].wrapping_add(0x768),16));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x7a8),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x7ac),32));
            c.push(m,c.get(6,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(5,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x524212);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x524228; continue; }
            pc = 0x52421b; continue;
            }
            0x52421b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xccu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x524228 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x524235);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xec),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x32u32,32); }
            if c.cond(5) { pc = 0x524393; continue; }
            pc = 0x524248; continue;
            }
            0x524248 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x5242ca; continue; }
            pc = 0x52425d; continue;
            }
            0x52425d => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x75c),32));
            pc = 0x524263; continue;
            }
            0x524263 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,0x38u32,8); }
            if c.cond(4) { pc = 0x524288; continue; }
            pc = 0x52426c; continue;
            }
            0x52426c => {
            { let a = c.get(1,8); c.sub(a,0x35u32,8); }
            if c.cond(4) { pc = 0x524288; continue; }
            pc = 0x524271; continue;
            }
            0x524271 => {
            { let a = c.get(1,8); c.sub(a,0x37u32,8); }
            if c.cond(4) { pc = 0x52427b; continue; }
            pc = 0x524276; continue;
            }
            0x524276 => {
            { let a = c.get(1,8); c.sub(a,0x34u32,8); }
            if c.cond(5) { pc = 0x5242bd; continue; }
            pc = 0x52427b; continue;
            }
            0x52427b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)),8));
            c.set(2,32,c.r[1].wrapping_add(0xffffffff));
            { let a = c.get(1,32); let r = c.add(a,0x3u32,32); c.set(1,32,r); }
            pc = 0x524293; continue;
            }
            0x524288 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)),8));
            c.set(2,32,c.r[1].wrapping_add(0x1));
            { let a = c.get(1,32); let r = c.add(a,0xfffffffdu32,32); c.set(1,32,r); }
            pc = 0x524293; continue;
            }
            0x524293 => {
            { let a = c.r[4].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,0xfffffffau32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x5242bd; continue;
            }
            0x5242bd => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x4),16));
            c.set(3,32,c.get(1,32));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x524263; continue; }
            pc = 0x5242ca; continue;
            }
            0x5242ca => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x768),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x52437a; continue; }
            pc = 0x5242df; continue;
            }
            0x5242df => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x75c),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x36u32,8); }
            if c.cond(5) { pc = 0x524363; continue; }
            pc = 0x5242f2; continue;
            }
            0x5242f2 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x524363; continue; }
            pc = 0x5242f8; continue;
            }
            0x5242f8 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(2,32,c.r[4].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x1c));
            c.set(3,8,rm(m,c.r[0],8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = rm(m,c.r[2],32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x52435a; continue; }
            pc = 0x524308; continue;
            }
            0x524308 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,32,c.get(2,32));
            c.set(2,32,c.r[4].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x20));
            { let a = rm(m,c.r[2],32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x524356; continue; }
            pc = 0x524317; continue;
            }
            0x524317 => {
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,8,0x1u32);
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,c.get(1,8)); }
            { let a = rm(m,c.r[3].wrapping_add(0x1),8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x524335; continue; }
            pc = 0x52432f; continue;
            }
            0x52432f => {
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0x1); wm(m,a,8,c.get(1,8)); }
            pc = 0x524339; continue;
            }
            0x524335 => {
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            pc = 0x524339; continue;
            }
            0x524339 => {
            c.set(3,32,c.get(6,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(5,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.r[3].wrapping_add(c.r[0]); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x1c),32));
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x20),32));
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,0x3e8u32);
            pc = 0x524356; continue;
            }
            0x524356 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x52435a; continue;
            }
            0x52435a => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x5242f8; continue; }
            pc = 0x52435f; continue;
            }
            0x52435f => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x524363; continue;
            }
            0x524363 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x5242df; continue; }
            pc = 0x52437a; continue;
            }
            0x52437a => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[3].wrapping_add(0x7a4); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xccu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x524393 => {
            { let a = c.get(0,32); c.sub(a,0x33u32,32); }
            if c.cond(7) { pc = 0x524424; continue; }
            pc = 0x52439c; continue;
            }
            0x52439c => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.r[4].wrapping_add(0xe0); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x5244d8; continue; }
            pc = 0x5243b9; continue;
            }
            0x5243b9 => {
            c.set(2,8,0x1u32);
            pc = 0x5243bb; continue;
            }
            0x5243bb => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[3].wrapping_mul(8)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x3),8));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x524402; continue; }
            pc = 0x5243cd; continue;
            }
            0x5243cd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1),8));
            c.set(3,8,rm(m,c.r[1],8));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,c.get(2,8)); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x1),8));
            { let a = c.get(2,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x5243f0; continue; }
            pc = 0x5243ea; continue;
            }
            0x5243ea => {
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0x1); wm(m,a,8,c.get(2,8)); }
            pc = 0x5243f4; continue;
            }
            0x5243f0 => {
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            pc = 0x5243f4; continue;
            }
            0x5243f4 => {
            c.set(3,32,c.get(6,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(5,32),32); c.set(3,32,r); }
            { let a = c.r[3].wrapping_add(c.r[0]); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xe0),32));
            pc = 0x524402; continue;
            }
            0x524402 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x5243bb; continue; }
            pc = 0x524415; continue;
            }
            0x524415 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xccu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x524424 => {
            { let a = c.get(0,32); c.sub(a,0x35u32,32); }
            if c.cond(4) { pc = 0x524432; continue; }
            pc = 0x524429; continue;
            }
            0x524429 => {
            { let a = c.get(0,32); c.sub(a,0x34u32,32); }
            if c.cond(5) { pc = 0x5244d8; continue; }
            pc = 0x524432; continue;
            }
            0x524432 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x5244d8; continue; }
            pc = 0x524447; continue;
            }
            0x524447 => {
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x75c),32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x6),8));
            c.set(1,32,c.r[1].wrapping_add(c.r[3].wrapping_mul(8)));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5244c1; continue; }
            pc = 0x524458; continue;
            }
            0x524458 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x37u32,8); }
            if c.cond(5) { pc = 0x524490; continue; }
            pc = 0x52445f; continue;
            }
            0x52445f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1),8));
            c.set(3,8,rm(m,c.r[1],8));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3]).wrapping_add(0xffffffff));
            pc = 0x524471; continue;
            }
            0x524471 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,0x1u32); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x1),8));
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0x1u32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(5,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x6u32,32); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3]).wrapping_add(0xffffffff));
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,0x1u32); }
            if c.cond(12) { pc = 0x524471; continue; }
            pc = 0x52448e; continue;
            }
            0x52448e => {
            pc = 0x5244c1; continue;
            }
            0x524490 => {
            { let a = c.get(0,8); c.sub(a,0x38u32,8); }
            if c.cond(5) { pc = 0x5244c1; continue; }
            pc = 0x524494; continue;
            }
            0x524494 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1),8));
            c.set(3,8,rm(m,c.r[1],8));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x1));
            pc = 0x5244a6; continue;
            }
            0x5244a6 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,0x1u32); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x1),8));
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0x1u32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(5,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x6u32,32); }
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,0x1u32); }
            if c.cond(12) { pc = 0x5244a6; continue; }
            pc = 0x5244c1; continue;
            }
            0x5244c1 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x524447; continue; }
            pc = 0x5244d8; continue;
            }
            0x5244d8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xccu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00524780(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x524780;
    loop {
        match pc {
            0x524780 => {
            { let a = c.get(4,32); let r = c.sub(a,0x34u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,0xffffu32);
            c.push(m,c.get(7,32));
            c.set(5,32,c.get(1,32));
            { let a = c.r[6]; wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x5); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x1a); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x22); wm(m,a,8,0x1bu32); }
            { let a = c.r[6].wrapping_add(0x23); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x784),16));
            { let a = c.get(1,32); let r = c.and(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(3,8)); }
            if c.cond(3) { pc = 0x52482a; continue; }
            pc = 0x5247e8; continue;
            }
            0x5247e8 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5247f3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,c.get(2,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x2ca),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(0,32));
            c.push(m, 0x52480e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5008c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5248e0; continue; }
            pc = 0x524816; continue;
            }
            0x524816 => {
            c.set(2,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x5247e8; continue; }
            pc = 0x524826; continue;
            }
            0x524826 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x52482a; continue;
            }
            0x52482a => {
            c.push(m,0x2au32);
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x524837);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x764),16));
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x776),16));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5248a6; continue; }
            pc = 0x524850; continue;
            }
            0x524850 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x75c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(8)),8));
            c.set(0,32,c.r[2].wrapping_add(c.r[7].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x524884; continue; }
            pc = 0x52486c; continue;
            }
            0x52486c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x524884; continue; }
            pc = 0x524879; continue;
            }
            0x524879 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x22); wm(m,a,8,c.get(1,8)); }
            pc = 0x524884; continue;
            }
            0x524884 => {
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x15u32,8); }
            if c.cond(3) { pc = 0x524899; continue; }
            pc = 0x52488b; continue;
            }
            0x52488b => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x18),16); let r = c.inc(a,16); { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x18); wm(m,a,16,r); } }
            c.set(0,32,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x18));
            pc = 0x524899; continue;
            }
            0x524899 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x776),16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x524850; continue; }
            pc = 0x5248a6; continue;
            }
            0x5248a6 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x22),8));
            { let a = c.get(0,8); c.sub(a,0x15u32,8); }
            if c.cond(3) { pc = 0x5248d2; continue; }
            pc = 0x5248ad; continue;
            }
            0x5248ad => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0xffffu32);
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x18),16));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[6].wrapping_add(0x1a); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,16,c.get(0,16)); }
            pc = 0x5248d2; continue;
            }
            0x5248d2 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x5248e0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = rm(m,c.r[0].wrapping_add(0x2c8),8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x524913; continue; }
            pc = 0x5248ed; continue;
            }
            0x5248ed => {
            { let a = rm(m,c.r[6].wrapping_add(0x22),8); c.sub(a,0x5u32,8); }
            if c.cond(4) { pc = 0x524913; continue; }
            pc = 0x5248f3; continue;
            }
            0x5248f3 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x7ac),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x80c),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x4c),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[2]),8); c.sub(a,0x80u32,8); }
            if c.cond(3) { pc = 0x524913; continue; }
            pc = 0x524910; continue;
            }
            0x524910 => {
            { let a = c.r[6].wrapping_add(0x5); wm(m,a,8,c.get(3,8)); }
            pc = 0x524913; continue;
            }
            0x524913 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00524c30(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x524c30;
    loop {
        match pc {
            0x524c30 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x768),16));
            { let a = c.r[6].wrapping_add(0x788); wm(m,a,8,0x0u32); }
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x78c); wm(m,a,16,0x1u32); }
            { let a = c.r[6].wrapping_add(0x78e); wm(m,a,16,0x1u32); }
            { let a = c.r[6].wrapping_add(0x786); wm(m,a,16,0x0u32); }
            c.push(m, 0x524c65);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x524b70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x784),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x524cb1; continue; }
            pc = 0x524c7e; continue;
            }
            0x524c7e => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x524c89);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2ce),16));
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x2ca),16));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x524ca3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x524b70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x524c7e; continue; }
            pc = 0x524cb1; continue;
            }
            0x524cb1 => {
            { let a = rm(m,c.r[6].wrapping_add(0x786),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x524cc4; continue; }
            pc = 0x524cbb; continue;
            }
            0x524cbb => {
            { let a = c.r[6].wrapping_add(0x786); wm(m,a,16,0x1u32); }
            pc = 0x524cc4; continue;
            }
            0x524cc4 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00524cd0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x524cd0;
    loop {
        match pc {
            0x524cd0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x524ce0);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x768),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x524d5a; continue; }
            pc = 0x524cf6; continue;
            }
            0x524cf6 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            pc = 0x524cfc; continue;
            }
            0x524cfc => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x524d16; continue; }
            pc = 0x524d03; continue;
            }
            0x524d03 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),16));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x524cfc; continue; }
            pc = 0x524d10; continue;
            }
            0x524d10 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x524d16 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x6f1),8));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x22u32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            { let a = c.r[4].wrapping_add(0xd); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)),8));
            { let a = c.r[4].wrapping_add(0xe); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x1),8));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0xf); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x33u32); }
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,0x0u32); }
            c.push(m, 0x524d5a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x524d5a; continue;
            }
            0x524d5a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00524e20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x524e20;
    loop {
        match pc {
            0x524e20 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x524e31);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2c0),8));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0x22u32); }
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0xffu32); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x524e4a; continue;
            }
            0x524e4a => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x524ef6; continue; }
            pc = 0x524e5b; continue;
            }
            0x524e5b => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x524e66);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x524e7a; continue; }
            pc = 0x524e6d; continue;
            }
            0x524e6d => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2c0),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x524e7e; continue; }
            pc = 0x524e7a; continue;
            }
            0x524e7a => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(3,8)); }
            pc = 0x524e7e; continue;
            }
            0x524e7e => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x524e4a; continue; }
            pc = 0x524e87; continue;
            }
            0x524e87 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x768),16));
            c.set(6,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x524ef6; continue; }
            pc = 0x524e9d; continue;
            }
            0x524e9d => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            pc = 0x524ea3; continue;
            }
            0x524ea3 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(2,8); c.sub(a,0x34u32,8); }
            if c.cond(4) { pc = 0x524ec5; continue; }
            pc = 0x524eac; continue;
            }
            0x524eac => {
            { let a = c.get(2,8); c.sub(a,0x35u32,8); }
            if c.cond(4) { pc = 0x524ec5; continue; }
            pc = 0x524eb1; continue;
            }
            0x524eb1 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),16));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x524ea3; continue; }
            pc = 0x524ebe; continue;
            }
            0x524ebe => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x524ec5 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)),8));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(2,8); let r = c.sub(a,0x3u32,8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x7),8));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x524ef6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x524ef6; continue;
            }
            0x524ef6 => {
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

pub fn f_00524f00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x524f00;
    loop {
        match pc {
            0x524f00 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x768),16));
            c.set(7,32,c.r[6].wrapping_add(0x7a4));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x524f21);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fcfe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x524f2c; continue; }
            pc = 0x524f25; continue;
            }
            0x524f25 => {
            c.set(5,32,0xffu32);
            pc = 0x524f42; continue;
            }
            0x524f2c => {
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x524f35);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fcfe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xfdu32,32); c.set(0,32,r); }
            c.set(5,32,c.get(0,32));
            pc = 0x524f42; continue;
            }
            0x524f42 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x524fb5; continue; }
            pc = 0x524f4f; continue;
            }
            0x524f4f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x2),8));
            c.set(7,32,c.r[1].wrapping_add(c.r[3].wrapping_mul(8)));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(7) { pc = 0x524fa2; continue; }
            pc = 0x524f62; continue;
            }
            0x524f62 => {
            c.set(5,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x37u32,8); }
            if c.cond(5) { pc = 0x524f7e; continue; }
            pc = 0x524f6b; continue;
            }
            0x524f6b => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(2,32); let r = c.sub(a,0x6u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x3u32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x524f93; continue;
            }
            0x524f7e => {
            { let a = c.get(0,8); c.sub(a,0x38u32,8); }
            if c.cond(5) { pc = 0x524fa2; continue; }
            pc = 0x524f82; continue;
            }
            0x524f82 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7],8));
            { let a = c.get(1,32); let r = c.sub(a,0x6u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x3u32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x524f93; continue;
            }
            0x524f93 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x524f9a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x524d60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5250ba; continue; }
            pc = 0x524fa2; continue;
            }
            0x524fa2 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,16,rm(m,c.r[7].wrapping_add(0x4),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x524f4f; continue; }
            pc = 0x524fb5; continue;
            }
            0x524fb5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x768),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x52502a; continue; }
            pc = 0x524fcb; continue;
            }
            0x524fcb => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(7) { pc = 0x525017; continue; }
            pc = 0x524fdd; continue;
            }
            0x524fdd => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x37u32,8); }
            if c.cond(5) { pc = 0x524ff5; continue; }
            pc = 0x524fe4; continue;
            }
            0x524fe4 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7],8));
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x525008; continue;
            }
            0x524ff5 => {
            { let a = c.get(0,8); c.sub(a,0x38u32,8); }
            if c.cond(5) { pc = 0x525017; continue; }
            pc = 0x524ff9; continue;
            }
            0x524ff9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7],8));
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x525008; continue;
            }
            0x525008 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x52500f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x524d60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5250ba; continue; }
            pc = 0x525017; continue;
            }
            0x525017 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x4),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x524fcb; continue; }
            pc = 0x52502a; continue;
            }
            0x52502a => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x768),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5250ba; continue; }
            pc = 0x525040; continue;
            }
            0x525040 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            c.set(1,8,0x32u32);
            pc = 0x525048; continue;
            }
            0x525048 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x2),8));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(7) { pc = 0x525058; continue; }
            pc = 0x525052; continue;
            }
            0x525052 => {
            { let a = rm(m,c.r[3].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x3),8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x52506d; continue; }
            pc = 0x525058; continue;
            }
            0x525058 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x4),16));
            c.set(7,32,c.get(2,32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x525048; continue; }
            pc = 0x525065; continue;
            }
            0x525065 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x52506d => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x525076);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6f1),8));
            c.set(2,8,rm(m,c.r[3].wrapping_add(c.r[7].wrapping_mul(8)),8));
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x1),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x22u32); }
            { let a = c.r[4].wrapping_add(0x1a); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x33u32); }
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,0x0u32); }
            c.push(m, 0x5250ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5250ba; continue;
            }
            0x5250ba => {
            { let v = c.pop(m); c.set(7,32,v); }
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

pub fn f_00525220(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x525220;
    loop {
        match pc {
            0x525220 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x525235);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5252e6; continue; }
            pc = 0x525242; continue;
            }
            0x525242 => {
            { let a = c.r[3]; wm(m,a,16,0x0u32); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x784),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4a8u32,16),16); }
            if c.cond(3) { pc = 0x5252e6; continue; }
            pc = 0x52525b; continue;
            }
            0x52525b => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            pc = 0x52525d; continue;
            }
            0x52525d => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x52526d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x2ce),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x5252d0; continue; }
            pc = 0x525285; continue;
            }
            0x525285 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)),8));
            c.set(7,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x7ac),32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x80c),32));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]),8); c.and(a,0x80u32,8); }
            if c.cond(5) { pc = 0x5252bd; continue; }
            pc = 0x5252ad; continue;
            }
            0x5252ad => {
            { let a = rm(m,c.r[3],16); let r = c.inc(a,16); { let a = c.r[3]; wm(m,a,16,r); } }
            c.set(2,16,rm(m,c.r[3],16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,c.get(1,16)); }
            pc = 0x5252bd; continue;
            }
            0x5252bd => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x4),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x525285; continue; }
            pc = 0x5252d0; continue;
            }
            0x5252d0 => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2ca),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4a8u32,16),16); }
            if c.cond(2) { pc = 0x52525d; continue; }
            pc = 0x5252e4; continue;
            }
            0x5252e4 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x5252e6; continue;
            }
            0x5252e6 => {
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00525390(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x525390;
    loop {
        match pc {
            0x525390 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x764),16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(2,32,c.r[1].wrapping_add(0x764));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x776),16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x5254a0; continue; }
            pc = 0x5253b1; continue;
            }
            0x5253b1 => {
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x75c),32));
            c.set(3,8,0x1fu32);
            pc = 0x5253b9; continue;
            }
            0x5253b9 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(6,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,8,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x5253db; continue; }
            pc = 0x5253ca; continue;
            }
            0x5253ca => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.set(2,32,c.r[6].wrapping_add(0x4));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x5253b9; continue; }
            pc = 0x5253d6; continue;
            }
            0x5253d6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5253db => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.set(3,16,rm(m,c.r[2],16));
            { let a = c.r[2]; wm(m,a,16,c.get(0,16)); }
            c.set(2,8,rm(m,0x672bd3u32,8));
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,c.r[1].wrapping_add(0x762));
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,8,0xffu32); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x776),16));
            c.set(0,16,rm(m,c.r[2],16));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x525434; continue; }
            pc = 0x525409; continue;
            }
            0x525409 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x75c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x30u32,8); }
            if c.cond(3) { pc = 0x525421; continue; }
            pc = 0x52541d; continue;
            }
            0x52541d => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x525428; continue;
            }
            0x525421 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,16); c.sub(a,0x3u32,16); }
            if c.cond(3) { pc = 0x5254a5; continue; }
            pc = 0x525428; continue;
            }
            0x525428 => {
            c.set(2,32,c.r[0].wrapping_add(0x4));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x525409; continue; }
            pc = 0x525434; continue;
            }
            0x525434 => {
            { let a = c.r[2]; wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,16,0xffffu32); }
            c.set(0,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5254a0; continue; }
            pc = 0x525446; continue;
            }
            0x525446 => {
            c.set(7,32,rm(m,c.r[0].wrapping_add(0xc324),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x2c0),8));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5254a0; continue; }
            pc = 0x525458; continue;
            }
            0x525458 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0xffu32);
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x7a3),8));
            c.set(1,32,sx(rm(m,0x672fdfu32,8),8)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[6],8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(0,32));
            c.push(m, 0x525480);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x590680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x6),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6],8));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x5254a0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x5891c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x5254a0; continue;
            }
            0x5254a0 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5254a5 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,16,c.get(1,16)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,16,c.get(3,16)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005259d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5259d0;
    loop {
        match pc {
            0x5259d0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x2cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x48),8));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2c0),8));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x5259f9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(6,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x40),8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x44),8));
            c.set(1,8,c.get(2,8));
            { let a = c.get(1,8); let r = c.add(a,0x6u32,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,c.get(2,8));
            { let a = c.get(1,8); let r = c.and(a,0x1u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.add(a,c.get(0,8),8); c.set(1,8,r); }
            { let a = c.get(0,8); let r = c.sub(a,0x3u32,8); c.set(0,8,r); }
            c.set(3,8,c.get(1,8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(3,8); let r = c.sub(a,0x2u32,8); c.set(3,8,r); }
            c.set(0,8,c.get(2,8));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,8,c.get(3,8)); }
            c.set(3,8,c.get(2,8));
            { let a = c.get(3,8); let r = c.add(a,0x7u32,8); c.set(3,8,r); }
            { let a = c.get(0,8); let r = c.sub(a,0x2u32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(1,8); let r = c.add(a,0x3u32,8); c.set(1,8,r); }
            { let a = c.get(2,8); let r = c.sub(a,0x3u32,8); c.set(2,8,r); }
            c.set(3,32,0x4u32);
            { let a = c.r[4].wrapping_add(0x1a); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x19); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x1d));
            pc = 0x525a56; continue;
            }
            0x525a56 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xfffffff7),8));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xfffffff8),8));
            { let a = c.get(3,8); c.and(a,0x1u32,8); }
            { let a = c.r[1].wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x525a78; continue; }
            pc = 0x525a66; continue;
            }
            0x525a66 => {
            c.set(3,8,c.get(0,8));
            { let a = c.get(3,8); let r = c.and(a,0x1u32,8); c.set(3,8,r); }
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(3,8); c.sub(a,0x1u32,8); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x525a87; continue; }
            pc = 0x525a74; continue;
            }
            0x525a74 => {
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            pc = 0x525a84; continue;
            }
            0x525a78 => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x525a87; continue; }
            pc = 0x525a82; continue;
            }
            0x525a82 => {
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            pc = 0x525a84; continue;
            }
            0x525a84 => {
            { let a = c.r[1].wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            pc = 0x525a87; continue;
            }
            0x525a87 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,16); c.sub(a,0x14u32,16); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(3,32)); }
            if c.cond(2) { pc = 0x525a56; continue; }
            pc = 0x525a99; continue;
            }
            0x525a99 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x525b08; continue; }
            pc = 0x525a9d; continue;
            }
            0x525a9d => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x2ce),16));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x776),16));
            { let a = c.get(7,16); c.sub(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x525b08; continue; }
            pc = 0x525ab4; continue;
            }
            0x525ab4 => {
            c.set(5,32,rm(m,c.r[5].wrapping_add(0x75c),32));
            pc = 0x525aba; continue;
            }
            0x525aba => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x48),8));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x3),8));
            c.set(6,32,c.r[5].wrapping_add(c.r[7].wrapping_mul(8)));
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x4),16));
            if c.cond(5) { pc = 0x525aff; continue; }
            pc = 0x525ad4; continue;
            }
            0x525ad4 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x525ad9; continue;
            }
            0x525ad9 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,8); c.sub(a,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x15),8),8); }
            if c.cond(5) { pc = 0x525af4; continue; }
            pc = 0x525ae8; continue;
            }
            0x525ae8 => {
            c.set(3,8,rm(m,c.r[6],8));
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x14),8),8); }
            if c.cond(4) { pc = 0x525b82; continue; }
            pc = 0x525af4; continue;
            }
            0x525af4 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,16); c.sub(a,0x14u32,16); }
            if c.cond(2) { pc = 0x525ad9; continue; }
            pc = 0x525afb; continue;
            }
            0x525afb => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            pc = 0x525aff; continue;
            }
            0x525aff => {
            { let a = c.get(7,16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x525aba; continue; }
            pc = 0x525b04; continue;
            }
            0x525b04 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x525b08; continue;
            }
            0x525b08 => {
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x762),16));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x776),16));
            { let a = c.get(7,16); c.sub(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x525b76; continue; }
            pc = 0x525b1f; continue;
            }
            0x525b1f => {
            c.set(5,32,rm(m,c.r[5].wrapping_add(0x75c),32));
            pc = 0x525b25; continue;
            }
            0x525b25 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x48),8));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(3,8,rm(m,c.r[5].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x3),8));
            c.set(1,32,c.r[5].wrapping_add(c.r[7].wrapping_mul(8)));
            { let a = c.get(3,8); c.sub(a,c.get(2,8),8); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x4),16));
            if c.cond(5) { pc = 0x525b71; continue; }
            pc = 0x525b3f; continue;
            }
            0x525b3f => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x2),8));
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x4c),8));
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x525b71; continue; }
            pc = 0x525b4a; continue;
            }
            0x525b4a => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x1),8));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x525b4f; continue;
            }
            0x525b4f => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,8); c.sub(a,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x15),8),8); }
            if c.cond(5) { pc = 0x525b66; continue; }
            pc = 0x525b5e; continue;
            }
            0x525b5e => {
            c.set(3,8,rm(m,c.r[1],8));
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x14),8),8); }
            if c.cond(4) { pc = 0x525b82; continue; }
            pc = 0x525b66; continue;
            }
            0x525b66 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,16); c.sub(a,0x14u32,16); }
            if c.cond(2) { pc = 0x525b4f; continue; }
            pc = 0x525b6d; continue;
            }
            0x525b6d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            pc = 0x525b71; continue;
            }
            0x525b71 => {
            { let a = c.get(7,16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x525b25; continue; }
            pc = 0x525b76; continue;
            }
            0x525b76 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x525b82 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00525ca0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x525ca0;
    loop {
        match pc {
            0x525ca0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1u32);
            c.set(7,32,c.r[6].wrapping_add(0x7a4));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x525cb3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fcfe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x525cbc; continue; }
            pc = 0x525cb7; continue;
            }
            0x525cb7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x525cbc => {
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x525cc5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fcfe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x525cde; continue; }
            pc = 0x525cc9; continue;
            }
            0x525cc9 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x766),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x776),16),16); }
            if c.cond(3) { pc = 0x525cde; continue; }
            pc = 0x525cd9; continue;
            }
            0x525cd9 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x525cde => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2d4),32));
            c.set(0,32,0x68db8badu32);
            { let a = c.get(1,32); let r = c.add(a,0x1388u32,32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xcu32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x770),8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            c.set(0,8,c.get(1,8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00525d10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x525d10;
    loop {
        match pc {
            0x525d10 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1u32);
            c.set(7,32,c.r[6].wrapping_add(0x7a4));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x525d23);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fcfe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x525d2c; continue; }
            pc = 0x525d27; continue;
            }
            0x525d27 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x525d2c => {
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x525d35);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fcfe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x525d4e; continue; }
            pc = 0x525d39; continue;
            }
            0x525d39 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x766),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x776),16),16); }
            if c.cond(3) { pc = 0x525d4e; continue; }
            pc = 0x525d49; continue;
            }
            0x525d49 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x525d4e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2d4),32));
            c.set(0,32,0x68db8badu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xcu32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x772),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,8,u32::from(c.cond(13)));
            c.set(0,8,c.get(1,8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00526b00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x526b00;
    loop {
        match pc {
            0x526b00 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,16,rm(m,c.r[6],16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x526b3c; continue; }
            pc = 0x526b11; continue;
            }
            0x526b11 => {
            c.set(0,32,c.get(2,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(7,16,rm(m,c.r[0],16));
            { let a = c.r[6]; wm(m,a,16,c.get(7,16)); }
            c.set(6,16,rm(m,c.r[1].wrapping_add(0x82e),16));
            { let a = c.r[0]; wm(m,a,16,c.get(6,16)); }
            { let a = c.r[1].wrapping_add(0x82e); wm(m,a,16,c.get(2,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x526b3c; continue;
            }
            0x526b3c => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00529660(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x529660;
    loop {
        match pc {
            0x529660 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x6f1),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x529698; continue; }
            pc = 0x529674; continue;
            }
            0x529674 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x2c0),8));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x52968d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x529694);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f9580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x529698 => {
            { let a = c.get(1,32); let r = c.or(a,0xffffffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x6737b4),8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00529e20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x529e20;
    loop {
        match pc {
            0x529e20 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            if c.cond(3) { pc = 0x529ee9; continue; }
            pc = 0x529e35; continue;
            }
            0x529e35 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,8); c.sub(a,0x4u32,8); }
            if c.cond(3) { pc = 0x529ee9; continue; }
            pc = 0x529e42; continue;
            }
            0x529e42 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x529e52);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x39c),8));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); c.sub(a,0x7u32,32); }
            if c.cond(7) { pc = 0x529ea4; continue; }
            pc = 0x529e67; continue;
            }
            0x529e67 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x529ef4)); continue;
            }
            0x529e6e => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0x0u32); }
            pc = 0x529ea4; continue;
            }
            0x529e75 => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0x1u32); }
            pc = 0x529ea4; continue;
            }
            0x529e7c => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0x2u32); }
            pc = 0x529ea4; continue;
            }
            0x529e83 => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0x3u32); }
            pc = 0x529ea4; continue;
            }
            0x529e8a => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0x5u32); }
            pc = 0x529ea4; continue;
            }
            0x529e91 => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0x7u32); }
            pc = 0x529ea4; continue;
            }
            0x529e98 => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0xau32); }
            pc = 0x529ea4; continue;
            }
            0x529e9f => {
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0xfu32); }
            pc = 0x529ea4; continue;
            }
            0x529ea4 => {
            c.push(m, 0x529ea9);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0xbu32);
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x6dc),8));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x529eee; continue; }
            pc = 0x529ee2; continue;
            }
            0x529ee2 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x529ee9 => {
            c.set(0,32,0x80000000u32);
            pc = 0x529eee; continue;
            }
            0x529eee => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0052a150(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x52a150;
    loop {
        match pc {
            0x52a150 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x4),8));
            { let a = c.get(0,8); c.sub(a,0x14u32,8); }
            if c.cond(6) { pc = 0x52a162; continue; }
            pc = 0x52a158; continue;
            }
            0x52a158 => {
            { let a = c.r[1].wrapping_add(0x6f8); wm(m,a,8,0xau32); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x52a162 => {
            { let a = c.r[1].wrapping_add(0x6f8); wm(m,a,8,c.get(0,8)); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0052de10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x52de10;
    loop {
        match pc {
            0x52de10 => {
            c.push(m,0x18u32);
            c.push(m, 0x52de17);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x52de24; continue; }
            pc = 0x52de22; continue;
            }
            0x52de22 => {
            c.set(1,32,c.get(0,32));
            pc = 0x52de24; continue;
            }
            0x52de24 => {
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x52de34; continue; }
            pc = 0x52de2e; continue;
            }
            0x52de2e => {
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x52de34 => {
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0052de40(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x52de40;
    loop {
        match pc {
            0x52de40 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x52de62; continue; }
            pc = 0x52de48; continue;
            }
            0x52de48 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            pc = 0x52de62; continue;
            }
            0x52de62 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0052ea80(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x52ea80;
    loop {
        match pc {
            0x52ea80 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x76c),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,c.r[6].wrapping_add(0x4f));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.and(a,0x100u32,32); }
            if c.cond(5) { pc = 0x52eac7; continue; }
            pc = 0x52eab1; continue;
            }
            0x52eab1 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); let r = c.and(a,0x7fffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            0x52eac7 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x672c14),8));
            c.fild32(m,c.r[4].wrapping_add(0x14));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x52eaf2; continue; }
            pc = 0x52ead5; continue;
            }
            0x52ead5 => {
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x673a80u32,4); c.fop('-',v); }
            c.fld(c.ld(m,c.r[6].wrapping_mul(4).wrapping_add(0x673a18),4));
            { let v = c.ld(m,0x66a91cu32,4); c.fop('*',v); }
            pc = 0x52eb0d; continue;
            }
            0x52eaf2 => {
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x673a80u32,4); c.fop('-',v); }
            c.fld(c.ld(m,c.r[6].wrapping_mul(4).wrapping_add(0x673a18),4));
            { let v = c.ld(m,0x66a918u32,4); c.fop('*',v); }
            pc = 0x52eb0d; continue;
            }
            0x52eb0d => {
            c.fopp('/');
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x673a14u32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x41u32,8); }
            if c.cond(10) { pc = 0x52eb2e; continue; }
            pc = 0x52eb24; continue;
            }
            0x52eb24 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,0x7fffffffu32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            0x52eb2e => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x673a10u32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(10) { pc = 0x52eb7b; continue; }
            pc = 0x52eb3f; continue;
            }
            0x52eb3f => {
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,0x673a10u32,4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('-',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x10),4); c.fop('*',v); }
            c.fld(c.ld(m,0x673a10u32,4));
            { let v = c.ld(m,0x673a14u32,4); c.fop('-',v); }
            c.fopp('/');
            c.push(m, 0x52eb74);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            0x52eb7b => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x673a0cu32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(10) { pc = 0x52ebc8; continue; }
            pc = 0x52eb8c; continue;
            }
            0x52eb8c => {
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,0x673a0cu32,4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('-',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x10),4); c.fop('*',v); }
            c.fld(c.ld(m,0x673a0cu32,4));
            { let v = c.ld(m,0x673a10u32,4); c.fop('-',v); }
            c.fopp('/');
            c.push(m, 0x52ebc1);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            0x52ebc8 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x673a08u32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(10) { pc = 0x52ec14; continue; }
            pc = 0x52ebd9; continue;
            }
            0x52ebd9 => {
            c.set(6,32,rm(m,c.r[2],32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,0x673a08u32,4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('-',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x10),4); c.fop('*',v); }
            c.fld(c.ld(m,0x673a08u32,4));
            { let v = c.ld(m,0x673a0cu32,4); c.fop('-',v); }
            c.fopp('/');
            c.push(m, 0x52ec0d);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            0x52ec14 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,c.r[1].wrapping_add(0x64),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(10) { pc = 0x52ec48; continue; }
            pc = 0x52ec22; continue;
            }
            0x52ec22 => {
            c.fld(c.ld(m,c.r[1].wrapping_add(0x64),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('-',v); }
            c.fild32(m,c.r[2]);
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.fld(c.ld(m,c.r[1].wrapping_add(0x64),4));
            { let v = c.ld(m,0x673a08u32,4); c.fop('-',v); }
            c.fopp('/');
            c.push(m, 0x52ec43);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            0x52ec48 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0052ec50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x52ec50;
    loop {
        match pc {
            0x52ec50 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,c.r[6].wrapping_add(0x4f));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x76c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(6,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x52ec8b; continue; }
            pc = 0x52ec80; continue;
            }
            0x52ec80 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); c.fpu_exit(); return c.r;
            }
            0x52ec8b => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x672c14),8));
            c.fild32(m,c.r[4].wrapping_add(0x18));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x52ecb6; continue; }
            pc = 0x52ec99; continue;
            }
            0x52ec99 => {
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x673a80u32,4); c.fop('+',v); }
            c.fld(c.ld(m,c.r[6].wrapping_mul(4).wrapping_add(0x673a18),4));
            { let v = c.ld(m,0x66a91cu32,4); c.fop('*',v); }
            pc = 0x52ecd1; continue;
            }
            0x52ecb6 => {
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x673a80u32,4); c.fop('+',v); }
            c.fld(c.ld(m,c.r[6].wrapping_mul(4).wrapping_add(0x673a18),4));
            { let v = c.ld(m,0x66a918u32,4); c.fop('*',v); }
            pc = 0x52ecd1; continue;
            }
            0x52ecd1 => {
            c.fopp('/');
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            if c.cond(6) { pc = 0x52ed19; continue; }
            pc = 0x52ecdf; continue;
            }
            0x52ecdf => {
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.fild64(m,c.r[4].wrapping_add(0x4));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,0x673a84u32,4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('-',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,0x673a84u32,4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('/',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('*',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            pc = 0x52ed19; continue;
            }
            0x52ed19 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x6f0),8));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x52ed33; continue; }
            pc = 0x52ed25; continue;
            }
            0x52ed25 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x673a7cu32,4); c.fop('*',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            pc = 0x52ed33; continue;
            }
            0x52ed33 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x6739f8u32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x41u32,8); }
            if c.cond(10) { pc = 0x52ed50; continue; }
            pc = 0x52ed44; continue;
            }
            0x52ed44 => {
            c.set(0,32,0x7fffffffu32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); c.fpu_exit(); return c.r;
            }
            0x52ed50 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x6739f4u32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(10) { pc = 0x52ed9f; continue; }
            pc = 0x52ed61; continue;
            }
            0x52ed61 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,0x6739f4u32,4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('-',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.fld(c.ld(m,0x6739f4u32,4));
            { let v = c.ld(m,0x6739f8u32,4); c.fop('-',v); }
            c.fopp('/');
            c.push(m, 0x52ed96);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); c.fpu_exit(); return c.r;
            }
            0x52ed9f => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x6739f0u32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(10) { pc = 0x52edee; continue; }
            pc = 0x52edb0; continue;
            }
            0x52edb0 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,0x6739f0u32,4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('-',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.fld(c.ld(m,0x6739f0u32,4));
            { let v = c.ld(m,0x6739f4u32,4); c.fop('-',v); }
            c.fopp('/');
            c.push(m, 0x52ede5);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); c.fpu_exit(); return c.r;
            }
            0x52edee => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x6739ecu32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(10) { pc = 0x52ee3c; continue; }
            pc = 0x52edff; continue;
            }
            0x52edff => {
            c.set(6,32,rm(m,c.r[1],32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,0x6739ecu32,4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('-',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.fld(c.ld(m,0x6739ecu32,4));
            { let v = c.ld(m,0x6739f0u32,4); c.fop('-',v); }
            c.fopp('/');
            c.push(m, 0x52ee33);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); c.fpu_exit(); return c.r;
            }
            0x52ee3c => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x6739e8u32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(10) { pc = 0x52ee7b; continue; }
            pc = 0x52ee4d; continue;
            }
            0x52ee4d => {
            c.fld(c.ld(m,0x6739e8u32,4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('-',v); }
            c.fild32(m,c.r[1]);
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('*',v); }
            c.fld(c.ld(m,0x6739e8u32,4));
            { let v = c.ld(m,0x6739ecu32,4); c.fop('-',v); }
            c.fopp('/');
            c.push(m, 0x52ee74);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); c.fpu_exit(); return c.r;
            }
            0x52ee7b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0052ee90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x52ee90;
    loop {
        match pc {
            0x52ee90 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(1,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(6,32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(6,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,c.r[1].wrapping_add(c.r[6].wrapping_mul(8)));
            c.set(6,32,c.get(0,32));
            c.set(3,32,c.r[0].wrapping_add(0x4f));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(c.r[2]).wrapping_add(0x4f4),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[3].wrapping_add(c.r[2]),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(6,32); let r = c.add(a,c.get(7,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x52eeee; continue; }
            pc = 0x52eee5; continue;
            }
            0x52eee5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x52eeee => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x310),32));
            { let a = c.get(2,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc4),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x64),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x20),32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x670),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32),32); c.set(2,32,r); }
            if c.cond(9) { pc = 0x52ef22; continue; }
            pc = 0x52ef20; continue;
            }
            0x52ef20 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x52ef22; continue;
            }
            0x52ef22 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x4f4),32));
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x52ef3a; continue; }
            pc = 0x52ef31; continue;
            }
            0x52ef31 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x52ef3a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[3].wrapping_add(c.r[0]),32));
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(3,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            if c.cond(12) { pc = 0x52f029; continue; }
            pc = 0x52ef65; continue;
            }
            0x52ef65 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            if c.cond(8) { pc = 0x52ef8c; continue; }
            pc = 0x52ef77; continue;
            }
            0x52ef77 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x52ef88; continue; }
            pc = 0x52ef7f; continue;
            }
            0x52ef7f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x52ef88 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x52ef8c; continue;
            }
            0x52ef8c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x52efcc; continue; }
            pc = 0x52ef9a; continue;
            }
            0x52ef9a => {
            c.set(1,32,c.get(5,32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x52efc2);
            { let r = (((m.r32(c.r[4].wrapping_add(4)) as u64) | ((m.r32(c.r[4].wrapping_add(8)) as u64) << 32)) as i64).checked_div(((m.r32(c.r[4].wrapping_add(12)) as u64) | ((m.r32(c.r[4].wrapping_add(16)) as u64) << 32)) as i64).expect("división por cero del x86"); c.r[0] = r as u32; c.r[2] = (r >> 32) as u32; }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(6,32),32); c.set(7,32,r); }
            pc = 0x52f002; continue;
            }
            0x52efcc => {
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(5,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x52effc);
            { let r = (((m.r32(c.r[4].wrapping_add(4)) as u64) | ((m.r32(c.r[4].wrapping_add(8)) as u64) << 32)) as i64).checked_div(((m.r32(c.r[4].wrapping_add(12)) as u64) | ((m.r32(c.r[4].wrapping_add(16)) as u64) << 32)) as i64).expect("división por cero del x86"); c.r[0] = r as u32; c.r[2] = (r >> 32) as u32; }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.get(0,32));
            pc = 0x52f002; continue;
            }
            0x52f002 => {
            c.set(0,32,c.get(1,32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(7,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(6,32,c.get(2,32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x52f025; continue; }
            pc = 0x52f00e; continue;
            }
            0x52f00e => {
            if c.cond(15) { pc = 0x52f014; continue; }
            pc = 0x52f010; continue;
            }
            0x52f010 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x52f025; continue; }
            pc = 0x52f014; continue;
            }
            0x52f014 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[7]));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x52f021);
            { let r = (((m.r32(c.r[4].wrapping_add(4)) as u64) | ((m.r32(c.r[4].wrapping_add(8)) as u64) << 32)) as i64).checked_div(((m.r32(c.r[4].wrapping_add(12)) as u64) | ((m.r32(c.r[4].wrapping_add(16)) as u64) << 32)) as i64).expect("división por cero del x86"); c.r[0] = r as u32; c.r[2] = (r >> 32) as u32; }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = rm(m,c.r[4].wrapping_add(0x14),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,r); } }
            pc = 0x52f025; continue;
            }
            0x52f025 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x52f029; continue;
            }
            0x52f029 => {
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

pub fn f_005312e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5312e0;
    loop {
        match pc {
            0x5312e0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(7,32));
            c.push(m,0x60u32);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.set(3,32,c.get(1,32));
            c.push(m, 0x5312f7);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,0x60u32);
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x531305);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,0x60u32);
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x531313);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            if c.cond(6) { pc = 0x531458; continue; }
            pc = 0x531327; continue;
            }
            0x531327 => {
            c.set(0,32,c.get(7,32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(5,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x53133f; continue;
            }
            0x53133b => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x53133f; continue;
            }
            0x53133f => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x68),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x18u32); }
            c.set(1,32,c.r[2].wrapping_add(0x64));
            pc = 0x53135f; continue;
            }
            0x53135b => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x53135f; continue;
            }
            0x53135f => {
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x2ac),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,rm(m,c.r[1],32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xfffffffc),32); let r = c.add(a,c.get(7,32),32); { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x5c),32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(6,32),32); c.set(7,32,r); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[0].wrapping_add(0xfffffffc); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x428),32));
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xfffffffc),32); let r = c.add(a,c.get(7,32),32); { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            if c.cond(5) { pc = 0x53135b; continue; }
            pc = 0x531395; continue;
            }
            0x531395 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x784),16));
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x8),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x5313fd; continue; }
            pc = 0x5313a8; continue;
            }
            0x5313a8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(7,32,c.get(5,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            pc = 0x5313b0; continue;
            }
            0x5313b0 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x74),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,0x18u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,c.r[6].wrapping_add(0x64));
            pc = 0x5313ce; continue;
            }
            0x5313ce => {
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x60),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[0]),32); let r = c.add(a,c.get(5,32),32); { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,32,r); } }
            c.set(5,32,rm(m,c.r[1].wrapping_add(0xfffffffc),32));
            { let a = rm(m,c.r[0],32); let r = c.add(a,c.get(5,32),32); { let a = c.r[0]; wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x5313ce; continue; }
            pc = 0x5313e2; continue;
            }
            0x5313e2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2ca),16));
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x8),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x5313b0; continue; }
            pc = 0x5313f5; continue;
            }
            0x5313f5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x5313fd; continue;
            }
            0x5313fd => {
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x9b4),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x531436; continue; }
            pc = 0x531407; continue;
            }
            0x531407 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x10),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c2c),8));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = rm(m,c.r[5].wrapping_add(c.r[7].wrapping_mul(4)),32); let r = c.add(a,c.get(2,32),32); { let a = c.r[5].wrapping_add(c.r[7].wrapping_mul(4)); wm(m,a,32,r); } }
            c.push(m, 0x531424);
            { let args = []; c.r[0] = host.call(m, 0x50ffb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[7].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            c.set(6,32,rm(m,c.r[6],32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x531407; continue; }
            pc = 0x531436; continue;
            }
            0x531436 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x9f8u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(2) { pc = 0x53133b; continue; }
            pc = 0x531458; continue;
            }
            0x531458 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00532420(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x532420;
    loop {
        match pc {
            0x532420 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x532480; continue; }
            pc = 0x532437; continue;
            }
            0x532437 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            pc = 0x532445; continue;
            }
            0x532445 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x78),32));
            { let a = c.get(1,32); let r = c.sub(a,0x650u32,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x8),16));
            c.set(2,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x8));
            { let a = c.get(4,8); c.and(a,c.get(4,8),8); }
            if c.cond(9) { pc = 0x53246f; continue; }
            pc = 0x53245b; continue;
            }
            0x53245b => {
            { let a = c.get(0,32); let r = c.and(a,0x7fffu32,32); c.set(0,32,r); }
            { let a = c.r[2]; wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x78),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x8),8); let r = c.or(a,0x1u32,8); { let a = c.r[0].wrapping_add(c.r[1]).wrapping_add(0x8); wm(m,a,8,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x8));
            pc = 0x53246f; continue;
            }
            0x53246f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x78),32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1]).wrapping_add(0x464); wm(m,a,32,0x3f800000u32); }
            if c.cond(5) { pc = 0x532445; continue; }
            pc = 0x532480; continue;
            }
            0x532480 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x53251a; continue; }
            pc = 0x532490; continue;
            }
            0x532490 => {
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            pc = 0x53249e; continue;
            }
            0x53249e => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x78),32));
            { let a = c.get(3,32); let r = c.sub(a,0x650u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = rm(m,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x532516; continue; }
            pc = 0x5324b4; continue;
            }
            0x5324b4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x532516; continue; }
            pc = 0x5324be; continue;
            }
            0x5324be => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,8,rm(m,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x19),8));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[7]).wrapping_add(0x8));
            pc = 0x5324d2; continue;
            }
            0x5324d2 => {
            { let a = c.get(1,32); let r = c.sub(a,0x650u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x11),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x5324ec; continue; }
            pc = 0x5324de; continue;
            }
            0x5324de => {
            c.set(3,8,rm(m,c.r[1],8));
            { let a = c.get(3,8); let r = c.and(a,0x7u32,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,0x1u32,8); }
            if c.cond(4) { pc = 0x5324f2; continue; }
            pc = 0x5324e8; continue;
            }
            0x5324e8 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5324ec; continue;
            }
            0x5324ec => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x5324d2; continue; }
            pc = 0x5324f0; continue;
            }
            0x5324f0 => {
            pc = 0x532516; continue;
            }
            0x5324f2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x53251a; continue; }
            pc = 0x5324fa; continue;
            }
            0x5324fa => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.get(0,32));
            { let a = c.r[7].wrapping_add(0x464); wm(m,a,32,0x40000000u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x78),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x8),8); let r = c.or(a,0x8u32,8); { let a = c.r[0].wrapping_add(0x8); wm(m,a,8,r); } }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x532516; continue;
            }
            0x532516 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(15) { pc = 0x53249e; continue; }
            pc = 0x53251a; continue;
            }
            0x53251a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x532578; continue; }
            pc = 0x532524; continue;
            }
            0x532524 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x532526; continue;
            }
            0x532526 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x532578; continue; }
            pc = 0x532530; continue;
            }
            0x532530 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x78),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x8),16));
            c.set(2,8,c.get(1,8));
            { let a = c.get(2,8); let r = c.and(a,0x7u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x532569; continue; }
            pc = 0x532543; continue;
            }
            0x532543 => {
            { let a = rm(m,c.r[0].wrapping_add(0x464),32); c.sub(a,0x3f800000u32,32); }
            if c.cond(5) { pc = 0x532569; continue; }
            pc = 0x53254f; continue;
            }
            0x53254f => {
            { let a = c.get(1,32); let r = c.or(a,0x8u32,32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x464); wm(m,a,32,0x40000000u32); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,c.get(1,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x532569; continue;
            }
            0x532569 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x650u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(7) { pc = 0x532526; continue; }
            pc = 0x532578; continue;
            }
            0x532578 => {
            c.set(1,32,rm(m,0x6dd744u32,32));
            c.set(0,32,rm(m,0x6dd748u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x5325c3; continue; }
            pc = 0x532592; continue;
            }
            0x532592 => {
            c.set(7,32,0x3u32);
            pc = 0x532597; continue;
            }
            0x532597 => {
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x5325a2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x6),16); c.sub(a,c.get(7,16),16); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            if c.cond(5) { pc = 0x5325b3; continue; }
            pc = 0x5325ae; continue;
            }
            0x5325ae => {
            c.set(1,32,0xffffu32);
            pc = 0x5325b3; continue;
            }
            0x5325b3 => {
            c.set(2,32,rm(m,0x6dd748u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x532597; continue; }
            pc = 0x5325c3; continue;
            }
            0x5325c3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x8e); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            if c.cond(6) { pc = 0x5326b6; continue; }
            pc = 0x5325db; continue;
            }
            0x5325db => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let r = c.add(a,0x2c8u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x5325e7; continue;
            }
            0x5325e7 => {
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); let r = c.and(a,0x40u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.sub(a,0x40u32,8); }
            if c.cond(5) { pc = 0x532699; continue; }
            pc = 0x5325f4; continue;
            }
            0x5325f4 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x53263e; continue; }
            pc = 0x532602; continue;
            }
            0x532602 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x78),32));
            pc = 0x532605; continue;
            }
            0x532605 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x532631; continue; }
            pc = 0x53261a; continue;
            }
            0x53261a => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x19),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x532631; continue; }
            pc = 0x532627; continue;
            }
            0x532627 => {
            c.set(5,32,0x1u32);
            c.set(0,32,0x3e8u32);
            pc = 0x532631; continue;
            }
            0x532631 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x532605; continue; }
            pc = 0x532636; continue;
            }
            0x532636 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x532699; continue; }
            pc = 0x53263e; continue;
            }
            0x53263e => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x8e),8));
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x532693; continue; }
            pc = 0x532649; continue;
            }
            0x532649 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x18),8));
            { let a = c.get(0,8); c.sub(a,0xeu32,8); }
            if c.cond(6) { pc = 0x53265d; continue; }
            pc = 0x532651; continue;
            }
            0x532651 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0xeu32,32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            pc = 0x532669; continue;
            }
            0x53265d => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,0xeu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            pc = 0x532669; continue;
            }
            0x532669 => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x18),8));
            { let a = c.get(0,8); c.sub(a,0xeu32,8); }
            if c.cond(6) { pc = 0x532683; continue; }
            pc = 0x532677; continue;
            }
            0x532677 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0xeu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            pc = 0x53268f; continue;
            }
            0x532683 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0xeu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            pc = 0x53268f; continue;
            }
            0x53268f => {
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x532699; continue; }
            pc = 0x532693; continue;
            }
            0x532693 => {
            { let a = c.r[6].wrapping_add(0x8e); wm(m,a,8,c.get(3,8)); }
            pc = 0x532699; continue;
            }
            0x532699 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x9f8u32,32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x5325e7; continue; }
            pc = 0x5326b6; continue;
            }
            0x5326b6 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x8e),8));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(5) { pc = 0x5326c9; continue; }
            pc = 0x5326c2; continue;
            }
            0x5326c2 => {
            { let a = c.r[6].wrapping_add(0x8e); wm(m,a,8,0x0u32); }
            pc = 0x5326c9; continue;
            }
            0x5326c9 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0x8a); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00532c50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x532c50;
    loop {
        match pc {
            0x532c50 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6dd748u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x532ccc; continue; }
            pc = 0x532c69; continue;
            }
            0x532c69 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x532c74);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x6),16); c.sub(a,0x32u32,16); }
            if c.cond(5) { pc = 0x532ccc; continue; }
            pc = 0x532c7b; continue;
            }
            0x532c7b => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0xd),8));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xa),16),16); }
            if c.cond(6) { pc = 0x532ccc; continue; }
            pc = 0x532c86; continue;
            }
            0x532c86 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xc),8));
            { let a = c.get(1,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x532ccc; continue; }
            pc = 0x532c90; continue;
            }
            0x532c90 => {
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x532e18)); continue;
            }
            0x532c97 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x532cae; continue; }
            pc = 0x532ca0; continue;
            }
            0x532ca0 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x14),8));
            pc = 0x532ca3; continue;
            }
            0x532ca3 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x18),8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x532d24; continue; }
            pc = 0x532ca9; continue;
            }
            0x532ca9 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x532ca3; continue; }
            pc = 0x532cae; continue;
            }
            0x532cae => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x532cb0; continue;
            }
            0x532cb0 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x6dd7ac),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x532cc4; continue; }
            pc = 0x532cba; continue;
            }
            0x532cba => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xe),8));
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x14),8));
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x532d24; continue; }
            pc = 0x532cc4; continue;
            }
            0x532cc4 => {
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x14u32,32); }
            if c.cond(2) { pc = 0x532cb0; continue; }
            pc = 0x532ccc; continue;
            }
            0x532ccc => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x532cd4 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x10),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x532d24; continue; }
            pc = 0x532ce0; continue;
            }
            0x532ce0 => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x76c),32));
            { let a = c.get(1,32); let r = c.and(a,0x8000u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x8000u32,32); }
            if c.cond(4) { pc = 0x532d24; continue; }
            pc = 0x532d00; continue;
            }
            0x532d00 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x11),8));
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x76c),32));
            { let a = c.get(2,32); let r = c.and(a,0x8000u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x8000u32,32); }
            if c.cond(5) { pc = 0x532ccc; continue; }
            pc = 0x532d24; continue;
            }
            0x532d24 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x532d2c => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x10),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x532d24; continue; }
            pc = 0x532d39; continue;
            }
            0x532d39 => {
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x134),16); c.sub(a,0x12u32,16); }
            if c.cond(5) { pc = 0x532d24; continue; }
            pc = 0x532d57; continue;
            }
            0x532d57 => {
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x42),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x532d24; continue; }
            pc = 0x532d64; continue;
            }
            0x532d64 => {
            c.set(7,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x2),8));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x12),16),16); }
            if c.cond(5) { pc = 0x532d96; continue; }
            pc = 0x532d80; continue;
            }
            0x532d80 => {
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x3),8));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),16),16); }
            if c.cond(5) { pc = 0x532d96; continue; }
            pc = 0x532d8b; continue;
            }
            0x532d8b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x532d96 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x532d9e => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x10),8));
            { let a = c.get(1,8); c.sub(a,0x5u32,8); }
            if c.cond(2) { pc = 0x532dae; continue; }
            pc = 0x532da6; continue;
            }
            0x532da6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x532dae => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x532dc7; continue; }
            pc = 0x532dbf; continue;
            }
            0x532dbf => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x532dc7 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xe),8));
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x11),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            c.set(1,8,u32::from(c.cond(4)));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,c.get(1,8));
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x532ddc => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x532df0; continue; }
            pc = 0x532de8; continue;
            }
            0x532de8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x532df0 => {
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x10),16));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2d4),32));
            { let a = c.get(6,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(6,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,u32::from(c.cond(12)));
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005333c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5333c0;
    loop {
        match pc {
            0x5333c0 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x8d),8));
            c.push(m,c.get(6,32));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.push(m,c.get(7,32));
            c.set(7,16,rm(m,c.r[1].wrapping_add(0xa),16));
            c.set(2,8,c.get(0,8));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(2,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x53341c; continue; }
            pc = 0x5333d8; continue;
            }
            0x5333d8 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x78),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x1c),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[2]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x8d),8));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x78),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1d),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x53341c => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(3,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x8e),8));
            c.set(3,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(3,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(2,32,c.r[2].wrapping_add(c.r[3].wrapping_mul(8)));
            c.set(3,16,rm(m,c.r[2].wrapping_add(0x6f1),8));
            { let a = c.get(3,16); c.sub(a,c.get(7,16),16); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(2) { pc = 0x53344c; continue; }
            pc = 0x533444; continue;
            }
            0x533444 => {
            { let a = c.get(6,32); let r = c.or(a,0xffffffffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0x3u32,32); c.set(6,32,r); }
            pc = 0x53344c; continue;
            }
            0x53344c => {
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x6d4),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[2]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x8e),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x6d8),8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00533cf0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x533cf0;
    loop {
        match pc {
            0x533cf0 => {
            c.set(1,32,rm(m,c.r[1],32));
            c.set(0,32,0x13u32);
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x533d03; continue; }
            pc = 0x533cfb; continue;
            }
            0x533cfb => {
            c.push(m, 0x533d00);
            { let args = []; c.r[0] = host.call(m, 0x533cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,0x13u32,32); c.set(0,32,r); }
            pc = 0x533d03; continue;
            }
            0x533d03 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00533d10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x533d10;
    loop {
        match pc {
            0x533d10 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0x4u32);
            c.set(0,32,c.r[7].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x533d24);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x8));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x533d33);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            { let a = c.r[6]; wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[6]; wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(0,32,c.r[7].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            { let a = c.r[6]; wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xf),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[6]; wm(m,a,8,c.get(2,8)); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m, 0x533d5a);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x0u32); }
            c.push(m, 0x533d72);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x533d8f; continue; }
            pc = 0x533d7e; continue;
            }
            0x533d7e => {
            { let a = c.r[6]; wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m, 0x533d8a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x533d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x533d8f => {
            { let a = c.r[6]; wm(m,a,8,0x0u32); }
            c.set(0,32,c.r[6].wrapping_add(0x1));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00533e80(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x533e80;
    loop {
        match pc {
            0x533e80 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x533e8e; continue; }
            pc = 0x533e89; continue;
            }
            0x533e89 => {
            c.push(m, 0x533e8e);
            { let args = []; c.r[0] = host.call(m, 0x533e80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x533e8e; continue;
            }
            0x533e8e => {
            c.push(m,c.get(6,32));
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            c.push(m, 0x533e9a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00534090(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x534090;
    loop {
        match pc {
            0x534090 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x5340c3; continue; }
            pc = 0x53409a; continue;
            }
            0x53409a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5340a6; continue;
            }
            0x5340a6 => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x5340bd; continue; }
            pc = 0x5340ab; continue;
            }
            0x5340ab => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xc),8));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x5340bd; continue; }
            pc = 0x5340b4; continue;
            }
            0x5340b4 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xd),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5340c8; continue; }
            pc = 0x5340bd; continue;
            }
            0x5340bd => {
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5340a6; continue; }
            pc = 0x5340c3; continue;
            }
            0x5340c3 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5340c8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005340d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5340d0;
    loop {
        match pc {
            0x5340d0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x534326; continue; }
            pc = 0x5340df; continue;
            }
            0x5340df => {
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x534326; continue; }
            pc = 0x5340e8; continue;
            }
            0x5340e8 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x5340f4; continue;
            }
            0x5340f4 => {
            c.set(0,32,rm(m,c.r[1],32));
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x53410d; continue; }
            pc = 0x5340fb; continue;
            }
            0x5340fb => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xc),8));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x53410d; continue; }
            pc = 0x534104; continue;
            }
            0x534104 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xd),8));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x53411a; continue; }
            pc = 0x53410d; continue;
            }
            0x53410d => {
            { let a = rm(m,c.r[0],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x534326; continue; }
            pc = 0x534116; continue;
            }
            0x534116 => {
            c.set(1,32,c.get(0,32));
            pc = 0x5340f4; continue;
            }
            0x53411a => {
            c.set(7,32,rm(m,c.r[1],32));
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[7]; wm(m,a,32,0x0u32); }
            if c.cond(5) { pc = 0x53426f; continue; }
            pc = 0x534132; continue;
            }
            0x534132 => {
            { let a = rm(m,c.r[7].wrapping_add(0x10),16); c.sub(a,0x18u32,16); }
            if c.cond(3) { pc = 0x534316; continue; }
            pc = 0x53413d; continue;
            }
            0x53413d => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xe),8));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x534150);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x53419f; continue; }
            pc = 0x534154; continue;
            }
            0x534154 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            c.set(6,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x534182; continue; }
            pc = 0x534168; continue;
            }
            0x534168 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x12),16));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            pc = 0x53430b; continue;
            }
            0x534182 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x12),16));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            pc = 0x53430b; continue;
            }
            0x53419f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x53430b; continue; }
            pc = 0x5341af; continue;
            }
            0x5341af => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5341ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xe),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x10),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x12),16));
            c.set(3,8,rm(m,c.r[2].wrapping_add(0x672c14),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(3,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            if c.cond(4) { pc = 0x5341e3; continue; }
            pc = 0x5341de; continue;
            }
            0x5341de => {
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            pc = 0x5341e9; continue;
            }
            0x5341e3 => {
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            pc = 0x5341e9; continue;
            }
            0x5341e9 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53430b; continue; }
            pc = 0x5341fd; continue;
            }
            0x5341fd => {
            c.set(5,32,0x4u32);
            pc = 0x534202; continue;
            }
            0x534202 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53422e; continue; }
            pc = 0x53421a; continue;
            }
            0x53421a => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x53422e; continue; }
            pc = 0x534223; continue;
            }
            0x534223 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x53422a);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x534249; continue; }
            pc = 0x53422e; continue;
            }
            0x53422e => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x534202; continue; }
            pc = 0x534244; continue;
            }
            0x534244 => {
            pc = 0x53430b; continue;
            }
            0x534249 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(5,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x54),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x54));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x534263);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x53426a);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x53430b; continue;
            }
            0x53426f => {
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x5342dd; continue; }
            pc = 0x534273; continue;
            }
            0x534273 => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x10),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x534316; continue; }
            pc = 0x534284; continue;
            }
            0x534284 => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x534316; continue; }
            pc = 0x534298; continue;
            }
            0x534298 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x6),16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x534316; continue; }
            pc = 0x5342ad; continue;
            }
            0x5342ad => {
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x134); wm(m,a,16,0x0u32); }
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x3c),8); let r = c.or(a,0x20u32,8); { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x3c); wm(m,a,8,r); } }
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            c.set(2,16,rm(m,0x6dd888u32,16));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x6); wm(m,a,16,c.get(2,16)); }
            { let a = 0x6dd888u32; wm(m,a,16,c.get(1,16)); }
            pc = 0x53430b; continue;
            }
            0x5342dd => {
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x534316; continue; }
            pc = 0x5342e1; continue;
            }
            0x5342e1 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x534316; continue; }
            pc = 0x5342f2; continue;
            }
            0x5342f2 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x534302);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x53430b; continue;
            }
            0x53430b => {
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x534316; continue; }
            pc = 0x534311; continue;
            }
            0x534311 => {
            c.push(m, 0x534316);
            { let args = []; c.r[0] = host.call(m, 0x533e80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x534316; continue;
            }
            0x534316 => {
            c.push(m,c.get(7,32));
            { let a = c.r[7]; wm(m,a,32,0x0u32); }
            c.push(m, 0x534322);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x534326; continue;
            }
            0x534326 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00534f10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x534f10;
    loop {
        match pc {
            0x534f10 => {
            c.push(m,c.get(1,32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x534f9a; continue; }
            pc = 0x534f27; continue;
            }
            0x534f27 => {
            c.set(0,32,c.get(5,32));
            c.set(1,32,0xfu32);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(7,32,rm(m,0x6de4b4u32,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(6,32,c.get(2,32));
            c.set(2,8,0x1eu32);
            c.set(0,32,c.get(6,32));
            pc = 0x534f42; continue;
            }
            0x534f42 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x6defb8),32),32); }
            if c.cond(6) { pc = 0x534f5f; continue; }
            pc = 0x534f53; continue;
            }
            0x534f53 => {
            { let a = rm(m,c.r[1].wrapping_add(0x6defb4),8); c.sub(a,c.get(2,8),8); }
            if c.cond(2) { pc = 0x534fdf; continue; }
            pc = 0x534f5f; continue;
            }
            0x534f5f => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            if c.cond(2) { pc = 0x534f42; continue; }
            pc = 0x534f65; continue;
            }
            0x534f65 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x534f6f; continue; }
            pc = 0x534f69; continue;
            }
            0x534f69 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,8,0x1u32);
            pc = 0x534f42; continue;
            }
            0x534f6f => {
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            if c.cond(2) { pc = 0x534f42; continue; }
            pc = 0x534f74; continue;
            }
            0x534f74 => {
            c.set(0,32,c.get(6,32));
            pc = 0x534f76; continue;
            }
            0x534f76 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6defb4),8); c.sub(a,c.get(2,8),8); }
            if c.cond(2) { pc = 0x534fdf; continue; }
            pc = 0x534f85; continue;
            }
            0x534f85 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            if c.cond(2) { pc = 0x534f76; continue; }
            pc = 0x534f8b; continue;
            }
            0x534f8b => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x534f95; continue; }
            pc = 0x534f8f; continue;
            }
            0x534f8f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,8,0x1u32);
            pc = 0x534f76; continue;
            }
            0x534f95 => {
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            if c.cond(2) { pc = 0x534f76; continue; }
            pc = 0x534f9a; continue;
            }
            0x534f9a => {
            c.set(6,32,rm(m,0x6df2e4u32,32));
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0xffffff9c));
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(2,32); let r = c.add(a,0x64u32,32); c.set(2,32,r); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x534fb3; continue;
            }
            0x534fb3 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0x1fu32,32); c.set(1,32,r); }
            c.set(7,32,0x1u32);
            { let a = c.get(7,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(c.r[1].wrapping_mul(4)),32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x535016; continue; }
            pc = 0x534fcd; continue;
            }
            0x534fcd => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x534fb3; continue; }
            pc = 0x534fd2; continue;
            }
            0x534fd2 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x535040; continue; }
            pc = 0x534fd6; continue;
            }
            0x534fd6 => {
            c.set(0,8,0x1u32);
            c.set(2,32,0x64u32);
            pc = 0x534fb3; continue;
            }
            0x534fdf => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,8,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6defb4),8));
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,16,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x673858),16));
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            c.set(2,16,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x67385a),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[0]; wm(m,a,16,c.get(2,16)); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x535016 => {
            c.set(0,32,rm(m,0x6df2e0u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,0x6df2e0u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x2),16));
            { let a = c.r[0]; wm(m,a,16,c.get(2,16)); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x535040 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[1]; wm(m,a,16,0xffffu32); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[2]; wm(m,a,16,0xffffu32); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005353a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5353a0;
    loop {
        match pc {
            0x5353a0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xfu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005492d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5492d0;
    loop {
        match pc {
            0x5492d0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x34u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x5492e2);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,8); let r = c.and(a,0xcu32,8); c.set(0,8,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x934),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            if c.cond(2) { pc = 0x549333; continue; }
            pc = 0x549316; continue;
            }
            0x549316 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x549322);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[7].wrapping_add(0x930); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x549333 => {
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(5,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(6,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x5493ae; continue; }
            pc = 0x54934e; continue;
            }
            0x54934e => {
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x10),16));
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.get(5,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(6,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            if c.cond(3) { pc = 0x5493b9; continue; }
            pc = 0x54937e; continue;
            }
            0x54937e => {
            c.set(3,8,0x2u32);
            pc = 0x549380; continue;
            }
            0x549380 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[6]));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0xe),8));
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(2) { pc = 0x5493a0; continue; }
            pc = 0x549391; continue;
            }
            0x549391 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x1u32); }
            c.set(0,32,0xffffu32);
            pc = 0x5493a6; continue;
            }
            0x5493a0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x6),16));
            pc = 0x5493a6; continue;
            }
            0x5493a6 => {
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x549380; continue; }
            pc = 0x5493aa; continue;
            }
            0x5493aa => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x5493b9; continue;
            }
            0x5493ae => {
            { let a = rm(m,c.r[5].wrapping_add(0xe),8); c.sub(a,0x2u32,8); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x5493b9; continue;
            }
            0x5493b9 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x1cu32); }
            pc = 0x5493c7; continue;
            }
            0x5493c7 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.set(3,32,rm(m,0x6de4b0u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2),8));
            c.set(1,32,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x54945b; continue; }
            pc = 0x5493e6; continue;
            }
            0x5493e6 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x549409; continue; }
            pc = 0x5493ee; continue;
            }
            0x5493ee => {
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(3,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x54945b; continue; }
            pc = 0x549409; continue;
            }
            0x549409 => {
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.push(m,0x0u32);
            c.set(1,8,c.get(2,8));
            c.set(2,16,rm(m,c.r[5],8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x549421);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x549455; continue; }
            pc = 0x549425; continue;
            }
            0x549425 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x54942b; continue;
            }
            0x54942b => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x930),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x54944c; continue; }
            pc = 0x549437; continue;
            }
            0x549437 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x7c));
            { let a = rm(m,c.r[0],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x54944c; continue; }
            pc = 0x549446; continue;
            }
            0x549446 => {
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            pc = 0x54944c; continue;
            }
            0x54944c => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,0x18u32,32); }
            if c.cond(2) { pc = 0x54942b; continue; }
            pc = 0x549455; continue;
            }
            0x549455 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); let r = c.inc(a,32); { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,r); } }
            pc = 0x54945e; continue;
            }
            0x54945b => {
            { let a = c.r[1]; wm(m,a,8,0xffu32); }
            pc = 0x54945e; continue;
            }
            0x54945e => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(c.r[0]),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.add(a,0x37u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xdcu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xdcu32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,16); c.and(a,c.get(1,16),16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5493c7; continue; }
            pc = 0x549498; continue;
            }
            0x549498 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x5494bf; continue; }
            pc = 0x5494a2; continue;
            }
            0x5494a2 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5494ae);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[7].wrapping_add(0x930); wm(m,a,32,c.get(6,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5494bf => {
            { let a = rm(m,c.r[4].wrapping_add(0x28),32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x5497c7; continue; }
            pc = 0x5494c9; continue;
            }
            0x5494c9 => {
            { let a = rm(m,c.r[5].wrapping_add(0x136),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x5494f4; continue; }
            pc = 0x5494d2; continue;
            }
            0x5494d2 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x934),32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x68u32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x5494f4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5494f4; continue;
            }
            0x5494f4 => {
            c.set(6,16,rm(m,c.r[5].wrapping_add(0x132),16));
            c.set(5,32,rm(m,0x6dd72cu32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,16); c.sub(a,rm(m,0x6dd72au32,16),16); }
            if c.cond(3) { pc = 0x549563; continue; }
            pc = 0x54950c; continue;
            }
            0x54950c => {
            c.set(1,32,c.get(6,32));
            c.set(2,32,0x1u32);
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(c.r[3].wrapping_mul(4)),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x549563; continue; }
            pc = 0x54952d; continue;
            }
            0x54952d => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,0x6dd72au32,16));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x54953a; continue;
            }
            0x54953a => {
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,16,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)),16));
            c.set(0,32,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x549561; continue; }
            pc = 0x549555; continue;
            }
            0x549555 => {
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x10),32),32); }
            if c.cond(7) { pc = 0x549561; continue; }
            pc = 0x54955b; continue;
            }
            0x54955b => {
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x54953a; continue; }
            pc = 0x54955f; continue;
            }
            0x54955f => {
            pc = 0x549563; continue;
            }
            0x549561 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x549563; continue;
            }
            0x549563 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5495a4; continue; }
            pc = 0x54956b; continue;
            }
            0x54956b => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x934),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x930),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x6cu32); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x2),8));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x54959c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5495a4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            if c.cond(6) { pc = 0x549913; continue; }
            pc = 0x5495be; continue;
            }
            0x5495be => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x7u32); }
            pc = 0x5495ce; continue;
            }
            0x5495ce => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,32,rm(m,0x6de4b0u32,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x2));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x2),8));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x549794; continue; }
            pc = 0x5495f5; continue;
            }
            0x5495f5 => {
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(0,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x2),8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x54965a; continue; }
            pc = 0x54960b; continue;
            }
            0x54960b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x6au32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x0u32); }
            c.set(1,8,rm(m,c.r[2],8));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x934),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x56),16));
            c.set(1,32,0x34u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x549936; continue; }
            pc = 0x549644; continue;
            }
            0x549644 => {
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x549650);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,0x6dd72cu32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x54965a; continue;
            }
            0x54965a => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x3),8));
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x3));
            { let a = c.get(0,8); c.sub(a,rm(m,c.r[1],8),8); }
            if c.cond(4) { pc = 0x5496a9; continue; }
            pc = 0x549670; continue;
            }
            0x549670 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x6du32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(6,32)); }
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x34u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x56),16));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x549936; continue; }
            pc = 0x549697; continue;
            }
            0x549697 => {
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5496a3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,0x6dd72cu32,32));
            pc = 0x5496a9; continue;
            }
            0x5496a9 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x5496af; continue;
            }
            0x5496af => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0]));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x4),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(3,32),32); c.set(5,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(c.r[5]).wrapping_add(0x4),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x549715; continue; }
            pc = 0x5496d7; continue;
            }
            0x5496d7 => {
            c.set(2,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,rm(m,0x6dd72cu32,32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(2,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x7c),32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x60),32),32); }
            if c.cond(5) { pc = 0x549711; continue; }
            pc = 0x5496fc; continue;
            }
            0x5496fc => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[5].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x1c),32));
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32),32); }
            if c.cond(4) { pc = 0x549778; continue; }
            pc = 0x549711; continue;
            }
            0x549711 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x549715; continue;
            }
            0x549715 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x69u32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,16,c.get(6,16)); }
            c.set(2,16,rm(m,c.r[2],8));
            { let a = c.r[4].wrapping_add(0x36); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[4].wrapping_add(0x3a); wm(m,a,16,c.get(3,16)); }
            c.set(5,32,c.r[2].wrapping_add(c.r[0]));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x60),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,0x34u32);
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x56),16));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x549936; continue; }
            pc = 0x549764; continue;
            }
            0x549764 => {
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x549770);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,0x6dd72cu32,32));
            pc = 0x54977c; continue;
            }
            0x549778 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x54977c; continue;
            }
            0x54977c => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x18u32,32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            if c.cond(2) { pc = 0x5496af; continue; }
            pc = 0x54978a; continue;
            }
            0x54978a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[1].wrapping_add(c.r[5]),16));
            pc = 0x549794; continue;
            }
            0x549794 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xdcu32,32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x37u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(2) { pc = 0x5495ce; continue; }
            pc = 0x5497c2; continue;
            }
            0x5497c2 => {
            pc = 0x549913; continue;
            }
            0x5497c7 => {
            { let a = c.r[5].wrapping_add(0x136); wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x934),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x930),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x6cu32); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x2),8));
            c.set(0,32,0x1u32);
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            if c.cond(6) { pc = 0x549824; continue; }
            pc = 0x5497f8; continue;
            }
            0x5497f8 => {
            c.set(6,32,rm(m,0x6de4b0u32,32));
            pc = 0x5497fe; continue;
            }
            0x5497fe => {
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x549806; continue; }
            pc = 0x549802; continue;
            }
            0x549802 => {
            c.set(0,32,c.get(3,32));
            pc = 0x54981b; continue;
            }
            0x549806 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(3,8,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x2),8));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            pc = 0x54981b; continue;
            }
            0x54981b => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x5497fe; continue; }
            pc = 0x549824; continue;
            }
            0x549824 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x549835);
            { let args = []; c.r[0] = host.call(m, 0x535760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x132),16));
            c.set(1,16,rm(m,0x6dd72au32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x549913; continue; }
            pc = 0x54984f; continue;
            }
            0x54984f => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x549913; continue; }
            pc = 0x549857; continue;
            }
            0x549857 => {
            c.set(2,32,rm(m,0x6dd72cu32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x54985f; continue;
            }
            0x54985f => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.set(1,8,rm(m,c.r[1].wrapping_add(c.r[5]).wrapping_add(0x2),8));
            c.set(6,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,rm(m,0x6de4b0u32,32),32); }
            if c.cond(3) { pc = 0x549906; continue; }
            pc = 0x54987d; continue;
            }
            0x54987d => {
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,0x18u32);
            c.set(6,32,c.r[0].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(6,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.r[6].wrapping_add(c.r[2]).wrapping_add(0x2); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.set(1,32,rm(m,0x6dd72cu32,32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[5]).wrapping_add(0x3),8));
            { let a = c.r[6].wrapping_add(c.r[1]).wrapping_add(0x3); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.set(1,32,rm(m,0x6dd72cu32,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[5]).wrapping_add(0x4));
            c.set(2,32,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5498be);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.set(2,32,rm(m,0x6dd72cu32,32));
            c.push(m,0x60u32);
            c.set(1,32,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x1c));
            c.set(0,32,c.r[6].wrapping_add(c.r[2]).wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5498db);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.set(0,32,rm(m,0x6dd72cu32,32));
            c.push(m,0x60u32);
            c.set(2,32,c.r[1].wrapping_add(c.r[5]).wrapping_add(0x7c));
            c.set(1,32,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x7c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5498f7);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6dd72cu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[2]),16));
            pc = 0x549906; continue;
            }
            0x549906 => {
            { let a = c.get(5,32); let r = c.add(a,0xdcu32,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x54985f; continue; }
            pc = 0x549913; continue;
            }
            0x549913 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x930),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x54991f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[7].wrapping_add(0x930); wm(m,a,32,0x0u32); }
            { let a = c.r[7].wrapping_add(0x934); wm(m,a,32,0xffffu32); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x549936; continue;
            }
            0x549936 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00612ee0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x612ee0;
    loop {
        match pc {
            0x612ee0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x668399u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x404),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x1u32); }
            c.set(6,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x0u32); }
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x612f6d; continue; }
            pc = 0x612f38; continue;
            }
            0x612f38 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,c.get(5,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x612f59; continue; }
            pc = 0x612f4b; continue;
            }
            0x612f4b => {
            c.push(m,c.get(3,32));
            c.push(m, 0x612f51);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6087f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x612ffc; continue; }
            pc = 0x612f59; continue;
            }
            0x612f59 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x404),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x612f38; continue; }
            pc = 0x612f6d; continue;
            }
            0x612f6d => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x612fa1; continue; }
            pc = 0x612f7a; continue;
            }
            0x612f7a => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            pc = 0x612fa1; continue;
            }
            0x612fa1 => {
            c.set(6,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(6,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x613019; continue; }
            pc = 0x612fae; continue;
            }
            0x612fae => {
            c.set(1,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3),8));
            c.push(m,c.get(2,32));
            c.push(m, 0x612fcf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x612fe0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x69c98cu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x6c77b8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x612ff7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x613028; continue;
            }
            0x612ffc => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.push(m, 0x61300f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x610b29, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x1u32); }
            pc = 0x613040; continue;
            }
            0x613019 => {
            c.set(0,32,rm(m,0x6a1780u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x613028);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x613028; continue;
            }
            0x613028 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x613038);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x1u32); }
            pc = 0x613040; continue;
            }
            0x613040 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x0u32); }
            c.push(m, 0x61304e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00524b70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x524b70;
    loop {
        match pc {
            0x524b70 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x776),16));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x524c25; continue; }
            pc = 0x524b86; continue;
            }
            0x524b86 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x75c),32));
            pc = 0x524b8e; continue;
            }
            0x524b8e => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            if c.cond(4) { pc = 0x524c12; continue; }
            pc = 0x524b96; continue;
            }
            0x524b96 => {
            { let a = c.get(0,8); c.sub(a,0x15u32,8); }
            if c.cond(3) { pc = 0x524bcc; continue; }
            pc = 0x524b9a; continue;
            }
            0x524b9a => {
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x7ac),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)),8));
            { let a = c.get(3,32); let r = c.imul(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x1),8));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x80c),32));
            { let a = rm(m,c.r[3].wrapping_add(c.r[0]),8); c.sub(a,0x80u32,8); }
            if c.cond(2) { pc = 0x524bc3; continue; }
            pc = 0x524bbc; continue;
            }
            0x524bbc => {
            { let a = rm(m,c.r[1].wrapping_add(0x78c),16); let r = c.inc(a,16); { let a = c.r[1].wrapping_add(0x78c); wm(m,a,16,r); } }
            pc = 0x524bc3; continue;
            }
            0x524bc3 => {
            { let a = rm(m,c.r[1].wrapping_add(0x78e),16); let r = c.inc(a,16); { let a = c.r[1].wrapping_add(0x78e); wm(m,a,16,r); } }
            pc = 0x524c12; continue;
            }
            0x524bcc => {
            { let a = c.get(0,8); c.sub(a,0x30u32,8); }
            if c.cond(2) { pc = 0x524c12; continue; }
            pc = 0x524bd0; continue;
            }
            0x524bd0 => {
            if c.cond(4) { pc = 0x524c0c; continue; }
            pc = 0x524bd2; continue;
            }
            0x524bd2 => {
            { let a = c.get(0,8); c.sub(a,0x31u32,8); }
            if c.cond(4) { pc = 0x524c0c; continue; }
            pc = 0x524bd6; continue;
            }
            0x524bd6 => {
            { let a = c.get(0,8); c.sub(a,0x32u32,8); }
            if c.cond(4) { pc = 0x524bf5; continue; }
            pc = 0x524bda; continue;
            }
            0x524bda => {
            { let a = c.get(0,8); c.sub(a,0x33u32,8); }
            if c.cond(4) { pc = 0x524bf5; continue; }
            pc = 0x524bde; continue;
            }
            0x524bde => {
            { let a = c.get(0,8); c.sub(a,0x36u32,8); }
            if c.cond(5) { pc = 0x524beb; continue; }
            pc = 0x524be2; continue;
            }
            0x524be2 => {
            { let a = rm(m,c.r[1].wrapping_add(0x786),16); let r = c.inc(a,16); { let a = c.r[1].wrapping_add(0x786); wm(m,a,16,r); } }
            pc = 0x524c12; continue;
            }
            0x524beb => {
            { let a = rm(m,c.r[1].wrapping_add(0x786),16); let r = c.add(a,0x3u32,16); { let a = c.r[1].wrapping_add(0x786); wm(m,a,16,r); } }
            pc = 0x524c12; continue;
            }
            0x524bf5 => {
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x788),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x786),16); let r = c.inc(a,16); { let a = c.r[1].wrapping_add(0x786); wm(m,a,16,r); } }
            { let a = c.r[1].wrapping_add(0x788); wm(m,a,8,c.get(3,8)); }
            pc = 0x524c12; continue;
            }
            0x524c0c => {
            { let a = rm(m,c.r[1].wrapping_add(0x788),8); let r = c.inc(a,8); { let a = c.r[1].wrapping_add(0x788); wm(m,a,8,r); } }
            pc = 0x524c12; continue;
            }
            0x524c12 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x4),16));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x524b8e; continue; }
            pc = 0x524c23; continue;
            }
            0x524c23 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x524c25; continue;
            }
            0x524c25 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
