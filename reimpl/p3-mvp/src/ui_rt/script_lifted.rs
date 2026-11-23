//! Generado por `reimpl/tools/ui_rt_script.py` con `tools/lift.py` (no editar a mano): intérprete de guiones de misión `0x004ECF64`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static SCRIPT: [(u32, super::core_table::CoreFn); 48] = [
    (0x4ecf64, f_004ecf64),
    (0x4f3860, f_004f3860),
    (0x4f38a0, f_004f38a0),
    (0x4f3680, f_004f3680),
    (0x4f38f0, f_004f38f0),
    (0x4f3960, f_004f3960),
    (0x4d7b10, f_004d7b10),
    (0x4f38d0, f_004f38d0),
    (0x4f38e0, f_004f38e0),
    (0x4f3660, f_004f3660),
    (0x4f3690, f_004f3690),
    (0x4f36a0, f_004f36a0),
    (0x4f36b0, f_004f36b0),
    (0x4f36c0, f_004f36c0),
    (0x4f3740, f_004f3740),
    (0x4f3760, f_004f3760),
    (0x4f37c0, f_004f37c0),
    (0x4f37d0, f_004f37d0),
    (0x4f3850, f_004f3850),
    (0x4f3870, f_004f3870),
    (0x4f3890, f_004f3890),
    (0x4f38b0, f_004f38b0),
    (0x4f3940, f_004f3940),
    (0x533210, f_00533210),
    (0x4d69d0, f_004d69d0),
    (0x4d7960, f_004d7960),
    (0x5483d0, f_005483d0),
    (0x535400, f_00535400),
    (0x4d7030, f_004d7030),
    (0x50df00, f_0050df00),
    (0x514d40, f_00514d40),
    (0x539b50, f_00539b50),
    (0x539c60, f_00539c60),
    (0x53ce30, f_0053ce30),
    (0x444030, f_00444030),
    (0x4d3b40, f_004d3b40),
    (0x505c90, f_00505c90),
    (0x50bc40, f_0050bc40),
    (0x50e340, f_0050e340),
    (0x5250d0, f_005250d0),
    (0x5854d0, f_005854d0),
    (0x59a420, f_0059a420),
    (0x612930, f_00612930),
    (0x638010, f_00638010),
    (0x4f89e0, f_004f89e0),
    (0x4fa7f0, f_004fa7f0),
    (0x500a70, f_00500a70),
    (0x569c20, f_00569c20),
];
pub fn f_004ecf64(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ecf64;
    loop {
        match pc {
            0x4ecf64 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662e44u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x534u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x550),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x464); wm(m,a,32,c.get(5,32)); }
            if c.cond(5) { pc = 0x4ecfb8; continue; }
            pc = 0x4ecf9e; continue;
            }
            0x4ecf9e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4ecfae; continue; }
            pc = 0x4ecfa5; continue;
            }
            0x4ecfa5 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4ecfab);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4ecfae; continue;
            }
            0x4ecfae => {
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4f2e17; continue;
            }
            0x4ecfb8 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(3,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            { let a = c.get(3,8); c.sub(a,0xfdu32,8); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4f2ccb; continue; }
            pc = 0x4ecfe2; continue;
            }
            0x4ecfe2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(5,32,c.r[1].wrapping_add(0xffffffff));
            { let a = c.get(5,32); c.sub(a,0xfau32,32); }
            if c.cond(7) { pc = 0x4f2d44; continue; }
            pc = 0x4ecffb; continue;
            }
            0x4ecffb => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x4f3054),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x4f2e34)); continue;
            }
            0x4ed00a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m, 0x4ed013);
            { let args = []; c.r[0] = host.call(m, 0x4dc800, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4f2c93; continue;
            }
            0x4ed018 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x2a0); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2a0),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,0x0u32); }
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ed048);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x4f2c93; continue; }
            pc = 0x4ed050; continue;
            }
            0x4ed050 => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ed05b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f3660, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4ed062);
            { let args = []; c.r[0] = host.call(m, 0x4f37d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4ed072; continue; }
            pc = 0x4ed066; continue;
            }
            0x4ed066 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[2]));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x4ed072; continue;
            }
            0x4ed072 => {
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.push(m, 0x4ed07d);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4ed050; continue; }
            pc = 0x4ed081; continue;
            }
            0x4ed081 => {
            pc = 0x4f2c93; continue;
            }
            0x4ed086 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(7,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4f2c97; continue; }
            pc = 0x4ed09f; continue;
            }
            0x4ed09f => {
            c.set(2,32,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4ed0b2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x19),8));
            c.push(m,0x0u32);
            c.push(m,0x26u32);
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4ed0cb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            c.push(m, 0x4ed0d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd786u32);
            c.push(m, 0x4ed0e9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x554); wm(m,a,32,0x0u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd787u32);
            c.push(m, 0x4ed105);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m,0x1du32);
            { let a = c.r[4].wrapping_add(0x560); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4ed120);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4f2c93; continue;
            }
            0x4ed125 => {
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4ed12f);
            { let args = []; c.r[0] = host.call(m, 0x4f3690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4ed1f9; continue; }
            pc = 0x4ed137; continue;
            }
            0x4ed137 => {
            c.set(1,32,c.r[4].wrapping_add(0xb4));
            c.push(m, 0x4ed143);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,8); c.sub(a,0xe9u32,8); }
            { let a = c.r[4].wrapping_add(0xb4); wm(m,a,32,0x82u32); }
            if c.cond(4) { pc = 0x4ed160; continue; }
            pc = 0x4ed153; continue;
            }
            0x4ed153 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            pc = 0x4ed163; continue;
            }
            0x4ed160 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x4ed163; continue;
            }
            0x4ed163 => {
            { let a = c.r[4].wrapping_add(0xb8); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0xba); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0xbb); wm(m,a,8,c.get(2,8)); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,16,c.get(1,16)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0xbe); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(3,8); c.sub(a,0xe9u32,8); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0xbf); wm(m,a,8,c.get(2,8)); }
            if c.cond(5) { pc = 0x4ed1d0; continue; }
            pc = 0x4ed1c3; continue;
            }
            0x4ed1c3 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x4ed1d3; continue;
            }
            0x4ed1d0 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x4ed1d3; continue;
            }
            0x4ed1d3 => {
            { let a = c.r[4].wrapping_add(0xc0); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(3,8); c.sub(a,0xe7u32,8); }
            c.set(0,32,c.r[4].wrapping_add(0xb4));
            c.set(1,32,0x6df2f0u32);
            c.set(2,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0xc5); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x4ed1f9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4ed1f9; continue;
            }
            0x4ed1f9 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,0xffffffffu32); }
            pc = 0x4f2c93; continue;
            }
            0x4ed20d => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x488); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x488),32));
            { let a = c.r[4].wrapping_add(0x490); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[5]),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4ed242);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x490),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(3,32,rm(m,c.r[0],32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[5]),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            if c.cond(13) { pc = 0x4ed28c; continue; }
            pc = 0x4ed271; continue;
            }
            0x4ed271 => {
            c.push(m, 0x4ed276);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,0x4bcu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ed28c => {
            c.push(m, 0x4ed291);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,0x4b8u32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ed2a7 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x4a0); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x498); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4ed2ce);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ed2da);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,0x1u32); }
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x9u32); }
            c.set(6,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x498),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4a0),32));
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x12); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            pc = 0x4f2c93; continue;
            }
            0x4ed332 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x474); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x474),32));
            { let a = c.r[4].wrapping_add(0x4a8); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x4a8),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x4ed37f);
            { let args = []; c.r[0] = host.call(m, 0x5252f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[7],32); c.sub(a,0xffu32,32); }
            if c.cond(5) { pc = 0x4f2c93; continue; }
            pc = 0x4ed39b; continue;
            }
            0x4ed39b => {
            { let a = c.r[7]; wm(m,a,32,0xffffffffu32); }
            pc = 0x4f2c93; continue;
            }
            0x4ed3a6 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0xf8); wm(m,a,8,c.get(0,8)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xf8),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(3,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ed3d0);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4ed3d8; continue;
            }
            0x4ed3d8 => {
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(2,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x4ed3ed);
            { let args = []; c.r[0] = host.call(m, 0x5252f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4ed3f5; continue;
            }
            0x4ed3f5 => {
            c.set(1,32,c.r[4].wrapping_add(0x4b4));
            c.push(m, 0x4ed401);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,0x6de4a2u32,16));
            { let a = c.r[4].wrapping_add(0x4b4); wm(m,a,32,0x8eu32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,16); c.sub(a,0x546u32,16); }
            { let a = c.r[4].wrapping_add(0x4b8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x4c4); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x4ed437; continue; }
            pc = 0x4ed42e; continue;
            }
            0x4ed42e => {
            { let a = c.r[4].wrapping_add(0x4bc); wm(m,a,32,c.get(1,32)); }
            pc = 0x4ed468; continue;
            }
            0x4ed437 => {
            { let a = c.get(0,16); c.sub(a,0x578u32,16); }
            c.set(1,32,0x6de4a0u32);
            if c.cond(3) { pc = 0x4ed453; continue; }
            pc = 0x4ed442; continue;
            }
            0x4ed442 => {
            c.push(m, 0x4ed447);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x4bc); wm(m,a,32,c.get(0,32)); }
            pc = 0x4ed468; continue;
            }
            0x4ed453 => {
            c.push(m, 0x4ed458);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x3u32);
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.r[4].wrapping_add(0x4bc); wm(m,a,32,c.get(2,32)); }
            pc = 0x4ed468; continue;
            }
            0x4ed468 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x2d4),32));
            { let a = c.get(0,32); let r = c.and(a,0x800000ffu32,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x4ed482; continue; }
            pc = 0x4ed47b; continue;
            }
            0x4ed47b => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xffffff00u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x4ed482; continue;
            }
            0x4ed482 => {
            c.set(1,32,c.r[4].wrapping_add(0x4b4));
            { let a = c.r[4].wrapping_add(0x4c0); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4ed49b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f2c93; continue;
            }
            0x4ed4a0 => {
            c.push(m,0x10u32);
            c.push(m, 0x4ed4a7);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x54c); wm(m,a,32,0x1u32); }
            if c.cond(4) { pc = 0x4ed4c8; continue; }
            pc = 0x4ed4bd; continue;
            }
            0x4ed4bd => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4ed4c4);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            pc = 0x4ed4ca; continue;
            }
            0x4ed4c8 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x4ed4ca; continue;
            }
            0x4ed4ca => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x54c); wm(m,a,32,0xffffffffu32); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[5].wrapping_add(0x5); wm(m,a,8,c.get(1,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(0,8); let r = c.add(a,0x3cu32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x40u32,8); }
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,8,c.get(0,8)); }
            if c.cond(6) { pc = 0x4ed4f7; continue; }
            pc = 0x4ed4f3; continue;
            }
            0x4ed4f3 => {
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,8,0x71u32); }
            pc = 0x4ed4f7; continue;
            }
            0x4ed4f7 => {
            c.push(m,0x10u32);
            c.push(m, 0x4ed4fe);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1000u32);
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4ed50b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[0].wrapping_add(0xa); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6),8));
            { let a = c.r[2].wrapping_add(0xb); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(0xd); wm(m,a,8,0xffu32); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(0,32,c.r[7].wrapping_add(0x8));
            { let a = c.r[1].wrapping_add(0xe); wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[2].wrapping_add(0xf); wm(m,a,8,0x0u32); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x8),32));
            pc = 0x4ed56d; continue;
            }
            0x4ed56d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,0x20u32,32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x4ed56d; continue; }
            pc = 0x4ed586; continue;
            }
            0x4ed586 => {
            c.set(5,32,rm(m,c.r[5].wrapping_add(0xc),32));
            pc = 0x4ed589; continue;
            }
            0x4ed589 => {
            c.set(1,8,rm(m,c.r[3],8));
            { let a = c.r[5]; wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[3],8));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4ed589; continue; }
            pc = 0x4ed596; continue;
            }
            0x4ed596 => {
            c.set(0,8,rm(m,c.r[3],8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4edca9; continue; }
            pc = 0x4ed5a4; continue;
            }
            0x4ed5a4 => {
            { let a = c.get(0,8); c.sub(a,0x25u32,8); }
            if c.cond(5) { pc = 0x4edc92; continue; }
            pc = 0x4ed5ac; continue;
            }
            0x4ed5ac => {
            c.set(0,32,sx(rm(m,c.r[3].wrapping_add(0x1),8),8)&0xffffffffu32);
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x25u32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,0x55u32,32); }
            c.set(5,32,0x6dd788u32);
            if c.cond(7) { pc = 0x4ed8ce; continue; }
            pc = 0x4ed5c3; continue;
            }
            0x4ed5c3 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x4f31cc),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x4f3150)); continue;
            }
            0x4ed5d2 => {
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x4ed8ce; continue; }
            pc = 0x4ed5ed; continue;
            }
            0x4ed5ed => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ed5fd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); let r = c.add(a,0x160u32,32); c.set(5,32,r); }
            pc = 0x4ed8ce; continue;
            }
            0x4ed60a => {
            c.set(1,32,c.r[4].wrapping_add(0x3d8));
            c.set(2,32,c.r[4].wrapping_add(0x100));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ed624);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5333c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3d8),32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4ed63b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x100),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4ed654);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.push(m,0x6c0304u32);
            c.set(2,32,c.r[4].wrapping_add(0x4e8));
            c.push(m,0x20u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4ed669);
            { let args = []; c.r[0] = host.call(m, 0x63ab8a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(5,32,c.r[4].wrapping_add(0x4dc));
            pc = 0x4ed8ce; continue;
            }
            0x4ed678 => {
            c.set(0,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0cb4),32));
            pc = 0x4ed8ce; continue;
            }
            0x4ed690 => {
            c.set(0,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4ed6b1; continue; }
            pc = 0x4ed6a7; continue;
            }
            0x4ed6a7 => {
            c.set(5,32,0x6dd789u32);
            pc = 0x4ed8ce; continue;
            }
            0x4ed6b1 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ed6c1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4ed6cc);
            { let args = []; c.r[0] = host.call(m, 0x4f3760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x1a),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(5,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a0608),32));
            pc = 0x4ed8ce; continue;
            }
            0x4ed6e9 => {
            c.set(0,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4ed70a; continue; }
            pc = 0x4ed700; continue;
            }
            0x4ed700 => {
            c.set(5,32,0x6dd78au32);
            pc = 0x4ed8ce; continue;
            }
            0x4ed70a => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ed71a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4ed725);
            { let args = []; c.r[0] = host.call(m, 0x4f3760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x1a),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(5,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a0718),32));
            pc = 0x4ed8ce; continue;
            }
            0x4ed742 => {
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ed74c);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(5,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            pc = 0x4ed8ce; continue;
            }
            0x4ed767 => {
            c.set(1,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ed77b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            pc = 0x4ed8ce; continue;
            }
            0x4ed782 => {
            c.set(1,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x4ed8ce; continue; }
            pc = 0x4ed79d; continue;
            }
            0x4ed79d => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ed7ad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            pc = 0x4ed8ce; continue;
            }
            0x4ed7b8 => {
            c.set(0,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x4ed8ce; continue; }
            pc = 0x4ed7d3; continue;
            }
            0x4ed7d3 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ed7e3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            pc = 0x4ed8ce; continue;
            }
            0x4ed7ee => {
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4ed802);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            pc = 0x4ed8ce; continue;
            }
            0x4ed809 => {
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.set(0,32,c.r[4].wrapping_add(0x2e8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,c.r[4].wrapping_add(0x110));
            c.push(m,c.get(1,32));
            c.set(5,32,c.r[4].wrapping_add(0x4e8));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ed839);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5311d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2e8),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0d9c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x4e8));
            c.push(m,0x6c030cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ed878);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x4ed8ce; continue;
            }
            0x4ed87d => {
            c.set(1,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x18u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(5,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            pc = 0x4ed8ce; continue;
            }
            0x4ed897 => {
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(5,32,c.r[4].wrapping_add(0x4dc));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4ed8ed; continue; }
            pc = 0x4ed8b1; continue;
            }
            0x4ed8b1 => {
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6c0318u32);
            pc = 0x4ed8be; continue;
            }
            0x4ed8be => {
            c.set(2,32,c.r[4].wrapping_add(0x4e4));
            c.push(m,c.get(2,32));
            pc = 0x4ed8c6; continue;
            }
            0x4ed8c6 => {
            c.push(m, 0x4ed8cb);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4ed8cb; continue;
            }
            0x4ed8cb => {
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4ed8ce; continue;
            }
            0x4ed8ce => {
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4edc9d; continue; }
            pc = 0x4ed8d9; continue;
            }
            0x4ed8d9 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x4ed8dd; continue;
            }
            0x4ed8dd => {
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4ed8dd; continue; }
            pc = 0x4ed8e8; continue;
            }
            0x4ed8e8 => {
            pc = 0x4edc99; continue;
            }
            0x4ed8ed => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(1,32,c.r[4].wrapping_add(0x4dc));
            c.push(m,c.get(0,32));
            c.push(m,0x6c0320u32);
            c.push(m,c.get(1,32));
            pc = 0x4ed8c6; continue;
            }
            0x4ed904 => {
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(5,32,c.r[4].wrapping_add(0x4dc));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6c0328u32);
            pc = 0x4ed8be; continue;
            }
            0x4ed91c => {
            c.set(0,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4ed936; continue; }
            pc = 0x4ed92f; continue;
            }
            0x4ed92f => {
            c.set(5,32,0x6c0330u32);
            pc = 0x4ed8ce; continue;
            }
            0x4ed936 => {
            c.set(5,32,0x6c0334u32);
            pc = 0x4ed8ce; continue;
            }
            0x4ed93d => {
            { let a = rm(m,c.r[3],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x4ed949; continue; }
            pc = 0x4ed942; continue;
            }
            0x4ed942 => {
            c.set(5,32,0x6c0338u32);
            pc = 0x4ed8ce; continue;
            }
            0x4ed949 => {
            c.set(5,32,0x6c033cu32);
            pc = 0x4ed8ce; continue;
            }
            0x4ed953 => {
            c.set(0,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,c.r[4].wrapping_add(0x4dc));
            c.push(m,c.get(2,32));
            c.push(m,0x6c0340u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ed96f);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[4].wrapping_add(0x4e8));
            pc = 0x4ed8cb; continue;
            }
            0x4ed97b => {
            c.set(1,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,0x66666667u32);
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x4ed9cd; continue; }
            pc = 0x4ed999; continue;
            }
            0x4ed999 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x4ed99d; continue;
            }
            0x4ed99d => {
            c.set(0,8,rm(m,0x6c0344u32,8));
            { let a = c.get(2,32); let r = c.add(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(5,32,0x6c0344u32);
            if c.cond(4) { pc = 0x4ed9c4; continue; }
            pc = 0x4ed9b5; continue;
            }
            0x4ed9b5 => {
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4ed9b5; continue; }
            pc = 0x4ed9c0; continue;
            }
            0x4ed9c0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x4ed9c4; continue;
            }
            0x4ed9c4 => {
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(7) { pc = 0x4ed99d; continue; }
            pc = 0x4ed9c9; continue;
            }
            0x4ed9c9 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            pc = 0x4ed9cd; continue;
            }
            0x4ed9cd => {
            { let a = c.get(2,32); let r = c.add(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x5u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x4eda21; continue; }
            pc = 0x4ed9ed; continue;
            }
            0x4ed9ed => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x4ed9f1; continue;
            }
            0x4ed9f1 => {
            c.set(0,8,rm(m,0x6c0350u32,8));
            { let a = c.get(2,32); let r = c.add(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(5,32,0x6c0350u32);
            if c.cond(4) { pc = 0x4eda18; continue; }
            pc = 0x4eda09; continue;
            }
            0x4eda09 => {
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4eda09; continue; }
            pc = 0x4eda14; continue;
            }
            0x4eda14 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x4eda18; continue;
            }
            0x4eda18 => {
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(7) { pc = 0x4ed9f1; continue; }
            pc = 0x4eda1d; continue;
            }
            0x4eda1d => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            pc = 0x4eda21; continue;
            }
            0x4eda21 => {
            { let a = c.get(2,32); let r = c.add(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            pc = 0x4ed8ce; continue;
            }
            0x4eda30 => {
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4eda51; continue; }
            pc = 0x4eda47; continue;
            }
            0x4eda47 => {
            c.set(5,32,0x6dd78bu32);
            pc = 0x4ed8ce; continue;
            }
            0x4eda51 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4eda61);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6b2a88),32));
            pc = 0x4ed8ce; continue;
            }
            0x4eda73 => {
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4eda94; continue; }
            pc = 0x4eda8a; continue;
            }
            0x4eda8a => {
            c.set(5,32,0x6dd78cu32);
            pc = 0x4ed8ce; continue;
            }
            0x4eda94 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4edaa4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6b2a90),32));
            pc = 0x4ed8ce; continue;
            }
            0x4edab6 => {
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4edad7; continue; }
            pc = 0x4edacd; continue;
            }
            0x4edacd => {
            c.set(5,32,0x6dd78du32);
            pc = 0x4ed8ce; continue;
            }
            0x4edad7 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4edae7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0ef8),32));
            pc = 0x4ed8ce; continue;
            }
            0x4edaf9 => {
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4edb1a; continue; }
            pc = 0x4edb10; continue;
            }
            0x4edb10 => {
            c.set(5,32,0x6dd78eu32);
            pc = 0x4ed8ce; continue;
            }
            0x4edb1a => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4edb2a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0ed8),32));
            pc = 0x4ed8ce; continue;
            }
            0x4edb3c => {
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4edb5d; continue; }
            pc = 0x4edb53; continue;
            }
            0x4edb53 => {
            c.set(5,32,0x6dd78fu32);
            pc = 0x4ed8ce; continue;
            }
            0x4edb5d => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4edb6d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0ee8),32));
            pc = 0x4ed8ce; continue;
            }
            0x4edb7f => {
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,0xcu32);
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(5,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0da0),32));
            pc = 0x4ed8ce; continue;
            }
            0x4edb9c => {
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4edba6);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(5,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dd8c0),32));
            pc = 0x4ed8ce; continue;
            }
            0x4edbc1 => {
            c.set(1,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4edbd6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            pc = 0x4ed8ce; continue;
            }
            0x4edbdd => {
            c.set(1,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),8));
            { let a = c.r[0].wrapping_add(0xd); wm(m,a,8,c.get(1,8)); }
            pc = 0x4ed8ce; continue;
            }
            0x4edbf5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x1),8));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[2].wrapping_add(0xe); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(0,8,rm(m,c.r[3],8));
            { let a = c.r[2].wrapping_add(0xf); wm(m,a,8,c.get(0,8)); }
            pc = 0x4ed8ce; continue;
            }
            0x4edc11 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[0]; wm(m,a,8,0x25u32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x4ed8ce; continue;
            }
            0x4edc22 => {
            c.set(1,32,sx(rm(m,c.r[3],8),8)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4edc43; continue; }
            pc = 0x4edc39; continue;
            }
            0x4edc39 => {
            c.set(5,32,0x6dd790u32);
            pc = 0x4ed8ce; continue;
            }
            0x4edc43 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4edc53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,sx(rm(m,c.r[3].wrapping_add(0xffffffff),8),8)&0xffffu32);
            c.set(5,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,0x30u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1a),8));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c3040),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4edc9d; continue; }
            pc = 0x4edc7d; continue;
            }
            0x4edc7d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x4edc81; continue;
            }
            0x4edc81 => {
            { let a = c.r[2]; wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4edc81; continue; }
            pc = 0x4edc8c; continue;
            }
            0x4edc8c => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            pc = 0x4edc9d; continue;
            }
            0x4edc92 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x4edc99; continue;
            }
            0x4edc99 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            pc = 0x4edc9d; continue;
            }
            0x4edc9d => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1),8));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4ed5a4; continue; }
            pc = 0x4edca9; continue;
            }
            0x4edca9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,8,rm(m,c.r[3],8));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x6),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(6) { pc = 0x4edcd2; continue; }
            pc = 0x4edcbc; continue;
            }
            0x4edcbc => {
            c.set(1,8,rm(m,c.r[3],8));
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[3],8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4edcbc; continue; }
            pc = 0x4edcc8; continue;
            }
            0x4edcc8 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x6),8));
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4edcbc; continue; }
            pc = 0x4edcd2; continue;
            }
            0x4edcd2 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x8),32));
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4ede88; continue; }
            pc = 0x4edcf2; continue;
            }
            0x4edcf2 => {
            { let a = rm(m,c.r[7].wrapping_add(0x1),8); c.sub(a,0x5u32,8); }
            if c.cond(4) { pc = 0x4ede88; continue; }
            pc = 0x4edcfc; continue;
            }
            0x4edcfc => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(13) { pc = 0x4ede88; continue; }
            pc = 0x4edd0e; continue;
            }
            0x4edd0e => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x4edd10; continue;
            }
            0x4edd10 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x6),8));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x8u32,32); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            if c.cond(12) { pc = 0x4edd10; continue; }
            pc = 0x4edd28; continue;
            }
            0x4edd28 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.push(m,0x20u32);
            c.push(m,0xffu32);
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            c.push(m, 0x4edd40);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x4ede14; continue; }
            pc = 0x4edd57; continue;
            }
            0x4edd57 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x784),16));
            c.set(1,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4edeaf; continue; }
            pc = 0x4edd90; continue;
            }
            0x4edd90 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4edd9b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x2c4),16));
            c.push(m,c.get(2,32));
            c.push(m, 0x4eddb1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4eddf6; continue; }
            pc = 0x4eddb8; continue;
            }
            0x4eddb8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2c4),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2c4),16));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4eddf0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x8),32));
            { let a = rm(m,c.r[0].wrapping_add(0xa),8); let r = c.inc(a,8); { let a = c.r[0].wrapping_add(0xa); wm(m,a,8,r); } }
            pc = 0x4eddf6; continue;
            }
            0x4eddf6 => {
            c.set(1,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2ca),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x4edd90; continue; }
            pc = 0x4ede0f; continue;
            }
            0x4ede0f => {
            pc = 0x4edeaf; continue;
            }
            0x4ede14 => {
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4edeaf; continue; }
            pc = 0x4ede1d; continue;
            }
            0x4ede1d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x4edeaf; continue; }
            pc = 0x4ede30; continue;
            }
            0x4ede30 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.push(m, 0x4ede45);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4ede74; continue; }
            pc = 0x4ede4c; continue;
            }
            0x4ede4c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(3,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.push(m,c.get(5,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4ede6e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x8),32));
            { let a = rm(m,c.r[0].wrapping_add(0xa),8); let r = c.inc(a,8); { let a = c.r[0].wrapping_add(0xa); wm(m,a,8,r); } }
            pc = 0x4ede74; continue;
            }
            0x4ede74 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4ede30; continue; }
            pc = 0x4ede86; continue;
            }
            0x4ede86 => {
            pc = 0x4edeaf; continue;
            }
            0x4ede88 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4edeaf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4edeaf; continue;
            }
            0x4edeaf => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x7),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4edf31; continue; }
            pc = 0x4edeb6; continue;
            }
            0x4edeb6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.push(m,0x0u32);
            c.push(m,0x12u32);
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ededf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(5,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4edf19; continue; }
            pc = 0x4edef4; continue;
            }
            0x4edef4 => {
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4edf19; continue; }
            pc = 0x4edef9; continue;
            }
            0x4edef9 => {
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x4edf31; continue; }
            pc = 0x4edefe; continue;
            }
            0x4edefe => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4edf15);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4edf31; continue; }
            pc = 0x4edf19; continue;
            }
            0x4edf19 => {
            c.set(1,32,0x6e55d0u32);
            c.push(m, 0x4edf23);
            { let args = []; c.r[0] = host.call(m, 0x4f3940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4edf31; continue; }
            pc = 0x4edf27; continue;
            }
            0x4edf27 => {
            c.set(1,32,0x6cc960u32);
            c.push(m, 0x4edf31);
            { let args = []; c.r[0] = host.call(m, 0x47d1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4edf31; continue;
            }
            0x4edf31 => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4edf3a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f3960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f2c93; continue;
            }
            0x4edf3f => {
            c.set(1,32,c.r[4].wrapping_add(0xa4));
            c.push(m, 0x4edf4b);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x54c); wm(m,a,32,0x2u32); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0xa9); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,8); let r = c.and(a,0x1u32,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0xac); wm(m,a,16,0x0u32); }
            { let a = c.get(1,8); let r = c.add(a,0x45u32,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0xa8); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0xae); wm(m,a,16,c.get(1,16)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_mul(8).wrapping_add(0x672340),16));
            { let a = c.r[4].wrapping_add(0xb0); wm(m,a,16,c.get(2,16)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.r[4].wrapping_add(0xa4));
            c.push(m,c.get(1,32));
            c.set(0,16,rm(m,c.r[2].wrapping_mul(8).wrapping_add(0x672344),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0xb6); wm(m,a,16,c.get(0,16)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4edff2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[6].wrapping_add(0x14),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,r); } }
            c.set(1,32,c.r[4].wrapping_add(0xa4));
            { let a = c.r[4].wrapping_add(0x54c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4ee00d);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4f2c97; continue;
            }
            0x4ee012 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x4a4); wm(m,a,8,c.get(2,8)); }
            c.set(7,32,rm(m,c.r[7],32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4a4),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4f2c93; continue; }
            pc = 0x4ee042; continue;
            }
            0x4ee042 => {
            c.set(1,32,c.r[4].wrapping_add(0x514));
            c.push(m, 0x4ee04e);
            { let args = []; c.r[0] = host.call(m, 0x4f3900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x514));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x518); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x51c); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x528); wm(m,a,32,0x3f000000u32); }
            { let a = c.r[4].wrapping_add(0x51e); wm(m,a,16,0x0u32); }
            c.push(m, 0x4ee085);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f2c93; continue;
            }
            0x4ee08a => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x110); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(5,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x110),32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4ee0c2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4ee0d9; continue; }
            pc = 0x4ee0ce; continue;
            }
            0x4ee0ce => {
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            pc = 0x4ee0e5; continue;
            }
            0x4ee0d9 => {
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            pc = 0x4ee0e5; continue;
            }
            0x4ee0e5 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x4ee107; continue; }
            pc = 0x4ee0e9; continue;
            }
            0x4ee0e9 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x54),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4ee0ff; continue; }
            pc = 0x4ee0f5; continue;
            }
            0x4ee0f5 => {
            { let a = c.r[3].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,0x0u32); }
            pc = 0x4ee140; continue;
            }
            0x4ee0ff => {
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.r[3].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(7,32)); }
            pc = 0x4ee140; continue;
            }
            0x4ee107 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4ee10e);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x4ee11b; continue; }
            pc = 0x4ee112; continue;
            }
            0x4ee112 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4ee119);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            pc = 0x4ee11b; continue;
            }
            0x4ee11b => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            c.set(1,32,0xc8u32);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            if c.cond(5) { pc = 0x4ee132; continue; }
            pc = 0x4ee12d; continue;
            }
            0x4ee12d => {
            c.set(1,32,0x7d0u32);
            pc = 0x4ee132; continue;
            }
            0x4ee132 => {
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            pc = 0x4ee140; continue;
            }
            0x4ee140 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4ee147);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4ee150);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f2c93; continue;
            }
            0x4ee155 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2f0); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ee175);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0x11u32,16); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4ee183; continue;
            }
            0x4ee183 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2f0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[0].wrapping_add(0x3f); wm(m,a,8,c.get(2,8)); }
            { let a = rm(m,c.r[0].wrapping_add(0x3d),8); let r = c.or(a,0x2u32,8); { let a = c.r[0].wrapping_add(0x3d); wm(m,a,8,r); } }
            pc = 0x4f2c93; continue;
            }
            0x4ee1a2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4ee1c1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4ee1c8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f3790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f2c93; continue;
            }
            0x4ee1cd => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x118); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4ee21e; continue; }
            pc = 0x4ee1ee; continue;
            }
            0x4ee1ee => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4ee1fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4ee201);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x518640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x118),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ee21e => {
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4ee22c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4ee233);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5186d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f2c93; continue;
            }
            0x4ee238 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6dd730u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4ee257);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d7030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4f2c93; continue;
            }
            0x4ee25c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4ee2b3; continue; }
            pc = 0x4ee273; continue;
            }
            0x4ee273 => {
            c.push(m, 0x4ee278);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd791u32);
            c.push(m, 0x4ee289);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x554); wm(m,a,32,0x3u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd792u32);
            c.push(m, 0x4ee2a5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,0x19u32);
            pc = 0x4ee2f0; continue;
            }
            0x4ee2b3 => {
            c.push(m, 0x4ee2b8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd793u32);
            c.push(m, 0x4ee2c9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x554); wm(m,a,32,0x4u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd794u32);
            c.push(m, 0x4ee2e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,0x6de4aau32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0x1au32);
            pc = 0x4ee2f0; continue;
            }
            0x4ee2f0 => {
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            { let a = c.r[4].wrapping_add(0x560); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4ee306);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4f2c93; continue;
            }
            0x4ee30b => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,c.get(1,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x3e0); wm(m,a,8,c.get(1,8)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x3e0),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4ee35a; continue; }
            pc = 0x4ee34f; continue;
            }
            0x4ee34f => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            pc = 0x4ee366; continue;
            }
            0x4ee35a => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            pc = 0x4ee366; continue;
            }
            0x4ee366 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4ee3d2; continue; }
            pc = 0x4ee36a; continue;
            }
            0x4ee36a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[5].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(7,32,c.r[0].wrapping_mul(4).wrapping_add(0x4));
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            if c.cond(12) { pc = 0x4ee3b4; continue; }
            pc = 0x4ee39e; continue;
            }
            0x4ee39e => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[0]); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = rm(m,c.r[7].wrapping_add(c.r[2]),32); let r = c.add(a,c.get(1,32),32); { let a = c.r[7].wrapping_add(c.r[2]); wm(m,a,32,r); } }
            pc = 0x4ee403; continue;
            }
            0x4ee3b4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[0]); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.r[7].wrapping_add(c.r[2]); wm(m,a,32,0x0u32); }
            pc = 0x4ee403; continue;
            }
            0x4ee3d2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[0]); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[5].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(7,32,rm(m,0x6de508u32,32));
            c.set(2,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4));
            { let a = rm(m,c.r[2],32); let r = c.add(a,c.get(1,32),32); { let a = c.r[2]; wm(m,a,32,r); } }
            pc = 0x4ee403; continue;
            }
            0x4ee403 => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            if c.cond(4) { pc = 0x4ee42d; continue; }
            pc = 0x4ee410; continue;
            }
            0x4ee410 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0]),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ee42d => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0]),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            pc = 0x4ee432; continue;
            }
            0x4ee432 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ee44a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(3,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(1,8)); }
            c.push(m,0x0u32);
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(4)),8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(4)),32));
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(1,16,c.get(1,8));
            c.set(0,8,c.get(2,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ee48f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4ee51d; continue; }
            pc = 0x4ee49b; continue;
            }
            0x4ee49b => {
            c.set(7,32,c.get(5,32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4ee4b8; continue; }
            pc = 0x4ee4ad; continue;
            }
            0x4ee4ad => {
            c.set(2,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            pc = 0x4ee4c4; continue;
            }
            0x4ee4b8 => {
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(2,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            pc = 0x4ee4c4; continue;
            }
            0x4ee4c4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4ee50b; continue; }
            pc = 0x4ee4d5; continue;
            }
            0x4ee4d5 => {
            c.set(5,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(3,32,c.get(2,32));
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4ee4f7; continue; }
            pc = 0x4ee4e1; continue;
            }
            0x4ee4e1 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            pc = 0x4ee534; continue;
            }
            0x4ee4f7 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[1].wrapping_add(c.r[2]); wm(m,a,32,c.get(5,32)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,0x0u32); }
            pc = 0x4ee534; continue;
            }
            0x4ee50b => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[1].wrapping_add(c.r[3]); wm(m,a,32,c.get(2,32)); }
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[0].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            pc = 0x4ee534; continue;
            }
            0x4ee51d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,32,0x0u32); }
            pc = 0x4ee534; continue;
            }
            0x4ee534 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4ee432; continue; }
            pc = 0x4ee54f; continue;
            }
            0x4ee54f => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ee567 => {
            c.set(1,32,c.r[4].wrapping_add(0xc8));
            c.push(m, 0x4ee573);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(1,32,0x6df2f0u32);
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x120); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4ee595);
            { let args = []; c.r[0] = host.call(m, 0x4f3690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4f2c93; continue; }
            pc = 0x4ee59d; continue;
            }
            0x4ee59d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x120),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0xd6); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0x8000u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8000u32,32); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4ee5dd; continue;
            }
            0x4ee5dd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x2d4),32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x789),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4ee618; continue; }
            pc = 0x4ee605; continue;
            }
            0x4ee605 => {
            c.set(0,32,0x1b4e81b5u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            pc = 0x4ee629; continue;
            }
            0x4ee618 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            pc = 0x4ee629; continue;
            }
            0x4ee629 => {
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0xd2); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0xd0); wm(m,a,16,0x0u32); }
            c.push(m, 0x4ee645);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0xd5));
            c.push(m,0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0xd8));
            { let a = c.get(0,32); let r = c.add(a,0xau32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xd8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4ee668);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x525b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = rm(m,c.r[4].wrapping_add(0xd4),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4ee676; continue;
            }
            0x4ee676 => {
            c.set(0,32,c.r[4].wrapping_add(0xc8));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0xcc); wm(m,a,32,0x7du32); }
            c.push(m, 0x4ee693);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f2c93; continue;
            }
            0x4ee698 => {
            c.set(1,32,c.r[4].wrapping_add(0xdc));
            c.push(m, 0x4ee6a4);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(3,8,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),8));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x2f8); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4ee6ce);
            { let args = []; c.r[0] = host.call(m, 0x4f3690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4f2c93; continue; }
            pc = 0x4ee6d6; continue;
            }
            0x4ee6d6 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2f8),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.push(m, 0x4ee6ed);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4ee6f5; continue;
            }
            0x4ee6f5 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ee700);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f3660, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0xea); wm(m,a,8,c.get(3,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x789),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x2d4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_mul(8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4ee744; continue; }
            pc = 0x4ee731; continue;
            }
            0x4ee731 => {
            c.set(0,32,0x1b4e81b5u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            pc = 0x4ee755; continue;
            }
            0x4ee744 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            pc = 0x4ee755; continue;
            }
            0x4ee755 => {
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0xe6); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0xe4); wm(m,a,16,0x0u32); }
            c.push(m, 0x4ee771);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,0xau32,32); c.set(0,32,r); }
            c.set(3,32,0x53u32);
            { let a = c.r[4].wrapping_add(0xe0); wm(m,a,32,c.get(0,32)); }
            c.set(5,32,0x7du32);
            pc = 0x4ee785; continue;
            }
            0x4ee785 => {
            c.set(2,32,c.r[4].wrapping_add(0xe9));
            c.push(m,c.get(3,32));
            c.set(0,32,c.r[4].wrapping_add(0xec));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4ee79d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x525b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = rm(m,c.r[4].wrapping_add(0xe8),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4ee7c0; continue; }
            pc = 0x4ee7a7; continue;
            }
            0x4ee7a7 => {
            c.set(1,32,c.r[4].wrapping_add(0xdc));
            { let a = c.r[4].wrapping_add(0xdc); wm(m,a,32,c.get(5,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4ee7c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4ee7c0; continue;
            }
            0x4ee7c0 => {
            { let a = c.get(3,32); let r = c.add(a,0xcu32,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x9bu32,32); }
            if c.cond(12) { pc = 0x4ee785; continue; }
            pc = 0x4ee7cb; continue;
            }
            0x4ee7cb => {
            pc = 0x4f2c93; continue;
            }
            0x4ee7d0 => {
            c.set(1,32,c.r[4].wrapping_add(0x8c));
            c.push(m, 0x4ee7dc);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4ee7e6);
            { let args = []; c.r[0] = host.call(m, 0x4f3690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4f2c93; continue; }
            pc = 0x4ee7ee; continue;
            }
            0x4ee7ee => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0x9e); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x9e),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2c8),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.and(a,0x8u32,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.sub(a,0x8u32,8); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4ee83d; continue;
            }
            0x4ee83d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x790),16));
            c.set(1,32,c.get(7,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x792),16));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            c.set(3,8,u32::from(c.cond(12)));
            c.push(m, 0x4ee86f);
            { let args = []; c.r[0] = host.call(m, 0x4f36b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,16,c.get(0,16));
            c.set(1,32,c.get(7,32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.push(m, 0x4ee87f);
            { let args = []; c.r[0] = host.call(m, 0x4f36a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x2d4),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4ee8a6; continue; }
            pc = 0x4ee8a5; continue;
            }
            0x4ee8a5 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            pc = 0x4ee8a6; continue;
            }
            0x4ee8a6 => {
            c.push(m, 0x4ee8ab);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6de4a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x9c); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x4ee8cb);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0xau32);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x94),32));
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4ee8f6);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x94),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            pc = 0x4ee908; continue;
            }
            0x4ee908 => {
            c.push(m, 0x4ee90d);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.and(a,0x1fu32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,0x31u32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(c.r[7]).wrapping_add(0x90); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4ee908; continue; }
            pc = 0x4ee91e; continue;
            }
            0x4ee91e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x9e),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x66666667u32);
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2d4),32));
            { let a = c.r[4].wrapping_add(0x98); wm(m,a,16,0x0u32); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x8c));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x9e); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x90); wm(m,a,32,0x7cu32); }
            c.push(m, 0x4ee97f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f2c93; continue;
            }
            0x4ee984 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x128); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4ee9a7; continue;
            }
            0x4ee9a7 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ee9b7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x128),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.set(1,32,c.get(0,32));
            { let a = c.r[0].wrapping_add(0x40); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x4ee9d6);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4f2c93; continue;
            }
            0x4ee9db => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4ee9f4; continue;
            }
            0x4ee9f4 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4eea04);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4eea0f; continue;
            }
            0x4eea0f => {
            c.set(1,32,c.get(0,32));
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            c.push(m, 0x4eea1d);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4f2c93; continue;
            }
            0x4eea22 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x458); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x458),32));
            { let a = c.r[4].wrapping_add(0x130); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4eea54);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x130),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eea6e => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x300); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4eea82);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            c.push(m, 0x4eea91);
            { let args = []; c.r[0] = host.call(m, 0x4f3850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x300),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(7,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eeaab => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x138); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x138),32));
            { let a = c.r[4].wrapping_add(0x3e8); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x140); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4eeaed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4eeb41; continue; }
            pc = 0x4eeafa; continue;
            }
            0x4eeafa => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x3e8),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            pc = 0x4eeb0a; continue;
            }
            0x4eeb0a => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4eeb1a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            { let a = c.get(1,32); c.sub(a,0x3u32,32); }
            if c.cond(15) { pc = 0x4eeb2e; continue; }
            pc = 0x4eeb25; continue;
            }
            0x4eeb25 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xe),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4eeb30; continue; }
            pc = 0x4eeb2e; continue;
            }
            0x4eeb2e => {
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            pc = 0x4eeb30; continue;
            }
            0x4eeb30 => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4eeb0a; continue; }
            pc = 0x4eeb3d; continue;
            }
            0x4eeb3d => {
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,c.get(3,8)); }
            pc = 0x4eeb41; continue;
            }
            0x4eeb41 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x140),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eeb62 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(7,16),16); }
            { let a = c.r[4].wrapping_add(0x308); wm(m,a,8,c.get(0,8)); }
            if c.cond(6) { pc = 0x4eeb9b; continue; }
            pc = 0x4eeb79; continue;
            }
            0x4eeb79 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4eeb84);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4eeb8d; continue; }
            pc = 0x4eeb8b; continue;
            }
            0x4eeb8b => {
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            pc = 0x4eeb8d; continue;
            }
            0x4eeb8d => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4eeb79; continue; }
            pc = 0x4eeb9b; continue;
            }
            0x4eeb9b => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x308),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m, 0x4eebb5);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[7]),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(1,32,sx(c.get(3,8),8)&0xffffffffu32);
            { let a = c.get(3,8); let r = c.or(a,0xffu32,8); c.set(3,8,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2d4),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(5,16),16); }
            { let a = c.r[4].wrapping_add(0x148); wm(m,a,8,c.get(2,8)); }
            if c.cond(6) { pc = 0x4eec2e; continue; }
            pc = 0x4eebf1; continue;
            }
            0x4eebf1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x148),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x4eec01; continue;
            }
            0x4eec01 => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4eec0c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4eec15; continue; }
            pc = 0x4eec13; continue;
            }
            0x4eec13 => {
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            pc = 0x4eec15; continue;
            }
            0x4eec15 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4eec2e; continue; }
            pc = 0x4eec20; continue;
            }
            0x4eec20 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4eec01; continue; }
            pc = 0x4eec2e; continue;
            }
            0x4eec2e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[7].wrapping_add(c.r[2]); wm(m,a,32,c.get(5,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eec39 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x494); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4eec4d);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x494),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eec65 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x150); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x150),32));
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4eec97);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x31),8));
            c.push(m, 0x4eeca6);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x4eecc6; continue; }
            pc = 0x4eecaa; continue;
            }
            0x4eecaa => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x88),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,0x1u32); }
            pc = 0x4f2c93; continue;
            }
            0x4eecc6 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x88),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            pc = 0x4f2c93; continue;
            }
            0x4eece2 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x158); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x310); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4eed03);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x310),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x158),32));
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eed2d => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x3f0); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3f0),32));
            { let a = c.r[4].wrapping_add(0x160); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4eed5f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x160),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eed7b => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x318); wm(m,a,8,c.get(2,8)); }
            c.set(7,32,0x80000000u32);
            { let a = rm(m,c.r[0].wrapping_add(0xc),8); c.sub(a,0x1u32,8); }
            if c.cond(6) { pc = 0x4eedd2; continue; }
            pc = 0x4eed95; continue;
            }
            0x4eed95 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x4eeddc; continue; }
            pc = 0x4eeda0; continue;
            }
            0x4eeda0 => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4eedab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4eedc2; continue; }
            pc = 0x4eedb2; continue;
            }
            0x4eedb2 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x46c),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x4eedc2; continue; }
            pc = 0x4eedbc; continue;
            }
            0x4eedbc => {
            c.set(7,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,c.get(3,8)); }
            pc = 0x4eedc2; continue;
            }
            0x4eedc2 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4eeda0; continue; }
            pc = 0x4eedd0; continue;
            }
            0x4eedd0 => {
            pc = 0x4eeddc; continue;
            }
            0x4eedd2 => {
            c.set(2,8,rm(m,0x6dfc14u32,8));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,c.get(2,8)); }
            pc = 0x4eeddc; continue;
            }
            0x4eeddc => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x318),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eedfd => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x168); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x6de4a0u32);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x168),32));
            { let a = c.r[4].wrapping_add(0x460); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m, 0x4eee2b);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.r[7].wrapping_add(c.r[2]); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4eee3b);
            { let args = []; c.r[0] = host.call(m, 0x4f38a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4eee76; continue; }
            pc = 0x4eee44; continue;
            }
            0x4eee44 => {
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4eee4e);
            { let args = []; c.r[0] = host.call(m, 0x4f38a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4eee5e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[1],32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],32); let r = c.add(a,c.get(2,32),32); { let a = c.r[1]; wm(m,a,32,r); } }
            pc = 0x4eee76; continue;
            }
            0x4eee76 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x8),16));
            c.set(3,32,rm(m,c.r[0],32));
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x460),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[7]),32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(4)),32); c.idiv32(b); }
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eeeaa => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x170); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x170),32));
            c.set(7,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(7,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eeed1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(1,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eeef3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eef15 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eef36 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(3,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(4)),32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eef59 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.r[4].wrapping_add(0x320); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4eef6d);
            { let args = []; c.r[0] = host.call(m, 0x506290, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x320),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4eef8a => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,32,c.r[4].wrapping_add(0x58));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0xffffu32);
            { let a = c.r[4].wrapping_add(0x180); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x400); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4eefbb);
            { let args = []; c.r[0] = host.call(m, 0x515750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x180),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4eefdb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            c.set(5,32,c.get(7,32));
            if c.cond(3) { pc = 0x4ef036; continue; }
            pc = 0x4eefea; continue;
            }
            0x4eefea => {
            c.set(0,32,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4eeffc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x120),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(7) { pc = 0x4ef034; continue; }
            pc = 0x4ef00a; continue;
            }
            0x4ef00a => {
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4ef012; continue; }
            pc = 0x4ef00e; continue;
            }
            0x4ef00e => {
            c.set(3,32,c.get(1,32));
            pc = 0x4ef023; continue;
            }
            0x4ef012 => {
            if c.cond(5) { pc = 0x4ef025; continue; }
            pc = 0x4ef014; continue;
            }
            0x4ef014 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4ef025; continue; }
            pc = 0x4ef01f; continue;
            }
            0x4ef01f => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            pc = 0x4ef023; continue;
            }
            0x4ef023 => {
            c.set(5,32,c.get(7,32));
            pc = 0x4ef025; continue;
            }
            0x4ef025 => {
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4eefea; continue; }
            pc = 0x4ef032; continue;
            }
            0x4ef032 => {
            pc = 0x4ef036; continue;
            }
            0x4ef034 => {
            c.set(5,32,c.get(7,32));
            pc = 0x4ef036; continue;
            }
            0x4ef036 => {
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(5) { pc = 0x4ef044; continue; }
            pc = 0x4ef03f; continue;
            }
            0x4ef03f => {
            c.set(5,32,0xffffu32);
            pc = 0x4ef044; continue;
            }
            0x4ef044 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3f8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(5,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef062 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x180); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x180),32));
            { let a = c.r[4].wrapping_add(0x328); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ef094);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4ef0f5; continue; }
            pc = 0x4ef0a1; continue;
            }
            0x4ef0a1 => {
            c.set(5,32,0x3u32);
            pc = 0x4ef0a6; continue;
            }
            0x4ef0a6 => {
            c.set(2,32,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4ef0b9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x8),16));
            { let a = rm(m,0x6dd896u32,16); c.sub(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x4ef0de; continue; }
            pc = 0x4ef0c8; continue;
            }
            0x4ef0c8 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ef0d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f3870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x38),16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x4ef0e8; continue; }
            pc = 0x4ef0de; continue;
            }
            0x4ef0de => {
            { let a = rm(m,c.r[3].wrapping_add(0x134),16); c.sub(a,0x14u32,16); }
            if c.cond(5) { pc = 0x4ef0f5; continue; }
            pc = 0x4ef0e8; continue;
            }
            0x4ef0e8 => {
            c.set(7,16,rm(m,c.r[3].wrapping_add(0x4),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4ef0a6; continue; }
            pc = 0x4ef0f5; continue;
            }
            0x4ef0f5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x328),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(7,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef112 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x188); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x188),32));
            { let a = c.r[4].wrapping_add(0x46c); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ef144);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x46c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1d),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(1,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef161 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x190); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x190),32));
            { let a = c.r[4].wrapping_add(0x330); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4ef192);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x330),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1c),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(1,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef1af => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x198); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x198),32));
            { let a = c.r[4].wrapping_add(0x400); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4ef1e0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x400),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(1,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef1fd => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x1a0); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1a0),32));
            { let a = c.r[4].wrapping_add(0x1a8); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4ef22e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4ef235);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1a8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x338); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x338),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef260 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x468); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x468),32));
            { let a = c.r[4].wrapping_add(0x1b0); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ef292);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4ef299);
            { let args = []; c.r[0] = host.call(m, 0x4f3760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1b0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef2b6 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,0x6dd73cu32);
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4ef2cc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.r[3].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(1,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef2df => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4ef3a2; continue; }
            pc = 0x4ef2fa; continue;
            }
            0x4ef2fa => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,8,rm(m,c.r[7].wrapping_add(0x6),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(1,16)); }
            pc = 0x4f2c97; continue;
            }
            0x4ef30b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4ef3ce; continue; }
            pc = 0x4ef326; continue;
            }
            0x4ef326 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(4,8,rm(m,c.r[7].wrapping_add(0x6),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(0,16)); }
            pc = 0x4f2c97; continue;
            }
            0x4ef337 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4ef2fa; continue; }
            pc = 0x4ef34e; continue;
            }
            0x4ef34e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(4,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(0,16)); }
            pc = 0x4f2c97; continue;
            }
            0x4ef35f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4ef326; continue; }
            pc = 0x4ef376; continue;
            }
            0x4ef376 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(6,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(2,16)); }
            pc = 0x4f2c97; continue;
            }
            0x4ef387 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4ef2fa; continue; }
            pc = 0x4ef3a2; continue;
            }
            0x4ef3a2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(4,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(0,16)); }
            pc = 0x4f2c97; continue;
            }
            0x4ef3b3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4ef326; continue; }
            pc = 0x4ef3ce; continue;
            }
            0x4ef3ce => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(6,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(2,16)); }
            pc = 0x4f2c97; continue;
            }
            0x4ef3df => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(1,16)); }
            pc = 0x4f2c97; continue;
            }
            0x4ef3f0 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x340); wm(m,a,8,c.get(0,8)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x340),32));
            { let a = c.r[4].wrapping_add(0x1b8); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1b8),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,0xffffffffu32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4f2c93; continue; }
            pc = 0x4ef445; continue;
            }
            0x4ef445 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4ef450);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(1,16); c.sub(a,0xfu32,16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4ef461; continue;
            }
            0x4ef461 => {
            { let a = c.get(1,16); c.sub(a,0x2u32,16); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4ef46b; continue;
            }
            0x4ef46b => {
            { let a = c.get(1,16); c.sub(a,0x3u32,16); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4ef475; continue;
            }
            0x4ef475 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x39),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef485 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x408); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4ef499);
            { let args = []; c.r[0] = host.call(m, 0x50df00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x408),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef4b6 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x1c0); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x4ef4ca);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.set(3,32,rm(m,c.r[2].wrapping_add(0xccc),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(7,32); let r = c.imul(a,c.get(0,32),32); c.set(7,32,r); }
            c.push(m, 0x4ef4f9);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef518 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x348); wm(m,a,8,c.get(2,8)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x348),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m, 0x4ef53c);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[2]));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x8),16));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x20bc),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x3u32,32); c.set(2,32,r); }
            c.set(1,32,0x64u32);
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[0],32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.r[7]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef588 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x1c8); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,0x6de4a0u32);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c8),32));
            { let a = c.r[4].wrapping_add(0x478); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m, 0x4ef5b6);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[2]));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x8),16));
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4910),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x478),32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[0],32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x3); wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x4ef61b; continue; }
            pc = 0x4ef609; continue;
            }
            0x4ef609 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(4) { pc = 0x4ef62f; continue; }
            pc = 0x4ef60e; continue;
            }
            0x4ef60e => {
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.set(0,8,!a & 0xffu32); }
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            if c.cond(4) { pc = 0x4ef62f; continue; }
            pc = 0x4ef61b; continue;
            }
            0x4ef61b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.push(m, 0x4ef62a);
            { let args = []; c.r[0] = host.call(m, 0x4f38d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4ef6f7; continue;
            }
            0x4ef62f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,0x6ddaa0u32);
            c.set(3,32,c.r[7].wrapping_add(c.r[2]));
            c.push(m, 0x4ef63f);
            { let args = []; c.r[0] = host.call(m, 0x4f38e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,c.get(0,16));
            c.set(0,32,rm(m,c.r[3],32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[3]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,8); let r = c.or(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef65f => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x1d0); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1d0),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x8),16));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[1]));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x3eec),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[2]));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4910),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(5,32,c.r[7].wrapping_add(c.r[0]));
            c.push(m, 0x4ef6cb);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[5],32),32); c.set(0,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[1]));
            { let a = c.get(3,32); let r = c.and(a,0xfffffffu32,32); c.set(3,32,r); }
            c.set(1,32,0x6ddaa0u32);
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.push(m, 0x4ef6f7);
            { let args = []; c.r[0] = host.call(m, 0x4f38f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4ef6f7; continue;
            }
            0x4ef6f7 => {
            c.set(1,16,c.get(0,16));
            c.set(0,32,rm(m,c.r[7],32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[7]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef70c => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x350); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x350),32));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4ef752; continue; }
            pc = 0x4ef73d; continue;
            }
            0x4ef73d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x70),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            pc = 0x4f2c93; continue;
            }
            0x4ef752 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ef75d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xa),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4ef76e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d7720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef788 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x1d8); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1d8),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),16));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1bu32); }
            { let a = c.r[0].wrapping_add(c.r[5]); wm(m,a,32,0xffffffffu32); }
            { let a = rm(m,c.r[6].wrapping_add(0x10),16); c.sub(a,0x134u32,16); }
            if c.cond(5) { pc = 0x4ef7d6; continue; }
            pc = 0x4ef7ce; continue;
            }
            0x4ef7ce => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x50u32); }
            pc = 0x4ef7d6; continue;
            }
            0x4ef7d6 => {
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4ef7e3; continue;
            }
            0x4ef7e3 => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ef7f4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4ef805; continue;
            }
            0x4ef805 => {
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            pc = 0x4ef818; continue;
            }
            0x4ef814 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4ef818; continue;
            }
            0x4ef818 => {
            c.set(0,32,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4ef82a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,rm(m,c.r[3].wrapping_add(0x672340),32),32); c.set(1,32,r); }
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,rm(m,c.r[3].wrapping_add(0x672344),32),32); c.set(2,32,r); }
            c.set(3,32,c.get(2,32));
            { let a = c.get(3,32); let r = c.imul(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4ef864; continue; }
            pc = 0x4ef852; continue;
            }
            0x4ef852 => {
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4ef814; continue; }
            pc = 0x4ef85f; continue;
            }
            0x4ef85f => {
            pc = 0x4f2c93; continue;
            }
            0x4ef864 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.r[0].wrapping_add(c.r[5]); wm(m,a,32,c.get(7,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef875 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,8); c.sub(a,0x27u32,8); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.r[4].wrapping_add(0x410); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xfu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4ef8b5; continue; }
            pc = 0x4ef8a9; continue;
            }
            0x4ef8a9 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            pc = 0x4ef8b7; continue;
            }
            0x4ef8b5 => {
            c.set(1,32,c.get(0,32));
            pc = 0x4ef8b7; continue;
            }
            0x4ef8b7 => {
            c.set(5,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.sbb(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            { let a = c.get(5,32); let r = c.and(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x4ef9c4; continue; }
            pc = 0x4ef8d9; continue;
            }
            0x4ef8d9 => {
            c.set(3,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            pc = 0x4ef8de; continue;
            }
            0x4ef8de => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ef8e9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4ef977; continue; }
            pc = 0x4ef8f4; continue;
            }
            0x4ef8f4 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4ef977; continue; }
            pc = 0x4ef908; continue;
            }
            0x4ef908 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4ef913);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x4ef93c; continue; }
            pc = 0x4ef921; continue;
            }
            0x4ef921 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x410),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4ef952; continue; }
            pc = 0x4ef93c; continue;
            }
            0x4ef93c => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4ef908; continue; }
            pc = 0x4ef950; continue;
            }
            0x4ef950 => {
            pc = 0x4ef977; continue;
            }
            0x4ef952 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x30),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(3,16); let r = c.add(a,rm(m,c.r[0].wrapping_add(0x1e),16),16); c.set(3,16,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x534); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(3,16)); }
            pc = 0x4ef977; continue;
            }
            0x4ef977 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4ef8de; continue; }
            pc = 0x4ef984; continue;
            }
            0x4ef984 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x4ef9c4; continue; }
            pc = 0x4ef98c; continue;
            }
            0x4ef98c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x84),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x534),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4ef9c4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,0xffffffffu32); }
            pc = 0x4f2c93; continue;
            }
            0x4ef9df => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1e0); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1e0),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            c.set(3,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),16));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(3,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4efa1e; continue;
            }
            0x4efa1e => {
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(3,32));
            c.push(m, 0x4efa2f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4f2c93; continue; }
            pc = 0x4efa3a; continue;
            }
            0x4efa3a => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4efa45);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.r[7].wrapping_add(c.r[2]); wm(m,a,32,c.get(1,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4efa54 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x358); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x1e8); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x358),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1e8),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x480); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[1]); wm(m,a,32,0xffffffffu32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4efaad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4efabe; continue;
            }
            0x4efabe => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x480),32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            pc = 0x4efad2; continue;
            }
            0x4efad2 => {
            c.set(2,32,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4efae5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x39),8));
            c.set(2,32,rm(m,c.r[3].wrapping_add(c.r[0]),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4efb19; continue; }
            pc = 0x4efaf6; continue;
            }
            0x4efaf6 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4efafd);
            { let args = []; c.r[0] = host.call(m, 0x519c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4efb15; continue; }
            pc = 0x4efb01; continue;
            }
            0x4efb01 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x136),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4efb15; continue; }
            pc = 0x4efb0b; continue;
            }
            0x4efb0b => {
            { let a = rm(m,c.r[5].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(2) { pc = 0x4efb2b; continue; }
            pc = 0x4efb15; continue;
            }
            0x4efb15 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4efb19; continue;
            }
            0x4efb19 => {
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x4),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4efad2; continue; }
            pc = 0x4efb26; continue;
            }
            0x4efb26 => {
            pc = 0x4f2c93; continue;
            }
            0x4efb2b => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,32,c.get(7,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4efb40 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x1f0); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x360); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1f0),32));
            { let a = c.r[4].wrapping_add(0x1f8); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x360),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4efb8d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x531b70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1f8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4efbaa => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x208); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x418); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x200); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x418),32));
            { let a = c.r[4].wrapping_add(0x470); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x200),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x368); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x368),32));
            { let a = c.r[4].wrapping_add(0x4fc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x208),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x500); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x470),32));
            c.push(m,0xffffu32);
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[0]),32));
            { let a = c.r[4].wrapping_add(0x508); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,c.r[4].wrapping_add(0x500));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[5]),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(1,8,u32::from(c.cond(5)));
            c.push(m,c.get(1,32));
            c.push(m,0x3u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4efc7c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x533210, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x210); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            { let a = c.r[2].wrapping_add(c.r[0]); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[0]),32));
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[5]),32));
            { let a = c.r[3].wrapping_add(c.r[0]); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x210),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4efcb9 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x228); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x370); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x6),8));
            { let a = c.r[4].wrapping_add(0x420); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x370),32));
            { let a = c.r[4].wrapping_add(0x378); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x218); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x218),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x420),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            { let a = c.r[4].wrapping_add(0x220); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x508); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x378),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x50c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x220),32));
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[0]),32));
            { let a = c.r[4].wrapping_add(0x510); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x22c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,c.r[4].wrapping_add(0x50c));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(2,8,u32::from(c.cond(5)));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[5]),32));
            c.push(m,0x3u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4efda2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x533210, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x4b0); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            { let a = c.r[2].wrapping_add(c.r[0]); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[3]),32));
            { let a = c.r[0].wrapping_add(c.r[7]); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[5]),32));
            { let a = c.r[0].wrapping_add(c.r[3]); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4b0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4efddf => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            if c.cond(3) { pc = 0x4efe7b; continue; }
            pc = 0x4efe03; continue;
            }
            0x4efe03 => {
            c.set(7,32,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x4efe16);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x39),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4efe63; continue; }
            pc = 0x4efe1b; continue;
            }
            0x4efe1b => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4efe26);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0x2u32,16); }
            if c.cond(2) { pc = 0x4efe49; continue; }
            pc = 0x4efe33; continue;
            }
            0x4efe33 => {
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(2) { pc = 0x4efe3f; continue; }
            pc = 0x4efe39; continue;
            }
            0x4efe39 => {
            { let a = c.get(0,16); c.sub(a,0x8u32,16); }
            if c.cond(6) { pc = 0x4efe49; continue; }
            pc = 0x4efe3f; continue;
            }
            0x4efe3f => {
            { let a = c.get(0,16); c.sub(a,0x10u32,16); }
            if c.cond(4) { pc = 0x4efe49; continue; }
            pc = 0x4efe45; continue;
            }
            0x4efe45 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4efe4e; continue;
            }
            0x4efe49 => {
            c.set(0,32,0x1u32);
            pc = 0x4efe4e; continue;
            }
            0x4efe4e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4efe63 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            pc = 0x4f2c93; continue;
            }
            0x4efe7b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            pc = 0x4f2c93; continue;
            }
            0x4efe91 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x230); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x230),32));
            { let a = c.r[4].wrapping_add(0x380); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x238); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4efecc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x238),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x380),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4eff0e; continue;
            }
            0x4eff0e => {
            c.set(5,32,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.push(m, 0x4eff21);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4eff51; continue; }
            pc = 0x4eff2c; continue;
            }
            0x4eff2c => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x4eff51; continue; }
            pc = 0x4eff36; continue;
            }
            0x4eff36 => {
            { let a = rm(m,c.r[0].wrapping_add(0x136),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4eff51; continue; }
            pc = 0x4eff3f; continue;
            }
            0x4eff3f => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x120),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4eff51; continue; }
            pc = 0x4eff49; continue;
            }
            0x4eff49 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(3,32,c.get(1,32));
            { let a = c.r[7].wrapping_add(c.r[2]); wm(m,a,32,c.get(5,32)); }
            pc = 0x4eff51; continue;
            }
            0x4eff51 => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4eff0e; continue; }
            pc = 0x4eff5e; continue;
            }
            0x4eff5e => {
            pc = 0x4f2c93; continue;
            }
            0x4eff63 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x428); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x428),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x240); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x248); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x388); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7]),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4effac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x240),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x248),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x388),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,0xffffffffu32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x47c); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            c.set(3,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4f2c93; continue; }
            pc = 0x4f0010; continue;
            }
            0x4f0010 => {
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f0021; continue;
            }
            0x4f0021 => {
            c.set(3,32,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x4f0034);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = rm(m,c.r[7].wrapping_add(0x39),8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4f0096; continue; }
            pc = 0x4f003f; continue;
            }
            0x4f003f => {
            { let a = rm(m,c.r[7].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x4f0096; continue; }
            pc = 0x4f0049; continue;
            }
            0x4f0049 => {
            { let a = rm(m,c.r[7].wrapping_add(0x136),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4f0096; continue; }
            pc = 0x4f0052; continue;
            }
            0x4f0052 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(2) { pc = 0x4f007e; continue; }
            pc = 0x4f0059; continue;
            }
            0x4f0059 => {
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            if c.cond(6) { pc = 0x4f0096; continue; }
            pc = 0x4f005d; continue;
            }
            0x4f005d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x47c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f0075);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f3660, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x2c8),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4f0096; continue; }
            pc = 0x4f007e; continue;
            }
            0x4f007e => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x120),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4f0096; continue; }
            pc = 0x4f008c; continue;
            }
            0x4f008c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[2].wrapping_add(c.r[5]); wm(m,a,32,c.get(3,32)); }
            pc = 0x4f0096; continue;
            }
            0x4f0096 => {
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4f0021; continue; }
            pc = 0x4f00a7; continue;
            }
            0x4f00a7 => {
            pc = 0x4f2c93; continue;
            }
            0x4f00ac => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x250); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x390); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x250),32));
            { let a = c.r[4].wrapping_add(0x258); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x430); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f00f2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x390),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x430),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x258),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            { let a = c.r[4].wrapping_add(0x260); wm(m,a,8,c.get(2,8)); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f014a; continue;
            }
            0x4f014a => {
            c.set(5,32,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.push(m, 0x4f015d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4f0198; continue; }
            pc = 0x4f0168; continue;
            }
            0x4f0168 => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x4f0198; continue; }
            pc = 0x4f0172; continue;
            }
            0x4f0172 => {
            { let a = rm(m,c.r[0].wrapping_add(0x136),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4f0198; continue; }
            pc = 0x4f017b; continue;
            }
            0x4f017b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x260),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x4f0198; continue; }
            pc = 0x4f0190; continue;
            }
            0x4f0190 => {
            c.set(3,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,c.get(5,32)); }
            pc = 0x4f0198; continue;
            }
            0x4f0198 => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4f014a; continue; }
            pc = 0x4f01a5; continue;
            }
            0x4f01a5 => {
            pc = 0x4f2c93; continue;
            }
            0x4f01aa => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x398); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x398),32));
            { let a = c.r[4].wrapping_add(0x268); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x49c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f01e6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x49c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x268),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(5,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f022d; continue;
            }
            0x4f022d => {
            c.set(3,32,c.get(5,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x4f0240);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = rm(m,c.r[5].wrapping_add(0x39),8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4f027b; continue; }
            pc = 0x4f024b; continue;
            }
            0x4f024b => {
            { let a = rm(m,c.r[5].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x4f027b; continue; }
            pc = 0x4f0255; continue;
            }
            0x4f0255 => {
            { let a = rm(m,c.r[5].wrapping_add(0x136),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4f027b; continue; }
            pc = 0x4f025e; continue;
            }
            0x4f025e => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4f0265);
            { let args = []; c.r[0] = host.call(m, 0x5170f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),32),32); }
            if c.cond(6) { pc = 0x4f027b; continue; }
            pc = 0x4f026b; continue;
            }
            0x4f026b => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x120),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[7].wrapping_add(c.r[2]); wm(m,a,32,c.get(3,32)); }
            pc = 0x4f027b; continue;
            }
            0x4f027b => {
            c.set(5,16,rm(m,c.r[5].wrapping_add(0x4),16));
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4f022d; continue; }
            pc = 0x4f0288; continue;
            }
            0x4f0288 => {
            pc = 0x4f2c93; continue;
            }
            0x4f028d => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x270); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x3a0); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x270),32));
            { let a = c.r[4].wrapping_add(0x278); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x438); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f02d3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x278),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3a0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x438),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,32,0xffffffffu32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,0x0u32); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f0334; continue;
            }
            0x4f0334 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4f0348);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(3,32,c.get(0,32));
            { let a = rm(m,c.r[3].wrapping_add(0x39),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4f039a; continue; }
            pc = 0x4f0353; continue;
            }
            0x4f0353 => {
            { let a = rm(m,c.r[3].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x4f039a; continue; }
            pc = 0x4f035d; continue;
            }
            0x4f035d => {
            { let a = rm(m,c.r[3].wrapping_add(0x136),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4f039a; continue; }
            pc = 0x4f0366; continue;
            }
            0x4f0366 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4f036d);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),32),32); }
            if c.cond(6) { pc = 0x4f039a; continue; }
            pc = 0x4f0373; continue;
            }
            0x4f0373 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4f037a);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,0x10624dd3u32);
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[5]); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f039a; continue;
            }
            0x4f039a => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4f0334; continue; }
            pc = 0x4f03a7; continue;
            }
            0x4f03a7 => {
            pc = 0x4f2c93; continue;
            }
            0x4f03ac => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x280); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x484); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x280),32));
            { let a = c.r[4].wrapping_add(0x3a8); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x290); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x288); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f03fb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3a8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x484),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x290),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x288),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,32,0xffffffffu32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x3b0); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,0x0u32); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f0473; continue;
            }
            0x4f0473 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4f0487);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(3,32,c.get(0,32));
            { let a = rm(m,c.r[3].wrapping_add(0x39),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4f0504; continue; }
            pc = 0x4f0492; continue;
            }
            0x4f0492 => {
            { let a = rm(m,c.r[3].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x4f0504; continue; }
            pc = 0x4f049c; continue;
            }
            0x4f049c => {
            { let a = rm(m,c.r[3].wrapping_add(0x136),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4f0504; continue; }
            pc = 0x4f04a5; continue;
            }
            0x4f04a5 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xe),8));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(2) { pc = 0x4f04d0; continue; }
            pc = 0x4f04ac; continue;
            }
            0x4f04ac => {
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            if c.cond(6) { pc = 0x4f0504; continue; }
            pc = 0x4f04b0; continue;
            }
            0x4f04b0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3b0),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4f04c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f3660, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x2c8),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4f0504; continue; }
            pc = 0x4f04d0; continue;
            }
            0x4f04d0 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4f04d7);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),32),32); }
            if c.cond(6) { pc = 0x4f0504; continue; }
            pc = 0x4f04dd; continue;
            }
            0x4f04dd => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4f04e4);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[7].wrapping_add(c.r[2]); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0x10624dd3u32);
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f0504; continue;
            }
            0x4f0504 => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4f0473; continue; }
            pc = 0x4f0515; continue;
            }
            0x4f0515 => {
            pc = 0x4f2c93; continue;
            }
            0x4f051a => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x298); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x3b8); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x298),32));
            { let a = c.r[4].wrapping_add(0x440); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x2a8); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0xf0); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f056a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x440),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2a8),32));
            c.set(5,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffu32); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0x4ac); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3b8),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,32,0xffffffffu32); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f05e5; continue;
            }
            0x4f05e5 => {
            c.set(0,32,c.get(5,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4f05f7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(3,32,c.get(0,32));
            { let a = rm(m,c.r[3].wrapping_add(0x39),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4f067c; continue; }
            pc = 0x4f0602; continue;
            }
            0x4f0602 => {
            { let a = rm(m,c.r[3].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x4f067c; continue; }
            pc = 0x4f060c; continue;
            }
            0x4f060c => {
            { let a = rm(m,c.r[3].wrapping_add(0x136),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4f067c; continue; }
            pc = 0x4f0615; continue;
            }
            0x4f0615 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xe),8));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(2) { pc = 0x4f0641; continue; }
            pc = 0x4f061c; continue;
            }
            0x4f061c => {
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            if c.cond(6) { pc = 0x4f067c; continue; }
            pc = 0x4f0620; continue;
            }
            0x4f0620 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4ac),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4f0638);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f3660, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x2c8),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4f067c; continue; }
            pc = 0x4f0641; continue;
            }
            0x4f0641 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4f0648);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); }
            if c.cond(6) { pc = 0x4f067c; continue; }
            pc = 0x4f064e; continue;
            }
            0x4f064e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = rm(m,0x6dd894u32,16); c.sub(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x4f066a; continue; }
            pc = 0x4f065b; continue;
            }
            0x4f065b => {
            c.set(1,16,rm(m,0x6dd896u32,16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[3].wrapping_add(0x8),16),16); }
            if c.cond(6) { pc = 0x4f066a; continue; }
            pc = 0x4f0668; continue;
            }
            0x4f0668 => {
            c.set(5,32,c.get(0,32));
            pc = 0x4f066a; continue;
            }
            0x4f066a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f067c; continue;
            }
            0x4f067c => {
            c.set(5,16,rm(m,c.r[3].wrapping_add(0x4),16));
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4f05e5; continue; }
            pc = 0x4f068d; continue;
            }
            0x4f068d => {
            pc = 0x4f2c93; continue;
            }
            0x4f0692 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6dd730u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f06b1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d7770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f06c1 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x2b0); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2b0),32));
            { let a = c.r[4].wrapping_add(0x3c0); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f06f3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x46c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f0711 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x2b8); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2b8),32));
            { let a = c.r[4].wrapping_add(0x448); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x2c0); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f074d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x448),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4f0768);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f3740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f0780 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x3c8); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c8),32));
            { let a = c.r[4].wrapping_add(0x48c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x2d0); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x4f07c7);
            { let args = []; c.r[0] = host.call(m, 0x51beb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48c),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x2c8); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c8),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,8); c.sub(a,0x14u32,8); }
            { let a = c.r[7].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x4f07fc; continue; }
            pc = 0x4f07f2; continue;
            }
            0x4f07f2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x672660),8));
            pc = 0x4f07fe; continue;
            }
            0x4f07fc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4f07fe; continue;
            }
            0x4f07fe => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2d0),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f0816 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x3d0); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3d0),32));
            { let a = c.r[4].wrapping_add(0x2d8); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f0848);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4f084f);
            { let args = []; c.r[0] = host.call(m, 0x4f36c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2d8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,sx(c.get(0,8),8)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f086a => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x450); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x450),32));
            { let a = c.r[4].wrapping_add(0xfc); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x2e0); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0xf4); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f08af);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2e0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[3].wrapping_add(c.r[1]); wm(m,a,32,0xffffffffu32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[1].wrapping_add(c.r[2]); wm(m,a,32,0xffffffffu32); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f0903; continue;
            }
            0x4f0903 => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f090e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0x0u32,16); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4f095b; continue; }
            pc = 0x4f091c; continue;
            }
            0x4f091c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xfc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x4f0957; continue; }
            pc = 0x4f0931; continue;
            }
            0x4f0931 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x39),8));
            c.set(0,32,c.get(2,32));
            pc = 0x4f093b; continue;
            }
            0x4f093b => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[7]).wrapping_add(0x5),8));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(4)),32),32); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            if c.cond(4) { pc = 0x4f094f; continue; }
            pc = 0x4f094a; continue;
            }
            0x4f094a => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4f093b; continue; }
            pc = 0x4f094f; continue;
            }
            0x4f094f => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x4f0957; continue;
            }
            0x4f0957 => {
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4f0974; continue; }
            pc = 0x4f095b; continue;
            }
            0x4f095b => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4f0903; continue; }
            pc = 0x4f096f; continue;
            }
            0x4f096f => {
            pc = 0x4f2c93; continue;
            }
            0x4f0974 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.r[3].wrapping_add(c.r[2]); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,32,c.get(5,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f098e => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x104); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x104),32));
            { let a = c.r[4].wrapping_add(0x10c); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f09c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f09eb; continue;
            }
            0x4f09eb => {
            c.set(2,32,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4f09fe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4f0a09);
            { let args = []; c.r[0] = host.call(m, 0x519c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4f13d3; continue; }
            pc = 0x4f0a11; continue;
            }
            0x4f0a11 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4f09eb; continue; }
            pc = 0x4f0a22; continue;
            }
            0x4f0a22 => {
            pc = 0x4f2c93; continue;
            }
            0x4f0a27 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(7,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2d4),32));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f0a54 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x16),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f0a6f => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x114); wm(m,a,8,c.get(2,8)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x114),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x11c); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            c.push(m, 0x4f0ab6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f0c5d; continue; }
            pc = 0x4f0ac7; continue;
            }
            0x4f0ac7 => {
            c.set(3,32,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x4f0ada);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(2) { pc = 0x4f0b3d; continue; }
            pc = 0x4f0ae7; continue;
            }
            0x4f0ae7 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4f0b3d; continue; }
            pc = 0x4f0aee; continue;
            }
            0x4f0aee => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4f0b0c; continue; }
            pc = 0x4f0b02; continue;
            }
            0x4f0b02 => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x4f0b32; continue; }
            pc = 0x4f0b0c; continue;
            }
            0x4f0b0c => {
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4f0b1d; continue; }
            pc = 0x4f0b11; continue;
            }
            0x4f0b11 => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0x3u32,16); }
            if c.cond(6) { pc = 0x4f0b32; continue; }
            pc = 0x4f0b1b; continue;
            }
            0x4f0b1b => {
            { let a = c.get(1,32); c.sub(a,c.get(1,32),32); }
            pc = 0x4f0b1d; continue;
            }
            0x4f0b1d => {
            if c.cond(14) { pc = 0x4f0b3d; continue; }
            pc = 0x4f0b1f; continue;
            }
            0x4f0b1f => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(1,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x4f0b32; continue; }
            pc = 0x4f0b2c; continue;
            }
            0x4f0b2c => {
            { let a = c.get(1,16); c.sub(a,0x3u32,16); }
            if c.cond(7) { pc = 0x4f0b3d; continue; }
            pc = 0x4f0b32; continue;
            }
            0x4f0b32 => {
            { let a = rm(m,c.r[0].wrapping_add(0x40),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x4f0b3d; continue; }
            pc = 0x4f0b39; continue;
            }
            0x4f0b39 => {
            { let a = rm(m,c.r[4].wrapping_add(0x1c),32); let r = c.inc(a,32); { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,r); } }
            pc = 0x4f0b3d; continue;
            }
            0x4f0b3d => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f0b48);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4f0ac7; continue; }
            pc = 0x4f0b59; continue;
            }
            0x4f0b59 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,16); c.and(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x4f0c5d; continue; }
            pc = 0x4f0b66; continue;
            }
            0x4f0b66 => {
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f0b70);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x2d4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let b = c.get(3,32); c.div32(b); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.set(3,32,c.get(2,32));
            c.push(m, 0x4f0ba7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4f0c5d; continue; }
            pc = 0x4f0bc1; continue;
            }
            0x4f0bc1 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            pc = 0x4f0bc5; continue;
            }
            0x4f0bc5 => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f0bd0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(6) { pc = 0x4f0c38; continue; }
            pc = 0x4f0bdd; continue;
            }
            0x4f0bdd => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4f0c38; continue; }
            pc = 0x4f0be4; continue;
            }
            0x4f0be4 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4f0c00; continue; }
            pc = 0x4f0bf6; continue;
            }
            0x4f0bf6 => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x4f0c26; continue; }
            pc = 0x4f0c00; continue;
            }
            0x4f0c00 => {
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4f0c11; continue; }
            pc = 0x4f0c05; continue;
            }
            0x4f0c05 => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0x3u32,16); }
            if c.cond(6) { pc = 0x4f0c26; continue; }
            pc = 0x4f0c0f; continue;
            }
            0x4f0c0f => {
            { let a = c.get(1,32); c.sub(a,c.get(1,32),32); }
            pc = 0x4f0c11; continue;
            }
            0x4f0c11 => {
            if c.cond(14) { pc = 0x4f0c38; continue; }
            pc = 0x4f0c13; continue;
            }
            0x4f0c13 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(1,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x4f0c26; continue; }
            pc = 0x4f0c20; continue;
            }
            0x4f0c20 => {
            { let a = c.get(1,16); c.sub(a,0x3u32,16); }
            if c.cond(7) { pc = 0x4f0c38; continue; }
            pc = 0x4f0c26; continue;
            }
            0x4f0c26 => {
            { let a = rm(m,c.r[0].wrapping_add(0x40),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x4f0c38; continue; }
            pc = 0x4f0c2d; continue;
            }
            0x4f0c2d => {
            { let a = c.get(3,16); c.and(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x4f0c5d; continue; }
            pc = 0x4f0c32; continue;
            }
            0x4f0c32 => {
            { let a = c.get(3,32); let r = c.add(a,0xffffu32,32); c.set(3,32,r); }
            pc = 0x4f0c38; continue;
            }
            0x4f0c38 => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f0c43);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4f0bc5; continue; }
            pc = 0x4f0c5d; continue;
            }
            0x4f0c5d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x11c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(5,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f0c74 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x124); wm(m,a,8,c.get(2,8)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x124),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x12c); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            c.push(m, 0x4f0cbb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f0e7d; continue; }
            pc = 0x4f0ccc; continue;
            }
            0x4f0ccc => {
            c.set(3,32,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x4f0cdf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4f0d35; continue; }
            pc = 0x4f0ce6; continue;
            }
            0x4f0ce6 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4f0d04; continue; }
            pc = 0x4f0cfa; continue;
            }
            0x4f0cfa => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x4f0d2a; continue; }
            pc = 0x4f0d04; continue;
            }
            0x4f0d04 => {
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4f0d15; continue; }
            pc = 0x4f0d09; continue;
            }
            0x4f0d09 => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0x3u32,16); }
            if c.cond(6) { pc = 0x4f0d2a; continue; }
            pc = 0x4f0d13; continue;
            }
            0x4f0d13 => {
            { let a = c.get(1,32); c.sub(a,c.get(1,32),32); }
            pc = 0x4f0d15; continue;
            }
            0x4f0d15 => {
            if c.cond(14) { pc = 0x4f0d35; continue; }
            pc = 0x4f0d17; continue;
            }
            0x4f0d17 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(1,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x4f0d2a; continue; }
            pc = 0x4f0d24; continue;
            }
            0x4f0d24 => {
            { let a = c.get(1,16); c.sub(a,0x3u32,16); }
            if c.cond(7) { pc = 0x4f0d35; continue; }
            pc = 0x4f0d2a; continue;
            }
            0x4f0d2a => {
            { let a = rm(m,c.r[0].wrapping_add(0x40),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x4f0d35; continue; }
            pc = 0x4f0d31; continue;
            }
            0x4f0d31 => {
            { let a = rm(m,c.r[4].wrapping_add(0x1c),32); let r = c.inc(a,32); { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,r); } }
            pc = 0x4f0d35; continue;
            }
            0x4f0d35 => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f0d40);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4f0ccc; continue; }
            pc = 0x4f0d51; continue;
            }
            0x4f0d51 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,16); c.and(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x4f0e7d; continue; }
            pc = 0x4f0d5e; continue;
            }
            0x4f0d5e => {
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f0d68);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x2d4),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(3,32); c.div32(b); }
            c.set(3,32,c.get(2,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f0d9f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4f0e7d; continue; }
            pc = 0x4f0db9; continue;
            }
            0x4f0db9 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            pc = 0x4f0dbd; continue;
            }
            0x4f0dbd => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f0dc8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x4f0dfd; continue; }
            pc = 0x4f0dd5; continue;
            }
            0x4f0dd5 => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f0de6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f3870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4f0ded);
            { let args = []; c.r[0] = host.call(m, 0x4f37c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f0dfd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f0dfd; continue;
            }
            0x4f0dfd => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4f0e58; continue; }
            pc = 0x4f0e04; continue;
            }
            0x4f0e04 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4f0e20; continue; }
            pc = 0x4f0e16; continue;
            }
            0x4f0e16 => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x4f0e46; continue; }
            pc = 0x4f0e20; continue;
            }
            0x4f0e20 => {
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4f0e31; continue; }
            pc = 0x4f0e25; continue;
            }
            0x4f0e25 => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0x3u32,16); }
            if c.cond(6) { pc = 0x4f0e46; continue; }
            pc = 0x4f0e2f; continue;
            }
            0x4f0e2f => {
            { let a = c.get(1,32); c.sub(a,c.get(1,32),32); }
            pc = 0x4f0e31; continue;
            }
            0x4f0e31 => {
            if c.cond(14) { pc = 0x4f0e58; continue; }
            pc = 0x4f0e33; continue;
            }
            0x4f0e33 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(1,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x4f0e46; continue; }
            pc = 0x4f0e40; continue;
            }
            0x4f0e40 => {
            { let a = c.get(1,16); c.sub(a,0x3u32,16); }
            if c.cond(7) { pc = 0x4f0e58; continue; }
            pc = 0x4f0e46; continue;
            }
            0x4f0e46 => {
            { let a = rm(m,c.r[0].wrapping_add(0x40),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x4f0e58; continue; }
            pc = 0x4f0e4d; continue;
            }
            0x4f0e4d => {
            { let a = c.get(3,16); c.and(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x4f0e7d; continue; }
            pc = 0x4f0e52; continue;
            }
            0x4f0e52 => {
            { let a = c.get(3,32); let r = c.add(a,0xffffu32,32); c.set(3,32,r); }
            pc = 0x4f0e58; continue;
            }
            0x4f0e58 => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f0e63);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4f0dbd; continue; }
            pc = 0x4f0e7d; continue;
            }
            0x4f0e7d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(5,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f0e94 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x1),32));
            { let a = c.get(7,32); let r = c.add(a,0x5u32,32); c.set(7,32,r); }
            c.push(m,0x0u32);
            c.set(7,32,rm(m,c.r[7],32));
            c.set(3,32,c.r[0].wrapping_add(c.r[1]));
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.push(m, 0x4f0eae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(7,32));
            c.push(m, 0x4f0ebb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x554); wm(m,a,32,0x5u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(3,32));
            c.push(m, 0x4f0ed3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,0x16u32);
            { let a = c.r[4].wrapping_add(0x560); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4f0ef5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4f2c93; continue;
            }
            0x4f0efa => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x134); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x13c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x134),32));
            { let a = c.r[4].wrapping_add(0x144); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x140),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),16));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f0f4b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x144),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f0f6a => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x14c); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14c),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.r[0].wrapping_add(c.r[5]); wm(m,a,32,0x0u32); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(7,16),16); }
            if c.cond(6) { pc = 0x4f2c93; continue; }
            pc = 0x4f0f9d; continue;
            }
            0x4f0f9d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.r[1].wrapping_add(c.r[5]));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f0fae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,c.get(2,16),16); }
            c.set(0,32,rm(m,c.r[3],32));
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4f0f9d; continue; }
            pc = 0x4f0fca; continue;
            }
            0x4f0fca => {
            pc = 0x4f2c93; continue;
            }
            0x4f0fcf => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),16));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(1,32,0x6dd7a0u32);
            if c.cond(4) { pc = 0x4f0ff7; continue; }
            pc = 0x4f0fed; continue;
            }
            0x4f0fed => {
            c.push(m, 0x4f0ff2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50df40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f2c93; continue;
            }
            0x4f0ff7 => {
            c.push(m, 0x4f0ffc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50df70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f2c93; continue;
            }
            0x4f1001 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x154); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x154),32));
            { let a = c.r[4].wrapping_add(0x15c); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f1033);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x40),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x15c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f1050 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x16c); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x174); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x164); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x164),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x16c),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[3].wrapping_add(c.r[0]); wm(m,a,32,0x0u32); }
            c.set(0,32,c.r[7].wrapping_add(c.r[1]));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x4f115c; continue; }
            pc = 0x4f10c0; continue;
            }
            0x4f10c0 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f10cb; continue;
            }
            0x4f10cb => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x4f10e0; continue; }
            pc = 0x4f10d4; continue;
            }
            0x4f10d4 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x20),32),32); }
            if c.cond(13) { pc = 0x4f112c; continue; }
            pc = 0x4f10da; continue;
            }
            0x4f10da => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            pc = 0x4f112c; continue;
            }
            0x4f10e0 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f10ed);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x174),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(7) { pc = 0x4f111f; continue; }
            pc = 0x4f1108; continue;
            }
            0x4f1108 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[5]),32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4f1142; continue; }
            pc = 0x4f1115; continue;
            }
            0x4f1115 => {
            { let a = c.r[5].wrapping_add(c.r[7].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            pc = 0x4f112c; continue;
            }
            0x4f111f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = rm(m,c.r[2].wrapping_add(c.r[5]),32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4f115c; continue; }
            pc = 0x4f112c; continue;
            }
            0x4f112c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x4f10cb; continue; }
            pc = 0x4f1140; continue;
            }
            0x4f1140 => {
            pc = 0x4f115c; continue;
            }
            0x4f1142 => {
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f114c);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[1].wrapping_add(c.r[7].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[3].wrapping_add(c.r[2]); wm(m,a,32,0x1u32); }
            pc = 0x4f115c; continue;
            }
            0x4f115c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4f2c93; continue; }
            pc = 0x4f1168; continue;
            }
            0x4f1168 => {
            { let a = rm(m,c.r[4].wrapping_add(0x20),32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4f2c93; continue; }
            pc = 0x4f1172; continue;
            }
            0x4f1172 => {
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f117c);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[3].wrapping_add(c.r[1]); wm(m,a,32,0x1u32); }
            pc = 0x4f2c93; continue;
            }
            0x4f11a5 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x17c); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x17c),32));
            { let a = c.r[4].wrapping_add(0x184); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f11e1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x184),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[2].wrapping_add(c.r[7]),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[7]),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x672c14),8));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7]),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4f123a; continue; }
            pc = 0x4f1221; continue;
            }
            0x4f1221 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f123a => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f1253 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x194); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x194),32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            c.set(5,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.set(7,32,c.r[5].wrapping_add(0x1));
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[3].wrapping_add(c.r[0]); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4f1286; continue;
            }
            0x4f1286 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4f1294; continue; }
            pc = 0x4f1292; continue;
            }
            0x4f1292 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4f1294; continue;
            }
            0x4f1294 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f129f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x4f12b0; continue; }
            pc = 0x4f12a6; continue;
            }
            0x4f12a6 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4f1286; continue; }
            pc = 0x4f12ab; continue;
            }
            0x4f12ab => {
            pc = 0x4f2c93; continue;
            }
            0x4f12b0 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[3].wrapping_add(c.r[1]); wm(m,a,32,c.get(7,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f12bb => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x1a4); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x19c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x19c),32));
            { let a = c.r[4].wrapping_add(0x1ac); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x1b4); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1bc); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f130b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1a4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1bc),32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1ac),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(3,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1b4),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,0xffffffffu32); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f1373; continue;
            }
            0x4f1373 => {
            c.set(0,32,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4f1385);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x18),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[1].wrapping_add(0x14),32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4f13a5; continue; }
            pc = 0x4f139f; continue;
            }
            0x4f139f => {
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4f13a9; continue; }
            pc = 0x4f13a3; continue;
            }
            0x4f13a3 => {
            pc = 0x4f13f3; continue;
            }
            0x4f13a5 => {
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4f13f3; continue; }
            pc = 0x4f13a9; continue;
            }
            0x4f13a9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); c.sub(a,0x2u32,32); }
            if c.cond(12) { pc = 0x4f13c5; continue; }
            pc = 0x4f13b2; continue;
            }
            0x4f13b2 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x4f13d3; continue; }
            pc = 0x4f13bf; continue;
            }
            0x4f13bf => {
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(2) { pc = 0x4f13d3; continue; }
            pc = 0x4f13c5; continue;
            }
            0x4f13c5 => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4f13e4; continue; }
            pc = 0x4f13c9; continue;
            }
            0x4f13c9 => {
            { let a = rm(m,c.r[1].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x4f13f3; continue; }
            pc = 0x4f13d3; continue;
            }
            0x4f13d3 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,c.get(7,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f13e4 => {
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4f13f3; continue; }
            pc = 0x4f13e9; continue;
            }
            0x4f13e9 => {
            { let a = rm(m,c.r[1].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(2) { pc = 0x4f13d3; continue; }
            pc = 0x4f13f3; continue;
            }
            0x4f13f3 => {
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4f1373; continue; }
            pc = 0x4f1404; continue;
            }
            0x4f1404 => {
            pc = 0x4f2c93; continue;
            }
            0x4f1409 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x1c4); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x1cc); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c4),32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f1442);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x134),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x4f1487; continue; }
            pc = 0x4f1458; continue;
            }
            0x4f1458 => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f1469);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f3870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4f1470);
            { let args = []; c.r[0] = host.call(m, 0x4f37c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f1480);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x134),16));
            pc = 0x4f1487; continue;
            }
            0x4f1487 => {
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(2) { pc = 0x4f14c4; continue; }
            pc = 0x4f148d; continue;
            }
            0x4f148d => {
            { let a = c.get(0,16); c.sub(a,0x8u32,16); }
            if c.cond(4) { pc = 0x4f14c4; continue; }
            pc = 0x4f1493; continue;
            }
            0x4f1493 => {
            { let a = c.get(0,16); c.sub(a,0x9u32,16); }
            if c.cond(4) { pc = 0x4f14c4; continue; }
            pc = 0x4f1499; continue;
            }
            0x4f1499 => {
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x4f14c4; continue; }
            pc = 0x4f149f; continue;
            }
            0x4f149f => {
            { let a = c.get(0,16); c.sub(a,0x12u32,16); }
            if c.cond(4) { pc = 0x4f14c4; continue; }
            pc = 0x4f14a5; continue;
            }
            0x4f14a5 => {
            { let a = c.get(0,16); c.sub(a,0x14u32,16); }
            if c.cond(4) { pc = 0x4f14c4; continue; }
            pc = 0x4f14ab; continue;
            }
            0x4f14ab => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            pc = 0x4f2c93; continue;
            }
            0x4f14c4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1cc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            c.push(m,0x100u32);
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[7].wrapping_add(0x14),32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x4f14fb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,0x1u32); }
            pc = 0x4f2c93; continue;
            }
            0x4f1514 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1d4),32));
            { let a = c.r[4].wrapping_add(0x1dc); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f1546);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1dc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[7].wrapping_add(0x14),32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[1].wrapping_add(c.r[2]); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x4f2c93; continue; }
            pc = 0x4f157c; continue;
            }
            0x4f157c => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4f2c93; continue; }
            pc = 0x4f1587; continue;
            }
            0x4f1587 => {
            { let a = c.r[1]; wm(m,a,32,0x1u32); }
            pc = 0x4f2c93; continue;
            }
            0x4f1592 => {
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f159c);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f15ac => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x1e4); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1e4),32));
            { let a = c.r[4].wrapping_add(0x1ec); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f15de);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6de4a8u32,32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xc),16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x4f161f; continue; }
            pc = 0x4f15f8; continue;
            }
            0x4f15f8 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.push(m, 0x4f1604);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2c8),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4f15f8; continue; }
            pc = 0x4f161f; continue;
            }
            0x4f161f => {
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f1629);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xc),16));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x4f1652; continue; }
            pc = 0x4f1637; continue;
            }
            0x4f1637 => {
            c.set(7,32,c.get(2,32));
            pc = 0x4f1639; continue;
            }
            0x4f1639 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f1644);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2c8),16));
            c.set(0,32,c.get(1,32));
            if c.cond(5) { pc = 0x4f1639; continue; }
            pc = 0x4f1652; continue;
            }
            0x4f1652 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f165d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x2c6),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1ec),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f167c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0x10u32,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.sub(a,0x10u32,8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f16b3 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x1fc); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x1f4); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.push(m, 0x4f16dc);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f16e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x8),16));
            c.set(1,32,0x6dd7a0u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,0x1u32); }
            c.set(6,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x4f1706);
            { let args = []; c.r[0] = host.call(m, 0x4f38a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            c.set(1,32,0x6dd7a0u32);
            if c.cond(3) { pc = 0x4f1721; continue; }
            pc = 0x4f1714; continue;
            }
            0x4f1714 => {
            c.push(m, 0x4f1719);
            { let args = []; c.r[0] = host.call(m, 0x4f38a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            pc = 0x4f1743; continue;
            }
            0x4f1721 => {
            c.push(m, 0x4f1726);
            { let args = []; c.r[0] = host.call(m, 0x4f3890, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f1741; continue; }
            pc = 0x4f172f; continue;
            }
            0x4f172f => {
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f1739);
            { let args = []; c.r[0] = host.call(m, 0x4f3890, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            pc = 0x4f1743; continue;
            }
            0x4f1741 => {
            c.push(m,0x0u32);
            pc = 0x4f1743; continue;
            }
            0x4f1743 => {
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f174d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1f4),32));
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,16,0x5u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(3,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),8));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1fc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]),8));
            { let a = c.r[7].wrapping_add(0x9); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[7].wrapping_add(0xa); wm(m,a,8,0xffu32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]),8));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[7].wrapping_add(0xb); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x4f179a);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,8,0x8u32); }
            { let a = c.r[7].wrapping_add(0x11); wm(m,a,8,0x0u32); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x1c),16));
            { let a = c.r[7].wrapping_add(0x12); wm(m,a,16,c.get(0,16)); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x20),16));
            { let a = c.r[7].wrapping_add(0x14); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[7].wrapping_add(0x16); wm(m,a,16,0xffu32); }
            pc = 0x4f2c93; continue;
            }
            0x4f17c0 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x204); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x214); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x204),32));
            { let a = c.r[4].wrapping_add(0x20c); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x21c); wm(m,a,8,c.get(2,8)); }
            c.push(m,0x0u32);
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x210),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),16));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f181b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x214),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x21c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[2].wrapping_add(c.r[7]),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[7]),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x672c14),8));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4f1874; continue; }
            pc = 0x4f185b; continue;
            }
            0x4f185b => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f1874 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f188d => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x224); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x224),32));
            { let a = c.r[4].wrapping_add(0x22c); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x234); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x23c); wm(m,a,8,c.get(1,8)); }
            c.push(m,0x0u32);
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x230),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f18e8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x234),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x23c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,8,rm(m,c.r[2].wrapping_add(0x672c14),8));
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xfffff8f8u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x7d0u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x2e8),32); let r = c.add(a,c.get(2,32),32); { let a = c.r[0].wrapping_add(0x2e8); wm(m,a,32,r); } }
            pc = 0x4f2c93; continue;
            }
            0x4f192d => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.fild32(m,c.r[3].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xfffff8f8u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x7d0u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            { let v = c.ld(m,c.r[0].wrapping_mul(4).wrapping_add(0x673a18),4); c.fop('*',v); }
            c.push(m, 0x4f197a);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[3].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f1987 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x244); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x244),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),16));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.r[0].wrapping_add(c.r[7]); wm(m,a,32,0x0u32); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x4f2c93; continue; }
            pc = 0x4f19c3; continue;
            }
            0x4f19c3 => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f19ce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4f19ee; continue; }
            pc = 0x4f19d5; continue;
            }
            0x4f19d5 => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0xa),16));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4f19e5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d7900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd736u32,16),16); }
            if c.cond(2) { pc = 0x4f1a00; continue; }
            pc = 0x4f19ee; continue;
            }
            0x4f19ee => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4f19c3; continue; }
            pc = 0x4f19fb; continue;
            }
            0x4f19fb => {
            pc = 0x4f2c93; continue;
            }
            0x4f1a00 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,0x1u32); }
            pc = 0x4f2c93; continue;
            }
            0x4f1a0f => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x24c); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24c),32));
            { let a = c.r[4].wrapping_add(0x254); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x25c); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xfffff8f8u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x7d0u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x258),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de3d8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4f1a7e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x25c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f1a96 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x264); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x264),32));
            { let a = c.r[4].wrapping_add(0x26c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x274); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f1ad1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x26c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.fild32(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x278),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x66bc78u32,4); c.fop('/',v); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,c.get(0,32));
            c.fstp32(m,c.r[4]);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f1b0e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4f3770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4f2c93; continue;
            }
            0x4f1b13 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x27c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x27c),32));
            { let a = c.r[4].wrapping_add(0x284); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f1b45);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x118),32));
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x284),32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f1b74 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x28c); wm(m,a,8,c.get(0,8)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28c),32));
            { let a = c.r[4].wrapping_add(0x29c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x294); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x294),32));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x784),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4a8u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f1be2; continue;
            }
            0x4f1be2 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x29c),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            pc = 0x4f1bf2; continue;
            }
            0x4f1bf2 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f1c02);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[5]),32));
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4f1c3d; continue; }
            pc = 0x4f1c0f; continue;
            }
            0x4f1c0f => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4f1c1a; continue; }
            pc = 0x4f1c12; continue;
            }
            0x4f1c12 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x4f1c5f; continue; }
            pc = 0x4f1c15; continue;
            }
            0x4f1c15 => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[7]),32); let r = c.inc(a,32); { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,32,r); } }
            pc = 0x4f1c5f; continue;
            }
            0x4f1c1a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x2c4),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f1c2e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x4f1c5f; continue; }
            pc = 0x4f1c35; continue;
            }
            0x4f1c35 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[7]));
            pc = 0x4f1c5d; continue;
            }
            0x4f1c3d => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x2c4),16));
            c.push(m,c.get(2,32));
            c.push(m, 0x4f1c51);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4f1c5f; continue; }
            pc = 0x4f1c58; continue;
            }
            0x4f1c58 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            pc = 0x4f1c5d; continue;
            }
            0x4f1c5d => {
            { let a = rm(m,c.r[0],32); let r = c.inc(a,32); { let a = c.r[0]; wm(m,a,32,r); } }
            pc = 0x4f1c5f; continue;
            }
            0x4f1c5f => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x2ca),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4a8u32,16),16); }
            if c.cond(2) { pc = 0x4f1bf2; continue; }
            pc = 0x4f1c6f; continue;
            }
            0x4f1c6f => {
            pc = 0x4f2c93; continue;
            }
            0x4f1c74 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x2ac); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x2b4); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x2a4); wm(m,a,8,c.get(2,8)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2a4),32));
            { let a = c.r[4].wrapping_add(0x2bc); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x2c4); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2ac),32));
            { let a = c.r[0].wrapping_add(c.r[5]); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2b4),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f1cf4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xa),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd736u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f1d05; continue;
            }
            0x4f1d05 => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,0x6dd730u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4f1d16);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f38b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = rm(m,c.r[7].wrapping_add(0x4),8); c.sub(a,0x72u32,8); }
            if c.cond(5) { pc = 0x4f1db4; continue; }
            pc = 0x4f1d22; continue;
            }
            0x4f1d22 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x4f1db4; continue; }
            pc = 0x4f1d31; continue;
            }
            0x4f1d31 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x2),16));
            c.set(0,8,rm(m,c.r[7],8));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,8); let r = c.and(a,0x7fu32,8); c.set(2,8,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f1d4d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x531260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2bc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4f1db4; continue; }
            pc = 0x4f1d63; continue;
            }
            0x4f1d63 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c4),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(c.r[0]),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4f1d86; continue; }
            pc = 0x4f1d78; continue;
            }
            0x4f1d78 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(c.r[5]),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(c.r[5]); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f1d86; continue;
            }
            0x4f1d86 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x4f1d9d; continue; }
            pc = 0x4f1d8f; continue;
            }
            0x4f1d8f => {
            c.set(3,32,rm(m,c.r[1].wrapping_add(c.r[5]),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,c.get(3,32)); }
            pc = 0x4f1d9d; continue;
            }
            0x4f1d9d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x4f1db4; continue; }
            pc = 0x4f1da6; continue;
            }
            0x4f1da6 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[5]),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f1db4; continue;
            }
            0x4f1db4 => {
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd736u32,16),16); }
            if c.cond(2) { pc = 0x4f1d05; continue; }
            pc = 0x4f1dc5; continue;
            }
            0x4f1dc5 => {
            pc = 0x4f2c93; continue;
            }
            0x4f1dca => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,0x6dd894u32,16); c.sub(a,c.get(7,16),16); }
            c.set(3,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),16));
            if c.cond(6) { pc = 0x4f2c93; continue; }
            pc = 0x4f1de7; continue;
            }
            0x4f1de7 => {
            c.set(5,32,0x12u32);
            pc = 0x4f1dec; continue;
            }
            0x4f1dec => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f1df7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x42),16));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4f1e0a; continue; }
            pc = 0x4f1e01; continue;
            }
            0x4f1e01 => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,c.get(5,16),16); }
            if c.cond(4) { pc = 0x4f1e20; continue; }
            pc = 0x4f1e0a; continue;
            }
            0x4f1e0a => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4f1dec; continue; }
            pc = 0x4f1e1b; continue;
            }
            0x4f1e1b => {
            pc = 0x4f2c93; continue;
            }
            0x4f1e20 => {
            c.push(m,0x100u32);
            c.set(1,32,c.get(0,32));
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            c.push(m, 0x4f1e33);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f2c93; continue;
            }
            0x4f1e38 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x2dc); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x2cc); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2cc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2d4); wm(m,a,8,c.get(2,8)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2d4),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(7,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x310),32));
            c.set(3,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x64),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x784),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4a8u32,16),16); }
            if c.cond(3) { pc = 0x4f1eda; continue; }
            pc = 0x4f1eb4; continue;
            }
            0x4f1eb4 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f1ec4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x64),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x2ca),16));
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4a8u32,16),16); }
            if c.cond(2) { pc = 0x4f1eb4; continue; }
            pc = 0x4f1eda; continue;
            }
            0x4f1eda => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            c.set(0,32,c.r[7].wrapping_mul(8));
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xfffff8f8u32,32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.add(a,0x7d0u32,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2dc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f1f14 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x6de52eu32,8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f1f2c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(1,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f1f47 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f1f62 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x2e4); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2e4),32));
            { let a = c.r[4].wrapping_add(0x2ec); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.push(m, 0x4f1f96);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x448610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2ec),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f1fb3 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x2f4); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x2fc); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2f4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(c.r[1]); wm(m,a,32,0x0u32); }
            c.set(7,32,rm(m,0x6dd738u32,32));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd736u32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f2005; continue;
            }
            0x4f2005 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2fc),32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            pc = 0x4f2015; continue;
            }
            0x4f2015 => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,0x6dd730u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4f2026);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f38b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x2),16));
            c.set(1,8,rm(m,c.r[7],8));
            { let a = c.get(0,8); let r = c.and(a,0x7fu32,8); c.set(0,8,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f2043);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x531260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = rm(m,c.r[3].wrapping_add(c.r[5]),32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x4f2c93; continue; }
            pc = 0x4f204c; continue;
            }
            0x4f204c => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.get(0,8); c.sub(a,0x3au32,8); }
            if c.cond(5) { pc = 0x4f2090; continue; }
            pc = 0x4f2053; continue;
            }
            0x4f2053 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)),32),32); }
            if c.cond(5) { pc = 0x4f2090; continue; }
            pc = 0x4f2068; continue;
            }
            0x4f2068 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x78),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)),32),32); }
            if c.cond(5) { pc = 0x4f20bd; continue; }
            pc = 0x4f207d; continue;
            }
            0x4f207d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,32,0x1u32); }
            pc = 0x4f2c93; continue;
            }
            0x4f2090 => {
            { let a = c.get(0,8); c.sub(a,0x39u32,8); }
            if c.cond(5) { pc = 0x4f20bd; continue; }
            pc = 0x4f2094; continue;
            }
            0x4f2094 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)),32),32); }
            if c.cond(5) { pc = 0x4f20bd; continue; }
            pc = 0x4f20a9; continue;
            }
            0x4f20a9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x78),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xd),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)),32),32); }
            if c.cond(4) { pc = 0x4f20d3; continue; }
            pc = 0x4f20bd; continue;
            }
            0x4f20bd => {
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd736u32,16),16); }
            if c.cond(2) { pc = 0x4f2015; continue; }
            pc = 0x4f20ce; continue;
            }
            0x4f20ce => {
            pc = 0x4f2c93; continue;
            }
            0x4f20d3 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[2].wrapping_add(c.r[1]); wm(m,a,32,0x1u32); }
            pc = 0x4f2c93; continue;
            }
            0x4f20e6 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x304); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x304),32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,0xffu32);
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f2126);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x509250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4f2c93; continue;
            }
            0x4f212b => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x31c); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x30c); wm(m,a,8,c.get(1,8)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x30c),32));
            { let a = c.r[4].wrapping_add(0x314); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x314),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x324); wm(m,a,8,c.get(0,8)); }
            c.push(m,0x0u32);
            { let a = c.r[2].wrapping_add(c.r[5]); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x320),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[7]),16));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f21a3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4f21ab; continue;
            }
            0x4f21ab => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x2ce),16));
            c.set(7,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(2,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x776),16),16); }
            c.set(7,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f21d7; continue;
            }
            0x4f21d7 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x324),32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            pc = 0x4f21e7; continue;
            }
            0x4f21e7 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f21f4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[3]),32),32); }
            if c.cond(5) { pc = 0x4f2204; continue; }
            pc = 0x4f2201; continue;
            }
            0x4f2201 => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[5]),32); let r = c.inc(a,32); { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,r); } }
            pc = 0x4f2204; continue;
            }
            0x4f2204 => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x4f21e7; continue; }
            pc = 0x4f2211; continue;
            }
            0x4f2211 => {
            pc = 0x4f2c93; continue;
            }
            0x4f2216 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x354); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x33c); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x32c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x344); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x32c),32));
            { let a = c.r[4].wrapping_add(0x334); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x6),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34c); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4f2272; continue;
            }
            0x4f2272 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x334),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x33c),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.push(m,0x1u32);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[5]),16));
            c.push(m,c.get(0,32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f22a7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4f22af; continue;
            }
            0x4f22af => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x2ce),16));
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[5]),32));
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(7,16); c.sub(a,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x776),16),16); }
            c.set(5,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f22db; continue;
            }
            0x4f22db => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x344),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f22ee; continue;
            }
            0x4f22ee => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4f22fc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x3),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[0]),32),32); }
            if c.cond(5) { pc = 0x4f2351; continue; }
            pc = 0x4f2316; continue;
            }
            0x4f2316 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x35c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4f2340);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5256f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[0].wrapping_add(c.r[2]),32),32); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4f2351; continue;
            }
            0x4f2351 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,16); c.sub(a,rm(m,c.r[5].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x4f22ee; continue; }
            pc = 0x4f235e; continue;
            }
            0x4f235e => {
            pc = 0x4f2c93; continue;
            }
            0x4f2363 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x35c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x35c),32));
            { let a = c.r[4].wrapping_add(0x36c); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x364); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x374); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4f23ab; continue;
            }
            0x4f23ab => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x364),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x36c),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[5]),16));
            c.push(m,0x0u32);
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[3]),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            c.push(m, 0x4f23e9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4f23f1; continue;
            }
            0x4f23f1 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x2ce),16));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[5]),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.get(7,16); c.sub(a,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x776),16),16); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f2420; continue;
            }
            0x4f2420 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x374),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2433; continue;
            }
            0x4f2433 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x4f2443);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[0].wrapping_add(c.r[1]),32),32); }
            if c.cond(5) { pc = 0x4f24c9; continue; }
            pc = 0x4f2456; continue;
            }
            0x4f2456 => {
            c.set(1,32,c.r[4].wrapping_add(0x4c8));
            c.push(m, 0x4f2462);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x4c8); wm(m,a,32,0x28u32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[5]),32));
            { let a = c.r[4].wrapping_add(0x4cc); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[0]),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x4d0); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,c.r[7],8));
            c.set(1,32,c.r[4].wrapping_add(0x4c8));
            { let a = c.r[4].wrapping_add(0x4d4); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x4dc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4f24b0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[2]),32),32); }
            if c.cond(4) { pc = 0x4f2c93; continue; }
            pc = 0x4f24c9; continue;
            }
            0x4f24c9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(7,16); c.sub(a,rm(m,c.r[2].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x4f2433; continue; }
            pc = 0x4f24de; continue;
            }
            0x4f24de => {
            pc = 0x4f2c93; continue;
            }
            0x4f24e3 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(3,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x6f1),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f2512 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x37c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x37c),32));
            { let a = c.r[4].wrapping_add(0x384); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f2544);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x384),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.set(1,32,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4f2560);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f36e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f2c93; continue;
            }
            0x4f2565 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x394); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x38c); wm(m,a,8,c.get(0,8)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x394),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0]),32));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(7,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,32,c.get(2,32)); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[5]),32));
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xfffff8f8u32,32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.add(a,0x7d0u32,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[7].wrapping_add(c.r[5]); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f25e8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,0x64u32);
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(7,16),16); }
            c.set(5,32,c.get(2,32));
            if c.cond(6) { pc = 0x4f2c93; continue; }
            pc = 0x4f2628; continue;
            }
            0x4f2628 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x80),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4f2640);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(1,16); c.and(a,c.get(1,16),16); }
            if c.cond(4) { pc = 0x4f26a0; continue; }
            pc = 0x4f2649; continue;
            }
            0x4f2649 => {
            { let a = c.get(1,8); c.and(a,0x6u32,8); }
            if c.cond(5) { pc = 0x4f26a0; continue; }
            pc = 0x4f264e; continue;
            }
            0x4f264e => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4f26a0; continue; }
            pc = 0x4f265b; continue;
            }
            0x4f265b => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f266b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x10000u32,32); }
            if c.cond(2) { pc = 0x4f268c; continue; }
            pc = 0x4f2675; continue;
            }
            0x4f2675 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x136),8));
            { let a = c.get(2,8); let r = c.or(a,0x20u32,8); c.set(2,8,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.r[0].wrapping_add(0x136); wm(m,a,8,c.get(2,8)); }
            pc = 0x4f2693; continue;
            }
            0x4f268c => {
            { let a = rm(m,c.r[0].wrapping_add(0x136),8); let r = c.and(a,0xdfu32,8); { let a = c.r[0].wrapping_add(0x136); wm(m,a,8,r); } }
            pc = 0x4f2693; continue;
            }
            0x4f2693 => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4f265b; continue; }
            pc = 0x4f26a0; continue;
            }
            0x4f26a0 => {
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(0,8,c.get(3,8));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x4f2628; continue; }
            pc = 0x4f26bb; continue;
            }
            0x4f26bb => {
            pc = 0x4f2c93; continue;
            }
            0x4f26c0 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x3a4); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3a4),32));
            { let a = c.r[4].wrapping_add(0x39c); wm(m,a,8,c.get(2,8)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x39c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m, 0x4f2701);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5136a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.push(m, 0x4f271b);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[7],32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4f2c93; continue; }
            pc = 0x4f2723; continue;
            }
            0x4f2723 => {
            { let a = c.r[7]; wm(m,a,32,0xffffffffu32); }
            pc = 0x4f2c93; continue;
            }
            0x4f272e => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x3ac); wm(m,a,8,c.get(1,8)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x3ac),32));
            { let a = c.r[4].wrapping_add(0x3b4); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3b4),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[7].wrapping_add(c.r[2]); wm(m,a,32,0xffffffffu32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,0x6dd894u32,16); c.sub(a,c.get(5,16),16); }
            c.set(3,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            if c.cond(6) { pc = 0x4f2c93; continue; }
            pc = 0x4f2780; continue;
            }
            0x4f2780 => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f278b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x42),16));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4f279f; continue; }
            pc = 0x4f2795; continue;
            }
            0x4f2795 => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0x12u32,16); }
            if c.cond(4) { pc = 0x4f27b3; continue; }
            pc = 0x4f279f; continue;
            }
            0x4f279f => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4f2780; continue; }
            pc = 0x4f27ae; continue;
            }
            0x4f27ae => {
            pc = 0x4f2c93; continue;
            }
            0x4f27b3 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,32,c.get(5,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f27be => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x3cc); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3bc); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x3c4); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x3dc); wm(m,a,8,c.get(2,8)); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3bc),32));
            { let a = c.r[4].wrapping_add(0x3d4); wm(m,a,32,c.get(2,32)); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f2816);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c8),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3d0),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[4].wrapping_add(0x3dc));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f2851);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x50e000, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3dc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4f2c93; continue;
            }
            0x4f286e => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x41c); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x414); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x404); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.r[4].wrapping_add(0x3e4); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x6),8));
            { let a = c.r[4].wrapping_add(0x3fc); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x7),8));
            { let a = c.r[4].wrapping_add(0x3f4); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x40c); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x3ec); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4f28c7);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x3e4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x54c); wm(m,a,32,0x6u32); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f28f3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3ec),32));
            c.set(3,32,0x3u32);
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3f4),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,0x3u32);
            c.set(3,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.set(5,32,c.get(2,32));
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4f294d; continue; }
            pc = 0x4f293f; continue;
            }
            0x4f293f => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            c.set(3,8,u32::from(c.cond(14)));
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0x5u32,32); c.set(3,32,r); }
            pc = 0x4f2980; continue;
            }
            0x4f294d => {
            { let a = c.get(5,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x4f296d; continue; }
            pc = 0x4f2952; continue;
            }
            0x4f2952 => {
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f295c);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,8); let r = c.and(a,0x1u32,8); c.set(3,8,r); }
            { let a = c.get(3,8); let r = c.neg(a,8); c.set(3,8,r); }
            { let a = c.get(3,32); let r = c.sbb(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xfffffff6u32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xau32,32); c.set(3,32,r); }
            pc = 0x4f2980; continue;
            }
            0x4f296d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            c.set(3,8,u32::from(c.cond(14)));
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0x5u32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x5u32,32); c.set(3,32,r); }
            pc = 0x4f2980; continue;
            }
            0x4f2980 => {
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f298a);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x5u32);
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[5]));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.and(a,0x3u32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x42); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3fc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x404),32));
            { let a = c.r[4].wrapping_add(0x43); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4f29ee; continue; }
            pc = 0x4f29dc; continue;
            }
            0x4f29dc => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x44),8));
            { let a = c.r[4].wrapping_add(0x46); wm(m,a,8,0x0u32); }
            { let a = c.get(1,8); let r = c.or(a,0x80u32,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,c.get(1,8)); }
            pc = 0x4f29f3; continue;
            }
            0x4f29ee => {
            { let a = c.r[4].wrapping_add(0x46); wm(m,a,8,0xfu32); }
            pc = 0x4f29f3; continue;
            }
            0x4f29f3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x46),8));
            { let a = c.get(1,8); let r = c.add(a,c.get(3,8),8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x45); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x46); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x414),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x41c),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[4].wrapping_add(0x47); wm(m,a,8,0xeu32); }
            { let a = c.r[4].wrapping_add(0x41); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x22u32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]),8));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x3d); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            c.set(1,32,0x6dd730u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4f2a6c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[6].wrapping_add(0x14),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,r); } }
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x54c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4f2a84);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4f2c97; continue;
            }
            0x4f2a89 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x42c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x424); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x434); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.push(m, 0x4f2abc);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f2ac8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[3],32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x8),16));
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,0x1u32); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(6,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f2ae9);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x424),32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x42c),32));
            { let a = c.r[7].wrapping_add(0xa); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x434),32));
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.r[7].wrapping_add(0xb); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,16,0xfu32); }
            pc = 0x4f2c93; continue;
            }
            0x4f2b3a => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x43c); wm(m,a,8,c.get(1,8)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x43c),32));
            { let a = c.r[4].wrapping_add(0x44c); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x444); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x444),32));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[5]); wm(m,a,32,0x0u32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4f2c93; continue; }
            pc = 0x4f2b8e; continue;
            }
            0x4f2b8e => {
            { let a = c.get(7,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f2b9b; continue;
            }
            0x4f2b9b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,0x0u32);
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f2bbd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f2bd0; continue; }
            pc = 0x4f2bc1; continue;
            }
            0x4f2bc1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,c.get(7,32));
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[5]); wm(m,a,32,c.get(2,32)); }
            pc = 0x4f2bd0; continue;
            }
            0x4f2bd0 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x4f2b8e; continue; }
            pc = 0x4f2bd5; continue;
            }
            0x4f2bd5 => {
            pc = 0x4f2c93; continue;
            }
            0x4f2bda => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),8));
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(0,8,c.get(1,8));
            { let a = c.r[4].wrapping_add(0x454); wm(m,a,8,c.get(1,8)); }
            { let a = rm(m,0x6dd894u32,16); c.sub(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x4f2c93; continue; }
            pc = 0x4f2c06; continue;
            }
            0x4f2c06 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x454),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4f2c1e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x136),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4f2c33; continue; }
            pc = 0x4f2c28; continue;
            }
            0x4f2c28 => {
            { let a = c.get(1,8); let r = c.or(a,0x40u32,8); c.set(1,8,r); }
            { let a = c.r[0].wrapping_add(0x136); wm(m,a,8,c.get(1,8)); }
            pc = 0x4f2c93; continue;
            }
            0x4f2c33 => {
            { let a = c.get(1,8); let r = c.and(a,0xbfu32,8); c.set(1,8,r); }
            { let a = c.r[0].wrapping_add(0x136); wm(m,a,8,c.get(1,8)); }
            pc = 0x4f2c93; continue;
            }
            0x4f2c3e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(3,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),8));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x45c); wm(m,a,8,c.get(2,8)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x45c),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            c.push(m, 0x4f2c70);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4f2c93; continue; }
            pc = 0x4f2c74; continue;
            }
            0x4f2c74 => {
            c.set(7,32,0x300u32);
            pc = 0x4f2c79; continue;
            }
            0x4f2c79 => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f2c84);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f3660, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(c.r[7]); wm(m,a,16,c.get(3,16)); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x308u32,32); }
            if c.cond(2) { pc = 0x4f2c79; continue; }
            pc = 0x4f2c93; continue;
            }
            0x4f2c93 => {
            { let a = rm(m,c.r[6].wrapping_add(0x14),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,r); } }
            pc = 0x4f2c97; continue;
            }
            0x4f2c97 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(3,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            c.set(7,32,c.r[2].wrapping_add(c.r[0]));
            { let a = c.get(3,8); c.sub(a,0xfdu32,8); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4ecfe2; continue; }
            pc = 0x4f2cc4; continue;
            }
            0x4f2cc4 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x464),32));
            pc = 0x4f2ccb; continue;
            }
            0x4f2ccb => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xfeu32,32); }
            if c.cond(15) { pc = 0x4f2de8; continue; }
            pc = 0x4f2cdf; continue;
            }
            0x4f2cdf => {
            if c.cond(4) { pc = 0x4f2db1; continue; }
            pc = 0x4f2ce5; continue;
            }
            0x4f2ce5 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f2d71; continue; }
            pc = 0x4f2ced; continue;
            }
            0x4f2ced => {
            { let a = c.get(0,32); c.sub(a,0xfdu32,32); }
            if c.cond(5) { pc = 0x4f2e15; continue; }
            pc = 0x4f2cf8; continue;
            }
            0x4f2cf8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f2d1b);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.set(5,32,0x1u32);
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,8,rm(m,c.r[0].wrapping_add(0x3),8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x2),8));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(1,16)); }
            pc = 0x4f2e15; continue;
            }
            0x4f2d44 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,rm(m,0x6dd8b4u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f2d53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f37e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f2d63; continue; }
            pc = 0x4f2d5a; continue;
            }
            0x4f2d5a => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4f2d60);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4f2d63; continue;
            }
            0x4f2d63 => {
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4f2e17; continue;
            }
            0x4f2d71 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,0x6de4a0u32);
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m, 0x4f2d94);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0xffffff00));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4f2da4; continue; }
            pc = 0x4f2d9e; continue;
            }
            0x4f2d9e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,c.get(2,32));
            pc = 0x4f2df3; continue;
            }
            0x4f2da4 => {
            { let a = c.r[6]; wm(m,a,32,c.get(7,32)); }
            { let a = rm(m,c.r[6].wrapping_add(0x14),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,r); } }
            c.set(5,32,0x1u32);
            pc = 0x4f2e15; continue;
            }
            0x4f2db1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x14),16));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4f2dd4);
            { let args = []; c.r[0] = host.call(m, 0x4f3860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(5,32,0x1u32);
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,0x0u32); }
            pc = 0x4f2e15; continue;
            }
            0x4f2de8 => {
            { let a = c.get(0,32); c.sub(a,0xffu32,32); }
            if c.cond(5) { pc = 0x4f2e15; continue; }
            pc = 0x4f2def; continue;
            }
            0x4f2def => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            pc = 0x4f2df3; continue;
            }
            0x4f2df3 => {
            c.set(1,32,rm(m,0x6dd8b4u32,32));
            c.push(m, 0x4f2dfe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f37e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f2e0e; continue; }
            pc = 0x4f2e05; continue;
            }
            0x4f2e05 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4f2e0b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4f2e0e; continue;
            }
            0x4f2e0e => {
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            pc = 0x4f2e15; continue;
            }
            0x4f2e15 => {
            c.set(0,32,c.get(5,32));
            pc = 0x4f2e17; continue;
            }
            0x4f2e17 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x544),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x540u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            0x4f2e32 => {
            c.set(7,32,c.get(7,32));
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3860(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3860;
    loop {
        match pc {
            0x4f3860 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x14),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f38a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f38a0;
    loop {
        match pc {
            0x4f38a0 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xea),16));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3680(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3680;
    loop {
        match pc {
            0x4f3680 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x10),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f38f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f38f0;
    loop {
        match pc {
            0x4f38f0 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xd4),16));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3960(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3960;
    loop {
        match pc {
            0x4f3960 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4f3968);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4f3978; continue; }
            pc = 0x4f396f; continue;
            }
            0x4f396f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4f3975);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4f3978; continue;
            }
            0x4f3978 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004d7b10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d7b10;
    loop {
        match pc {
            0x4d7b10 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x662a04u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6e51acu32,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x4d7d33; continue; }
            pc = 0x4d7b34; continue;
            }
            0x4d7b34 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m, 0x4d7b49);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,0x6dfc18u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,0x6dfc14u32,32));
            if c.cond(4) { pc = 0x4d7b8d; continue; }
            pc = 0x4d7b59; continue;
            }
            0x4d7b59 => {
            c.set(1,32,rm(m,0x6dfc08u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4d7b67; continue; }
            pc = 0x4d7b63; continue;
            }
            0x4d7b63 => {
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4d7b95; continue; }
            pc = 0x4d7b67; continue;
            }
            0x4d7b67 => {
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4d7d33; continue; }
            pc = 0x4d7b72; continue;
            }
            0x4d7b72 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = rm(m,c.r[0].wrapping_add(0x1),8); let r = c.or(a,0x40u32,8); { let a = c.r[0].wrapping_add(0x1); wm(m,a,8,r); } }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4d7b8d => {
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4d7d33; continue; }
            pc = 0x4d7b95; continue;
            }
            0x4d7b95 => {
            { let a = rm(m,0x6e59ccu32,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x4d7d33; continue; }
            pc = 0x4d7ba2; continue;
            }
            0x4d7ba2 => {
            c.set(0,8,rm(m,0x6e59cdu32,8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4d7bb8; continue; }
            pc = 0x4d7bab; continue;
            }
            0x4d7bab => {
            c.set(0,8,rm(m,0x6e59fcu32,8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4d7d33; continue; }
            pc = 0x4d7bb8; continue;
            }
            0x4d7bb8 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x26),8));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4d7be2; continue; }
            pc = 0x4d7bd4; continue;
            }
            0x4d7bd4 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = rm(m,c.r[1].wrapping_add(0x6c0198),8); c.sub(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4d7c06; continue; }
            pc = 0x4d7be2; continue;
            }
            0x4d7be2 => {
            { let a = c.get(0,8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x4d7bf4; continue; }
            pc = 0x4d7be6; continue;
            }
            0x4d7be6 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = rm(m,c.r[2].wrapping_add(0x6c0198),8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x4d7c06; continue; }
            pc = 0x4d7bf4; continue;
            }
            0x4d7bf4 => {
            { let a = c.get(0,8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x4d7c22; continue; }
            pc = 0x4d7bf8; continue;
            }
            0x4d7bf8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = rm(m,c.r[0].wrapping_add(0x6c0198),8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x4d7c22; continue; }
            pc = 0x4d7c06; continue;
            }
            0x4d7c06 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x40u32);
            c.push(m,0xffffffffu32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x1770u32);
            c.push(m, 0x4d7c22);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x443150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            pc = 0x4d7c22; continue;
            }
            0x4d7c22 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x26),8));
            { let a = c.get(2,8); c.and(a,0x8u32,8); }
            if c.cond(4) { pc = 0x4d7c67; continue; }
            pc = 0x4d7c2a; continue;
            }
            0x4d7c2a => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x4),8));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x6c0198),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4d7c67; continue; }
            pc = 0x4d7c3d; continue;
            }
            0x4d7c3d => {
            { let a = c.get(1,8); c.sub(a,0x3cu32,8); }
            if c.cond(2) { pc = 0x4d7c55; continue; }
            pc = 0x4d7c42; continue;
            }
            0x4d7c42 => {
            { let a = c.get(1,8); c.sub(a,0x40u32,8); }
            if c.cond(7) { pc = 0x4d7c55; continue; }
            pc = 0x4d7c47; continue;
            }
            0x4d7c47 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,0x6a4570u32,32));
            pc = 0x4d7cec; continue;
            }
            0x4d7c55 => {
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a52d0),32));
            c.set(2,32,rm(m,0x6a4570u32,32));
            pc = 0x4d7cec; continue;
            }
            0x4d7c67 => {
            { let a = c.get(2,8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x4d7cb8; continue; }
            pc = 0x4d7c6c; continue;
            }
            0x4d7c6c => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x4),8));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x6c0198),8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x4d7cb8; continue; }
            pc = 0x4d7c7f; continue;
            }
            0x4d7c7f => {
            { let a = c.get(1,8); c.sub(a,0x3cu32,8); }
            if c.cond(2) { pc = 0x4d7c8e; continue; }
            pc = 0x4d7c84; continue;
            }
            0x4d7c84 => {
            { let a = c.get(1,8); c.sub(a,0x40u32,8); }
            if c.cond(7) { pc = 0x4d7c8e; continue; }
            pc = 0x4d7c89; continue;
            }
            0x4d7c89 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            pc = 0x4d7c95; continue;
            }
            0x4d7c8e => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a52d0),32));
            pc = 0x4d7c95; continue;
            }
            0x4d7c95 => {
            c.set(1,32,rm(m,0x6a4574u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4d7ca7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            pc = 0x4d7d07; continue;
            }
            0x4d7cb8 => {
            { let a = c.get(2,8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x4d7d22; continue; }
            pc = 0x4d7cbd; continue;
            }
            0x4d7cbd => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x4),8));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x6c0198),8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x4d7d22; continue; }
            pc = 0x4d7cd0; continue;
            }
            0x4d7cd0 => {
            { let a = c.get(1,8); c.sub(a,0x3cu32,8); }
            if c.cond(2) { pc = 0x4d7cdf; continue; }
            pc = 0x4d7cd5; continue;
            }
            0x4d7cd5 => {
            { let a = c.get(1,8); c.sub(a,0x40u32,8); }
            if c.cond(7) { pc = 0x4d7cdf; continue; }
            pc = 0x4d7cda; continue;
            }
            0x4d7cda => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            pc = 0x4d7ce6; continue;
            }
            0x4d7cdf => {
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a52d0),32));
            pc = 0x4d7ce6; continue;
            }
            0x4d7ce6 => {
            c.set(2,32,rm(m,0x6a4578u32,32));
            pc = 0x4d7cec; continue;
            }
            0x4d7cec => {
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d7cf8);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.set(1,32,c.get(4,32));
            c.push(m,c.get(2,32));
            pc = 0x4d7d07; continue;
            }
            0x4d7d07 => {
            c.push(m, 0x4d7d0c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x4d7d17);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42bb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x4d7d22);
            { let args = []; c.r[0] = host.call(m, 0x4237d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4d7d22; continue;
            }
            0x4d7d22 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4d7d33);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4d7d33; continue;
            }
            0x4d7d33 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f38d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f38d0;
    loop {
        match pc {
            0x4f38d0 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xd0),16));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f38e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f38e0;
    loop {
        match pc {
            0x4f38e0 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xd2),16));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3660(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3660;
    loop {
        match pc {
            0x4f3660 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x68),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3690(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3690;
    loop {
        match pc {
            0x4f3690 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x2c),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f36a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f36a0;
    loop {
        match pc {
            0x4f36a0 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x771),8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f36b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f36b0;
    loop {
        match pc {
            0x4f36b0 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x773),8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f36c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f36c0;
    loop {
        match pc {
            0x4f36c0 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x17),8));
            { let a = c.get(0,8); c.sub(a,0x14u32,8); }
            if c.cond(14) { pc = 0x4f36cd; continue; }
            pc = 0x4f36c7; continue;
            }
            0x4f36c7 => {
            c.set(0,32,0x14u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4f36cd => {
            { let a = c.get(0,8); c.sub(a,0xecu32,8); }
            if c.cond(13) { pc = 0x4f36d7; continue; }
            pc = 0x4f36d1; continue;
            }
            0x4f36d1 => {
            c.set(0,32,0xffffffecu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4f36d7 => {
            c.set(0,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3740(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3740;
    loop {
        match pc {
            0x4f3740 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.fld(c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2fc),4));
            { let v = c.ld(m,0x672850u32,4); c.fop('*',v); }
            c.push(m, 0x4f3756);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3760(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3760;
    loop {
        match pc {
            0x4f3760 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x19),8));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x39c),8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f37c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f37c0;
    loop {
        match pc {
            0x4f37c0 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x10),16));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f37d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f37d0;
    loop {
        match pc {
            0x4f37d0 => {
            { let a = rm(m,c.r[1].wrapping_add(0x82c),8); c.sub(a,0x14u32,8); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3850(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3850;
    loop {
        match pc {
            0x4f3850 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3870(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3870;
    loop {
        match pc {
            0x4f3870 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3890(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3890;
    loop {
        match pc {
            0x4f3890 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xe8),16));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f38b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f38b0;
    loop {
        match pc {
            0x4f38b0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x6),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(7) { pc = 0x4f38c8; continue; }
            pc = 0x4f38be; continue;
            }
            0x4f38be => {
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4f38c8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3940(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3940;
    loop {
        match pc {
            0x4f3940 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x3fd),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4f3959; continue; }
            pc = 0x4f394a; continue;
            }
            0x4f394a => {
            { let a = rm(m,c.r[1].wrapping_add(0x3fc),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x4f3959; continue; }
            pc = 0x4f3953; continue;
            }
            0x4f3953 => {
            c.set(0,32,0x1u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4f3959 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00533210(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x533210;
    loop {
        match pc {
            0x533210 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(5,32,0xfffffffu32);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            if c.cond(3) { pc = 0x533261; continue; }
            pc = 0x53323a; continue;
            }
            0x53323a => {
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x533261; continue; }
            pc = 0x533243; continue;
            }
            0x533243 => {
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0xe),8));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,c.get(0,8)); }
            pc = 0x533261; continue;
            }
            0x533261 => {
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x5332fd; continue; }
            pc = 0x533270; continue;
            }
            0x533270 => {
            c.set(1,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,32); let r = c.add(a,0x2c8u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x533280; continue;
            }
            0x533280 => {
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5332e0; continue; }
            pc = 0x533284; continue;
            }
            0x533284 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x6c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]),8));
            c.set(6,32,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(3) { pc = 0x5332e0; continue; }
            pc = 0x5332a0; continue;
            }
            0x5332a0 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5332b8; continue; }
            pc = 0x5332a8; continue;
            }
            0x5332a8 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = rm(m,c.r[0],8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x5332b8; continue; }
            pc = 0x5332b1; continue;
            }
            0x5332b1 => {
            { let a = rm(m,c.r[4].wrapping_add(0x38),8); c.sub(a,0x1u32,8); }
            if c.cond(7) { pc = 0x5332e0; continue; }
            pc = 0x5332b8; continue;
            }
            0x5332b8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x5332da; continue; }
            pc = 0x5332c2; continue;
            }
            0x5332c2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32),32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            if c.cond(4) { pc = 0x5332d6; continue; }
            pc = 0x5332cf; continue;
            }
            0x5332cf => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x5332c2; continue; }
            pc = 0x5332d4; continue;
            }
            0x5332d4 => {
            pc = 0x5332da; continue;
            }
            0x5332d6 => {
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x5332e0; continue; }
            pc = 0x5332da; continue;
            }
            0x5332da => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(5,32,c.get(6,32));
            pc = 0x5332e0; continue;
            }
            0x5332e0 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x9f8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            if c.cond(2) { pc = 0x533280; continue; }
            pc = 0x5332f9; continue;
            }
            0x5332f9 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            pc = 0x5332fd; continue;
            }
            0x5332fd => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x34),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5333b0; continue; }
            pc = 0x533309; continue;
            }
            0x533309 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x5333b0; continue; }
            pc = 0x533313; continue;
            }
            0x533313 => {
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(5,32,c.r[1].wrapping_add(0x2c8));
            pc = 0x53332b; continue;
            }
            0x53332b => {
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x53339a; continue; }
            pc = 0x53332f; continue;
            }
            0x53332f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x53339a; continue; }
            pc = 0x533348; continue;
            }
            0x533348 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x3u32);
            { let b = c.get(1,32); c.div32(b); }
            { let a = rm(m,c.r[4].wrapping_add(0x34),32); let r = c.inc(a,32); { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,r); } }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x53339a; continue; }
            pc = 0x53335d; continue;
            }
            0x53335d => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x533372; continue; }
            pc = 0x533365; continue;
            }
            0x533365 => {
            { let a = rm(m,c.r[5],8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x533372; continue; }
            pc = 0x53336b; continue;
            }
            0x53336b => {
            { let a = rm(m,c.r[4].wrapping_add(0x38),8); c.sub(a,0x1u32,8); }
            if c.cond(7) { pc = 0x53339a; continue; }
            pc = 0x533372; continue;
            }
            0x533372 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x53338a; continue; }
            pc = 0x53337c; continue;
            }
            0x53337c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x533380; continue;
            }
            0x533380 => {
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32),32); }
            if c.cond(4) { pc = 0x533396; continue; }
            pc = 0x533385; continue;
            }
            0x533385 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x533380; continue; }
            pc = 0x53338a; continue;
            }
            0x53338a => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            0x533396 => {
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x53338a; continue; }
            pc = 0x53339a; continue;
            }
            0x53339a => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x9f8u32,32); c.set(5,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x5333b0; continue; }
            pc = 0x5333a7; continue;
            }
            0x5333a7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x53332b; continue;
            }
            0x5333b0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004d69d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d69d0;
    loop {
        match pc {
            0x4d69d0 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4d6a46; continue; }
            pc = 0x4d69e4; continue;
            }
            0x4d69e4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4d6a46; continue; }
            pc = 0x4d69ec; continue;
            }
            0x4d69ec => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4d6a46; continue; }
            pc = 0x4d69f9; continue;
            }
            0x4d69f9 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4d6a04);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xa),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4d6a46; continue; }
            pc = 0x4d6a16; continue;
            }
            0x4d6a16 => {
            c.set(2,32,c.get(7,32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x4d6a46; continue; }
            pc = 0x4d6a1d; continue;
            }
            0x4d6a1d => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4d6a3a; continue; }
            pc = 0x4d6a21; continue;
            }
            0x4d6a21 => {
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x6),16));
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x4d6a16; continue; }
            pc = 0x4d6a33; continue;
            }
            0x4d6a33 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4d6a3a => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1));
            { let a = rm(m,c.r[0],8); let r = c.and(a,0x7fu32,8); { let a = c.r[0]; wm(m,a,8,r); } }
            pc = 0x4d6a46; continue;
            }
            0x4d6a46 => {
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

pub fn f_004d7960(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d7960;
    loop {
        match pc {
            0x4d7960 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6629f2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(1,32));
            c.push(m, 0x4d7997);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4d79a7);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(5,32,c.r[6].wrapping_add(0x4));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[5],16),16); }
            if c.cond(3) { pc = 0x4d79bb; continue; }
            pc = 0x4d79b7; continue;
            }
            0x4d79b7 => {
            { let a = c.r[5]; wm(m,a,16,c.get(0,16)); }
            pc = 0x4d79bb; continue;
            }
            0x4d79bb => {
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x4d7a17; continue; }
            pc = 0x4d79c4; continue;
            }
            0x4d79c4 => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4d79cf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.add(a,0xau32,32); c.set(0,32,r); }
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x6),16),16); }
            if c.cond(3) { pc = 0x4d7a0a; continue; }
            pc = 0x4d79dc; continue;
            }
            0x4d79dc => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x4d79f0; continue; }
            pc = 0x4d79e9; continue;
            }
            0x4d79e9 => {
            { let a = c.r[0]; wm(m,a,16,0xffffu32); }
            pc = 0x4d7a01; continue;
            }
            0x4d79f0 => {
            { let a = c.r[1]; wm(m,a,8,0x1u32); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0],16));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x6));
            pc = 0x4d7a01; continue;
            }
            0x4d7a01 => {
            c.set(1,16,rm(m,c.r[0],16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x6),16),16); }
            if c.cond(2) { pc = 0x4d79dc; continue; }
            pc = 0x4d7a0a; continue;
            }
            0x4d7a0a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4d79c4; continue; }
            pc = 0x4d7a17; continue;
            }
            0x4d7a17 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(1,32,c.r[6].wrapping_add(0x8));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x6),16),16); }
            if c.cond(3) { pc = 0x4d7a51; continue; }
            pc = 0x4d7a24; continue;
            }
            0x4d7a24 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x4d7a43; continue; }
            pc = 0x4d7a30; continue;
            }
            0x4d7a30 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,8,0x1u32); }
            c.set(2,16,rm(m,c.r[1],16));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x6));
            pc = 0x4d7a48; continue;
            }
            0x4d7a43 => {
            { let a = c.r[1]; wm(m,a,16,0xffffu32); }
            pc = 0x4d7a48; continue;
            }
            0x4d7a48 => {
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x6),16),16); }
            if c.cond(2) { pc = 0x4d7a24; continue; }
            pc = 0x4d7a51; continue;
            }
            0x4d7a51 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(1,32,c.get(5,32));
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x4d7a8d; continue; }
            pc = 0x4d7a5f; continue;
            }
            0x4d7a5f => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x4d7a7e; continue; }
            pc = 0x4d7a6b; continue;
            }
            0x4d7a6b => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,8,0x1u32); }
            c.set(2,16,rm(m,c.r[1],16));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x6));
            pc = 0x4d7a81; continue;
            }
            0x4d7a7e => {
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            pc = 0x4d7a81; continue;
            }
            0x4d7a81 => {
            c.set(0,16,rm(m,c.r[1],16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x4d7a5f; continue; }
            pc = 0x4d7a8d; continue;
            }
            0x4d7a8d => {
            c.push(m,c.get(7,32));
            c.push(m, 0x4d7a93);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4d7aa7);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
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

pub fn f_005483d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5483d0;
    loop {
        match pc {
            0x5483d0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66390bu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,0x2au32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(7) { pc = 0x548798; continue; }
            pc = 0x548403; continue;
            }
            0x548403 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x548834),8));
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x5487e8)); continue;
            }
            0x548412 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x548452; continue; }
            pc = 0x548426; continue;
            }
            0x548426 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x160));
            c.set(2,32,rm(m,0x6b23f0u32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x54844a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5487a9; continue;
            }
            0x548452 => {
            c.set(1,32,rm(m,0x6b28a8u32,32));
            c.push(m,c.get(1,32));
            pc = 0x5487a0; continue;
            }
            0x54845e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6b28a8),32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x54847c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5487a9; continue;
            }
            0x548484 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x54858f; continue; }
            pc = 0x54849c; continue;
            }
            0x54849c => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6b28a8),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x39),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5484c9);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5487a9; continue;
            }
            0x5484d1 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x54858f; continue; }
            pc = 0x5484e9; continue;
            }
            0x5484e9 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6b28a8),32));
            { let a = c.get(0,32); let r = c.add(a,0x160u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x54850f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5487a9; continue;
            }
            0x548517 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x548526);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6b2900u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x548538);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5487a9; continue;
            }
            0x548540 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x54858f; continue; }
            pc = 0x548554; continue;
            }
            0x548554 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(2,32,rm(m,0x6de4b0u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x38),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x54858f; continue; }
            pc = 0x548571; continue;
            }
            0x548571 => {
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[0].wrapping_add(0x160));
            c.push(m,c.get(2,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xe),8));
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0e38),32));
            pc = 0x5485e1; continue;
            }
            0x54858f => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x548598);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5487a9; continue;
            }
            0x54859d => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x54858f; continue; }
            pc = 0x5485b1; continue;
            }
            0x5485b1 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(2,32,rm(m,0x6de4b0u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.get(0,32); let r = c.add(a,0x160u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x5485fb; continue; }
            pc = 0x5485d3; continue;
            }
            0x5485d3 => {
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6c3638u32,32));
            pc = 0x5485e1; continue;
            }
            0x5485e1 => {
            c.set(1,32,rm(m,0x6b28bcu32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5485f3);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x5487a9; continue;
            }
            0x5485fb => {
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b28b4u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x54860c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5487a9; continue;
            }
            0x548614 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x54863d; continue; }
            pc = 0x548628; continue;
            }
            0x548628 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x160));
            pc = 0x548642; continue;
            }
            0x54863d => {
            c.set(0,32,0x6dfc8au32);
            pc = 0x548642; continue;
            }
            0x548642 => {
            c.set(2,32,rm(m,0x6b28b8u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x548654);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5487a9; continue;
            }
            0x54865c => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x54858f; continue; }
            pc = 0x548674; continue;
            }
            0x548674 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x160));
            c.set(2,32,rm(m,0x6b28c8u32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x548698);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5487a9; continue;
            }
            0x5486a0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(3) { pc = 0x5486b5; continue; }
            pc = 0x5486a9; continue;
            }
            0x5486a9 => {
            c.set(1,32,rm(m,0x6c3594u32,32));
            c.push(m,c.get(1,32));
            pc = 0x5487a0; continue;
            }
            0x5486b5 => {
            c.set(2,32,rm(m,0x6b291cu32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5486c7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5487a9; continue;
            }
            0x5486cf => {
            c.set(2,32,0x1u32);
            pc = 0x5486d4; continue;
            }
            0x5486d4 => {
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            pc = 0x5486d7; continue;
            }
            0x5486d7 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x5486d8; continue;
            }
            0x5486d8 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x5486d9; continue;
            }
            0x5486d9 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x6u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5487a9; continue; }
            pc = 0x5486f4; continue;
            }
            0x5486f4 => {
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6b04f4),32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x160u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x54871a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5487a9; continue;
            }
            0x548722 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(1,32,rm(m,0x6b294cu32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x548761);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x5487a9; continue;
            }
            0x548766 => {
            { let a = rm(m,c.r[1].wrapping_add(0x928),32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5487c5; continue; }
            pc = 0x54876e; continue;
            }
            0x54876e => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(1,32,rm(m,0x6b2950u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x548795);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x548798; continue;
            }
            0x548798 => {
            c.set(0,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6b28a8),32));
            c.push(m,c.get(0,32));
            pc = 0x5487a0; continue;
            }
            0x5487a0 => {
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x5487a9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5487a9; continue;
            }
            0x5487a9 => {
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5487ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x5487c5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42b4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5487c5; continue;
            }
            0x5487c5 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5487d6);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00535400(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x535400;
    loop {
        match pc {
            0x535400 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            c.set(3,32,c.get(1,32));
            if c.cond(3) { pc = 0x53544b; continue; }
            pc = 0x535411; continue;
            }
            0x535411 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(6,32,c.r[3].wrapping_add(0xa08));
            pc = 0x53541b; continue;
            }
            0x53541b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(c.r[7]).wrapping_add(0xa00),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x535440; continue; }
            pc = 0x53542c; continue;
            }
            0x53542c => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x535440; continue; }
            pc = 0x535432; continue;
            }
            0x535432 => {
            { let a = c.r[0].wrapping_add(c.r[5]); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x535440);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x535630, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x535440; continue;
            }
            0x535440 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,0x8u32,32); }
            if c.cond(2) { pc = 0x53541b; continue; }
            pc = 0x535449; continue;
            }
            0x535449 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x53544b; continue;
            }
            0x53544b => {
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004d7030(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d7030;
    loop {
        match pc {
            0x4d7030 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4d7134; continue; }
            pc = 0x4d704a; continue;
            }
            0x4d704a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,0x64u32,32); }
            if c.cond(3) { pc = 0x4d7134; continue; }
            pc = 0x4d7057; continue;
            }
            0x4d7057 => {
            c.set(2,16,rm(m,c.r[0].wrapping_mul(8).wrapping_add(0x671fd0),16));
            c.set(0,16,rm(m,c.r[0].wrapping_mul(8).wrapping_add(0x671fd4),16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(1,32));
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x4d7082);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); let r = c.add(a,0xau32,32); c.set(5,32,r); }
            c.set(2,16,rm(m,c.r[5],16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x6),16),16); }
            if c.cond(3) { pc = 0x4d7134; continue; }
            pc = 0x4d7095; continue;
            }
            0x4d7095 => {
            c.set(1,32,c.get(3,32));
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x4d7134; continue; }
            pc = 0x4d70a0; continue;
            }
            0x4d70a0 => {
            c.set(7,32,rm(m,c.r[6],32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x4),8));
            { let a = c.get(0,8); c.sub(a,0x45u32,8); }
            if c.cond(5) { pc = 0x4d70f9; continue; }
            pc = 0x4d70b7; continue;
            }
            0x4d70b7 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = rm(m,c.r[1].wrapping_add(0xc),16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x4d7117; continue; }
            pc = 0x4d70c2; continue;
            }
            0x4d70c2 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x18),16));
            { let a = rm(m,c.r[1].wrapping_add(0xe),16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x4d7117; continue; }
            pc = 0x4d70cd; continue;
            }
            0x4d70cd => {
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[7]).wrapping_add(0x6),16));
            { let a = c.r[5]; wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0x4); wm(m,a,8,0xffu32); }
            c.set(0,32,rm(m,c.r[6],32));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0x6); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,16,c.get(2,16)); }
            pc = 0x4d7126; continue;
            }
            0x4d70f9 => {
            { let a = c.get(0,8); c.sub(a,0x46u32,8); }
            if c.cond(5) { pc = 0x4d7117; continue; }
            pc = 0x4d70fd; continue;
            }
            0x4d70fd => {
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = rm(m,c.r[1].wrapping_add(0xc),16); c.sub(a,c.get(2,16),16); }
            if c.cond(5) { pc = 0x4d7117; continue; }
            pc = 0x4d7108; continue;
            }
            0x4d7108 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x18),16));
            { let a = rm(m,c.r[1].wrapping_add(0xe),16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x4d7117; continue; }
            pc = 0x4d7113; continue;
            }
            0x4d7113 => {
            { let a = rm(m,c.r[1].wrapping_add(0xb),8); let r = c.or(a,0xfeu32,8); { let a = c.r[1].wrapping_add(0xb); wm(m,a,8,r); } }
            pc = 0x4d7117; continue;
            }
            0x4d7117 => {
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[5],16));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(5,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x6));
            pc = 0x4d7126; continue;
            }
            0x4d7126 => {
            c.set(2,16,rm(m,c.r[5],16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x6),16),16); }
            if c.cond(2) { pc = 0x4d7095; continue; }
            pc = 0x4d7134; continue;
            }
            0x4d7134 => {
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

pub fn f_0050df00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50df00;
    loop {
        match pc {
            0x50df00 => {
            c.push(m,c.get(6,32));
            c.set(6,16,rm(m,c.r[1].wrapping_add(0xf0),16));
            c.push(m,c.get(7,32));
            pc = 0x50df09; continue;
            }
            0x50df09 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xee),16));
            c.set(7,32,0x133u32);
            { let a = c.get(0,16); let r = c.add(a,c.get(6,16),16); c.set(0,16,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[1].wrapping_add(0xee); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6ddb78u32,16),16); }
            if c.cond(3) { pc = 0x50df09; continue; }
            pc = 0x50df32; continue;
            }
            0x50df32 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00514d40(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x514d40;
    loop {
        match pc {
            0x514d40 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66316fu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x2cu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.r[1].wrapping_add(0x12));
            c.set(2,32,0xffffu32);
            c.set(3,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,16,rm(m,0x6dd894u32,16));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(0,16,rm(m,c.r[3],16));
            c.push(m,c.get(7,32));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x1u32); }
            if c.cond(3) { pc = 0x51508b; continue; }
            pc = 0x514d9b; continue;
            }
            0x514d9b => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            c.set(5,32,c.r[6].wrapping_add(0x42));
            if c.cond(3) { pc = 0x51506c; continue; }
            pc = 0x514dc2; continue;
            }
            0x514dc2 => {
            c.set(2,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m, 0x514dd8);
            { let args = []; c.r[0] = host.call(m, 0x4fe150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x515061; continue; }
            pc = 0x514de0; continue;
            }
            0x514de0 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x51506c; continue; }
            pc = 0x514dee; continue;
            }
            0x514dee => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(2,16,rm(m,c.r[3],16));
            { let a = c.r[3]; wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,16,0xffffu32); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x15d),8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x51506c; continue; }
            pc = 0x514e20; continue;
            }
            0x514e20 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4c),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x51506c; continue; }
            pc = 0x514e30; continue;
            }
            0x514e30 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x514e39);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x514e48);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,0x1u32); }
            c.push(m, 0x514e5c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(3,16),16); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(3,8)); }
            if c.cond(6) { pc = 0x514ea3; continue; }
            pc = 0x514e6d; continue;
            }
            0x514e6d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x514e82);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x514e8c; continue; }
            pc = 0x514e88; continue;
            }
            0x514e88 => {
            { let a = rm(m,c.r[4].wrapping_add(0x4c),8); let r = c.inc(a,8); { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,r); } }
            pc = 0x514e8c; continue;
            }
            0x514e8c => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x50),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(0,8)); }
            c.set(0,16,c.get(0,8));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x514e6d; continue; }
            pc = 0x514ea3; continue;
            }
            0x514ea3 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x514eb7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(3,8)); }
            pc = 0x514ebf; continue;
            }
            0x514ebf => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x514ed4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,c.get(3,16),16); }
            if c.cond(5) { pc = 0x514ede; continue; }
            pc = 0x514eda; continue;
            }
            0x514eda => {
            { let a = rm(m,c.r[4].wrapping_add(0x4c),8); let r = c.inc(a,8); { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,r); } }
            pc = 0x514ede; continue;
            }
            0x514ede => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x50),8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x4c),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(0,8)); }
            if c.cond(6) { pc = 0x514ebf; continue; }
            pc = 0x514ef0; continue;
            }
            0x514ef0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x514f04);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6dd7a0u32,32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x2au32); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x19),8));
            { let a = c.r[4].wrapping_add(0x31); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1a),8));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x514f35);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x35); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x19),8));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x39c),8));
            { let a = c.r[4].wrapping_add(0x36); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1c),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.r[4].wrapping_add(0x37); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1d),8));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x39); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x3),8));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.set(1,32,0x6dd730u32);
            c.push(m,c.get(0,32));
            c.push(m,0xfffffffeu32);
            { let a = c.r[4].wrapping_add(0x42); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x43); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x514f7e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0xau32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x514f89);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x514ff2; continue; }
            pc = 0x514f8d; continue;
            }
            0x514f8d => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,0x6dd73cu32);
            { let a = c.get(0,32); let r = c.add(a,0x300u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x514fa2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,16,0x5u32); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x15d),8));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x15d),8));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x514fc6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x4c),8));
            c.set(2,8,c.get(0,8));
            { let a = c.r[7].wrapping_add(0x9); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[7].wrapping_add(0xa); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[7].wrapping_add(0xb); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,8,0x4u32); }
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x11); wm(m,a,8,c.get(3,8)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x15e),16));
            { let a = c.r[7].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            pc = 0x514ff2; continue;
            }
            0x514ff2 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x514ffe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x516840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,0xffffu32);
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(7,32); let r = c.and(a,c.get(5,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x515010);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x51501b);
            { let args = []; c.r[0] = host.call(m, 0x453df0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x3c),8); let r = c.or(a,0x20u32,8); { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,r); } }
            c.push(m,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,16,0x11u32); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[6].wrapping_add(0x52); wm(m,a,16,c.get(5,16)); }
            c.push(m, 0x51503e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x507fb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x51504b);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x51505c);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5152d9; continue;
            }
            0x515061 => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x51506c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5098b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x51506c; continue;
            }
            0x51506c => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(1,16,rm(m,0x6dd894u32,16));
            c.set(3,32,c.r[6].wrapping_add(0x6));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(2) { pc = 0x514d9b; continue; }
            pc = 0x515083; continue;
            }
            0x515083 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x51508b; continue;
            }
            0x51508b => {
            { let a = c.get(2,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x5152d9; continue; }
            pc = 0x515094; continue;
            }
            0x515094 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x5150a4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5062f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x515138; continue; }
            pc = 0x5150c9; continue;
            }
            0x5150c9 => {
            c.set(2,16,rm(m,c.r[6],16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x515138; continue; }
            pc = 0x5150d5; continue;
            }
            0x5150d5 => {
            c.set(7,16,rm(m,c.r[4].wrapping_add(0x4c),16));
            pc = 0x5150da; continue;
            }
            0x5150da => {
            { let a = rm(m,c.r[4].wrapping_add(0x50),16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x515138; continue; }
            pc = 0x5150e1; continue;
            }
            0x5150e1 => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),32),32); }
            if c.cond(12) { pc = 0x515129; continue; }
            pc = 0x5150fe; continue;
            }
            0x5150fe => {
            c.set(5,16,rm(m,c.r[0].wrapping_add(0x42),16));
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(2) { pc = 0x515129; continue; }
            pc = 0x51510b; continue;
            }
            0x51510b => {
            c.set(5,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.r[6]; wm(m,a,16,c.get(5,16)); }
            c.set(5,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,c.get(5,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(0xa); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            pc = 0x51512c; continue;
            }
            0x515129 => {
            c.set(6,32,c.r[0].wrapping_add(0x6));
            pc = 0x51512c; continue;
            }
            0x51512c => {
            c.set(2,16,rm(m,c.r[6],16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x5150da; continue; }
            pc = 0x515138; continue;
            }
            0x515138 => {
            c.set(5,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x51529e; continue; }
            pc = 0x515149; continue;
            }
            0x515149 => {
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.get(5,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(6,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xe),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbb0),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x1e); wm(m,a,16,c.get(2,16)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xe),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6ddbb4),16));
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x22); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x3e); wm(m,a,16,0x700u32); }
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,16,0x12u32); }
            { let a = c.r[6].wrapping_add(0x15c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x5151b8);
            { let args = []; c.r[0] = host.call(m, 0x515750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x132),16));
            { let a = c.r[6]; wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x1); wm(m,a,8,c.get(3,8)); }
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x18),16));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[6].wrapping_add(0x124); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x128); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x12c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x130); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x136); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x13b); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x13a); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd72au32,16),16); }
            if c.cond(7) { pc = 0x515220; continue; }
            pc = 0x515205; continue;
            }
            0x515205 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd728u32);
            c.push(m, 0x515217);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d4dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[6].wrapping_add(0x132); wm(m,a,16,0xffffu32); }
            pc = 0x515220; continue;
            }
            0x515220 => {
            c.set(0,16,rm(m,0x6dd894u32,16));
            { let a = rm(m,c.r[6].wrapping_add(0xa),16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x515232; continue; }
            pc = 0x51522c; continue;
            }
            0x51522c => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x51523d; continue; }
            pc = 0x515232; continue;
            }
            0x515232 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x51523d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x51523d; continue;
            }
            0x51523d => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x37); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x39); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,0xffffu32); }
            c.push(m, 0x51525f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x516840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,16,c.get(3,16)); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[6].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0xffffu32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,0x7fffffffu32); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x52); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x515289);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x515149; continue; }
            pc = 0x51529a; continue;
            }
            0x51529a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x51529e; continue;
            }
            0x51529e => {
            { let a = c.r[1]; wm(m,a,8,0xffu32); }
            { let a = c.r[1].wrapping_add(0x12); wm(m,a,16,0x12u32); }
            { let a = c.r[1].wrapping_add(0x14); wm(m,a,16,0x2u32); }
            { let a = c.r[1].wrapping_add(0x16); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[1].wrapping_add(0x18); wm(m,a,32,0xffffu32); }
            { let a = c.r[1].wrapping_add(0x1c); wm(m,a,16,0xffu32); }
            { let a = c.r[1].wrapping_add(0x1e); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[1].wrapping_add(0x1f); wm(m,a,8,0xffu32); }
            c.push(m, 0x5152ca);
            { let args = []; c.r[0] = host.call(m, 0x501be0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x5152d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50aae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5152d9; continue;
            }
            0x5152d9 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00539b50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x539b50;
    loop {
        match pc {
            0x539b50 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x539c52; continue; }
            pc = 0x539b65; continue;
            }
            0x539b65 => {
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x539c52; continue; }
            pc = 0x539b8b; continue;
            }
            0x539b8b => {
            { let a = rm(m,c.r[7].wrapping_add(0x12),16); c.sub(a,0xfu32,16); }
            if c.cond(3) { pc = 0x539bb2; continue; }
            pc = 0x539b92; continue;
            }
            0x539b92 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m, 0x539baa);
            { let args = []; c.r[0] = host.call(m, 0x519ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x539c52; continue; }
            pc = 0x539bb2; continue;
            }
            0x539bb2 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(3,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(5,32));
            c.set(5,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0xffu32); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,0x76u32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,0x640u32); }
            c.push(m, 0x539be0);
            { let args = []; c.r[0] = host.call(m, 0x537670, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x10u32);
            c.set(0,32,c.r[7].wrapping_add(0x24));
            c.push(m,0xffu32);
            c.push(m,c.get(0,32));
            { let a = c.r[7].wrapping_add(0x38); wm(m,a,16,c.get(5,16)); }
            c.push(m, 0x539bf4);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4u32);
            c.set(1,32,c.r[7].wrapping_add(0x34));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x539c01);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x539c48; continue; }
            pc = 0x539c14; continue;
            }
            0x539c14 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x539c24);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x539c34; continue; }
            pc = 0x539c2a; continue;
            }
            0x539c2a => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(3,8,u32::from(c.cond(14)));
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.and(a,c.get(1,32),32); c.set(3,32,r); }
            pc = 0x539c34; continue;
            }
            0x539c34 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(0,8); c.sub(a,rm(m,0x6de52bu32,8),8); }
            if c.cond(6) { pc = 0x539c48; continue; }
            pc = 0x539c43; continue;
            }
            0x539c43 => {
            { let a = 0x6de52bu32; wm(m,a,8,c.get(0,8)); }
            pc = 0x539c48; continue;
            }
            0x539c48 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x20),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.r[7].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x539c52; continue;
            }
            0x539c52 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00539c60(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x539c60;
    loop {
        match pc {
            0x539c60 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x539ce1; continue; }
            pc = 0x539c71; continue;
            }
            0x539c71 => {
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x539ce1; continue; }
            pc = 0x539c93; continue;
            }
            0x539c93 => {
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); c.sub(a,0xfu32,16); }
            if c.cond(3) { pc = 0x539cb6; continue; }
            pc = 0x539c9a; continue;
            }
            0x539c9a => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m, 0x539cb2);
            { let args = []; c.r[0] = host.call(m, 0x519ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x539ce1; continue; }
            pc = 0x539cb6; continue;
            }
            0x539cb6 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x24); wm(m,a,16,c.get(0,16)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x6),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x26); wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x6),16));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x34); wm(m,a,8,c.get(1,8)); }
            pc = 0x539ce1; continue;
            }
            0x539ce1 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053ce30(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53ce30;
    loop {
        match pc {
            0x53ce30 => {
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x5),8));
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x53cef2; continue; }
            pc = 0x53ce44; continue;
            }
            0x53ce44 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x8),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x53ce95; continue; }
            pc = 0x53ce4c; continue;
            }
            0x53ce4c => {
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x8),8));
            c.set(2,32,0x36u32);
            { let a = c.get(3,8); c.sub(a,0x2cu32,8); }
            if c.cond(5) { pc = 0x53ce78; continue; }
            pc = 0x53ce59; continue;
            }
            0x53ce59 => {
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x4),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51b890, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x53ce78 => {
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            c.push(m,c.get(6,32));
            pc = 0x53ce7f; continue;
            }
            0x53ce7f => {
            c.set(6,32,sx(rm(m,c.r[2].wrapping_add(0x672fbf),8),8)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x53cec7; continue; }
            pc = 0x53ce8b; continue;
            }
            0x53ce8b => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(15) { pc = 0x53ce7f; continue; }
            pc = 0x53ce8f; continue;
            }
            0x53ce8f => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            pc = 0x53ce90; continue;
            }
            0x53ce90 => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(12) { pc = 0x53cef2; continue; }
            pc = 0x53ce95; continue;
            }
            0x53ce95 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x8),8));
            { let a = c.get(2,8); c.sub(a,0x30u32,8); }
            if c.cond(3) { pc = 0x53cecc; continue; }
            pc = 0x53ce9d; continue;
            }
            0x53ce9d => {
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x9),8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x7),8));
            c.push(m,c.get(2,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x6),8));
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x4),8));
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x53cec3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x521900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x53cec7 => {
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,8,c.get(2,8)); }
            pc = 0x53ce90; continue;
            }
            0x53cecc => {
            { let a = c.get(2,8); c.sub(a,0x33u32,8); }
            if c.cond(7) { pc = 0x53cef6; continue; }
            pc = 0x53ced1; continue;
            }
            0x53ced1 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x4),8));
            c.push(m,c.get(2,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x7),8));
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x6),8));
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x53cef2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x523830, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x53cef2; continue;
            }
            0x53cef2 => {
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x53cef6 => {
            { let a = c.get(2,8); c.sub(a,0x36u32,8); }
            if c.cond(3) { pc = 0x53cf20; continue; }
            pc = 0x53cefb; continue;
            }
            0x53cefb => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x4),8));
            c.push(m,c.get(2,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x7),8));
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x6),8));
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x53cf1c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x523d70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x53cf20 => {
            { let a = c.get(2,8); c.sub(a,0x3au32,8); }
            if c.cond(3) { pc = 0x53cef2; continue; }
            pc = 0x53cf25; continue;
            }
            0x53cf25 => {
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.sub(a,0x36u32,32); c.set(2,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x522b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00444030(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x444030;
    loop {
        match pc {
            0x444030 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1c6),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44418a; continue; }
            pc = 0x444044; continue;
            }
            0x444044 => {
            c.set(3,32,rm(m,c.r[6],32));
            { let a = c.r[6].wrapping_add(0x1c6); wm(m,a,8,0x0u32); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4440db; continue; }
            pc = 0x444055; continue;
            }
            0x444055 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1d8),8));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x1bc),32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(7,32,c.get(2,32));
            { let a = c.get(7,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x444081; continue; }
            pc = 0x44407c; continue;
            }
            0x44407c => {
            c.set(7,32,0x7fu32);
            pc = 0x444081; continue;
            }
            0x444081 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(1,32,0x1388u32);
            if c.cond(4) { pc = 0x444091; continue; }
            pc = 0x44408a; continue;
            }
            0x44408a => {
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(1,32,c.get(0,32));
            pc = 0x444091; continue;
            }
            0x444091 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c8),32));
            { let a = c.r[6].wrapping_add(0x1c6); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4440db; continue; }
            pc = 0x4440af; continue;
            }
            0x4440af => {
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a5a0u32,32); c.push(m, 0x4440b6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x4440c5; continue; }
            pc = 0x4440bc; continue;
            }
            0x4440bc => {
            c.set(5,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x1c6); wm(m,a,8,0x0u32); }
            pc = 0x4440c5; continue;
            }
            0x4440c5 => {
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x4440cf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x1c8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4440db; continue;
            }
            0x4440db => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x44418a; continue; }
            pc = 0x4440e6; continue;
            }
            0x4440e6 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1d9),8));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x1bc),32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x444112; continue; }
            pc = 0x44410d; continue;
            }
            0x44410d => {
            c.set(1,32,0x7fu32);
            pc = 0x444112; continue;
            }
            0x444112 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(7,32,0x1388u32);
            if c.cond(4) { pc = 0x444122; continue; }
            pc = 0x44411b; continue;
            }
            0x44411b => {
            c.set(0,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(7,32,c.get(0,32));
            pc = 0x444122; continue;
            }
            0x444122 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1cc),32));
            { let a = c.r[6].wrapping_add(0x1c6); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x44418a; continue; }
            pc = 0x444140; continue;
            }
            0x444140 => {
            c.push(m,c.get(5,32));
            { let target = rm(m,0x66a5a0u32,32); c.push(m, 0x444147); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x444173; continue; }
            pc = 0x44414d; continue;
            }
            0x44414d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a61cu32,32); c.push(m, 0x444159); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a620u32,32); c.push(m, 0x444163); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x1c6); wm(m,a,8,0x0u32); }
            pc = 0x44418a; continue;
            }
            0x444173 => {
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x44417e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x1cc); wm(m,a,32,c.get(1,32)); }
            pc = 0x44418a; continue;
            }
            0x44418a => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1c7),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x444324; continue; }
            pc = 0x444198; continue;
            }
            0x444198 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,0x0u32); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x444235; continue; }
            pc = 0x4441aa; continue;
            }
            0x4441aa => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1da),8));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x1b8),32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(7,32,c.get(2,32));
            { let a = c.get(7,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x4441d6; continue; }
            pc = 0x4441d1; continue;
            }
            0x4441d1 => {
            c.set(7,32,0x7fu32);
            pc = 0x4441d6; continue;
            }
            0x4441d6 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(1,32,0x1388u32);
            if c.cond(4) { pc = 0x4441e6; continue; }
            pc = 0x4441df; continue;
            }
            0x4441df => {
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(1,32,c.get(0,32));
            pc = 0x4441e6; continue;
            }
            0x4441e6 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1d0),32));
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x444235; continue; }
            pc = 0x444204; continue;
            }
            0x444204 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,0x66a5a0u32,32));
            { let target = c.get(3,32); c.push(m, 0x44420d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x44421c; continue; }
            pc = 0x444213; continue;
            }
            0x444213 => {
            c.set(5,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,0x0u32); }
            pc = 0x44421c; continue;
            }
            0x44421c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x444227); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x1d0); wm(m,a,32,c.get(2,32)); }
            pc = 0x44423b; continue;
            }
            0x444235 => {
            c.set(3,32,rm(m,0x66a5a0u32,32));
            pc = 0x44423b; continue;
            }
            0x44423b => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x444324; continue; }
            pc = 0x444246; continue;
            }
            0x444246 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1db),8));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x1b8),32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x444272; continue; }
            pc = 0x44426d; continue;
            }
            0x44426d => {
            c.set(1,32,0x7fu32);
            pc = 0x444272; continue;
            }
            0x444272 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(7,32,0x1388u32);
            if c.cond(4) { pc = 0x444282; continue; }
            pc = 0x44427b; continue;
            }
            0x44427b => {
            c.set(0,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(7,32,c.get(0,32));
            pc = 0x444282; continue;
            }
            0x444282 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1d4),32));
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x444324; continue; }
            pc = 0x4442a4; continue;
            }
            0x4442a4 => {
            c.push(m,c.get(5,32));
            { let target = c.get(3,32); c.push(m, 0x4442a7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1dc),8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4442e3; continue; }
            pc = 0x4442b3; continue;
            }
            0x4442b3 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1c4),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x4442e3; continue; }
            pc = 0x4442bf; continue;
            }
            0x4442bf => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,c.get(1,8));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,0x0u32); }
            c.set(7,32,c.get(2,32));
            { let target = c.get(3,32); c.push(m, 0x4442d2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x444324; continue; }
            pc = 0x4442d6; continue;
            }
            0x4442d6 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x4442e1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x444324; continue;
            }
            0x4442e3 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x44430d; continue; }
            pc = 0x4442e7; continue;
            }
            0x4442e7 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,0x0u32); }
            { let target = rm(m,0x66a61cu32,32); c.push(m, 0x4442fa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a620u32,32); c.push(m, 0x444304); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            pc = 0x444324; continue;
            }
            0x44430d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x444318); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x1d4); wm(m,a,32,c.get(2,32)); }
            pc = 0x444324; continue;
            }
            0x444324 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1de),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44435e; continue; }
            pc = 0x44432e; continue;
            }
            0x44432e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44435e; continue; }
            pc = 0x444335; continue;
            }
            0x444335 => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a63cu32,32); c.push(m, 0x44433c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x44435e; continue; }
            pc = 0x444341; continue;
            }
            0x444341 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a61cu32,32); c.push(m, 0x44434d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a620u32,32); c.push(m, 0x444357); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            pc = 0x44435e; continue;
            }
            0x44435e => {
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

pub fn f_004d3b40(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d3b40;
    loop {
        match pc {
            0x4d3b40 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x7),8));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x4),16));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[7]; wm(m,a,16,c.get(0,16)); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(0xc),16));
            { let a = c.r[7].wrapping_add(0x2); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.fld(c.ld(m,c.r[5].wrapping_add(0x10),4));
            { let v = c.ld(m,0x66bc80u32,4); c.fcomp(v); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(6,32,c.get(1,32));
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x4d3ba3; continue; }
            pc = 0x4d3b9d; continue;
            }
            0x4d3b9d => {
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,8,0x64u32); }
            pc = 0x4d3bd3; continue;
            }
            0x4d3ba3 => {
            c.fld(c.ld(m,c.r[5].wrapping_add(0x10),4));
            { let v = c.ld(m,0x66bc6cu32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x41u32,8); }
            if c.cond(10) { pc = 0x4d3bb8; continue; }
            pc = 0x4d3bb3; continue;
            }
            0x4d3bb3 => {
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,8,c.get(2,8)); }
            pc = 0x4d3bd3; continue;
            }
            0x4d3bb8 => {
            c.fld(c.ld(m,c.r[5].wrapping_add(0x10),4));
            { let v = c.ld(m,0x66c1e8u32,4); c.fop('*',v); }
            { let v = c.ld(m,0x66bc58u32,4); c.fop('+',v); }
            c.push(m, 0x4d3bcc);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,8,c.get(0,8)); }
            pc = 0x4d3bd3; continue;
            }
            0x4d3bd3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xc),16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[7].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x6),8));
            c.set(3,8,rm(m,c.r[2].wrapping_add(0x6729d0),8));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x7),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2c6),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x20),32),32); c.set(0,32,r); }
            c.set(3,32,c.r[0].wrapping_mul(8));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x6),8));
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x4),16));
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x672a00),8),8)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x55555556u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[7].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x6),8));
            { let a = c.get(0,32); let r = c.add(a,0xfffffffcu32,32); c.set(0,32,r); }
            c.set(2,32,0x10u32);
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(7) { pc = 0x4d422f; continue; }
            pc = 0x4d3c6b; continue;
            }
            0x4d3c6b => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4d43b8)); continue;
            }
            0x4d3c72 => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0x9u32); }
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(0,32,0x55555556u32);
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,8,0xcu32); }
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.r[7].wrapping_add(0x19); wm(m,a,8,0x11u32); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xe8),32));
            c.set(6,32,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            pc = 0x4d422f; continue;
            }
            0x4d3ccb => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0x2u32); }
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0x55555556u32);
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x7),8));
            c.set(6,32,c.get(2,32));
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2cc),32); c.sub(a,0x0u32,32); }
            if c.cond(6) { pc = 0x4d3d65; continue; }
            pc = 0x4d3d0d; continue;
            }
            0x4d3d0d => {
            c.set(2,32,sx(rm(m,0x672a15u32,8),8)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x4),16));
            { let a = c.r[7].wrapping_add(0x7); wm(m,a,8,0xau32); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x55555556u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x7),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2cc),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(1,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(1,32,r); }
            pc = 0x4d3d65; continue;
            }
            0x4d3d65 => {
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,8,0x4u32); }
            { let a = c.r[7].wrapping_add(0x19); wm(m,a,8,0x11u32); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xcc),32));
            c.set(5,32,c.r[0].wrapping_mul(8));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(5,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x4d3dce; continue;
            }
            0x4d3d92 => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0x3u32); }
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            { let a = c.r[7].wrapping_add(0x19); wm(m,a,8,0xbu32); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xd0),32));
            c.set(6,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(5,32),32); c.set(6,32,r); }
            c.set(5,32,c.r[0].wrapping_mul(8));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,c.get(5,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x4d3dce; continue;
            }
            0x4d3dce => {
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(5,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            pc = 0x4d422f; continue;
            }
            0x4d3de8 => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0xcu32); }
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[7].wrapping_add(0x19); wm(m,a,8,0xbu32); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0xf4),32));
            c.set(6,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(0,32,c.r[2].wrapping_mul(8));
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x4d422f; continue;
            }
            0x4d3e1f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0x5u32); }
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x2c8),32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            c.set(0,32,c.r[1].wrapping_add(c.r[1]));
            if c.cond(4) { pc = 0x4d3e40; continue; }
            pc = 0x4d3e39; continue;
            }
            0x4d3e39 => {
            c.set(0,32,c.r[1].wrapping_mul(4));
            pc = 0x4d3e40; continue;
            }
            0x4d3e40 => {
            { let a = c.get(6,32); c.and(a,0x2000u32,32); }
            if c.cond(4) { pc = 0x4d3e4c; continue; }
            pc = 0x4d3e48; continue;
            }
            0x4d3e48 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            pc = 0x4d3e57; continue;
            }
            0x4d3e4c => {
            { let a = c.get(6,32); c.and(a,0x4000u32,32); }
            if c.cond(4) { pc = 0x4d3e57; continue; }
            pc = 0x4d3e54; continue;
            }
            0x4d3e54 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            pc = 0x4d3e57; continue;
            }
            0x4d3e57 => {
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,0x55555556u32);
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            { let a = c.get(6,32); let n = 0xcu32; let r = c.sar(a,n,32); c.set(6,32,r); }
            pc = 0x4d422f; continue;
            }
            0x4d3e7c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0x0u32); }
            c.set(6,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x2c8),32));
            { let a = c.get(0,8); c.and(a,0x2u32,8); }
            if c.cond(5) { pc = 0x4d3e93; continue; }
            pc = 0x4d3e91; continue;
            }
            0x4d3e91 => {
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x4d3e93; continue;
            }
            0x4d3e93 => {
            { let a = c.get(4,8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x4d3eaa; continue; }
            pc = 0x4d3e98; continue;
            }
            0x4d3e98 => {
            c.set(0,32,0x55555556u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            pc = 0x4d3eb1; continue;
            }
            0x4d3eaa => {
            { let a = c.get(4,8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x4d3eb1; continue; }
            pc = 0x4d3eaf; continue;
            }
            0x4d3eaf => {
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x4d3eb1; continue;
            }
            0x4d3eb1 => {
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(6,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0xa3d70au32,32); }
            if c.cond(14) { pc = 0x4d3ee9; continue; }
            pc = 0x4d3ebf; continue;
            }
            0x4d3ebf => {
            { let a = c.get(5,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0x55555556u32);
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x2aaaaaabu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            pc = 0x4d4226; continue;
            }
            0x4d3ee9 => {
            c.set(5,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            pc = 0x4d420e; continue;
            }
            0x4d3ef9 => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0x1u32); }
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(0,32,0x55555556u32);
            { let a = c.r[7].wrapping_add(0x7); wm(m,a,8,0xdu32); }
            c.set(5,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,8,0x4u32); }
            { let a = c.r[7].wrapping_add(0x19); wm(m,a,8,0xbu32); }
            c.set(1,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,c.get(2,32));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xf8),32));
            c.set(3,32,rm(m,c.r[3].wrapping_add(0xc8),32));
            c.set(1,32,c.get(2,32));
            c.set(5,32,c.r[0].wrapping_mul(8));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,c.r[3].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(5,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(3,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(5,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            pc = 0x4d422f; continue;
            }
            0x4d3fa2 => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0xbu32); }
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x3fffffu32,32); }
            if c.cond(14) { pc = 0x4d3fcd; continue; }
            pc = 0x4d3fb4; continue;
            }
            0x4d3fb4 => {
            { let a = c.get(5,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(5,32,r); }
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0x55555556u32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            pc = 0x4d4226; continue;
            }
            0x4d3fcd => {
            c.set(5,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            pc = 0x4d4069; continue;
            }
            0x4d3fdd => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0x8u32); }
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,8,0xeu32); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xe4),32));
            c.set(6,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x4d422f; continue;
            }
            0x4d400d => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0x4u32); }
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,8,0xbu32); }
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0x66666667u32);
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xd4),32));
            c.set(6,32,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x4d422f; continue;
            }
            0x4d4055 => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,c.get(2,8)); }
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(5,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            pc = 0x4d4069; continue;
            }
            0x4d4069 => {
            c.set(0,32,0x55555556u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            pc = 0x4d422f; continue;
            }
            0x4d4081 => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0xeu32); }
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(0,32,0x66666667u32);
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            pc = 0x4d422f; continue;
            }
            0x4d40b2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0x7u32); }
            c.set(6,32,c.r[1].wrapping_add(c.r[1]));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x2c8),32));
            { let a = c.get(0,8); c.and(a,0x2u32,8); }
            if c.cond(5) { pc = 0x4d40ce; continue; }
            pc = 0x4d40c7; continue;
            }
            0x4d40c7 => {
            c.set(6,32,c.r[1].wrapping_mul(4));
            pc = 0x4d40ce; continue;
            }
            0x4d40ce => {
            { let a = c.get(4,8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x4d40d7; continue; }
            pc = 0x4d40d3; continue;
            }
            0x4d40d3 => {
            { let a = c.get(6,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            pc = 0x4d40de; continue;
            }
            0x4d40d7 => {
            { let a = c.get(4,8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x4d40de; continue; }
            pc = 0x4d40dc; continue;
            }
            0x4d40dc => {
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x4d40de; continue;
            }
            0x4d40de => {
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(6,32),32); c.set(5,32,r); }
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            { let a = c.get(6,32); let n = 0xcu32; let r = c.sar(a,n,32); c.set(6,32,r); }
            pc = 0x4d422f; continue;
            }
            0x4d4109 => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0x12u32); }
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(2,32,c.get(5,32));
            c.set(0,32,0x66666667u32);
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,8,0xbu32); }
            c.set(5,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(2)));
            { let (lo,hi) = c.imul1(c.r[0],c.get(5,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x10c),32));
            c.set(6,32,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x4d422f; continue;
            }
            0x4d414f => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0x13u32); }
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,8,0xbu32); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(6,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x110),32));
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x4d422f; continue;
            }
            0x4d4187 => {
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0xfu32); }
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,8,0xbu32); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(5,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x100),32));
            c.set(6,32,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(6,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x4d422f; continue;
            }
            0x4d41ca => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,8,0x11u32); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x2c8),32));
            { let a = c.get(0,8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x4d41e1; continue; }
            pc = 0x4d41dc; continue;
            }
            0x4d41dc => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            pc = 0x4d41fe; continue;
            }
            0x4d41e1 => {
            { let a = c.get(4,8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x4d41ef; continue; }
            pc = 0x4d41e6; continue;
            }
            0x4d41e6 => {
            c.set(0,32,c.r[1].wrapping_mul(4));
            pc = 0x4d41fe; continue;
            }
            0x4d41ef => {
            { let a = c.get(4,8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x4d41f9; continue; }
            pc = 0x4d41f4; continue;
            }
            0x4d41f4 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(8)));
            pc = 0x4d41fe; continue;
            }
            0x4d41f9 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4d41fe; continue;
            }
            0x4d41fe => {
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(5,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x4d420e; continue;
            }
            0x4d420e => {
            c.set(0,32,0x55555556u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x2aaaaaabu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(2,32,r); }
            pc = 0x4d4226; continue;
            }
            0x4d4226 => {
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            pc = 0x4d422f; continue;
            }
            0x4d422f => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x7),8));
            { let a = c.get(0,8); c.sub(a,0x18u32,8); }
            if c.cond(3) { pc = 0x4d4277; continue; }
            pc = 0x4d4236; continue;
            }
            0x4d4236 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x6),8));
            c.set(5,32,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc4),32));
            c.set(2,32,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc4),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            c.set(5,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            pc = 0x4d425d; continue;
            }
            0x4d425d => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x4d429e; continue; }
            pc = 0x4d4261; continue;
            }
            0x4d4261 => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.get(0,8); c.sub(a,0x64u32,8); }
            { let a = c.r[7].wrapping_add(0x5); wm(m,a,8,c.get(0,8)); }
            if c.cond(6) { pc = 0x4d42a2; continue; }
            pc = 0x4d4271; continue;
            }
            0x4d4271 => {
            { let a = c.r[7].wrapping_add(0x5); wm(m,a,8,0x64u32); }
            pc = 0x4d42a2; continue;
            }
            0x4d4277 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6),8));
            { let a = c.get(0,8); c.sub(a,0x18u32,8); }
            if c.cond(3) { pc = 0x4d4295; continue; }
            pc = 0x4d427e; continue;
            }
            0x4d427e => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc4),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            pc = 0x4d425d; continue;
            }
            0x4d4295 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(6,32,0x1u32);
            pc = 0x4d4261; continue;
            }
            0x4d429e => {
            { let a = c.r[7].wrapping_add(0x5); wm(m,a,8,0x0u32); }
            pc = 0x4d42a2; continue;
            }
            0x4d42a2 => {
            c.set(1,32,0xffffffe8u32);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(5,32,c.r[7].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,c.r[7].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            pc = 0x4d42bb; continue;
            }
            0x4d42bb => {
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,8); c.sub(a,0x18u32,8); }
            if c.cond(3) { pc = 0x4d432f; continue; }
            pc = 0x4d42c2; continue;
            }
            0x4d42c2 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x64),32),32); }
            if c.cond(13) { pc = 0x4d42e6; continue; }
            pc = 0x4d42d3; continue;
            }
            0x4d42d3 => {
            { let a = c.r[1].wrapping_add(c.r[7]).wrapping_add(0x1a); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x4d42eb; continue; }
            pc = 0x4d42e0; continue;
            }
            0x4d42e0 => {
            { let a = c.r[7].wrapping_add(0x5); wm(m,a,8,0x0u32); }
            pc = 0x4d42eb; continue;
            }
            0x4d42e6 => {
            { let a = c.r[1].wrapping_add(c.r[7]).wrapping_add(0x1a); wm(m,a,8,0x0u32); }
            pc = 0x4d42eb; continue;
            }
            0x4d42eb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5],8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[1],32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            if c.cond(4) { pc = 0x4d4321; continue; }
            pc = 0x4d4303; continue;
            }
            0x4d4303 => {
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            pc = 0x4d430f; continue;
            }
            0x4d430f => {
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4d4336; continue;
            }
            0x4d4321 => {
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            pc = 0x4d430f; continue;
            }
            0x4d432f => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            pc = 0x4d4336; continue;
            }
            0x4d4336 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xffffffee),8));
            { let a = c.get(0,8); c.sub(a,0x18u32,8); }
            if c.cond(3) { pc = 0x4d4388; continue; }
            pc = 0x4d433d; continue;
            }
            0x4d433d => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc4),32));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,c.r[0].wrapping_mul(8));
            if c.cond(4) { pc = 0x4d4371; continue; }
            pc = 0x4d435a; continue;
            }
            0x4d435a => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4d438e; continue;
            }
            0x4d4371 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4d438e; continue;
            }
            0x4d4388 => {
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            pc = 0x4d438e; continue;
            }
            0x4d438e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[5]));
            { let a = c.get(2,32); c.sub(a,0x2u32,32); }
            if c.cond(12) { pc = 0x4d42bb; continue; }
            pc = 0x4d43ad; continue;
            }
            0x4d43ad => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00505c90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x505c90;
    loop {
        match pc {
            0x505c90 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662ff8u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(0xf4),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            if c.cond(3) { pc = 0x505de9; continue; }
            pc = 0x505cfd; continue;
            }
            0x505cfd => {
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3f),8));
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(1,8); c.sub(a,0x2u32,8); }
            if c.cond(13) { pc = 0x505d26; continue; }
            pc = 0x505d16; continue;
            }
            0x505d16 => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[2].wrapping_add(0x13),8));
            { let a = c.get(2,8); let r = c.add(a,c.get(1,8),8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.r[0].wrapping_add(0x3f); wm(m,a,8,c.get(2,8)); }
            pc = 0x505d26; continue;
            }
            0x505d26 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x673660),8));
            { let a = rm(m,c.r[0].wrapping_add(0x40),16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x505d6d; continue; }
            pc = 0x505d43; continue;
            }
            0x505d43 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x505d6d; continue; }
            pc = 0x505d55; continue;
            }
            0x505d55 => {
            c.set(2,32,rm(m,0x672cc8u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x120),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x505d89; continue; }
            pc = 0x505d6d; continue;
            }
            0x505d6d => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x15d),8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x505d89; continue; }
            pc = 0x505d7e; continue;
            }
            0x505d7e => {
            c.set(3,32,0x1u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            pc = 0x505dbd; continue;
            }
            0x505d89 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x118),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x505d9e; continue; }
            pc = 0x505d93; continue;
            }
            0x505d93 => {
            c.set(3,32,0x1u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            pc = 0x505dbd; continue;
            }
            0x505d9e => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x10),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x11c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(3,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            pc = 0x505dbd; continue;
            }
            0x505dbd => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x136),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(9) { pc = 0x505dcc; continue; }
            pc = 0x505dc7; continue;
            }
            0x505dc7 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x505dcc; continue;
            }
            0x505dcc => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(0xf4),16));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x505cfd; continue; }
            pc = 0x505ddd; continue;
            }
            0x505ddd => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(7,32); c.sub(a,0xea60u32,32); }
            if c.cond(13) { pc = 0x505ded; continue; }
            pc = 0x505de9; continue;
            }
            0x505de9 => {
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            pc = 0x505df2; continue;
            }
            0x505ded => {
            c.set(7,32,0x7530u32);
            pc = 0x505df2; continue;
            }
            0x505df2 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(6,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(3,32); let r = c.or(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x505e65; continue; }
            pc = 0x505e07; continue;
            }
            0x505e07 => {
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x10),16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x136),8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x505e65; continue; }
            pc = 0x505e25; continue;
            }
            0x505e25 => {
            { let a = c.get(6,32); c.sub(a,0x7d0u32,32); }
            if c.cond(13) { pc = 0x505e65; continue; }
            pc = 0x505e2d; continue;
            }
            0x505e2d => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0xa),16));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xf4),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x505e65; continue; }
            pc = 0x505e40; continue;
            }
            0x505e40 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xf4),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x505e40; continue; }
            pc = 0x505e65; continue;
            }
            0x505e65 => {
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x10),16));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(7,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x1e),16));
            { let a = c.get(1,16); c.sub(a,0x3d2u32,16); }
            if c.cond(5) { pc = 0x505e9d; continue; }
            pc = 0x505e8b; continue;
            }
            0x505e8b => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x220u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x368u32); }
            pc = 0x505ead; continue;
            }
            0x505e9d => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x3d2u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x317u32); }
            pc = 0x505ead; continue;
            }
            0x505ead => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(0xf8),16));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x505f9a; continue; }
            pc = 0x505ec2; continue;
            }
            0x505ec2 => {
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); c.sub(a,0x3u32,32); }
            if c.cond(6) { pc = 0x505f9a; continue; }
            pc = 0x505ecd; continue;
            }
            0x505ecd => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x42),8));
            c.set(1,32,rm(m,0x6de4a4u32,32));
            { let a = c.get(2,32); let r = c.and(a,0x7u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xfu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(6,32,c.get(2,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[2].wrapping_add(0x2c8));
            pc = 0x505f02; continue;
            }
            0x505f02 => {
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); let r = c.and(a,0x40u32,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.sub(a,0x40u32,8); }
            if c.cond(5) { pc = 0x505f11; continue; }
            pc = 0x505f0c; continue;
            }
            0x505f0c => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x505f1f; continue; }
            pc = 0x505f10; continue;
            }
            0x505f10 => {
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            pc = 0x505f11; continue;
            }
            0x505f11 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x9f8u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            if c.cond(2) { pc = 0x505f02; continue; }
            pc = 0x505f1d; continue;
            }
            0x505f1d => {
            pc = 0x505f46; continue;
            }
            0x505f1f => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6de4b8),8));
            c.set(0,32,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x6ddbb0),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x6ddbb4),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x505f46; continue;
            }
            0x505f46 => {
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = rm(m,c.r[4].wrapping_add(0x18),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x506206; continue; }
            pc = 0x505f54; continue;
            }
            0x505f54 => {
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = rm(m,c.r[4].wrapping_add(0x14),32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x506206; continue; }
            pc = 0x505f62; continue;
            }
            0x505f62 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0x3fu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x505f70);
            { let args = []; c.r[0] = host.call(m, 0x5159c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x6ddbb0),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x6ddbb4),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x506206; continue;
            }
            0x505f9a => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x506048; continue; }
            pc = 0x505fa2; continue;
            }
            0x505fa2 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m, 0x505fb4);
            { let args = []; c.r[0] = host.call(m, 0x51a470, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x50603a; continue; }
            pc = 0x505fc1; continue;
            }
            0x505fc1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xe),8));
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6ddbb0),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x506029; continue; }
            pc = 0x505fe0; continue;
            }
            0x505fe0 => {
            c.set(3,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x506029; continue; }
            pc = 0x505ff1; continue;
            }
            0x505ff1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x10),16));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,0xffffu32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x506016);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x506022);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x514b80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,0x1u32);
            pc = 0x50626d; continue;
            }
            0x506029 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x50603a; continue;
            }
            0x50603a => {
            { let a = rm(m,c.r[7].wrapping_add(0x138),16); let r = c.add(a,0x700u32,16); { let a = c.r[7].wrapping_add(0x138); wm(m,a,16,r); } }
            pc = 0x506206; continue;
            }
            0x506048 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x52),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[5].wrapping_add(0xf4),16),16); }
            if c.cond(3) { pc = 0x5060df; continue; }
            pc = 0x506059; continue;
            }
            0x506059 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            c.set(6,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x5060df; continue; }
            pc = 0x506072; continue;
            }
            0x506072 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x1e),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x22),16));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[4].wrapping_add(0x4a); wm(m,a,16,c.get(1,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x48));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x506099);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x446d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.sub(a,0x28u32,32); }
            if c.cond(5) { pc = 0x5060df; continue; }
            pc = 0x50609e; continue;
            }
            0x50609e => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x22),16));
            c.set(6,16,rm(m,c.r[6].wrapping_add(0x1e),16));
            c.set(0,32,0x673568u32);
            pc = 0x5060ab; continue;
            }
            0x5060ab => {
            { let a = c.get(6,16); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffffc),16),16); }
            if c.cond(12) { pc = 0x5060c2; continue; }
            pc = 0x5060b1; continue;
            }
            0x5060b1 => {
            { let a = c.get(6,16); c.sub(a,rm(m,c.r[0],16),16); }
            if c.cond(15) { pc = 0x5060c2; continue; }
            pc = 0x5060b6; continue;
            }
            0x5060b6 => {
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffffe),16),16); }
            if c.cond(12) { pc = 0x5060c2; continue; }
            pc = 0x5060bc; continue;
            }
            0x5060bc => {
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x2),16),16); }
            if c.cond(14) { pc = 0x5060df; continue; }
            pc = 0x5060c2; continue;
            }
            0x5060c2 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x673580u32,32); }
            if c.cond(12) { pc = 0x5060ab; continue; }
            pc = 0x5060cc; continue;
            }
            0x5060cc => {
            c.set(2,32,sx(c.get(6,16),16)&0xffffffffu32);
            c.set(0,32,sx(c.get(1,16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x506206; continue;
            }
            0x5060df => {
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x5060e8);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,0x0u32); }
            c.push(m, 0x5060ff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x159),8));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x506130; continue; }
            pc = 0x50611d; continue;
            }
            0x50611d => {
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            pc = 0x50613e; continue;
            }
            0x506130 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x50613a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            pc = 0x50613e; continue;
            }
            0x50613e => {
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x20),32));
            c.set(3,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x5061f5; continue; }
            pc = 0x506152; continue;
            }
            0x506152 => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x50615c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,0x6cbdc8u32,32));
            c.set(5,32,rm(m,0x66e0d0u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x50617f; continue;
            }
            0x50617f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.set(1,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.imul(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,0x1f4u32,32); c.set(1,32,r); }
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(0,32,0x10624dd3u32);
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x44));
            c.set(1,32,sx(rm(m,c.r[4].wrapping_add(0x4a),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.imul(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x1f4u32,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(1,32,sx(c.get(7,16),16)&0xffffffffu32);
            c.set(0,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x44));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x5061df; continue; }
            pc = 0x5061dd; continue;
            }
            0x5061dd => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x5061df; continue;
            }
            0x5061df => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x1c),32),32); }
            if c.cond(5) { pc = 0x5061f1; continue; }
            pc = 0x5061e5; continue;
            }
            0x5061e5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x50617f; continue; }
            pc = 0x5061f1; continue;
            }
            0x5061f1 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x5061f5; continue;
            }
            0x5061f5 => {
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x506206);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x506206; continue;
            }
            0x506206 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,0xffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m, 0x506233);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5172a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x10),16));
            c.set(5,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x36),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x50626b; continue; }
            pc = 0x50624f; continue;
            }
            0x50624f => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(1,16); c.sub(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x50626b; continue; }
            pc = 0x506258; continue;
            }
            0x506258 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[5]).wrapping_add(0x18); wm(m,a,32,0x1u32); }
            pc = 0x50626b; continue;
            }
            0x50626b => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x50626d; continue;
            }
            0x50626d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050bc40(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50bc40;
    loop {
        match pc {
            0x50bc40 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x10),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x50bc6b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x10),16));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x50bc79);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xa),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x10),16));
            { let a = c.r[5].wrapping_add(0x12); wm(m,a,16,0x14u32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xf4),16),16); }
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x1),8));
            if c.cond(3) { pc = 0x50bcbf; continue; }
            pc = 0x50bc9f; continue;
            }
            0x50bc9f => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xf4),16),16); }
            if c.cond(2) { pc = 0x50bc9f; continue; }
            pc = 0x50bcbf; continue;
            }
            0x50bcbf => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x10),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x50));
            c.set(1,16,rm(m,c.r[0],16));
            { let a = c.r[0]; wm(m,a,16,0xffffu32); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.get(1,32));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x10),16));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0x52); wm(m,a,16,0xffffu32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(3,16,rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x8),16));
            c.set(2,8,rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x1),8));
            { let a = c.get(3,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xf6),16),16); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            if c.cond(3) { pc = 0x50be86; continue; }
            pc = 0x50bd23; continue;
            }
            0x50bd23 => {
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xf4),16));
            c.set(7,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x10),16));
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x50bd5f; continue; }
            pc = 0x50bd4b; continue;
            }
            0x50bd4b => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x50bd67; continue;
            }
            0x50bd5f => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            pc = 0x50bd67; continue;
            }
            0x50bd67 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xa),16));
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x50bdd9; continue; }
            pc = 0x50bd74; continue;
            }
            0x50bd74 => {
            pc = 0x50bd7a; continue;
            }
            0x50bd76 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x50bd7a; continue;
            }
            0x50bd7a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x20),8));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x6),16));
            { let a = c.r[1].wrapping_add(0x1); wm(m,a,8,c.get(2,8)); }
            c.set(2,16,rm(m,c.r[1],8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x50bdb2; continue; }
            pc = 0x50bda7; continue;
            }
            0x50bda7 => {
            { let a = c.r[1].wrapping_add(0x134); wm(m,a,16,0x14u32); }
            pc = 0x50bdbb; continue;
            }
            0x50bdb2 => {
            { let a = c.r[1].wrapping_add(0x134); wm(m,a,16,0x12u32); }
            pc = 0x50bdbb; continue;
            }
            0x50bdbb => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[1].wrapping_add(0x3c); wm(m,a,16,0xffffu32); }
            c.push(m,c.get(0,32));
            c.push(m, 0x50bdcb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5176c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x10),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xf4),16),16); }
            if c.cond(2) { pc = 0x50bd76; continue; }
            pc = 0x50bdd9; continue;
            }
            0x50bdd9 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xec),16));
            c.set(1,32,c.r[6].wrapping_add(0xec));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x50be17; continue; }
            pc = 0x50bdf1; continue;
            }
            0x50bdf1 => {
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xf6),16),16); }
            if c.cond(3) { pc = 0x50be5d; continue; }
            pc = 0x50bdfa; continue;
            }
            0x50bdfa => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),16));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x50bdf1; continue; }
            pc = 0x50be17; continue;
            }
            0x50be17 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(2,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(3,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),16));
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x10),16));
            { let a = c.r[3].wrapping_add(0x8); wm(m,a,16,0xffffu32); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xf4),16),16); }
            if c.cond(3) { pc = 0x50be5d; continue; }
            pc = 0x50be3e; continue;
            }
            0x50be3e => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x50be4b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x10),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x50be5d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x44f940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x50be5d; continue;
            }
            0x50be5d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = rm(m,c.r[0].wrapping_add(0x12),16); c.sub(a,0x12u32,16); }
            if c.cond(5) { pc = 0x50be7b; continue; }
            pc = 0x50be68; continue;
            }
            0x50be68 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x10),16));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x50be76);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x50bf6c; continue;
            }
            0x50be7b => {
            { let a = c.r[0].wrapping_add(0x12); wm(m,a,16,0x14u32); }
            pc = 0x50bf6c; continue;
            }
            0x50be86 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x50be8e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x505b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[5],8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x50beae; continue; }
            pc = 0x50be9c; continue;
            }
            0x50be9c => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x10),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x50beae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x44f940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x50beae; continue;
            }
            0x50beae => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffu32); }
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            c.set(1,16,rm(m,c.r[3],8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x50bed3; continue; }
            pc = 0x50bec8; continue;
            }
            0x50bec8 => {
            { let a = c.r[3].wrapping_add(0x134); wm(m,a,16,0x14u32); }
            pc = 0x50bee8; continue;
            }
            0x50bed3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            { let a = c.r[3].wrapping_add(0x134); wm(m,a,16,0x12u32); }
            c.push(m, 0x50bee8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x50bee8; continue;
            }
            0x50bee8 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[3].wrapping_add(0x3c); wm(m,a,16,0xffffu32); }
            c.push(m, 0x50beff);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5062f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,8,rm(m,c.r[3],8));
            c.set(5,32,c.r[0].wrapping_add(c.r[2]));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x18),32));
            { let a = c.get(5,8); let r = c.or(a,0x4u32,8); c.set(5,8,r); }
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0xe),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x1); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x12); wm(m,a,16,0x14u32); }
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x14); wm(m,a,16,0x12u32); }
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x16); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x37),8));
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x1c); wm(m,a,16,c.get(1,16)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x39),8));
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x1e); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x38),8));
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x1f); wm(m,a,8,c.get(1,8)); }
            pc = 0x50bf6c; continue;
            }
            0x50bf6c => {
            c.set(1,32,0x6e55d0u32);
            c.push(m, 0x50bf76);
            { let args = []; c.r[0] = host.call(m, 0x611cd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,16); let r = c.xor(a,c.get(3,16),16); c.set(3,16,r); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x10),16));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.set(3,8,c.get(0,8));
            c.push(m, 0x50bf89);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x10),16));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x50bf9b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[2]).wrapping_add(0x10),16));
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x10),16));
            c.push(m,0x0u32);
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6e55d0),32));
            c.push(m, 0x50bfc3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x605420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050e340(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50e340;
    loop {
        match pc {
            0x50e340 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x8),16));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x4),16));
            c.set(0,32,0x673568u32);
            pc = 0x50e34f; continue;
            }
            0x50e34f => {
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffffc),16),16); }
            if c.cond(12) { pc = 0x50e366; continue; }
            pc = 0x50e355; continue;
            }
            0x50e355 => {
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[0],16),16); }
            if c.cond(15) { pc = 0x50e366; continue; }
            pc = 0x50e35a; continue;
            }
            0x50e35a => {
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffffe),16),16); }
            if c.cond(12) { pc = 0x50e366; continue; }
            pc = 0x50e360; continue;
            }
            0x50e360 => {
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x2),16),16); }
            if c.cond(14) { pc = 0x50e375; continue; }
            pc = 0x50e366; continue;
            }
            0x50e366 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x673580u32,32); }
            if c.cond(12) { pc = 0x50e34f; continue; }
            pc = 0x50e370; continue;
            }
            0x50e370 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x50e375 => {
            c.set(0,8,0x1u32);
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005250d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5250d0;
    loop {
        match pc {
            0x5250d0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(3,8,rm(m,c.r[5].wrapping_add(0x789),8));
            { let a = c.get(3,8); c.sub(a,0x1u32,8); }
            if c.cond(3) { pc = 0x5250e8; continue; }
            pc = 0x5250e1; continue;
            }
            0x5250e1 => {
            c.set(3,32,0x1u32);
            pc = 0x5250ef; continue;
            }
            0x5250e8 => {
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            pc = 0x5250ef; continue;
            }
            0x5250ef => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x800),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x801),8));
            { let a = c.get(3,8); c.and(a,0x4u32,8); }
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            if c.cond(5) { pc = 0x52512e; continue; }
            pc = 0x52510c; continue;
            }
            0x52510c => {
            { let a = c.get(3,8); c.and(a,0x2u32,8); }
            if c.cond(5) { pc = 0x52511c; continue; }
            pc = 0x525111; continue;
            }
            0x525111 => {
            c.set(1,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x525143; continue;
            }
            0x52511c => {
            c.set(2,32,c.get(3,32));
            c.set(0,32,0x1u32);
            { let a = c.get(2,32); let r = c.and(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x52514a; continue;
            }
            0x52512e => {
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x1u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.and(a,0x2u32,32); c.set(1,32,r); }
            pc = 0x525143; continue;
            }
            0x525143 => {
            c.set(2,32,0x1u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            pc = 0x52514a; continue;
            }
            0x52514a => {
            { let a = c.get(3,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            c.set(6,32,c.r[3].wrapping_mul(8).wrapping_add(0x19));
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(2,32,c.get(3,32));
            c.set(6,32,0x2u32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x7a8),32));
            if c.cond(14) { pc = 0x525189; continue; }
            pc = 0x525177; continue;
            }
            0x525177 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,0x4u32,32); c.set(6,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.get(2,8); let r = c.add(a,0x2u32,8); c.set(2,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            pc = 0x5251ad; continue;
            }
            0x525189 => {
            c.set(2,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x2));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x5251a9; continue; }
            pc = 0x525191; continue;
            }
            0x525191 => {
            c.set(2,8,0x3u32);
            { let a = c.get(6,32); let r = c.add(a,0xfffffffcu32,32); c.set(6,32,r); }
            { let (lo,hi) = c.imul1(c.get(0,8),c.get(2,8),8); c.set(0,16,lo | (hi<<8)); }
            { let a = c.get(0,8); let r = c.add(a,c.get(3,8),8); c.set(0,8,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.get(2,8); let r = c.add(a,0x2u32,8); c.set(2,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            pc = 0x5251ad; continue;
            }
            0x5251a9 => {
            { let a = c.get(3,8); let r = c.add(a,c.get(0,8),8); c.set(3,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(3,8)); }
            pc = 0x5251ad; continue;
            }
            0x5251ad => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(6,32,0x2u32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(2,8,rm(m,c.r[3],8));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x7ac),32));
            if c.cond(14) { pc = 0x5251dd; continue; }
            pc = 0x5251cb; continue;
            }
            0x5251cb => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.sub(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.get(2,8); let r = c.add(a,0x2u32,8); c.set(2,8,r); }
            pc = 0x5251ff; continue;
            }
            0x5251dd => {
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x2));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x5251fd; continue; }
            pc = 0x5251e5; continue;
            }
            0x5251e5 => {
            c.set(0,8,c.get(1,8));
            c.set(1,8,0x3u32);
            { let (lo,hi) = c.imul1(c.get(0,8),c.get(1,8),8); c.set(0,16,lo | (hi<<8)); }
            { let a = c.get(0,8); let r = c.add(a,c.get(2,8),8); c.set(0,8,r); }
            { let a = c.get(6,32); let r = c.add(a,0xfffffffcu32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.get(2,8); let r = c.add(a,0x2u32,8); c.set(2,8,r); }
            pc = 0x5251ff; continue;
            }
            0x5251fd => {
            { let a = c.get(2,8); let r = c.add(a,c.get(1,8),8); c.set(2,8,r); }
            pc = 0x5251ff; continue;
            }
            0x5251ff => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[5].wrapping_add(0x7a4));
            { let a = c.r[3]; wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x52520e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fa7f0, c.r[1], &args); }
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

pub fn f_005854d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5854d0;
    loop {
        match pc {
            0x5854d0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x6),16));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,c.get(0,32));
            c.set(0,32,sx(rm(m,c.r[1].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(6,32,sx(rm(m,c.r[2].wrapping_add(0x2),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(8)));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,sx(rm(m,c.r[1].wrapping_add(0x10),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[2],16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x58553c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0059a420(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x59a420;
    loop {
        match pc {
            0x59a420 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc668),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x59a490; continue; }
            pc = 0x59a440; continue;
            }
            0x59a440 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x59a44a; continue;
            }
            0x59a44a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc59c),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x59a47f; continue; }
            pc = 0x59a456; continue;
            }
            0x59a456 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc680),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),32));
            c.set(0,32,c.get(5,32));
            c.set(1,32,rm(m,c.r[2],32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.sub(a,rm(m,c.r[2].wrapping_add(0x4),32),32); c.set(1,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1685au32,32); }
            if c.cond(14) { pc = 0x59a49c; continue; }
            pc = 0x59a47f; continue;
            }
            0x59a47f => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),8); let r = c.inc(a,8); { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,r); } }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x59a44a; continue; }
            pc = 0x59a490; continue;
            }
            0x59a490 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x59a49c => {
            c.push(m,0x18u32);
            c.push(m, 0x59a4a3);
            { let args = []; c.r[0] = host.call(m, 0x557d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(6,32,c.get(0,32));
            c.set(3,32,c.r[2].wrapping_add(0x3f8));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x59a4bb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x568a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x59a4c8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x568b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let a = c.get(7,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m,0x20u32);
            c.push(m,c.get(7,32));
            { let a = c.get(5,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x59a4e0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00612930(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x612930;
    loop {
        match pc {
            0x612930 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x612a0d; continue; }
            pc = 0x612955; continue;
            }
            0x612955 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x612a0d; continue; }
            pc = 0x612960; continue;
            }
            0x612960 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x118),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,0x1000u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xe),8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_mul(2).wrapping_add(0x67adc0),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0xcu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(3,32); c.div32(b); }
            { let a = c.get(0,32); let r = c.add(a,0x71u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xa6u32,32); }
            if c.cond(3) { pc = 0x6129b8; continue; }
            pc = 0x6129b1; continue;
            }
            0x6129b1 => {
            c.set(0,32,0xa6u32);
            pc = 0x6129c4; continue;
            }
            0x6129b8 => {
            { let a = c.get(0,32); c.sub(a,0x100u32,32); }
            if c.cond(6) { pc = 0x6129c4; continue; }
            pc = 0x6129bf; continue;
            }
            0x6129bf => {
            c.set(0,32,0x100u32);
            pc = 0x6129c4; continue;
            }
            0x6129c4 => {
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(6,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(0,32); let n = 0xau32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(6,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x612a12; continue; }
            pc = 0x6129d7; continue;
            }
            0x6129d7 => {
            c.set(2,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[2]).wrapping_add(0x9),8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(1,32); let r = c.add(a,0x9f6u32,32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x9b34ce69u32);
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0xbu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x612a0d => {
            c.set(0,32,0x1u32);
            pc = 0x612a12; continue;
            }
            0x612a12 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00638010(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x638010;
    loop {
        match pc {
            0x638010 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x198),32));
            c.set(3,32,c.r[6].wrapping_add(0x198));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x638028); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xb0),32));
            c.set(7,32,c.r[6].wrapping_add(0xb0));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x63803b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x288),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x63806a; continue; }
            pc = 0x63804c; continue;
            }
            0x63804c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2a8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2a4),32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(12)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x638066); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63806a => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x638077; continue; }
            pc = 0x63806e; continue;
            }
            0x63806e => {
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x638077); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x638077; continue;
            }
            0x638077 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f89e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f89e0;
    loop {
        match pc {
            0x4f89e0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2d4),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(7,32); c.sub(a,0x1u32,32); }
            if c.cond(12) { pc = 0x4f8a49; continue; }
            pc = 0x4f8a09; continue;
            }
            0x4f8a09 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x789),8));
            c.set(0,32,c.get(2,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            if c.cond(13) { pc = 0x4f8a49; continue; }
            pc = 0x4f8a2a; continue;
            }
            0x4f8a2a => {
            c.fld(c.ld(m,c.r[1].wrapping_add(0x464),4));
            { let v = c.ld(m,0x66fd5cu32,4); c.fop('*',v); }
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let v = c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x11c),4); c.fop('+',v); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x11c));
            c.fstp32(m,c.r[0]);
            pc = 0x4f8a49; continue;
            }
            0x4f8a49 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004fa7f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4fa7f0;
    loop {
        match pc {
            0x4fa7f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(0,8,rm(m,c.r[2],8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2],8));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(6,32,0x1u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,0x84u32);
            pc = 0x4fa828; continue;
            }
            0x4fa828 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            c.set(2,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.and(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            c.set(5,32,c.r[3].wrapping_add(c.r[6]));
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4fa86b; continue; }
            pc = 0x4fa85d; continue;
            }
            0x4fa85d => {
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,0x0u32); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x4fa874; continue;
            }
            0x4fa86b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,0x1u32); }
            pc = 0x4fa874; continue;
            }
            0x4fa874 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x4fa88b; continue; }
            pc = 0x4fa878; continue;
            }
            0x4fa878 => {
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            pc = 0x4fa890; continue;
            }
            0x4fa88b => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            pc = 0x4fa890; continue;
            }
            0x4fa890 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x4fa8a9; continue; }
            pc = 0x4fa897; continue;
            }
            0x4fa897 => {
            c.set(5,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            pc = 0x4fa8b2; continue;
            }
            0x4fa8a9 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x4fa8b2; continue;
            }
            0x4fa8b2 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4fa8d8; continue; }
            pc = 0x4fa8b6; continue;
            }
            0x4fa8b6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x4fa8dd; continue;
            }
            0x4fa8d8 => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x1u32); }
            pc = 0x4fa8dd; continue;
            }
            0x4fa8dd => {
            c.set(0,32,c.get(7,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4fa98c; continue; }
            pc = 0x4fa8f8; continue;
            }
            0x4fa8f8 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x4fa99d; continue; }
            pc = 0x4fa900; continue;
            }
            0x4fa900 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x68),32));
            pc = 0x4fa907; continue;
            }
            0x4fa907 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[0]),8));
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fa977; continue; }
            pc = 0x4fa90f; continue;
            }
            0x4fa90f => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x1),8));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4fa940; continue; }
            pc = 0x4fa917; continue;
            }
            0x4fa917 => {
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fa928; continue; }
            pc = 0x4fa91b; continue;
            }
            0x4fa91b => {
            c.set(2,32,c.r[6].wrapping_add(c.r[7]));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fa928; continue; }
            pc = 0x4fa924; continue;
            }
            0x4fa924 => {
            c.set(3,8,0x1u32);
            pc = 0x4fa92a; continue;
            }
            0x4fa928 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4fa92a; continue;
            }
            0x4fa92a => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fa96b; continue; }
            pc = 0x4fa930; continue;
            }
            0x4fa930 => {
            c.set(2,32,c.r[6].wrapping_add(c.r[7]));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fa96b; continue; }
            pc = 0x4fa939; continue;
            }
            0x4fa939 => {
            c.set(2,32,0x1u32);
            pc = 0x4fa96d; continue;
            }
            0x4fa940 => {
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fa952; continue; }
            pc = 0x4fa944; continue;
            }
            0x4fa944 => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fa952; continue; }
            pc = 0x4fa94e; continue;
            }
            0x4fa94e => {
            c.set(3,8,0x1u32);
            pc = 0x4fa954; continue;
            }
            0x4fa952 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4fa954; continue;
            }
            0x4fa954 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fa96b; continue; }
            pc = 0x4fa95a; continue;
            }
            0x4fa95a => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fa96b; continue; }
            pc = 0x4fa964; continue;
            }
            0x4fa964 => {
            c.set(2,32,0x1u32);
            pc = 0x4fa96d; continue;
            }
            0x4fa96b => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x4fa96d; continue;
            }
            0x4fa96d => {
            { let a = c.get(3,8); let r = c.or(a,c.get(2,8),8); c.set(3,8,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4fac45; continue; }
            pc = 0x4fa977; continue;
            }
            0x4fa977 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            if c.cond(15) { pc = 0x4fa907; continue; }
            pc = 0x4fa986; continue;
            }
            0x4fa986 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x4fa99d; continue;
            }
            0x4fa98c => {
            { let a = c.get(3,32); let r = c.add(a,c.get(5,32),32); c.set(3,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            pc = 0x4fa99d; continue;
            }
            0x4fa99d => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x11),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4faa4c; continue; }
            pc = 0x4fa9a9; continue;
            }
            0x4fa9a9 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x4faa62; continue; }
            pc = 0x4fa9b5; continue;
            }
            0x4fa9b5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x68),32));
            pc = 0x4fa9bc; continue;
            }
            0x4fa9bc => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[0]),8));
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faa2c; continue; }
            pc = 0x4fa9c4; continue;
            }
            0x4fa9c4 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x1),8));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4fa9f5; continue; }
            pc = 0x4fa9cc; continue;
            }
            0x4fa9cc => {
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fa9dd; continue; }
            pc = 0x4fa9d0; continue;
            }
            0x4fa9d0 => {
            c.set(2,32,c.r[6].wrapping_add(c.r[7]));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fa9dd; continue; }
            pc = 0x4fa9d9; continue;
            }
            0x4fa9d9 => {
            c.set(3,8,0x1u32);
            pc = 0x4fa9df; continue;
            }
            0x4fa9dd => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4fa9df; continue;
            }
            0x4fa9df => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faa20; continue; }
            pc = 0x4fa9e5; continue;
            }
            0x4fa9e5 => {
            c.set(2,32,c.r[6].wrapping_add(c.r[7]));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faa20; continue; }
            pc = 0x4fa9ee; continue;
            }
            0x4fa9ee => {
            c.set(2,32,0x1u32);
            pc = 0x4faa22; continue;
            }
            0x4fa9f5 => {
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faa07; continue; }
            pc = 0x4fa9f9; continue;
            }
            0x4fa9f9 => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faa07; continue; }
            pc = 0x4faa03; continue;
            }
            0x4faa03 => {
            c.set(3,8,0x1u32);
            pc = 0x4faa09; continue;
            }
            0x4faa07 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4faa09; continue;
            }
            0x4faa09 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faa20; continue; }
            pc = 0x4faa0f; continue;
            }
            0x4faa0f => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faa20; continue; }
            pc = 0x4faa19; continue;
            }
            0x4faa19 => {
            c.set(2,32,0x1u32);
            pc = 0x4faa22; continue;
            }
            0x4faa20 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x4faa22; continue;
            }
            0x4faa22 => {
            { let a = c.get(3,8); let r = c.or(a,c.get(2,8),8); c.set(3,8,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4fac09; continue; }
            pc = 0x4faa2c; continue;
            }
            0x4faa2c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(15) { pc = 0x4fa9bc; continue; }
            pc = 0x4faa42; continue;
            }
            0x4faa42 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4faa62; continue;
            }
            0x4faa4c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            pc = 0x4faa62; continue;
            }
            0x4faa62 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4fab0d; continue; }
            pc = 0x4faa72; continue;
            }
            0x4faa72 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x4fab1e; continue; }
            pc = 0x4faa7a; continue;
            }
            0x4faa7a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x68),32));
            pc = 0x4faa81; continue;
            }
            0x4faa81 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[0]),8));
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faaf4; continue; }
            pc = 0x4faa89; continue;
            }
            0x4faa89 => {
            { let a = rm(m,c.r[4].wrapping_add(0x14),8); c.and(a,0x1u32,8); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x1),8));
            if c.cond(4) { pc = 0x4faabd; continue; }
            pc = 0x4faa94; continue;
            }
            0x4faa94 => {
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faaa5; continue; }
            pc = 0x4faa98; continue;
            }
            0x4faa98 => {
            c.set(2,32,c.r[6].wrapping_add(c.r[7]));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faaa5; continue; }
            pc = 0x4faaa1; continue;
            }
            0x4faaa1 => {
            c.set(3,8,0x1u32);
            pc = 0x4faaa7; continue;
            }
            0x4faaa5 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4faaa7; continue;
            }
            0x4faaa7 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faae8; continue; }
            pc = 0x4faaad; continue;
            }
            0x4faaad => {
            c.set(2,32,c.r[6].wrapping_add(c.r[7]));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faae8; continue; }
            pc = 0x4faab6; continue;
            }
            0x4faab6 => {
            c.set(2,32,0x1u32);
            pc = 0x4faaea; continue;
            }
            0x4faabd => {
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faacf; continue; }
            pc = 0x4faac1; continue;
            }
            0x4faac1 => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faacf; continue; }
            pc = 0x4faacb; continue;
            }
            0x4faacb => {
            c.set(3,8,0x1u32);
            pc = 0x4faad1; continue;
            }
            0x4faacf => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4faad1; continue;
            }
            0x4faad1 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faae8; continue; }
            pc = 0x4faad7; continue;
            }
            0x4faad7 => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faae8; continue; }
            pc = 0x4faae1; continue;
            }
            0x4faae1 => {
            c.set(2,32,0x1u32);
            pc = 0x4faaea; continue;
            }
            0x4faae8 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x4faaea; continue;
            }
            0x4faaea => {
            { let a = c.get(3,8); let r = c.or(a,c.get(2,8),8); c.set(3,8,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4fac27; continue; }
            pc = 0x4faaf4; continue;
            }
            0x4faaf4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            if c.cond(15) { pc = 0x4faa81; continue; }
            pc = 0x4fab07; continue;
            }
            0x4fab07 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x4fab1e; continue;
            }
            0x4fab0d => {
            { let a = c.get(3,32); let r = c.sub(a,c.get(5,32),32); c.set(3,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            pc = 0x4fab1e; continue;
            }
            0x4fab1e => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4fabca; continue; }
            pc = 0x4fab2a; continue;
            }
            0x4fab2a => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x4fabca; continue; }
            pc = 0x4fab36; continue;
            }
            0x4fab36 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x68),32));
            pc = 0x4fab3d; continue;
            }
            0x4fab3d => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[0]),8));
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fabb0; continue; }
            pc = 0x4fab45; continue;
            }
            0x4fab45 => {
            { let a = rm(m,c.r[4].wrapping_add(0x14),8); c.and(a,0x1u32,8); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x1),8));
            if c.cond(4) { pc = 0x4fab79; continue; }
            pc = 0x4fab50; continue;
            }
            0x4fab50 => {
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fab61; continue; }
            pc = 0x4fab54; continue;
            }
            0x4fab54 => {
            c.set(2,32,c.r[6].wrapping_add(c.r[7]));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fab61; continue; }
            pc = 0x4fab5d; continue;
            }
            0x4fab5d => {
            c.set(3,8,0x1u32);
            pc = 0x4fab63; continue;
            }
            0x4fab61 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4fab63; continue;
            }
            0x4fab63 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faba4; continue; }
            pc = 0x4fab69; continue;
            }
            0x4fab69 => {
            c.set(2,32,c.r[6].wrapping_add(c.r[7]));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faba4; continue; }
            pc = 0x4fab72; continue;
            }
            0x4fab72 => {
            c.set(2,32,0x1u32);
            pc = 0x4faba6; continue;
            }
            0x4fab79 => {
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fab8b; continue; }
            pc = 0x4fab7d; continue;
            }
            0x4fab7d => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4fab8b; continue; }
            pc = 0x4fab87; continue;
            }
            0x4fab87 => {
            c.set(3,8,0x1u32);
            pc = 0x4fab8d; continue;
            }
            0x4fab8b => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x4fab8d; continue;
            }
            0x4fab8d => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faba4; continue; }
            pc = 0x4fab93; continue;
            }
            0x4fab93 => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4faba4; continue; }
            pc = 0x4fab9d; continue;
            }
            0x4fab9d => {
            c.set(2,32,0x1u32);
            pc = 0x4faba6; continue;
            }
            0x4faba4 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x4faba6; continue;
            }
            0x4faba6 => {
            { let a = c.get(3,8); let r = c.or(a,c.get(2,8),8); c.set(3,8,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4fac45; continue; }
            pc = 0x4fabb0; continue;
            }
            0x4fabb0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(15) { pc = 0x4fab3d; continue; }
            pc = 0x4fabc6; continue;
            }
            0x4fabc6 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x4fabca; continue;
            }
            0x4fabca => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(6,32)); }
            if c.cond(5) { pc = 0x4fa828; continue; }
            pc = 0x4fabdb; continue;
            }
            0x4fabdb => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x11),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4fa828; continue; }
            pc = 0x4fabe7; continue;
            }
            0x4fabe7 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4fa828; continue; }
            pc = 0x4fabf3; continue;
            }
            0x4fabf3 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4fa828; continue; }
            pc = 0x4fabff; continue;
            }
            0x4fabff => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4fac09 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x18),8));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4fac27 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x18),8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4fac45 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x18),8));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00500a70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x500a70;
    loop {
        match pc {
            0x500a70 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x500b7a; continue; }
            pc = 0x500a92; continue;
            }
            0x500a92 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2c4),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x500aa6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x500ac9; continue; }
            pc = 0x500ab6; continue;
            }
            0x500ab6 => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.and(a,0x42u32,8); c.set(0,8,r); }
            { let a = c.get(0,32); let r = c.add(a,0xc8u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x500ad1; continue;
            }
            0x500ac9 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x7d0u32); }
            pc = 0x500ad1; continue;
            }
            0x500ad1 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x500b7a; continue; }
            pc = 0x500ae5; continue;
            }
            0x500ae5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x2c6),8));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x39),8));
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(0,8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x500b62; continue; }
            pc = 0x500b0a; continue;
            }
            0x500b0a => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x500b62; continue; }
            pc = 0x500b14; continue;
            }
            0x500b14 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x500b62; continue; }
            pc = 0x500b1c; continue;
            }
            0x500b1c => {
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x500b32; continue; }
            pc = 0x500b23; continue;
            }
            0x500b23 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,0x0u32); }
            pc = 0x500b52; continue;
            }
            0x500b32 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5]; wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            pc = 0x500b52; continue;
            }
            0x500b52 => {
            c.push(m,0x4u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x500b5b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x500b62);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x500b62; continue;
            }
            0x500b62 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x500ae5; continue; }
            pc = 0x500b76; continue;
            }
            0x500b76 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x500b7a; continue;
            }
            0x500b7a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x500b9a; continue; }
            pc = 0x500b85; continue;
            }
            0x500b85 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[5]; wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x500b9a => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,0x0u32); }
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

pub fn f_00569c20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x569c20;
    loop {
        match pc {
            0x569c20 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            if c.cond(4) { pc = 0x569c36; continue; }
            pc = 0x569c31; continue;
            }
            0x569c31 => {
            { let a = c.r[6].wrapping_add(0x2d); wm(m,a,8,c.get(0,8)); }
            pc = 0x569c48; continue;
            }
            0x569c36 => {
            c.set(7,32,rm(m,0x6e1844u32,32));
            c.push(m, 0x569c41);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            { let a = c.r[6].wrapping_add(0x2d); wm(m,a,8,c.get(2,8)); }
            pc = 0x569c48; continue;
            }
            0x569c48 => {
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,0x0u32); }
            c.set(7,32,rm(m,0x6e51acu32,32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.set(5,32,0x7fffffffu32);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc668),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x569cc2; continue; }
            pc = 0x569c63; continue;
            }
            0x569c63 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x569c65; continue;
            }
            0x569c65 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc59c),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[1]),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x569ca9; continue; }
            pc = 0x569c71; continue;
            }
            0x569c71 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xc680),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[2].wrapping_add(0x4),32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(13) { pc = 0x569ca9; continue; }
            pc = 0x569c9e; continue;
            }
            0x569c9e => {
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,c.get(3,8)); }
            c.set(7,32,rm(m,0x6e51acu32,32));
            c.set(5,32,c.get(0,32));
            pc = 0x569ca9; continue;
            }
            0x569ca9 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc668),32));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x569c65; continue; }
            pc = 0x569cc2; continue;
            }
            0x569cc2 => {
            c.set(1,32,rm(m,0x6e1844u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x569cd5; continue; }
            pc = 0x569cd1; continue;
            }
            0x569cd1 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x569cde; continue;
            }
            0x569cd5 => {
            c.set(1,32,rm(m,0x6e1848u32,32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x569cde; continue;
            }
            0x569cde => {
            c.push(m,0x2u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x569ce9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            c.push(m, 0x569cfb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x559cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x40),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x41),8));
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc680),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc680),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x39); wm(m,a,8,0x1u32); }
            c.push(m, 0x569d4c);
            { let args = []; c.r[0] = host.call(m, 0x56a240, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3a),8));
            c.set(5,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(5,32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x569d66);
            { let args = []; c.r[0] = host.call(m, 0x559260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(3,8,c.get(0,8));
            c.push(m,c.get(2,32));
            c.push(m, 0x569d7b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x569d8f; continue; }
            pc = 0x569d7f; continue;
            }
            0x569d7f => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m, 0x569d8f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x569d8f; continue;
            }
            0x569d8f => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x569d96);
            { let args = []; c.r[0] = host.call(m, 0x559810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6e51acu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x41),8));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xc59c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
