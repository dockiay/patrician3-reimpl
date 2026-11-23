//! Generado por `reimpl/tools/ui_rt_names.py` con `tools/lift.py` (no editar a mano): carga del banco de nombres.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static NAMES: [(u32, super::core_table::CoreFn); 7] = [
    (0x5121c0, f_005121c0),
    (0x5125e0, f_005125e0),
    (0x5128d0, f_005128d0),
    (0x51e440, f_0051e440),
    (0x512730, f_00512730),
    (0x4f6420, f_004f6420),
    (0x4f6450, f_004f6450),
];
pub fn f_005121c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5121c0;
    loop {
        match pc {
            0x5121c0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[6],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x5121f0; continue; }
            pc = 0x5121cb; continue;
            }
            0x5121cb => {
            c.push(m,0x6c051cu32);
            c.push(m, 0x5121d5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5125e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(5,32,0x6ddb90u32);
            c.set(3,32,0x28u32);
            pc = 0x5121e1; continue;
            }
            0x5121e1 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5121e9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51e440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x34u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5121e1; continue; }
            pc = 0x5121f0; continue;
            }
            0x5121f0 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5121f7);
            { let args = []; c.r[0] = host.call(m, 0x512730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            c.set(7,32,c.r[6].wrapping_add(0xa8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5122e7; continue; }
            pc = 0x51220b; continue;
            }
            0x51220b => {
            c.push(m,0x6c0504u32);
            c.push(m,0x50u32);
            c.push(m,0x6c053cu32);
            c.push(m,0x6c04f4u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x512226);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0xd0); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0]).wrapping_add(0x2a));
            c.push(m,c.get(0,32));
            c.push(m, 0x51223c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xbc); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd0),16));
            c.set(2,32,c.r[1].wrapping_add(c.r[1]).wrapping_add(0x28));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x512258);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbc),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd0),16));
            c.push(m,c.get(7,32));
            c.push(m,0x6c054cu32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x512279);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5128d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,0x1u32);
            pc = 0x51227e; continue;
            }
            0x51227e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xbc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd0),16));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.set(5,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(5,16); let r = c.add(a,c.get(3,16),16); c.set(5,16,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x15u32,32); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(5,16)); }
            if c.cond(2) { pc = 0x51227e; continue; }
            pc = 0x5122a6; continue;
            }
            0x5122a6 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xbc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd0),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.set(3,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x28),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x5122e7; continue; }
            pc = 0x5122c6; continue;
            }
            0x5122c6 => {
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,8,0x0u32); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd0),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xbc),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(3,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x28),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x5122c6; continue; }
            pc = 0x5122e7; continue;
            }
            0x5122e7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.set(7,32,c.r[6].wrapping_add(0xac));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5123d7; continue; }
            pc = 0x5122fb; continue;
            }
            0x5122fb => {
            c.push(m,0x6c0504u32);
            c.push(m,0x50u32);
            c.push(m,0x6c0568u32);
            c.push(m,0x6c04f4u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x512316);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0xd2); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0]).wrapping_add(0x2a));
            c.push(m,c.get(0,32));
            c.push(m, 0x51232c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd2),16));
            c.set(2,32,c.r[1].wrapping_add(c.r[1]).wrapping_add(0x2a));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x512348);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd2),16));
            c.push(m,c.get(7,32));
            c.push(m,0x6c0574u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x512369);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5128d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,0x1u32);
            pc = 0x51236e; continue;
            }
            0x51236e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd2),16));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.set(5,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(5,16); let r = c.add(a,c.get(3,16),16); c.set(5,16,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x15u32,32); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(5,16)); }
            if c.cond(2) { pc = 0x51236e; continue; }
            pc = 0x512396; continue;
            }
            0x512396 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd2),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.set(3,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x28),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x5123d7; continue; }
            pc = 0x5123b6; continue;
            }
            0x5123b6 => {
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,8,0x0u32); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd2),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(3,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x28),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x5123b6; continue; }
            pc = 0x5123d7; continue;
            }
            0x5123d7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb0),32));
            c.set(7,32,c.r[6].wrapping_add(0xb0));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5124f0; continue; }
            pc = 0x5123eb; continue;
            }
            0x5123eb => {
            c.push(m,0x6c0504u32);
            c.push(m,0x50u32);
            c.push(m,0x6c058cu32);
            c.push(m,0x6c04f4u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x512406);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x6c0504u32);
            c.push(m,0x6u32);
            c.push(m,0x6c059cu32);
            c.push(m,0x6c04f4u32);
            c.set(1,32,0x6dd524u32);
            { let a = c.r[6].wrapping_add(0xd4); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x512428);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0xd4),16),16); c.set(0,16,r); }
            { let a = c.r[6].wrapping_add(0xd6); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0]).wrapping_add(0x2a));
            c.push(m,c.get(0,32));
            c.push(m, 0x512445);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd6),16));
            c.set(2,32,c.r[1].wrapping_add(c.r[1]).wrapping_add(0x2a));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x512461);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd6),16));
            c.push(m,c.get(7,32));
            c.push(m,0x6c05acu32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x512482);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5128d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,0x1u32);
            pc = 0x512487; continue;
            }
            0x512487 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd6),16));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.set(5,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(5,16); let r = c.add(a,c.get(3,16),16); c.set(5,16,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x15u32,32); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(5,16)); }
            if c.cond(2) { pc = 0x512487; continue; }
            pc = 0x5124af; continue;
            }
            0x5124af => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd6),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.set(3,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x28),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x5124f0; continue; }
            pc = 0x5124cf; continue;
            }
            0x5124cf => {
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,8,0x0u32); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd6),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(3,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x28),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x5124cf; continue; }
            pc = 0x5124f0; continue;
            }
            0x5124f0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            c.set(7,32,c.r[6].wrapping_add(0xb4));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x512579; continue; }
            pc = 0x512502; continue;
            }
            0x512502 => {
            c.push(m,0x6c0504u32);
            c.push(m,0x12cu32);
            c.push(m,0x6c05c8u32);
            c.push(m,0x6c04f4u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x512520);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0xd8); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0xda); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0]).wrapping_add(0x2));
            c.push(m,c.get(0,32));
            c.push(m, 0x51253d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd8),16));
            c.set(2,32,c.r[1].wrapping_add(c.r[1]).wrapping_add(0x2));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x512558);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xda),16));
            c.push(m,c.get(7,32));
            c.push(m,0x6c05d8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x512579);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5128d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x512579; continue;
            }
            0x512579 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb8),32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xd8),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0xda); wm(m,a,16,c.get(2,16)); }
            if c.cond(4) { pc = 0x5125a0; continue; }
            pc = 0x512591; continue;
            }
            0x512591 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x512597);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xb8); wm(m,a,32,c.get(3,32)); }
            pc = 0x5125a0; continue;
            }
            0x5125a0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5125b9; continue; }
            pc = 0x5125aa; continue;
            }
            0x5125aa => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5125b0);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xcc); wm(m,a,32,c.get(3,32)); }
            pc = 0x5125b9; continue;
            }
            0x5125b9 => {
            { let a = c.r[6].wrapping_add(0xdc); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0xe0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xe4); wm(m,a,32,c.get(3,32)); }
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

pub fn f_005125e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5125e0;
    loop {
        match pc {
            0x5125e0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x512601);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x6389ee, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x512700; continue; }
            pc = 0x512610; continue;
            }
            0x512610 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x51261c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x8));
            c.set(3,32,c.r[0].wrapping_add(c.r[2]));
            c.set(5,32,0x28u32);
            pc = 0x512637; continue;
            }
            0x512637 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[7]; wm(m,a,32,c.get(1,32)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.sub(a,0x20u32,8); }
            if c.cond(2) { pc = 0x512663; continue; }
            pc = 0x512643; continue;
            }
            0x512643 => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x512663; continue; }
            pc = 0x512647; continue;
            }
            0x512647 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.sub(a,0x20u32,8); }
            if c.cond(3) { pc = 0x512643; continue; }
            pc = 0x512663; continue;
            }
            0x512663 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[0]; wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = rm(m,c.r[0],8); c.sub(a,0x20u32,8); }
            if c.cond(3) { pc = 0x512687; continue; }
            pc = 0x512679; continue;
            }
            0x512679 => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x512687; continue; }
            pc = 0x51267d; continue;
            }
            0x51267d => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[0],8); c.sub(a,0x20u32,8); }
            if c.cond(2) { pc = 0x512679; continue; }
            pc = 0x512687; continue;
            }
            0x512687 => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x512637; continue; }
            pc = 0x51268d; continue;
            }
            0x51268d => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x5126e2; continue; }
            pc = 0x512691; continue;
            }
            0x512691 => {
            c.set(1,8,rm(m,c.r[0],8));
            pc = 0x512693; continue;
            }
            0x512693 => {
            { let a = c.get(1,8); c.sub(a,0x20u32,8); }
            if c.cond(2) { pc = 0x5126b7; continue; }
            pc = 0x512698; continue;
            }
            0x512698 => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x5126b7; continue; }
            pc = 0x51269c; continue;
            }
            0x51269c => {
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.sub(a,0x20u32,8); }
            if c.cond(3) { pc = 0x512698; continue; }
            pc = 0x5126b7; continue;
            }
            0x5126b7 => {
            { let a = c.r[2]; wm(m,a,8,0x0u32); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.get(5,32));
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.sub(a,0x20u32,8); }
            if c.cond(3) { pc = 0x5126de; continue; }
            pc = 0x5126ce; continue;
            }
            0x5126ce => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x5126e2; continue; }
            pc = 0x5126d2; continue;
            }
            0x5126d2 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.sub(a,0x20u32,8); }
            if c.cond(2) { pc = 0x5126ce; continue; }
            pc = 0x5126de; continue;
            }
            0x5126de => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x512693; continue; }
            pc = 0x5126e2; continue;
            }
            0x5126e2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5126f3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6389f4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x512700 => {
            c.set(7,32,c.r[6].wrapping_add(0x8));
            c.set(1,32,0x28u32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.stos(m,32,true);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005128d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5128d0;
    loop {
        match pc {
            0x5128d0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(6) { pc = 0x512a3c; continue; }
            pc = 0x5128e3; continue;
            }
            0x5128e3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = rm(m,c.r[0],8); c.sub(a,0x23u32,8); }
            if c.cond(4) { pc = 0x512a3c; continue; }
            pc = 0x5128f0; continue;
            }
            0x5128f0 => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x512906);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x6389ee, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x512a3c; continue; }
            pc = 0x512915; continue;
            }
            0x512915 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.add(a,0x1e0u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x512925);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(7,32,c.r[6].wrapping_add(c.r[2]));
            c.set(2,32,0x1u32);
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x2); wm(m,a,16,0x0u32); }
            { let a = c.r[0]; wm(m,a,16,0x0u32); }
            if c.cond(3) { pc = 0x5129b0; continue; }
            pc = 0x512955; continue;
            }
            0x512955 => {
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.get(1,8); c.sub(a,0x41u32,8); }
            if c.cond(3) { pc = 0x512994; continue; }
            pc = 0x51295c; continue;
            }
            0x51295c => {
            { let a = c.get(1,8); c.sub(a,0x20u32,8); }
            if c.cond(4) { pc = 0x512994; continue; }
            pc = 0x512961; continue;
            }
            0x512961 => {
            { let a = c.get(1,8); c.sub(a,0x2eu32,8); }
            if c.cond(4) { pc = 0x512994; continue; }
            pc = 0x512966; continue;
            }
            0x512966 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x512970; continue; }
            pc = 0x51296b; continue;
            }
            0x51296b => {
            { let a = rm(m,c.r[6],8); c.sub(a,0x41u32,8); }
            if c.cond(2) { pc = 0x512966; continue; }
            pc = 0x512970; continue;
            }
            0x512970 => {
            c.set(5,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16));
            { let a = c.r[1].wrapping_add(c.r[5]); wm(m,a,8,0x0u32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16); let r = c.inc(a,16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,r); } }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(3) { pc = 0x5129c9; continue; }
            pc = 0x51298c; continue;
            }
            0x51298c => {
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x2); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x5129ac; continue;
            }
            0x512994 => {
            c.set(5,32,rm(m,c.r[3],32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16));
            { let a = c.r[7].wrapping_add(c.r[5]); wm(m,a,8,c.get(1,8)); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16); let r = c.inc(a,16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,r); } }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x5129ac; continue;
            }
            0x5129ac => {
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x512955; continue; }
            pc = 0x5129b0; continue;
            }
            0x5129b0 => {
            c.set(6,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0xffffffff),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x5129f3; continue; }
            pc = 0x5129c0; continue;
            }
            0x5129c0 => {
            { let a = c.r[1]; wm(m,a,8,0x0u32); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16); let r = c.inc(a,16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,r); } }
            pc = 0x5129f4; continue;
            }
            0x5129c9 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[1].wrapping_add(c.r[2]); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5129e6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6389f4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5129f3 => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            pc = 0x5129f4; continue;
            }
            0x5129f4 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.set(7,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            { let a = c.r[0].wrapping_add(c.r[5].wrapping_mul(2)); wm(m,a,16,c.get(1,16)); }
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,8,0x0u32); }
            if c.cond(3) { pc = 0x512a1e; continue; }
            pc = 0x512a0f; continue;
            }
            0x512a0f => {
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.stos(m,32,true);
            { let a = c.get(1,32); let r = c.adc(a,c.get(1,32),32); c.set(1,32,r); }
            c.stos(m,16,true);
            pc = 0x512a1e; continue;
            }
            0x512a1e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x512a2f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6389f4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x512a3c => {
            c.push(m,0x4u32);
            c.push(m, 0x512a43);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[0]; wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0051e440(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51e440;
    loop {
        match pc {
            0x51e440 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6631f0u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0x6c06a4u32);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(3,32,0x1u32);
            c.push(m, 0x51e46f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,c.r[4].wrapping_add(0x8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(3,8)); }
            c.push(m,c.get(1,32));
            c.push(m,0x6c06c0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x51e49e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6c06c8u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e4bc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6c06d4u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e4da);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6c06e0u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x51e4f8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6c06ecu32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e516);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6c06f8u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e534);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6c0708u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x51e551);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6c0718u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e56f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6c072cu32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e58d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6c0740u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x51e5ab);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6c0754u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e5c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6c0768u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e5e7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            c.push(m,0xffu32);
            c.push(m,0x6c0784u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x51e609);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x2e); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6c0790u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e627);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6c0798u32);
            c.push(m,c.get(1,32));
            c.set(2,8,u32::from(c.cond(4)));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[6].wrapping_add(0x2f); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x51e64a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6c07a0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e668);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0x31); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(1,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x6de3b2u32);
            c.push(m,0x6c07b8u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e693);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x51e6a1; continue; }
            pc = 0x51e697; continue;
            }
            0x51e697 => {
            c.set(0,8,rm(m,0x6dbb58u32,8));
            { let a = c.r[6].wrapping_add(0x33); wm(m,a,8,c.get(0,8)); }
            pc = 0x51e6a3; continue;
            }
            0x51e6a1 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x51e6a3; continue;
            }
            0x51e6a3 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6de3b3u32);
            c.push(m,0x6c07d4u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e6cb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x51e6de; continue; }
            pc = 0x51e6cf; continue;
            }
            0x51e6cf => {
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[6].wrapping_add(0x28));
            c.push(m, 0x51e6dc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x51e6e0; continue;
            }
            0x51e6de => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x51e6e0; continue;
            }
            0x51e6e0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m,0x2du32);
            c.push(m,0x6c07e8u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x51e6fb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[6].wrapping_add(0x32); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            c.push(m, 0x51e70c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x51e71d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00512730(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x512730;
    loop {
        match pc {
            0x512730 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(6,32,rm(m,c.r[5],32));
            c.set(7,32,c.r[5].wrapping_add(0x8));
            pc = 0x51273e; continue;
            }
            0x51273e => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x512757; continue; }
            pc = 0x512744; continue;
            }
            0x512744 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5],32),32); }
            if c.cond(2) { pc = 0x51274e; continue; }
            pc = 0x512749; continue;
            }
            0x512749 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x4),32),32); }
            if c.cond(2) { pc = 0x512757; continue; }
            pc = 0x51274e; continue;
            }
            0x51274e => {
            c.push(m,c.get(0,32));
            c.push(m, 0x512754);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x512757; continue;
            }
            0x512757 => {
            { let a = c.r[7]; wm(m,a,32,c.get(6,32)); }
            { let a = c.r[3].wrapping_add(0x6dda00); wm(m,a,32,c.get(6,32)); }
            pc = 0x51275f; continue;
            }
            0x51275f => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x51275f; continue; }
            pc = 0x512766; continue;
            }
            0x512766 => {
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,0xa0u32,32); }
            if c.cond(2) { pc = 0x51273e; continue; }
            pc = 0x512774; continue;
            }
            0x512774 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x512776; continue;
            }
            0x512776 => {
            { let a = c.r[1].wrapping_add(0x6dd960); wm(m,a,32,c.get(6,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x51278f; continue; }
            pc = 0x512782; continue;
            }
            0x512782 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            pc = 0x512785; continue;
            }
            0x512785 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5127b8; continue; }
            pc = 0x51278a; continue;
            }
            0x51278a => {
            { let a = rm(m,c.r[6],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x512785; continue; }
            pc = 0x51278f; continue;
            }
            0x51278f => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x512790; continue;
            }
            0x512790 => {
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0xa0u32,32); }
            if c.cond(2) { pc = 0x512776; continue; }
            pc = 0x51279b; continue;
            }
            0x51279b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x51279d; continue;
            }
            0x51279d => {
            { let a = c.r[1].wrapping_add(0x6dd8c0); wm(m,a,32,c.get(6,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5127c2; continue; }
            pc = 0x5127a9; continue;
            }
            0x5127a9 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            pc = 0x5127ac; continue;
            }
            0x5127ac => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5127bd; continue; }
            pc = 0x5127b1; continue;
            }
            0x5127b1 => {
            { let a = rm(m,c.r[6],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x5127ac; continue; }
            pc = 0x5127b6; continue;
            }
            0x5127b6 => {
            pc = 0x5127c2; continue;
            }
            0x5127b8 => {
            c.set(6,32,c.r[0].wrapping_add(0xffffffff));
            pc = 0x512790; continue;
            }
            0x5127bd => {
            c.set(6,32,c.r[0].wrapping_add(0xffffffff));
            pc = 0x5127c3; continue;
            }
            0x5127c2 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x5127c3; continue;
            }
            0x5127c3 => {
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0xa0u32,32); }
            if c.cond(2) { pc = 0x51279d; continue; }
            pc = 0x5127ce; continue;
            }
            0x5127ce => {
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

pub fn f_004f6420(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f6420;
    loop {
        match pc {
            0x4f6420 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1a),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1c),8));
            if c.cond(4) { pc = 0x4f6431; continue; }
            pc = 0x4f642d; continue;
            }
            0x4f642d => {
            { let a = c.get(0,8); let r = c.and(a,0x7fu32,8); c.set(0,8,r); }
            pc = 0x4f6433; continue;
            }
            0x4f6431 => {
            { let a = c.get(0,8); let r = c.or(a,0x80u32,8); c.set(0,8,r); }
            pc = 0x4f6433; continue;
            }
            0x4f6433 => {
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1c),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4f6446);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xe8); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f6450(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f6450;
    loop {
        match pc {
            0x4f6450 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1d),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f6463);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xe4); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
