//! Generado por `reimpl/tools/ui_rt_grp.py sie` con `tools/lift.py` (no editar a mano): grupo `sie`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_SIE: [(u32, super::core_table::CoreFn); 39] = [
    (0x5623b0, f_005623b0),
    (0x5624c0, f_005624c0),
    (0x570f70, f_00570f70),
    (0x5710a0, f_005710a0),
    (0x5740f0, f_005740f0),
    (0x5743a0, f_005743a0),
    (0x5758e0, f_005758e0),
    (0x577fb0, f_00577fb0),
    (0x571350, f_00571350),
    (0x5719a0, f_005719a0),
    (0x574360, f_00574360),
    (0x575560, f_00575560),
    (0x575810, f_00575810),
    (0x575730, f_00575730),
    (0x576990, f_00576990),
    (0x576e80, f_00576e80),
    (0x576d20, f_00576d20),
    (0x62fb90, f_0062fb90),
    (0x55ffc0, f_0055ffc0),
    (0x564810, f_00564810),
    (0x564ac0, f_00564ac0),
    (0x564a80, f_00564a80),
    (0x566fe0, f_00566fe0),
    (0x566260, f_00566260),
    (0x565d70, f_00565d70),
    (0x565e10, f_00565e10),
    (0x565f50, f_00565f50),
    (0x5661d0, f_005661d0),
    (0x5663d0, f_005663d0),
    (0x5669d0, f_005669d0),
    (0x566e30, f_00566e30),
    (0x56a700, f_0056a700),
    (0x56a7f0, f_0056a7f0),
    (0x56f0b0, f_0056f0b0),
    (0x56f1a0, f_0056f1a0),
    (0x572a90, f_00572a90),
    (0x5595f0, f_005595f0),
    (0x55e250, f_0055e250),
    (0x55e8c0, f_0055e8c0),
];
pub fn f_005623b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5623b0;
    loop {
        match pc {
            0x5623b0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,0x6e16e0u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x562432; continue; }
            pc = 0x5623cf; continue;
            }
            0x5623cf => {
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            pc = 0x5623d9; continue;
            }
            0x5623d9 => {
            c.set(2,32,rm(m,0x6e16e8u32,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2]),8));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(5) { pc = 0x56241e; continue; }
            pc = 0x5623e9; continue;
            }
            0x5623e9 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xc),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x562408; continue; }
            pc = 0x5623f1; continue;
            }
            0x5623f1 => {
            c.push(m, 0x5623f6);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x56240e; continue; }
            pc = 0x562408; continue;
            }
            0x562408 => {
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,8,c.get(3,8)); }
            pc = 0x56240e; continue;
            }
            0x56240e => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xc),8));
            c.set(1,32,rm(m,0x6e16e0u32,32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(0,8)); }
            pc = 0x56241e; continue;
            }
            0x56241e => {
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5623d9; continue; }
            pc = 0x562431; continue;
            }
            0x562431 => {
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x562432; continue;
            }
            0x562432 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x88),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x562442; continue; }
            pc = 0x56243e; continue;
            }
            0x56243e => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x56244b; continue;
            }
            0x562442 => {
            c.set(2,32,rm(m,0x6e16e4u32,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x56244b; continue;
            }
            0x56244b => {
            c.set(2,32,rm(m,0x6e16e8u32,32));
            c.push(m,0x0u32);
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[2]),8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,8); let r = c.and(a,0x7fu32,8); c.set(0,8,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,8); let r = c.and(a,0x1u32,8); c.set(2,8,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x562485);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40))]; c.r[0] = host.call(m, 0x55e620, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(44);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005624c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5624c0;
    loop {
        match pc {
            0x5624c0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e16e0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x88),8));
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.push(m,c.get(6,32));
            if c.cond(2) { pc = 0x5624dc; continue; }
            pc = 0x5624d8; continue;
            }
            0x5624d8 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x5624e5; continue;
            }
            0x5624dc => {
            c.set(1,32,rm(m,0x6e16e4u32,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x5624e5; continue;
            }
            0x5624e5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x6a),8));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m, 0x562506);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x562521; continue; }
            pc = 0x56250a; continue;
            }
            0x56250a => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x56251c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x562521 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x6a),8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(6,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x14),16),16); }
            if c.cond(5) { pc = 0x56259d; continue; }
            pc = 0x562544; continue;
            }
            0x562544 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x16),16),16); }
            if c.cond(5) { pc = 0x56259d; continue; }
            pc = 0x56254e; continue;
            }
            0x56254e => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x36),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x56255f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,16,c.get(0,16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x44),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x0u32);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x66),16));
            c.push(m,0x1u32);
            c.set(6,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(1,32,c.get(3,32));
            { let b = c.get(6,32); c.div32(b); }
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x562598);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x56259d => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x36),16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x5625b4; continue; }
            pc = 0x5625a6; continue;
            }
            0x5625a6 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x38),16),16); }
            if c.cond(4) { pc = 0x56250a; continue; }
            pc = 0x5625b4; continue;
            }
            0x5625b4 => {
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x3e),16),16); }
            if c.cond(5) { pc = 0x56261e; continue; }
            pc = 0x5625ba; continue;
            }
            0x5625ba => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x40),16),16); }
            if c.cond(5) { pc = 0x56261e; continue; }
            pc = 0x5625c4; continue;
            }
            0x5625c4 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x5625d6; continue; }
            pc = 0x5625ce; continue;
            }
            0x5625ce => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            pc = 0x5625dc; continue;
            }
            0x5625d6 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            pc = 0x5625dc; continue;
            }
            0x5625dc => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5625e3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,16,c.get(0,16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x68),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3c),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.sub(a,0x100u32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x1u32);
            { let b = c.get(1,32); c.div32(b); }
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x562619);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x56261e => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x56262c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,16,c.get(0,16));
            c.set(0,32,0x8u32);
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x8u32,32); }
            if c.cond(3) { pc = 0x562641; continue; }
            pc = 0x56263f; continue;
            }
            0x56263f => {
            c.set(0,32,c.get(7,32));
            pc = 0x562641; continue;
            }
            0x562641 => {
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6f),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x56265f; continue; }
            pc = 0x56264e; continue;
            }
            0x56264e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x66),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            pc = 0x5626ab; continue;
            }
            0x56265f => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x44),16));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x254),16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x100u32,32); }
            if c.cond(6) { pc = 0x562681; continue; }
            pc = 0x56267c; continue;
            }
            0x56267c => {
            c.set(1,32,0x100u32);
            pc = 0x562681; continue;
            }
            0x562681 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[2].wrapping_add(c.r[1]));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x5626a5; continue; }
            pc = 0x56268e; continue;
            }
            0x56268e => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x66),16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(1,32),32); c.set(5,32,r); }
            { let b = c.get(5,32); c.div32(b); }
            pc = 0x5626a7; continue;
            }
            0x5626a5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5626a7; continue;
            }
            0x5626a7 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5626ab; continue;
            }
            0x5626ab => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5626bf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
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

pub fn f_00570f70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x570f70;
    loop {
        match pc {
            0x570f70 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6e2dacu32,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x570f84);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let b = c.get(7,32); c.div32(b); }
            c.set(0,32,c.get(3,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x90); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x8c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x91); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,0x6e2dacu32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x570fc0; continue; }
            pc = 0x570fbc; continue;
            }
            0x570fbc => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x570fc9; continue;
            }
            0x570fc0 => {
            c.set(1,32,rm(m,0x6e2db0u32,32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            pc = 0x570fc9; continue;
            }
            0x570fc9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,0x1u32);
            c.set(2,8,c.get(3,8));
            c.push(m,c.get(5,32));
            c.push(m,c.get(5,32));
            { let a = c.get(2,8); let r = c.and(a,0x7fu32,8); c.set(2,8,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x570ff1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40))]; c.r[0] = host.call(m, 0x55e620, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(44);
            { let a = c.get(7,8); c.and(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0x82); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[6].wrapping_add(0x84); wm(m,a,16,c.get(5,16)); }
            if c.cond(4) { pc = 0x571058; continue; }
            pc = 0x571004; continue;
            }
            0x571004 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571011);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.push(m,0x18u32);
            c.set(2,32,c.get(0,32));
            c.push(m,c.get(7,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(1,32,c.r[6].wrapping_add(0x2c));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x5d2),8));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x571041);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x57104b);
            { let args = []; c.r[0] = host.call(m, 0x571da0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x571058);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x571058; continue;
            }
            0x571058 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005710a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5710a0;
    loop {
        match pc {
            0x5710a0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e2dacu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x91),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5710bd; continue; }
            pc = 0x5710b9; continue;
            }
            0x5710b9 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x5710c6; continue;
            }
            0x5710bd => {
            c.set(1,32,rm(m,0x6e2db0u32,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x5710c6; continue;
            }
            0x5710c6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6a),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m, 0x5710e7);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6df270u32,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x571128; continue; }
            pc = 0x5710f1; continue;
            }
            0x5710f1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6a),8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x571110; continue; }
            pc = 0x571108; continue;
            }
            0x571108 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x571110 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x4u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x571122);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x571128 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6a),8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(6,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x88),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x57115d; continue; }
            pc = 0x571145; continue;
            }
            0x571145 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x58),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x57115d; continue; }
            pc = 0x57114c; continue;
            }
            0x57114c => {
            c.set(1,32,c.get(7,32));
            { let a = c.r[7].wrapping_add(0x88); wm(m,a,32,0x0u32); }
            c.push(m, 0x57115d);
            { let args = []; c.r[0] = host.call(m, 0x571da0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x57115d; continue;
            }
            0x57115d => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x14),16),16); }
            if c.cond(5) { pc = 0x5711c1; continue; }
            pc = 0x571167; continue;
            }
            0x571167 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x16),16),16); }
            if c.cond(5) { pc = 0x5711c1; continue; }
            pc = 0x571171; continue;
            }
            0x571171 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x36),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571182);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,16,c.get(0,16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x44),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x0u32);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x66),16));
            c.push(m,0x1u32);
            c.set(6,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(1,32,c.get(7,32));
            { let b = c.get(6,32); c.div32(b); }
            c.push(m,c.get(0,32));
            c.push(m,0x4u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x5711bb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5711c1 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x36),16));
            { let a = c.get(1,16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x571209; continue; }
            pc = 0x5711ca; continue;
            }
            0x5711ca => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x38),16),16); }
            if c.cond(5) { pc = 0x571209; continue; }
            pc = 0x5711d4; continue;
            }
            0x5711d4 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x79),8));
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5711f3; continue; }
            pc = 0x5711dd; continue;
            }
            0x5711dd => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x4u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x5711ed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5711f3 => {
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x4u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x571203);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x571209 => {
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x3e),16),16); }
            if c.cond(5) { pc = 0x57127f; continue; }
            pc = 0x57120f; continue;
            }
            0x57120f => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x40),16),16); }
            if c.cond(5) { pc = 0x57127f; continue; }
            pc = 0x571219; continue;
            }
            0x571219 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x57122c; continue; }
            pc = 0x571224; continue;
            }
            0x571224 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x571232; continue;
            }
            0x57122c => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x571232; continue;
            }
            0x571232 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571239);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,16,c.get(0,16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x68),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3c),16));
            c.set(6,32,c.get(1,32));
            { let a = c.get(6,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x100u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x200u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x0u32);
            { let b = c.get(1,32); c.div32(b); }
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,0x4u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x571279);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x57127f => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x57128c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,16,c.get(0,16));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6f),8));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x5712e0; continue; }
            pc = 0x5712a0; continue;
            }
            0x5712a0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x66),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(6) { pc = 0x5712cb; continue; }
            pc = 0x5712b6; continue;
            }
            0x5712b6 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,0x4u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x5712c5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5712cb => {
            c.push(m,0x4u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,0x4u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x5712da);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5712e0 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x44),16));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x254),16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x100u32,32); }
            if c.cond(6) { pc = 0x571302; continue; }
            pc = 0x5712fd; continue;
            }
            0x5712fd => {
            c.set(1,32,0x100u32);
            pc = 0x571302; continue;
            }
            0x571302 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[2].wrapping_add(c.r[1]));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x571329; continue; }
            pc = 0x57130f; continue;
            }
            0x57130f => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x66),16));
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            { let b = c.get(3,32); c.div32(b); }
            { let a = c.get(0,32); c.sub(a,0xau32,32); }
            if c.cond(3) { pc = 0x57132e; continue; }
            pc = 0x571329; continue;
            }
            0x571329 => {
            c.set(0,32,0xau32);
            pc = 0x57132e; continue;
            }
            0x57132e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,0x4u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x571343);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
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

pub fn f_005740f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5740f0;
    loop {
        match pc {
            0x5740f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6e2e14u32,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x574104);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x49); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x35); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,0x6e2e14u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x57412f; continue; }
            pc = 0x57412b; continue;
            }
            0x57412b => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x574137; continue;
            }
            0x57412f => {
            c.set(0,32,rm(m,0x6e2e18u32,32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            pc = 0x574137; continue;
            }
            0x574137 => {
            c.push(m,0x2u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x574142);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x574150);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x559cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x48),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x48),8));
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xc678),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xc674),32));
            c.set(5,32,c.r[1].wrapping_add(c.r[0]));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x4),32));
            c.set(1,8,rm(m,c.r[5],8));
            c.set(5,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            c.set(2,32,rm(m,c.r[2],32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[1].wrapping_add(c.r[5]),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0xfu32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.set(5,32,rm(m,c.r[2].wrapping_add(0xc678),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xc674),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[5]).wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,0x1u32); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xe),16));
            { let a = c.get(0,32); let n = 0xfu32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x12),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5741e1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x575560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x42),8));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5741fb);
            { let args = []; c.r[0] = host.call(m, 0x559260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m, 0x574212);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[4].wrapping_add(0x48),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x574228; continue; }
            pc = 0x574218; continue;
            }
            0x574218 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x574228);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x574228; continue;
            }
            0x574228 => {
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x574232);
            { let args = []; c.r[0] = host.call(m, 0x559810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57424f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x49),8));
            c.set(5,32,c.get(0,32));
            c.push(m,0x10u32);
            { let a = c.get(5,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[5].wrapping_mul(8)));
            c.set(5,32,rm(m,0x6df270u32,32));
            c.set(5,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x574281);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792fu32,8));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57429d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x10u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5742b0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,0x67792au32,8));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5742cb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x10u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5742de);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,8,rm(m,0x67792fu32,8));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x5742fa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x10u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x57430d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,0x3ab20u32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = rm(m,c.r[7].wrapping_add(0xc),32); c.div32(b); }
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x58); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x2e); wm(m,a,16,0xffffu32); }
            { let a = c.r[6].wrapping_add(0x5c); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x64); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005743a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5743a0;
    loop {
        match pc {
            0x5743a0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x5cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e2e14u32,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x35),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5743c2; continue; }
            pc = 0x5743b8; continue;
            }
            0x5743b8 => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            pc = 0x5743cf; continue;
            }
            0x5743c2 => {
            c.set(1,32,rm(m,0x6e2e18u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            pc = 0x5743cf; continue;
            }
            0x5743cf => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x49),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x5743f5; continue; }
            pc = 0x5743eb; continue;
            }
            0x5743eb => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x5cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5743f5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x414),32));
            { let a = c.get(5,32); c.sub(a,0x3e8u32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            if c.cond(6) { pc = 0x57442b; continue; }
            pc = 0x574422; continue;
            }
            0x574422 => {
            c.set(5,32,0x3e8u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            pc = 0x57442b; continue;
            }
            0x57442b => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x34),8));
            c.set(0,32,c.r[6].wrapping_add(0x34));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.set(2,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[7].wrapping_add(0x4),32); c.push(m, 0x574448); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,c.r[4].wrapping_add(0x3c));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x574466);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5745e9; continue; }
            pc = 0x574471; continue;
            }
            0x574471 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x64),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x574491; continue; }
            pc = 0x574478; continue;
            }
            0x574478 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x3f8u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x57448a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x64); wm(m,a,32,0x0u32); }
            pc = 0x574491; continue;
            }
            0x574491 => {
            c.set(0,8,rm(m,0x677929u32,8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0xb0023u32);
            c.push(m,0x12u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5744b5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(0,8,rm(m,0x677929u32,8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0xc2u32);
            c.push(m,0x12u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5744d9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.push(m,0xffffff80u32);
            c.push(m,0x7d0u32);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5744e7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x57af40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,0x5u32);
            pc = 0x5744ec; continue;
            }
            0x5744ec => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5744f5);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,0x0u32);
            c.push(m,0x13u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x57450d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x5744ec; continue; }
            pc = 0x574510; continue;
            }
            0x574510 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xea7eu32);
            c.push(m, 0x574520);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x57455a; continue; }
            pc = 0x574524; continue;
            }
            0x574524 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xea7eu32);
            c.push(m, 0x574534);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x57453b);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let r = c.sub(a,0x1582u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57455a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x595b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x57455a; continue;
            }
            0x57455a => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x54),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x50),16));
            c.set(7,32,c.r[6].wrapping_add(0x54));
            { let a = c.get(6,32); let r = c.add(a,0x50u32,32); c.set(6,32,r); }
            c.push(m,0x10u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574573);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,0x67792fu32,8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x574586);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,16,rm(m,c.r[7],16));
            c.set(2,16,rm(m,c.r[6],16));
            c.push(m,0x10u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574597);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,0x67792au32,8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5745aa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,16,rm(m,c.r[7],16));
            c.set(2,16,rm(m,c.r[6],16));
            c.push(m,0x10u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5745bb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,0x67792fu32,8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5745ce);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,16,rm(m,c.r[7],16));
            c.set(2,16,rm(m,c.r[6],16));
            c.push(m,0x10u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5745df);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x5cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5745e9 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            c.set(0,8,0x6u32);
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x57469b; continue; }
            pc = 0x5745f7; continue;
            }
            0x5745f7 => {
            { let a = rm(m,c.r[6].wrapping_add(0x15),8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x57469b; continue; }
            pc = 0x574600; continue;
            }
            0x574600 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xee7au32);
            c.push(m, 0x574610);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x57464a; continue; }
            pc = 0x574614; continue;
            }
            0x574614 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xee7au32);
            c.push(m, 0x574624);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x57462b);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,0x1186u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x57464a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x595b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x57464a; continue;
            }
            0x57464a => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,8); c.sub(a,0x80u32,8); }
            if c.cond(3) { pc = 0x57465c; continue; }
            pc = 0x574654; continue;
            }
            0x574654 => {
            { let a = c.get(7,32); let r = c.add(a,0x1a0000u32,32); c.set(7,32,r); }
            pc = 0x574662; continue;
            }
            0x57465c => {
            { let a = c.get(7,32); let r = c.sub(a,0x1a0000u32,32); c.set(7,32,r); }
            pc = 0x574662; continue;
            }
            0x574662 => {
            { let a = c.get(0,8); c.sub(a,0x60u32,8); }
            if c.cond(4) { pc = 0x574674; continue; }
            pc = 0x574666; continue;
            }
            0x574666 => {
            { let a = c.get(0,8); c.sub(a,0xa0u32,8); }
            if c.cond(4) { pc = 0x574674; continue; }
            pc = 0x57466a; continue;
            }
            0x57466a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.sub(a,0x1b0000u32,32); c.set(0,32,r); }
            pc = 0x57467c; continue;
            }
            0x574674 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.add(a,0x50000u32,32); c.set(0,32,r); }
            pc = 0x57467c; continue;
            }
            0x57467c => {
            c.set(2,8,rm(m,0x677929u32,8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,0x80020u32);
            c.push(m,0x12u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x57469b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            pc = 0x57469b; continue;
            }
            0x57469b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(0,8,0x1u32);
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            if c.cond(4) { pc = 0x5746b2; continue; }
            pc = 0x5746a8; continue;
            }
            0x5746a8 => {
            { let a = c.get(0,8); let r = c.or(a,0xffu32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            pc = 0x5746b2; continue;
            }
            0x5746b2 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,8,0x0u32); }
            if c.cond(6) { pc = 0x575008; continue; }
            pc = 0x5746bf; continue;
            }
            0x5746bf => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x16),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5746cd; continue; }
            pc = 0x5746c7; continue;
            }
            0x5746c7 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            pc = 0x5746cd; continue;
            }
            0x5746cd => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x3c));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5746fb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = rm(m,c.r[6].wrapping_add(0x28),8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x5749dc; continue; }
            pc = 0x574705; continue;
            }
            0x574705 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x40),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x57474e; continue; }
            pc = 0x57470c; continue;
            }
            0x57470c => {
            { let a = rm(m,c.r[3].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(7) { pc = 0x57471a; continue; }
            pc = 0x574713; continue;
            }
            0x574713 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x78),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x57474e; continue; }
            pc = 0x57471a; continue;
            }
            0x57471a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x57474e; continue; }
            pc = 0x574721; continue;
            }
            0x574721 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x41),8));
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,8,0x1u32); }
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x57473a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x575560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4a),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x57474e; continue; }
            pc = 0x574743; continue;
            }
            0x574743 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4c),16));
            { let a = c.get(2,16); let r = c.sub(a,c.get(0,16),16); c.set(2,16,r); }
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,c.get(2,16)); }
            pc = 0x57474e; continue;
            }
            0x57474e => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x44),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x574767; continue; }
            pc = 0x57475c; continue;
            }
            0x57475c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x38),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x574796; continue; }
            pc = 0x574763; continue;
            }
            0x574763 => {
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x57476e; continue; }
            pc = 0x574767; continue;
            }
            0x574767 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x38),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x574796; continue; }
            pc = 0x57476e; continue;
            }
            0x57476e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x46),16));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x57478b; continue; }
            pc = 0x57477c; continue;
            }
            0x57477c => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x574796; continue; }
            pc = 0x574783; continue;
            }
            0x574783 => {
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x5748cb; continue; }
            pc = 0x57478b; continue;
            }
            0x57478b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5748cb; continue; }
            pc = 0x574796; continue;
            }
            0x574796 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x40),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5747af; continue; }
            pc = 0x57479d; continue;
            }
            0x57479d => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x41),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x575199; continue; }
            pc = 0x5747a8; continue;
            }
            0x5747a8 => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,c.get(0,8)); }
            pc = 0x5747ff; continue;
            }
            0x5747af => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x48),8));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xc674),32));
            c.set(7,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x41),8));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[7]),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5747ff; continue; }
            pc = 0x5747d5; continue;
            }
            0x5747d5 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x41),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xc674),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[7]),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5747ff; continue; }
            pc = 0x5747f6; continue;
            }
            0x5747f6 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            pc = 0x5747ff; continue;
            }
            0x5747ff => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x46),16));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x44),16));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x574826);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x575560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[6].wrapping_add(0x4a),16); let r = c.add(a,c.get(5,16),16); { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,r); } }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4a),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4c),16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x5748a0; continue; }
            pc = 0x574837; continue;
            }
            0x574837 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x5748a0; continue; }
            pc = 0x57483e; continue;
            }
            0x57483e => {
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x574884; continue; }
            pc = 0x574857; continue;
            }
            0x574857 => {
            { let a = rm(m,c.r[6].wrapping_add(0x4a),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x57486f; continue; }
            pc = 0x57485e; continue;
            }
            0x57485e => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x4a),16); let r = c.add(a,c.get(5,16),16); { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,r); } }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            pc = 0x57489c; continue;
            }
            0x57486f => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,c.get(1,16)); }
            pc = 0x57489c; continue;
            }
            0x574884 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x42),8));
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x30),16));
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            { let a = rm(m,c.r[6].wrapping_add(0x4a),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,r); } }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            pc = 0x57489c; continue;
            }
            0x57489c => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            pc = 0x5748a0; continue;
            }
            0x5748a0 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4a),16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(2) { pc = 0x5749dc; continue; }
            pc = 0x5748ad; continue;
            }
            0x5748ad => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,0x0u32); }
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            pc = 0x5749dc; continue;
            }
            0x5748cb => {
            { let a = rm(m,c.r[6].wrapping_add(0x4a),16); let r = c.add(a,c.get(5,16),16); { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,r); } }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x4a),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4c),16));
            { let a = c.get(7,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x57493c; continue; }
            pc = 0x5748dc; continue;
            }
            0x5748dc => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x57493c; continue; }
            pc = 0x5748e3; continue;
            }
            0x5748e3 => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x574927; continue; }
            pc = 0x5748fa; continue;
            }
            0x5748fa => {
            { let a = rm(m,c.r[6].wrapping_add(0x4a),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x574912; continue; }
            pc = 0x574901; continue;
            }
            0x574901 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x4a),16); let r = c.add(a,c.get(5,16),16); { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,r); } }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            pc = 0x574938; continue;
            }
            0x574912 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,c.get(1,16)); }
            pc = 0x574938; continue;
            }
            0x574927 => {
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x30),16));
            { let a = rm(m,c.r[6].wrapping_add(0x4a),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,r); } }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            pc = 0x574938; continue;
            }
            0x574938 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            pc = 0x57493c; continue;
            }
            0x57493c => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4a),16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(2) { pc = 0x5749bc; continue; }
            pc = 0x574945; continue;
            }
            0x574945 => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x38),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,0x0u32); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x57498e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5749a8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            pc = 0x5749d8; continue;
            }
            0x5749bc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            pc = 0x5749d8; continue;
            }
            0x5749d8 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x5749dc; continue;
            }
            0x5749dc => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x3c));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x574a04);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x50),32));
            c.set(7,32,c.r[6].wrapping_add(0x50));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            if c.cond(5) { pc = 0x574a1f; continue; }
            pc = 0x574a16; continue;
            }
            0x574a16 => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[7].wrapping_add(0x4),32),32); }
            if c.cond(4) { pc = 0x574f89; continue; }
            pc = 0x574a1f; continue;
            }
            0x574a1f => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x54),16));
            c.set(5,32,c.r[6].wrapping_add(0x54));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x10u32);
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,8,0x1u32); }
            c.push(m, 0x574a4b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792fu32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x574a5f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[5],16));
            c.set(0,16,rm(m,c.r[7],16));
            c.push(m,0x10u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574a71);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792au32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x574a85);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[5],16));
            c.set(0,16,rm(m,c.r[7],16));
            c.push(m,0x10u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574a97);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792fu32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x574aab);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[5],16));
            c.set(0,16,rm(m,c.r[7],16));
            c.push(m,0x10u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574abd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x38),16));
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x36),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574ace);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x630760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,16,c.get(0,16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x42),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x66),16));
            { let b = c.get(1,32); c.div32(b); }
            c.set(2,16,rm(m,c.r[5],16));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[7],16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x574afe);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x630760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x574b10; continue; }
            pc = 0x574b0b; continue;
            }
            0x574b0b => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x574b10; continue;
            }
            0x574b10 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x40),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x574bd0; continue; }
            pc = 0x574b1b; continue;
            }
            0x574b1b => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x42),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x574b30);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x575730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x574bd0; continue; }
            pc = 0x574b38; continue;
            }
            0x574b38 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x42),8));
            { let a = rm(m,c.r[3].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            if c.cond(7) { pc = 0x574b62; continue; }
            pc = 0x574b46; continue;
            }
            0x574b46 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x78),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x574b62; continue; }
            pc = 0x574b4d; continue;
            }
            0x574b4d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,0xfu32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x574b5c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            pc = 0x574bc4; continue;
            }
            0x574b62 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x41),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x574b7f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x575560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4a),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x574ba2; continue; }
            pc = 0x574b93; continue;
            }
            0x574b93 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4c),16));
            { let a = c.get(2,16); let r = c.sub(a,c.get(1,16),16); c.set(2,16,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,c.get(2,16)); }
            pc = 0x574ba8; continue;
            }
            0x574ba2 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,c.get(0,16)); }
            pc = 0x574ba8; continue;
            }
            0x574ba8 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4a),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x574bbc; continue; }
            pc = 0x574bb1; continue;
            }
            0x574bb1 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4c),16));
            { let a = c.get(1,16); let r = c.sub(a,c.get(0,16),16); c.set(1,16,r); }
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,c.get(1,16)); }
            pc = 0x574bbc; continue;
            }
            0x574bbc => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            pc = 0x574bc4; continue;
            }
            0x574bc4 => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            pc = 0x574bd0; continue;
            }
            0x574bd0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574be3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x574bec; continue; }
            pc = 0x574be7; continue;
            }
            0x574be7 => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x574bec; continue;
            }
            0x574bec => {
            c.set(1,8,rm(m,0x67792fu32,8));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x574c08);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574c1b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x574c24; continue; }
            pc = 0x574c1f; continue;
            }
            0x574c1f => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x574c24; continue;
            }
            0x574c24 => {
            c.set(0,8,rm(m,0x67792au32,8));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x574c3f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574c52);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x574c5b; continue; }
            pc = 0x574c56; continue;
            }
            0x574c56 => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x574c5b; continue;
            }
            0x574c5b => {
            c.set(2,8,rm(m,0x67792fu32,8));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x574c77);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574c8a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x574c93; continue; }
            pc = 0x574c8e; continue;
            }
            0x574c8e => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x574c93; continue;
            }
            0x574c93 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x46),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x44),16));
            c.set(2,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x574cb1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x574ccb; continue; }
            pc = 0x574cc1; continue;
            }
            0x574cc1 => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x58),32),32); }
            if c.cond(4) { pc = 0x574e6f; continue; }
            pc = 0x574ccb; continue;
            }
            0x574ccb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x42),8));
            { let a = c.get(0,32); let r = c.add(a,0xffffffe0u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xc0u32,32); }
            if c.cond(7) { pc = 0x574e6f; continue; }
            pc = 0x574cde; continue;
            }
            0x574cde => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x57543c),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x575428)); continue;
            }
            0x574ced => {
            c.set(0,8,rm(m,0x67792au32,8));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x574d08);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574d1b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x574d24; continue; }
            pc = 0x574d1f; continue;
            }
            0x574d1f => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x574d24; continue;
            }
            0x574d24 => {
            c.set(2,8,rm(m,0x67792fu32,8));
            pc = 0x574e42; continue;
            }
            0x574d2f => {
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.set(1,8,rm(m,0x67792au32,8));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x574d53);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574d66);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x574d6f; continue; }
            pc = 0x574d6a; continue;
            }
            0x574d6a => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x574d6f; continue;
            }
            0x574d6f => {
            c.set(0,8,rm(m,0x67792du32,8));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x574d8a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x574e64; continue;
            }
            0x574d9b => {
            c.set(2,8,rm(m,0x67792eu32,8));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x574dbf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574dd2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x574ddb; continue; }
            pc = 0x574dd6; continue;
            }
            0x574dd6 => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x574ddb; continue;
            }
            0x574ddb => {
            c.set(1,8,rm(m,0x67792bu32,8));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x574df7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x574e64; continue;
            }
            0x574e05 => {
            c.set(0,8,rm(m,0x67792eu32,8));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x574e20);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574e33);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x574e3c; continue; }
            pc = 0x574e37; continue;
            }
            0x574e37 => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x574e3c; continue;
            }
            0x574e3c => {
            c.set(2,8,rm(m,0x677929u32,8));
            pc = 0x574e42; continue;
            }
            0x574e42 => {
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x574e58);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x574e64; continue;
            }
            0x574e64 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574e6b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x574e77; continue; }
            pc = 0x574e6f; continue;
            }
            0x574e6f => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x574ee1; continue; }
            pc = 0x574e77; continue;
            }
            0x574e77 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,c.get(0,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4a),16));
            c.set(7,32,c.get(0,32));
            c.set(5,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(5,32)); }
            if c.cond(6) { pc = 0x574eb3; continue; }
            pc = 0x574ea4; continue;
            }
            0x574ea4 => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x4c),16));
            { let a = c.get(7,16); let r = c.sub(a,c.get(1,16),16); c.set(7,16,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,c.get(7,16)); }
            pc = 0x574eb9; continue;
            }
            0x574eb3 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,c.get(0,16)); }
            pc = 0x574eb9; continue;
            }
            0x574eb9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x574eee; continue; }
            pc = 0x574ed8; continue;
            }
            0x574ed8 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            pc = 0x574eee; continue;
            }
            0x574ee1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.r[7]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            pc = 0x574eee; continue;
            }
            0x574eee => {
            c.push(m,0x10u32);
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574ef9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,8,rm(m,0x67792fu32,8));
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x574f15);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,0x10u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574f28);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792au32,8));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x574f44);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,0x10u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574f57);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,0x67792fu32,8));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x574f72);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,0x10u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x574f85);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x574f89; continue;
            }
            0x574f89 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x574fd3; continue; }
            pc = 0x574f90; continue;
            }
            0x574f90 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x574fd3; continue; }
            pc = 0x574f97; continue;
            }
            0x574f97 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x17),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x34),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x574fe6; continue; }
            pc = 0x574fa2; continue;
            }
            0x574fa2 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x42),8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x574fe6; continue; }
            pc = 0x574fad; continue;
            }
            0x574fad => {
            { let a = rm(m,c.r[6].wrapping_add(0x28),8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x574fe6; continue; }
            pc = 0x574fb3; continue;
            }
            0x574fb3 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x574fca; continue; }
            pc = 0x574fc1; continue;
            }
            0x574fc1 => {
            { let a = c.get(1,8); let r = c.add(a,0x20u32,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            pc = 0x574fe6; continue;
            }
            0x574fca => {
            { let a = c.get(1,8); let r = c.add(a,0xe0u32,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            pc = 0x574fe6; continue;
            }
            0x574fd3 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x15),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x42),8));
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            if c.cond(5) { pc = 0x574fe6; continue; }
            pc = 0x574fe2; continue;
            }
            0x574fe2 => {
            { let a = c.r[6].wrapping_add(0x58); wm(m,a,8,0x0u32); }
            pc = 0x574fe6; continue;
            }
            0x574fe6 => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x574ff2; continue; }
            pc = 0x574fea; continue;
            }
            0x574fea => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x15),8));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            pc = 0x574ff6; continue;
            }
            0x574ff2 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x16),8));
            pc = 0x574ff6; continue;
            }
            0x574ff6 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x15),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x5746bf; continue; }
            pc = 0x575008; continue;
            }
            0x575008 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x57503e; continue; }
            pc = 0x575019; continue;
            }
            0x575019 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x57503e; continue; }
            pc = 0x575028; continue;
            }
            0x575028 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x57503e; continue; }
            pc = 0x575033; continue;
            }
            0x575033 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x575056; continue; }
            pc = 0x57503e; continue;
            }
            0x57503e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,c.r[4].wrapping_add(0x5c));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(3,8,0x1u32);
            c.push(m, 0x575053);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x575056; continue;
            }
            0x575056 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x57508c; continue; }
            pc = 0x575068; continue;
            }
            0x575068 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(0,32,c.r[4].wrapping_add(0x5c));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x575089);
            { let args = []; c.r[0] = host.call(m, 0x559790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x57508c; continue;
            }
            0x57508c => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x57541b; continue; }
            pc = 0x575097; continue;
            }
            0x575097 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2e),16));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(2,32); c.sub(a,0x20000u32,32); }
            if c.cond(13) { pc = 0x575328; continue; }
            pc = 0x5750ac; continue;
            }
            0x5750ac => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5c),32));
            c.set(2,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x57530b; continue; }
            pc = 0x5750bd; continue;
            }
            0x5750bd => {
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,0x677929u32,8));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0x160024u32);
            c.push(m,0x12u32);
            c.push(m, 0x5750d6);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0xcf0u32);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.sub(a,0x678u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5750f2);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x1400u32);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let r = c.sub(a,0xa00u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x575114);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2e),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xc8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x5c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.add(a,0x19u32,8); c.set(0,8,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6e6220u32);
            { let a = c.r[7].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x575165);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x618d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(0x40));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x57525c; continue; }
            pc = 0x575184; continue;
            }
            0x575184 => {
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x57524e; continue; }
            pc = 0x57518d; continue;
            }
            0x57518d => {
            c.set(3,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x675f58),32));
            pc = 0x57527d; continue;
            }
            0x575199 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x54),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x50),16));
            c.set(7,32,c.r[6].wrapping_add(0x54));
            c.set(5,32,c.r[6].wrapping_add(0x50));
            c.push(m,0x10u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5751b2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792fu32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5751c6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[7],16));
            c.set(0,16,rm(m,c.r[5],16));
            c.push(m,0x10u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5751d8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792au32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5751ec);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[7],16));
            c.set(0,16,rm(m,c.r[5],16));
            c.push(m,0x10u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5751fe);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792fu32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x575212);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[7],16));
            c.set(0,16,rm(m,c.r[5],16));
            c.push(m,0x10u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x575224);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x64),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x575244; continue; }
            pc = 0x57522b; continue;
            }
            0x57522b => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x3f8u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x57523d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x64); wm(m,a,32,0x0u32); }
            pc = 0x575244; continue;
            }
            0x575244 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x5cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x57524e => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,32,0x676158u32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(3,32,rm(m,c.r[3],32));
            pc = 0x57527d; continue;
            }
            0x57525c => {
            c.set(3,32,c.r[0].wrapping_add(0xffffff80));
            { let a = c.get(3,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x57526d; continue; }
            pc = 0x575264; continue;
            }
            0x575264 => {
            c.set(0,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x675f58),32));
            pc = 0x575279; continue;
            }
            0x57526d => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,32,0x676358u32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            pc = 0x575279; continue;
            }
            0x575279 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            pc = 0x57527d; continue;
            }
            0x57527d => {
            { let a = c.get(2,8); c.sub(a,0x80u32,8); }
            if c.cond(3) { pc = 0x57529e; continue; }
            pc = 0x575282; continue;
            }
            0x575282 => {
            { let a = c.get(2,8); c.sub(a,0x40u32,8); }
            if c.cond(7) { pc = 0x575290; continue; }
            pc = 0x575287; continue;
            }
            0x575287 => {
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x675f58),32));
            pc = 0x5752c1; continue;
            }
            0x575290 => {
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,0x676158u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[2],32));
            pc = 0x5752c1; continue;
            }
            0x57529e => {
            c.set(0,32,c.r[1].wrapping_add(0xffffff80));
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x5752af; continue; }
            pc = 0x5752a6; continue;
            }
            0x5752a6 => {
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x675f58),32));
            pc = 0x5752bf; continue;
            }
            0x5752af => {
            c.set(0,32,c.r[1].wrapping_mul(4));
            c.set(1,32,0x676358u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            pc = 0x5752bf; continue;
            }
            0x5752bf => {
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            pc = 0x5752c1; continue;
            }
            0x5752c1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x1f4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(0,32));
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x1f4u32,32); c.set(1,32,r); }
            c.set(0,32,0x10624dd3u32);
            { let a = c.r[7].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            pc = 0x575328; continue;
            }
            0x57530b => {
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xc8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x575328; continue; }
            pc = 0x575325; continue;
            }
            0x575325 => {
            { let a = c.r[6].wrapping_add(0x5c); wm(m,a,32,c.get(1,32)); }
            pc = 0x575328; continue;
            }
            0x575328 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2e),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); c.sub(a,0x10000u32,32); }
            if c.cond(13) { pc = 0x5753f7; continue; }
            pc = 0x57533d; continue;
            }
            0x57533d => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,8,0x3u32);
            { let a = c.get(0,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,8); let r = c.sub(a,c.get(0,8),8); c.set(1,8,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x64),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(1,8)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5753b6; continue; }
            pc = 0x575356; continue;
            }
            0x575356 => {
            c.set(2,32,rm(m,0x6e2e6cu32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x2c),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x575369; continue; }
            pc = 0x575365; continue;
            }
            0x575365 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x575372; continue;
            }
            0x575369 => {
            c.set(2,32,rm(m,0x6e2e70u32,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            pc = 0x575372; continue;
            }
            0x575372 => {
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x575399; continue; }
            pc = 0x575381; continue;
            }
            0x575381 => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.sub(a,0x16u32,32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x575397); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x5753b6; continue;
            }
            0x575399 => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[2].wrapping_add(0x3f8));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x3f8),32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x5753af); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x64); wm(m,a,32,0x0u32); }
            pc = 0x5753b6; continue;
            }
            0x5753b6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x64),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x57541b; continue; }
            pc = 0x5753bd; continue;
            }
            0x5753bd => {
            c.set(1,8,rm(m,0x677929u32,8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(7,32); let r = c.add(a,0x1600b0u32,32); c.set(7,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.push(m,0x12u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5753ea);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.r[6].wrapping_add(0x64); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x5cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5753f7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x64),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x57541b; continue; }
            pc = 0x5753fe; continue;
            }
            0x5753fe => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[2].wrapping_add(0x3f8));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x3f8),32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x575414); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x64); wm(m,a,32,0x0u32); }
            pc = 0x57541b; continue;
            }
            0x57541b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x5cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005758e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5758e0;
    loop {
        match pc {
            0x5758e0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x35),8));
            c.set(1,32,rm(m,0x6e2e14u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5758f2; continue; }
            pc = 0x5758ef; continue;
            }
            0x5758ef => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5758f2 => {
            c.set(1,32,rm(m,0x6e2e18u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00577fb0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x577fb0;
    loop {
        match pc {
            0x577fb0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x38u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,0x6df270u32,32));
            c.set(3,8,rm(m,c.r[7].wrapping_add(0x6a),8));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(5,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x578f8d; continue; }
            pc = 0x577fe4; continue;
            }
            0x577fe4 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x78),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x577ff5; continue; }
            pc = 0x577feb; continue;
            }
            0x577feb => {
            { let a = rm(m,c.r[1].wrapping_add(0x6f),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x578f8d; continue; }
            pc = 0x577ff5; continue;
            }
            0x577ff5 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x3a),16));
            c.set(6,32,rm(m,0x6e2e60u32,32));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x578016; continue; }
            pc = 0x578004; continue;
            }
            0x578004 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x90),8));
            { let a = rm(m,c.r[2].wrapping_add(c.r[6]),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x578f8d; continue; }
            pc = 0x578016; continue;
            }
            0x578016 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x7fffffffu32); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x90),8));
            { let a = rm(m,c.r[2].wrapping_add(c.r[6]),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x5784a7; continue; }
            pc = 0x578030; continue;
            }
            0x578030 => {
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x578f8d; continue; }
            pc = 0x578039; continue;
            }
            0x578039 => {
            { let a = rm(m,c.r[1].wrapping_add(0x3c),16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x578f8d; continue; }
            pc = 0x578043; continue;
            }
            0x578043 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x6f),8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x57818b; continue; }
            pc = 0x57804e; continue;
            }
            0x57804e => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x258),32));
            c.set(1,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x578071);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.set(0,8,rm(m,c.r[6],8));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x57809e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3),8));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0x672f84),8),8)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x672f64),8),8)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x578f8d; continue; }
            pc = 0x5780bb; continue;
            }
            0x5780bb => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x672f74),8),8)&0xffffffffu32);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x5780da; continue;
            }
            0x5780da => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x578161; continue; }
            pc = 0x5780e2; continue;
            }
            0x5780e2 => {
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(2,32),32); c.set(5,32,r); }
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x578113; continue;
            }
            0x578113 => {
            c.set(0,32,0x2e8ba2e9u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(3,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x57814d; continue; }
            pc = 0x57813a; continue;
            }
            0x57813a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x4u32); }
            c.set(6,32,c.get(5,32));
            pc = 0x57814d; continue;
            }
            0x57814d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(5,32); let r = c.add(a,0x11u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,0x176u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x578113; continue; }
            pc = 0x578161; continue;
            }
            0x578161 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(3,32); let r = c.sub(a,0x11u32,32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0xbu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5780da; continue; }
            pc = 0x578186; continue;
            }
            0x578186 => {
            pc = 0x5787e4; continue;
            }
            0x57818b => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x1004),16));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x578254; continue; }
            pc = 0x5781a8; continue;
            }
            0x5781a8 => {
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[3].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            pc = 0x5781ba; continue;
            }
            0x5781ba => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6df270u32,32));
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14),32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x57822b; continue; }
            pc = 0x5781da; continue;
            }
            0x5781da => {
            c.set(6,32,sx(rm(m,c.r[5].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(2,32,c.get(6,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x57822b; continue; }
            pc = 0x57821e; continue;
            }
            0x57821e => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x57822b; continue;
            }
            0x57822b => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,16,c.get(0,8));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x3c),16),16); }
            if c.cond(2) { pc = 0x5781ba; continue; }
            pc = 0x578244; continue;
            }
            0x578244 => {
            { let a = rm(m,c.r[4].wrapping_add(0x18),32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(5) { pc = 0x5787e4; continue; }
            pc = 0x578252; continue;
            }
            0x578252 => {
            pc = 0x57825c; continue;
            }
            0x578254 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x57825c; continue;
            }
            0x57825c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            { let a = c.get(2,32); let r = c.add(a,0x1006u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            pc = 0x578277; continue;
            }
            0x578277 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.set(0,16,rm(m,c.r[0],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x57837d; continue; }
            pc = 0x578290; continue;
            }
            0x578290 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x5782e5; continue; }
            pc = 0x578297; continue;
            }
            0x578297 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x5782c2; continue; }
            pc = 0x57829a; continue;
            }
            0x57829a => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x578308; continue; }
            pc = 0x57829d; continue;
            }
            0x57829d => {
            c.set(2,32,c.get(3,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[3].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6df270u32,32));
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc08),32));
            pc = 0x578308; continue;
            }
            0x5782c2 => {
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x80c),32));
            pc = 0x578308; continue;
            }
            0x5782e5 => {
            c.set(2,32,c.get(3,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[3].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6df270u32,32));
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x410),32));
            pc = 0x578308; continue;
            }
            0x578308 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x578363; continue; }
            pc = 0x57830f; continue;
            }
            0x57830f => {
            c.set(6,32,sx(rm(m,c.r[5].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(2,32,c.get(6,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x578363; continue; }
            pc = 0x578353; continue;
            }
            0x578353 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,c.get(0,8)); }
            pc = 0x578363; continue;
            }
            0x578363 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x40),16),16); }
            if c.cond(2) { pc = 0x578290; continue; }
            pc = 0x57837d; continue;
            }
            0x57837d => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            if c.cond(2) { pc = 0x578277; continue; }
            pc = 0x5783a3; continue;
            }
            0x5783a3 => {
            { let a = rm(m,c.r[4].wrapping_add(0x18),32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(5) { pc = 0x5787e4; continue; }
            pc = 0x5783b1; continue;
            }
            0x5783b1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = rm(m,c.r[0].wrapping_add(0x242),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x578f8d; continue; }
            pc = 0x5783c3; continue;
            }
            0x5783c3 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x6c),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x250),32));
            c.set(1,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5783f0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x57841b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x578435);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x57843d);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x57845a; continue; }
            pc = 0x578444; continue;
            }
            0x578444 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x578450; continue; }
            pc = 0x578448; continue;
            }
            0x578448 => {
            { let a = c.get(6,32); let r = c.add(a,0x11u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,0x11u32,32); c.set(3,32,r); }
            pc = 0x578462; continue;
            }
            0x578450 => {
            if c.cond(14) { pc = 0x578455; continue; }
            pc = 0x578452; continue;
            }
            0x578452 => {
            { let a = c.get(6,32); let r = c.sub(a,0x11u32,32); c.set(6,32,r); }
            pc = 0x578455; continue;
            }
            0x578455 => {
            { let a = c.get(3,32); let r = c.sub(a,0x11u32,32); c.set(3,32,r); }
            pc = 0x578462; continue;
            }
            0x57845a => {
            { let a = c.get(3,32); c.sub(a,0xbu32,32); }
            if c.cond(14) { pc = 0x578462; continue; }
            pc = 0x57845f; continue;
            }
            0x57845f => {
            { let a = c.get(3,32); let r = c.sub(a,0x16u32,32); c.set(3,32,r); }
            pc = 0x578462; continue;
            }
            0x578462 => {
            c.set(1,32,c.get(3,32));
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(2,32,c.get(6,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(13) { pc = 0x578f8d; continue; }
            pc = 0x578495; continue;
            }
            0x578495 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x4u32); }
            pc = 0x5787e4; continue;
            }
            0x5784a7 => {
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x18),16));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            { let a = c.get(1,16); c.and(a,c.get(1,16),16); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            if c.cond(6) { pc = 0x578553; continue; }
            pc = 0x5784c5; continue;
            }
            0x5784c5 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x5784c9; continue;
            }
            0x5784c9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x7c),32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x57852c; continue; }
            pc = 0x5784e1; continue;
            }
            0x5784e1 => {
            c.set(6,32,sx(rm(m,c.r[5].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(2,32,c.get(6,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x57852c; continue; }
            pc = 0x578521; continue;
            }
            0x578521 => {
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x57852c; continue;
            }
            0x57852c => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,16,c.get(0,8));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x40),16),16); }
            if c.cond(2) { pc = 0x5784c9; continue; }
            pc = 0x578541; continue;
            }
            0x578541 => {
            { let a = c.get(3,32); c.sub(a,0x7fffffffu32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            if c.cond(5) { pc = 0x5787e4; continue; }
            pc = 0x578551; continue;
            }
            0x578551 => {
            pc = 0x578557; continue;
            }
            0x578553 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x578557; continue;
            }
            0x578557 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x1u32); }
            c.set(0,32,c.r[2].wrapping_add(0x1a));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            pc = 0x578573; continue;
            }
            0x578573 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.set(0,16,rm(m,c.r[0],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x578640; continue; }
            pc = 0x57858c; continue;
            }
            0x57858c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x5785bd; continue; }
            pc = 0x578593; continue;
            }
            0x578593 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x5785ab; continue; }
            pc = 0x578596; continue;
            }
            0x578596 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x5785cd; continue; }
            pc = 0x578599; continue;
            }
            0x578599 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x1bc),32));
            pc = 0x5785cd; continue;
            }
            0x5785ab => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x174),32));
            pc = 0x5785cd; continue;
            }
            0x5785bd => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x11c),32));
            pc = 0x5785cd; continue;
            }
            0x5785cd => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x578622; continue; }
            pc = 0x5785d4; continue;
            }
            0x5785d4 => {
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(6,32,sx(rm(m,c.r[5].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(2,32,c.get(6,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x578622; continue; }
            pc = 0x578614; continue;
            }
            0x578614 => {
            c.set(3,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,c.get(0,8)); }
            pc = 0x578622; continue;
            }
            0x578622 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x40),16),16); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            if c.cond(2) { pc = 0x57858c; continue; }
            pc = 0x578640; continue;
            }
            0x578640 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(2) { pc = 0x578573; continue; }
            pc = 0x57866a; continue;
            }
            0x57866a => {
            { let a = c.get(3,32); c.sub(a,0x7fffffffu32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            if c.cond(5) { pc = 0x5787e4; continue; }
            pc = 0x57867a; continue;
            }
            0x57867a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x578),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,16,c.get(0,8));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(0x20),16),16); }
            if c.cond(3) { pc = 0x57873b; continue; }
            pc = 0x578696; continue;
            }
            0x578696 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1f8),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x2c),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x57873b; continue; }
            pc = 0x5786b6; continue;
            }
            0x5786b6 => {
            c.set(3,32,sx(rm(m,c.r[0].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(6,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,0xbu32,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x5786d3);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x5786db);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x5786f2; continue; }
            pc = 0x5786e2; continue;
            }
            0x5786e2 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x5786eb; continue; }
            pc = 0x5786e6; continue;
            }
            0x5786e6 => {
            { let a = c.get(6,32); let r = c.add(a,0x11u32,32); c.set(6,32,r); }
            pc = 0x5786fe; continue;
            }
            0x5786eb => {
            if c.cond(14) { pc = 0x5786fe; continue; }
            pc = 0x5786ed; continue;
            }
            0x5786ed => {
            { let a = c.get(6,32); let r = c.sub(a,0x11u32,32); c.set(6,32,r); }
            pc = 0x5786fe; continue;
            }
            0x5786f2 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x5786fb; continue; }
            pc = 0x5786f6; continue;
            }
            0x5786f6 => {
            { let a = c.get(3,32); let r = c.sub(a,0xbu32,32); c.set(3,32,r); }
            pc = 0x5786fe; continue;
            }
            0x5786fb => {
            { let a = c.get(3,32); let r = c.add(a,0xbu32,32); c.set(3,32,r); }
            pc = 0x5786fe; continue;
            }
            0x5786fe => {
            c.set(1,32,c.get(3,32));
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(2,32,c.get(6,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(13) { pc = 0x578737; continue; }
            pc = 0x57872d; continue;
            }
            0x57872d => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x4u32); }
            pc = 0x5787d8; continue;
            }
            0x578737 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x57873b; continue;
            }
            0x57873b => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x574),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x578f8d; continue; }
            pc = 0x57874d; continue;
            }
            0x57874d => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x2c),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x578f8d; continue; }
            pc = 0x578758; continue;
            }
            0x578758 => {
            c.set(3,32,sx(rm(m,c.r[0].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(6,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,0xbu32,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x578775);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x57877d);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x578794; continue; }
            pc = 0x578784; continue;
            }
            0x578784 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x57878d; continue; }
            pc = 0x578788; continue;
            }
            0x578788 => {
            { let a = c.get(6,32); let r = c.add(a,0x11u32,32); c.set(6,32,r); }
            pc = 0x5787a0; continue;
            }
            0x57878d => {
            if c.cond(14) { pc = 0x5787a0; continue; }
            pc = 0x57878f; continue;
            }
            0x57878f => {
            { let a = c.get(6,32); let r = c.sub(a,0x11u32,32); c.set(6,32,r); }
            pc = 0x5787a0; continue;
            }
            0x578794 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x57879d; continue; }
            pc = 0x578798; continue;
            }
            0x578798 => {
            { let a = c.get(3,32); let r = c.sub(a,0xbu32,32); c.set(3,32,r); }
            pc = 0x5787a0; continue;
            }
            0x57879d => {
            { let a = c.get(3,32); let r = c.add(a,0xbu32,32); c.set(3,32,r); }
            pc = 0x5787a0; continue;
            }
            0x5787a0 => {
            c.set(1,32,c.get(3,32));
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(2,32,c.get(6,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(13) { pc = 0x578f8d; continue; }
            pc = 0x5787d3; continue;
            }
            0x5787d3 => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x5u32); }
            pc = 0x5787d8; continue;
            }
            0x5787d8 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x5787e4; continue;
            }
            0x5787e4 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(5,32); c.sub(a,0x70e40u32,32); }
            if c.cond(15) { pc = 0x578f8d; continue; }
            pc = 0x5787f4; continue;
            }
            0x5787f4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = rm(m,c.r[2].wrapping_add(0x6f),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x57880a; continue; }
            pc = 0x5787fe; continue;
            }
            0x5787fe => {
            { let a = c.get(5,32); c.sub(a,0x1210u32,32); }
            if c.cond(15) { pc = 0x578f8d; continue; }
            pc = 0x57880a; continue;
            }
            0x57880a => {
            { let a = rm(m,c.r[4].wrapping_add(0x12),8); c.sub(a,0x4u32,8); }
            if c.cond(2) { pc = 0x578817; continue; }
            pc = 0x578811; continue;
            }
            0x578811 => {
            { let a = rm(m,c.r[7].wrapping_add(0x7b),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x578862; continue; }
            pc = 0x578817; continue;
            }
            0x578817 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x578824; continue; }
            pc = 0x57881f; continue;
            }
            0x57881f => {
            c.set(1,32,c.r[0].wrapping_add(0x8));
            pc = 0x578826; continue;
            }
            0x578824 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x578826; continue;
            }
            0x578826 => {
            c.set(6,32,sx(rm(m,c.r[1].wrapping_add(0x6),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x578839; continue; }
            pc = 0x578834; continue;
            }
            0x578834 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x57883b; continue;
            }
            0x578839 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x57883b; continue;
            }
            0x57883b => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            pc = 0x578862; continue;
            }
            0x578862 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(3,32,c.get(6,32));
            if c.cond(15) { pc = 0x57886a; continue; }
            pc = 0x578868; continue;
            }
            0x578868 => {
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            pc = 0x57886a; continue;
            }
            0x57886a => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.get(1,32));
            if c.cond(15) { pc = 0x578876; continue; }
            pc = 0x578874; continue;
            }
            0x578874 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x578876; continue;
            }
            0x578876 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x5788b3; continue; }
            pc = 0x57889f; continue;
            }
            0x57889f => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x5788ab; continue; }
            pc = 0x5788a3; continue;
            }
            0x5788a3 => {
            c.set(3,8,rm(m,0x67792au32,8));
            pc = 0x578910; continue;
            }
            0x5788ab => {
            c.set(3,8,rm(m,0x67792eu32,8));
            pc = 0x578910; continue;
            }
            0x5788b3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(3,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x5788e8; continue; }
            pc = 0x5788d4; continue;
            }
            0x5788d4 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x5788e0; continue; }
            pc = 0x5788d8; continue;
            }
            0x5788d8 => {
            c.set(3,8,rm(m,0x67792cu32,8));
            pc = 0x578910; continue;
            }
            0x5788e0 => {
            c.set(3,8,rm(m,0x677928u32,8));
            pc = 0x578910; continue;
            }
            0x5788e8 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x578900; continue; }
            pc = 0x5788ec; continue;
            }
            0x5788ec => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x5788f8; continue; }
            pc = 0x5788f0; continue;
            }
            0x5788f0 => {
            c.set(3,8,rm(m,0x67792bu32,8));
            pc = 0x578910; continue;
            }
            0x5788f8 => {
            c.set(3,8,rm(m,0x67792du32,8));
            pc = 0x578910; continue;
            }
            0x578900 => {
            c.set(3,8,rm(m,0x677929u32,8));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(15) { pc = 0x578910; continue; }
            pc = 0x57890a; continue;
            }
            0x57890a => {
            c.set(3,8,rm(m,0x67792fu32,8));
            pc = 0x578910; continue;
            }
            0x578910 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x79),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x578be3; continue; }
            pc = 0x578923; continue;
            }
            0x578923 => {
            c.set(0,32,rm(m,0x6e2e60u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x90),8));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x578be3; continue; }
            pc = 0x57893a; continue;
            }
            0x57893a => {
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x578954);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6a),8));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(6,32)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x7a4));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[0].wrapping_add(0x68),32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[6]),8); c.sub(a,0x0u32,8); }
            if c.cond(9) { pc = 0x578f8d; continue; }
            pc = 0x578990; continue;
            }
            0x578990 => {
            { let a = c.get(3,8); let r = c.add(a,0x80u32,8); c.set(3,8,r); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5789b1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6a),8));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x7a4));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0x68),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x578a2c; continue; }
            pc = 0x5789e8; continue;
            }
            0x5789e8 => {
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5789f3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x578a2c; continue; }
            pc = 0x5789f7; continue;
            }
            0x5789f7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x578a0d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x561330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x578a2c; continue; }
            pc = 0x578a11; continue;
            }
            0x578a11 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x578a22);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,0x7fffu32,16); }
            if c.cond(2) { pc = 0x578b88; continue; }
            pc = 0x578a2c; continue;
            }
            0x578a2c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(3,8); let r = c.add(a,0x20u32,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x578a5d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6a),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x7a4));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0x68),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x578ad4; continue; }
            pc = 0x578a90; continue;
            }
            0x578a90 => {
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x578a9b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x578ad4; continue; }
            pc = 0x578a9f; continue;
            }
            0x578a9f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x578ab5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x561330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x578ad4; continue; }
            pc = 0x578ab9; continue;
            }
            0x578ab9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x578aca);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,0x7fffu32,16); }
            if c.cond(2) { pc = 0x578b88; continue; }
            pc = 0x578ad4; continue;
            }
            0x578ad4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(3,8); let r = c.add(a,0xc0u32,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x578b05);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6a),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x7a4));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0x68),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x578f8d; continue; }
            pc = 0x578b3c; continue;
            }
            0x578b3c => {
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x578b47);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x578f8d; continue; }
            pc = 0x578b4f; continue;
            }
            0x578b4f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x578b65);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x561330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x578f8d; continue; }
            pc = 0x578b6d; continue;
            }
            0x578b6d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x578b7e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,0x7fffu32,16); }
            if c.cond(3) { pc = 0x578f8d; continue; }
            pc = 0x578b88; continue;
            }
            0x578b88 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,0x2u32);
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x3c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x578ba6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e2e58u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x90),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x578bca; continue; }
            pc = 0x578bb8; continue;
            }
            0x578bb8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x578f4d; continue;
            }
            0x578bca => {
            c.set(2,32,rm(m,0x6e2e5cu32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x578f4d; continue;
            }
            0x578be3 => {
            { let a = c.get(5,32); c.sub(a,0x484u32,32); }
            if c.cond(15) { pc = 0x578d43; continue; }
            pc = 0x578bef; continue;
            }
            0x578bef => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x90),8));
            { let a = c.r[7].wrapping_add(0x74); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6e2e60u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[1]),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x578c15; continue; }
            pc = 0x578c0a; continue;
            }
            0x578c0a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(0,8); let r = c.add(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.r[7].wrapping_add(0x78); wm(m,a,8,c.get(0,8)); }
            pc = 0x578c1c; continue;
            }
            0x578c15 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.r[7].wrapping_add(0x78); wm(m,a,8,c.get(2,8)); }
            pc = 0x578c1c; continue;
            }
            0x578c1c => {
            { let a = c.r[7].wrapping_add(0x3c); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xee8eu32);
            c.push(m, 0x578c2f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x578c69; continue; }
            pc = 0x578c33; continue;
            }
            0x578c33 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xee8eu32);
            c.push(m, 0x578c43);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            c.push(m, 0x578c4a);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,0x1172u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x578c69);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x595b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x578c69; continue;
            }
            0x578c69 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = rm(m,c.r[2].wrapping_add(0x6f),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x578d37; continue; }
            pc = 0x578c77; continue;
            }
            0x578c77 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3c),8));
            { let a = c.get(0,32); c.sub(a,0xe0u32,32); }
            if c.cond(7) { pc = 0x578d0a; continue; }
            pc = 0x578c87; continue;
            }
            0x578c87 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x578fc0),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x578f9c)); continue;
            }
            0x578c96 => {
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,0x16u32,32); c.set(0,32,r); }
            pc = 0x578d12; continue;
            }
            0x578ca3 => {
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,0x11u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0xbu32,32); c.set(0,32,r); }
            pc = 0x578d12; continue;
            }
            0x578cb3 => {
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,0x22u32,32); c.set(1,32,r); }
            pc = 0x578d12; continue;
            }
            0x578cc0 => {
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,0x11u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xbu32,32); c.set(0,32,r); }
            pc = 0x578d12; continue;
            }
            0x578cd0 => {
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,0x16u32,32); c.set(0,32,r); }
            pc = 0x578d12; continue;
            }
            0x578cdd => {
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,0x11u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xbu32,32); c.set(0,32,r); }
            pc = 0x578d12; continue;
            }
            0x578ced => {
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,0x22u32,32); c.set(1,32,r); }
            pc = 0x578d12; continue;
            }
            0x578cfa => {
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,0x11u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0xbu32,32); c.set(0,32,r); }
            pc = 0x578d12; continue;
            }
            0x578d0a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            pc = 0x578d12; continue;
            }
            0x578d12 => {
            c.set(2,8,rm(m,0x677929u32,8));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,0x80020u32);
            c.push(m,0x12u32);
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x578d37);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            pc = 0x578d37; continue;
            }
            0x578d37 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x578d43 => {
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x578d5d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6a),8));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x7a4));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[0].wrapping_add(0x68),32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[6]),8); c.sub(a,0x0u32,8); }
            if c.cond(8) { pc = 0x578f8d; continue; }
            pc = 0x578d99; continue;
            }
            0x578d99 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x578db3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x578dca);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x578e03; continue; }
            pc = 0x578dce; continue;
            }
            0x578dce => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x578de4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x561330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x578e03; continue; }
            pc = 0x578de8; continue;
            }
            0x578de8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x578df9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,0x7fffu32,16); }
            if c.cond(2) { pc = 0x578f05; continue; }
            pc = 0x578e03; continue;
            }
            0x578e03 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(3,8); let r = c.add(a,0x20u32,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x578e34);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x578e47);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x578e80; continue; }
            pc = 0x578e4b; continue;
            }
            0x578e4b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x578e61);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x561330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x578e80; continue; }
            pc = 0x578e65; continue;
            }
            0x578e65 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x578e76);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,0x7fffu32,16); }
            if c.cond(2) { pc = 0x578f05; continue; }
            pc = 0x578e80; continue;
            }
            0x578e80 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(3,8); let r = c.add(a,0xc0u32,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x578eb1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x578ec4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x578f8d; continue; }
            pc = 0x578ecc; continue;
            }
            0x578ecc => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x578ee2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x561330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x578f8d; continue; }
            pc = 0x578eea; continue;
            }
            0x578eea => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x578efb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,0x7fffu32,16); }
            if c.cond(3) { pc = 0x578f8d; continue; }
            pc = 0x578f05; continue;
            }
            0x578f05 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,0x2u32);
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x3c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x578f23);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e2e58u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x90),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x578f39; continue; }
            pc = 0x578f35; continue;
            }
            0x578f35 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x578f42; continue;
            }
            0x578f39 => {
            c.set(1,32,rm(m,0x6e2e5cu32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x578f42; continue;
            }
            0x578f42 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x578f4d; continue;
            }
            0x578f4d => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x578f54);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x560750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x578f6e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x578f81);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x2u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x578f8d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00571350(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x571350;
    loop {
        match pc {
            0x571350 => {
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6df270u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(7,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x8),8));
            c.set(5,32,c.r[1].wrapping_add(0xfffffffb));
            { let a = c.get(5,32); c.sub(a,0xf1u32,32); }
            if c.cond(7) { pc = 0x5716ff; continue; }
            pc = 0x571388; continue;
            }
            0x571388 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x571894),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x57186c)); continue;
            }
            0x571397 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x5713a4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5713ad);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.r[1].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.r[2].wrapping_add(0x68); wm(m,a,16,0x1u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5713c9; continue; }
            pc = 0x5713c4; continue;
            }
            0x5713c4 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x5713cb; continue;
            }
            0x5713c9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5713cb; continue;
            }
            0x5713cb => {
            c.set(1,8,rm(m,0x67792du32,8));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5713e0; continue; }
            pc = 0x5713db; continue;
            }
            0x5713db => {
            c.set(7,32,c.r[1].wrapping_add(0x8));
            pc = 0x5713e2; continue;
            }
            0x5713e0 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x5713e2; continue;
            }
            0x5713e2 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0xb1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x5713ec); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x2cu32,32); c.set(1,32,r); }
            c.push(m, 0x5713f8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[7].wrapping_add(0xd); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x571408); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x571411);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xffffffe8),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xffffffe4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xffffffe0),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,0x82u32);
            c.push(m,0x12u32);
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x571437);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.push(m,0xffffffc1u32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(0,32));
            c.push(m, 0x571441);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x57af40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x571441; continue;
            }
            0x571441 => {
            c.push(m,0x18u32);
            pc = 0x5716f8; continue;
            }
            0x571448 => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x571496; continue; }
            pc = 0x57144c; continue;
            }
            0x57144c => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xffffffe6),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xffffffe2),16),16)&0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x571466);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x571479);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x571479; continue;
            }
            0x571479 => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x571496; continue; }
            pc = 0x57147d; continue;
            }
            0x57147d => {
            { let a = rm(m,c.r[7].wrapping_add(0x78),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x571496; continue; }
            pc = 0x571482; continue;
            }
            0x571482 => {
            { let a = rm(m,c.r[6].wrapping_add(0x5c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x571441; continue; }
            pc = 0x571487; continue;
            }
            0x571487 => {
            c.set(1,32,c.r[6].wrapping_add(0xffffffd4));
            c.push(m, 0x57148f);
            { let args = []; c.r[0] = host.call(m, 0x571da0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x18u32);
            pc = 0x5716f8; continue;
            }
            0x571496 => {
            { let a = rm(m,c.r[7].wrapping_add(0x79),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x5714a5; continue; }
            pc = 0x57149b; continue;
            }
            0x57149b => {
            c.push(m,0xf6u32);
            pc = 0x5716f8; continue;
            }
            0x5714a5 => {
            { let a = rm(m,c.r[7].wrapping_add(0x3a),16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x571441; continue; }
            pc = 0x5714ab; continue;
            }
            0x5714ab => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xffffffe8),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xffffffe4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xffffffe0),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m,0x81u32);
            c.push(m,0x12u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x5714cb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.push(m,0xffffff80u32);
            c.push(m,0xea60u32);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5714d9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x57af40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x97u32);
            pc = 0x5716f8; continue;
            }
            0x5714e3 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x64),8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x571578; continue; }
            pc = 0x5714fc; continue;
            }
            0x5714fc => {
            { let a = rm(m,c.r[1].wrapping_add(0x5a),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x571578; continue; }
            pc = 0x571501; continue;
            }
            0x571501 => {
            { let a = rm(m,c.r[1].wrapping_add(0x58),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x571578; continue; }
            pc = 0x571506; continue;
            }
            0x571506 => {
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x57150b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(3,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,0x6df270u32,32));
            c.set(5,16,rm(m,c.r[2].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x1004),16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x2c),16));
            { let a = c.get(5,32); let r = c.sub(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x232),16));
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            if c.cond(13) { pc = 0x571576; continue; }
            pc = 0x571552; continue;
            }
            0x571552 => {
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x64),8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),32));
            { let a = c.r[2].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x2c),16); let r = c.inc(a,16); { let a = c.r[7].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x2c); wm(m,a,16,r); } }
            c.set(2,32,rm(m,0x6df270u32,32));
            pc = 0x571576; continue;
            }
            0x571576 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x571578; continue;
            }
            0x571578 => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x57161f; continue; }
            pc = 0x571580; continue;
            }
            0x571580 => {
            { let a = rm(m,c.r[7].wrapping_add(0x78),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x57161f; continue; }
            pc = 0x571589; continue;
            }
            0x571589 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5715ee; continue; }
            pc = 0x5715a0; continue;
            }
            0x5715a0 => {
            { let a = rm(m,c.r[1].wrapping_add(0x78),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x5715ee; continue; }
            pc = 0x5715a5; continue;
            }
            0x5715a5 => {
            c.set(7,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x64),8));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5715ee; continue; }
            pc = 0x5715be; continue;
            }
            0x5715be => {
            { let a = rm(m,c.r[0].wrapping_add(0x58),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x5715ee; continue; }
            pc = 0x5715c3; continue;
            }
            0x5715c3 => {
            { let a = rm(m,c.r[0].wrapping_add(0x5a),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x5715ee; continue; }
            pc = 0x5715c8; continue;
            }
            0x5715c8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x64),8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),32));
            { let a = rm(m,c.r[0].wrapping_add(0x5a),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x5715ee; continue; }
            pc = 0x5715e7; continue;
            }
            0x5715e7 => {
            c.push(m,0xfu32);
            pc = 0x5716f8; continue;
            }
            0x5715ee => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x64),8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            c.set(1,32,c.get(6,32));
            if c.cond(4) { pc = 0x571615; continue; }
            pc = 0x57160e; continue;
            }
            0x57160e => {
            c.push(m,0x19u32);
            pc = 0x5716fa; continue;
            }
            0x571615 => {
            c.push(m,0xf5u32);
            pc = 0x5716fa; continue;
            }
            0x57161f => {
            c.push(m,0xf5u32);
            pc = 0x5716f8; continue;
            }
            0x571629 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x571636); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57163f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x57164b; continue; }
            pc = 0x571646; continue;
            }
            0x571646 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x57164d; continue;
            }
            0x57164b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x57164d; continue;
            }
            0x57164d => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xffffffe8),8));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x57165f; continue; }
            pc = 0x57165a; continue;
            }
            0x57165a => {
            c.set(5,32,c.r[1].wrapping_add(0x8));
            pc = 0x571661; continue;
            }
            0x57165f => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x571661; continue;
            }
            0x571661 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x2u32);
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x571668); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x2cu32,32); c.set(1,32,r); }
            c.push(m, 0x571674);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[5].wrapping_add(0xd); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x571684); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57168d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[0].wrapping_add(0x8c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x5716a8; continue; }
            pc = 0x57169f; continue;
            }
            0x57169f => {
            { let a = rm(m,c.r[7].wrapping_add(0x78),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x571441; continue; }
            pc = 0x5716a8; continue;
            }
            0x5716a8 => {
            { let a = rm(m,c.r[7].wrapping_add(0x79),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x5716b4; continue; }
            pc = 0x5716ad; continue;
            }
            0x5716ad => {
            c.push(m,0xf6u32);
            pc = 0x5716f8; continue;
            }
            0x5716b4 => {
            { let a = rm(m,c.r[7].wrapping_add(0x3a),16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x571441; continue; }
            pc = 0x5716be; continue;
            }
            0x5716be => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xffffffe8),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xffffffe4),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xffffffe0),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x81u32);
            c.push(m,0x12u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5716de);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.push(m,0xffffff80u32);
            c.push(m,0xea60u32);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5716ec);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x57af40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x97u32);
            pc = 0x5716f8; continue;
            }
            0x5716f3 => {
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            pc = 0x5716f6; continue;
            }
            0x5716f6 => {
            c.push(m,0x5u32);
            pc = 0x5716f8; continue;
            }
            0x5716f8 => {
            c.set(1,32,c.get(6,32));
            pc = 0x5716fa; continue;
            }
            0x5716fa => {
            c.push(m, 0x5716ff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5716ff; continue;
            }
            0x5716ff => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x57170b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x55ffc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x9),8));
            { let a = c.get(0,32); let r = c.add(a,0xfffffffeu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xbu32,32); }
            if c.cond(7) { pc = 0x571858; continue; }
            pc = 0x57171c; continue;
            }
            0x57171c => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x571990),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x571988)); continue;
            }
            0x57172b => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(7,32,c.r[6].wrapping_add(0xffffffd4));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x57173f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xffffffe8),8));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(1,8); let r = c.add(a,0x80u32,8); c.set(1,8,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0xffffffe8); wm(m,a,8,c.get(1,8)); }
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x571755);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0xdu32);
            pc = 0x5716f8; continue;
            }
            0x571759 => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5716ff; continue; }
            pc = 0x57175d; continue;
            }
            0x57175d => {
            { let a = rm(m,c.r[7].wrapping_add(0x78),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x5716ff; continue; }
            pc = 0x571762; continue;
            }
            0x571762 => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xffffffe6),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xffffffe2),16),16)&0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57177c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x64),8));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,0x6df270u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x5ca),16));
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5716ff; continue; }
            pc = 0x5717b0; continue;
            }
            0x5717b0 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x5ce),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5716ff; continue; }
            pc = 0x5717c6; continue;
            }
            0x5717c6 => {
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x5d2),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xffffffe8),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x5716f6; continue; }
            pc = 0x5717da; continue;
            }
            0x5717da => {
            c.push(m,0x96u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5717e6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x64),8));
            c.set(1,32,c.get(0,32));
            c.push(m,c.get(3,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(0,32,0x2e8u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.push(m,c.get(0,32));
            c.set(0,32,0x2e6u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.push(m,c.get(2,32));
            c.push(m, 0x571822);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x62b860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xe),16));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xc),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57184a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x62b960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x571853);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x62dd70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5716ff; continue;
            }
            0x571858 => {
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0xe); wm(m,a,16,0x1u32); }
            pc = 0x571862; continue;
            }
            0x571862 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005719a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5719a0;
    loop {
        match pc {
            0x5719a0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6a),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x571d92; continue; }
            pc = 0x5719cd; continue;
            }
            0x5719cd => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x571d92; continue; }
            pc = 0x5719d5; continue;
            }
            0x5719d5 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x78),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x571d92; continue; }
            pc = 0x5719e0; continue;
            }
            0x5719e0 => {
            { let a = rm(m,c.r[6].wrapping_add(0x6f),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x571d92; continue; }
            pc = 0x5719ea; continue;
            }
            0x5719ea => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x90),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x5ca),16));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x5ce),16));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x571a28);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(0,32,c.get(5,32));
            c.set(2,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x64u32,32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x571d92; continue; }
            pc = 0x571a55; continue;
            }
            0x571a55 => {
            { let a = c.get(0,32); c.sub(a,0x70e40u32,32); }
            if c.cond(15) { pc = 0x571d92; continue; }
            pc = 0x571a60; continue;
            }
            0x571a60 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x90),8));
            c.set(1,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x5ca),16));
            { let a = rm(m,c.r[6].wrapping_add(0x10),16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x571a84; continue; }
            pc = 0x571a76; continue;
            }
            0x571a76 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x5ce),16),16); }
            if c.cond(4) { pc = 0x571ab3; continue; }
            pc = 0x571a84; continue;
            }
            0x571a84 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x5ce),16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571a97);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x62b860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xe),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xc),16));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571aaa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x62b960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571ab3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x62dd70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x571ab3; continue;
            }
            0x571ab3 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(0,32,c.get(3,32));
            if c.cond(15) { pc = 0x571abb; continue; }
            pc = 0x571ab9; continue;
            }
            0x571ab9 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x571abb; continue;
            }
            0x571abb => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(1,32,c.get(5,32));
            if c.cond(15) { pc = 0x571ac3; continue; }
            pc = 0x571ac1; continue;
            }
            0x571ac1 => {
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            pc = 0x571ac3; continue;
            }
            0x571ac3 => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,c.get(0,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x571af8; continue; }
            pc = 0x571ae4; continue;
            }
            0x571ae4 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x571af0; continue; }
            pc = 0x571ae8; continue;
            }
            0x571ae8 => {
            c.set(3,8,rm(m,0x67792au32,8));
            pc = 0x571b4e; continue;
            }
            0x571af0 => {
            c.set(3,8,rm(m,0x67792eu32,8));
            pc = 0x571b4e; continue;
            }
            0x571af8 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x571b26; continue; }
            pc = 0x571b12; continue;
            }
            0x571b12 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x571b1e; continue; }
            pc = 0x571b16; continue;
            }
            0x571b16 => {
            c.set(3,8,rm(m,0x67792cu32,8));
            pc = 0x571b4e; continue;
            }
            0x571b1e => {
            c.set(3,8,rm(m,0x677928u32,8));
            pc = 0x571b4e; continue;
            }
            0x571b26 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x571b3e; continue; }
            pc = 0x571b2a; continue;
            }
            0x571b2a => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x571b36; continue; }
            pc = 0x571b2e; continue;
            }
            0x571b2e => {
            c.set(3,8,rm(m,0x67792bu32,8));
            pc = 0x571b4e; continue;
            }
            0x571b36 => {
            c.set(3,8,rm(m,0x67792du32,8));
            pc = 0x571b4e; continue;
            }
            0x571b3e => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(3,8,rm(m,0x677929u32,8));
            if c.cond(15) { pc = 0x571b4e; continue; }
            pc = 0x571b48; continue;
            }
            0x571b48 => {
            c.set(3,8,rm(m,0x67792fu32,8));
            pc = 0x571b4e; continue;
            }
            0x571b4e => {
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x571b6c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571b7d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x571bab);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571bbe);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x571bf8; continue; }
            pc = 0x571bc2; continue;
            }
            0x571bc2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x571bd8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x561330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x571bf8; continue; }
            pc = 0x571bdc; continue;
            }
            0x571bdc => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571bed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),16),16); }
            if c.cond(6) { pc = 0x571d0a; continue; }
            pc = 0x571bf8; continue;
            }
            0x571bf8 => {
            { let a = rm(m,c.r[4].wrapping_add(0x1c),32); c.sub(a,0x484u32,32); }
            if c.cond(14) { pc = 0x571d92; continue; }
            pc = 0x571c06; continue;
            }
            0x571c06 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(3,8); let r = c.add(a,0x20u32,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x571c37);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571c4a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x571c84; continue; }
            pc = 0x571c4e; continue;
            }
            0x571c4e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x571c64);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x561330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x571c84; continue; }
            pc = 0x571c68; continue;
            }
            0x571c68 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571c79);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),16),16); }
            if c.cond(6) { pc = 0x571d0a; continue; }
            pc = 0x571c84; continue;
            }
            0x571c84 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(3,8); let r = c.add(a,0xc0u32,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x571cb5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571cc8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x571d92; continue; }
            pc = 0x571cd0; continue;
            }
            0x571cd0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x571ce6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x561330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x571d92; continue; }
            pc = 0x571cee; continue;
            }
            0x571cee => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571cff);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),16),16); }
            if c.cond(7) { pc = 0x571d92; continue; }
            pc = 0x571d0a; continue;
            }
            0x571d0a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x1u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x3c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x571d28);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e2dacu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x91),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x571d3e; continue; }
            pc = 0x571d3a; continue;
            }
            0x571d3a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x571d47; continue;
            }
            0x571d3e => {
            c.set(2,32,rm(m,0x6e2db0u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x571d47; continue;
            }
            0x571d47 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x571d59);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x560750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x571d73);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x571d86);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x2u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x571d92 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00574360(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x574360;
    loop {
        match pc {
            0x574360 => {
            c.set(2,32,rm(m,0x6e2e14u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x35),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x574380; continue; }
            pc = 0x57436f; continue;
            }
            0x57436f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57437c);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x574380 => {
            c.set(2,32,rm(m,0x6e2e18u32,32));
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x574395);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00575560(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x575560;
    loop {
        match pc {
            0x575560 => {
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,0x6e2e14u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x35),8));
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x575575; continue; }
            pc = 0x575571; continue;
            }
            0x575571 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x57557e; continue;
            }
            0x575575 => {
            c.set(2,32,rm(m,0x6e2e18u32,32));
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x57557e; continue;
            }
            0x57557e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x48),8));
            c.set(3,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[3].wrapping_add(0xc678),32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x41),8));
            { let a = c.get(6,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            c.set(3,32,rm(m,c.r[3].wrapping_add(0xc674),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[6]));
            c.set(3,32,rm(m,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x4),32));
            c.set(2,8,rm(m,c.r[7],8));
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(2,8); let r = c.sub(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[3].wrapping_add(c.r[0]),32));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.add(a,rm(m,c.r[7].wrapping_add(c.r[2]),32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let a = c.r[1].wrapping_add(0x44); wm(m,a,16,c.get(3,16)); }
            c.set(7,32,rm(m,0x6e51acu32,32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0xc678),32));
            c.set(7,32,rm(m,c.r[7].wrapping_add(0xc674),32));
            c.set(3,32,rm(m,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x4),32));
            c.set(6,32,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(c.r[2]).wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x16));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x44),16));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[4].wrapping_add(0xc),16),16); }
            { let a = c.r[1].wrapping_add(0x46); wm(m,a,16,c.get(0,16)); }
            if c.cond(3) { pc = 0x57569c; continue; }
            pc = 0x575613; continue;
            }
            0x575613 => {
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x10),16),16); }
            if c.cond(3) { pc = 0x575649; continue; }
            pc = 0x57561a; continue;
            }
            0x57561a => {
            c.set(0,8,rm(m,0x67792fu32,8));
            { let a = c.r[1].wrapping_add(0x42); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0xc56106ebu32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xdu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            pc = 0x575674; continue;
            }
            0x575649 => {
            c.set(0,8,rm(m,0x67792du32,8));
            { let a = c.r[1].wrapping_add(0x42); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0xc56106ebu32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xdu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            pc = 0x575674; continue;
            }
            0x575674 => {
            { let a = c.r[1].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            c.set(5,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(2,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0xd1b71759u32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x9u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x57569c => {
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x10),16),16); }
            if c.cond(3) { pc = 0x5756d2; continue; }
            pc = 0x5756a3; continue;
            }
            0x5756a3 => {
            c.set(0,8,rm(m,0x677929u32,8));
            { let a = c.r[1].wrapping_add(0x42); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0xc56106ebu32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xdu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            pc = 0x5756fd; continue;
            }
            0x5756d2 => {
            c.set(0,8,rm(m,0x67792bu32,8));
            { let a = c.r[1].wrapping_add(0x42); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0xc56106ebu32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xdu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            pc = 0x5756fd; continue;
            }
            0x5756fd => {
            { let a = c.r[1].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            c.set(5,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(2,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0xd1b71759u32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x9u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00575810(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x575810;
    loop {
        match pc {
            0x575810 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = rm(m,c.r[6].wrapping_add(0x8),8); c.sub(a,0xfu32,8); }
            if c.cond(5) { pc = 0x575873; continue; }
            pc = 0x575819; continue;
            }
            0x575819 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x29),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = rm(m,c.r[0].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(7) { pc = 0x57583f; continue; }
            pc = 0x575838; continue;
            }
            0x575838 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x78),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x575873; continue; }
            pc = 0x57583f; continue;
            }
            0x57583f => {
            c.push(m,0x2u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x575848);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x57586d; continue; }
            pc = 0x57584f; continue;
            }
            0x57584f => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x21),8));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xfffffff2),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xffffffee),16));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0xffffffe0));
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x21); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x57586d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x575560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x57586d; continue;
            }
            0x57586d => {
            { let a = c.r[6].wrapping_add(0x2a); wm(m,a,16,0x0u32); }
            pc = 0x575873; continue;
            }
            0x575873 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00575730(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x575730;
    loop {
        match pc {
            0x575730 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57575f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(6,32));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x575775);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc314),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5757fd; continue; }
            pc = 0x575789; continue;
            }
            0x575789 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0xc318),32));
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(3) { pc = 0x5757fd; continue; }
            pc = 0x575799; continue;
            }
            0x575799 => {
            { let a = c.get(0,32); let r = c.imul(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc30c),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x5757fd; continue; }
            pc = 0x5757ad; continue;
            }
            0x5757ad => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x98),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x5757c2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0x2fu32,32); }
            if c.cond(5) { pc = 0x5757fd; continue; }
            pc = 0x5757c7; continue;
            }
            0x5757c7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x49),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            c.push(m, 0x5757ef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x630760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,0xau32,16); }
            if c.cond(3) { pc = 0x5757fd; continue; }
            pc = 0x5757f5; continue;
            }
            0x5757f5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x5757fd => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00576990(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x576990;
    loop {
        match pc {
            0x576990 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.r[6].wrapping_add(0x36); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,0x6e2e44u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            if c.cond(6) { pc = 0x576a08; continue; }
            pc = 0x5769af; continue;
            }
            0x5769af => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            pc = 0x5769bb; continue;
            }
            0x5769bb => {
            c.set(1,32,rm(m,0x6e2e4cu32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x5769ef; continue; }
            pc = 0x5769c9; continue;
            }
            0x5769c9 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5769e8; continue; }
            pc = 0x5769d1; continue;
            }
            0x5769d1 => {
            c.push(m, 0x5769d6);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5769eb; continue; }
            pc = 0x5769e8; continue;
            }
            0x5769e8 => {
            { let a = c.r[6].wrapping_add(0x36); wm(m,a,8,c.get(3,8)); }
            pc = 0x5769eb; continue;
            }
            0x5769eb => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),8); let r = c.inc(a,8); { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,r); } }
            pc = 0x5769ef; continue;
            }
            0x5769ef => {
            c.set(1,32,rm(m,0x6e2e44u32,32));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5769bb; continue; }
            pc = 0x576a08; continue;
            }
            0x576a08 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x3e),8));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x18u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x576a4f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,16,c.get(0,16)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x38),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x3e); wm(m,a,16,c.get(1,16)); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3a),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x34));
            c.fstp64(m,c.r[4].wrapping_add(0x10));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x576ac1);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x576ac9);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,16,c.get(0,16)); }
            if c.cond(5) { pc = 0x576ad8; continue; }
            pc = 0x576ad2; continue;
            }
            0x576ad2 => {
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,16,0x1u32); }
            pc = 0x576ad8; continue;
            }
            0x576ad8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x36),8));
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,c.get(3,8)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6e2e4cu32,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m, 0x576afb);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let b = rm(m,c.r[3].wrapping_add(0x8),32); c.div32(b); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x34),16));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x34));
            c.fstp64(m,c.r[4].wrapping_add(0x10));
            { let a = c.get(2,8); let r = c.add(a,c.get(0,8),8); c.set(2,8,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0x2e); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x576b29);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x576b31);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2e),8));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x2e); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,0x6e2e44u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x36),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x576b57; continue; }
            pc = 0x576b4e; continue;
            }
            0x576b4e => {
            c.set(2,32,rm(m,0x6e2e48u32,32));
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x576b57; continue;
            }
            0x576b57 => {
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x576b62);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            c.push(m, 0x576b74);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x559cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x38),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x576b95);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m, 0x576b9e);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x576bbf; continue; }
            pc = 0x576bab; continue;
            }
            0x576bab => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,rm(m,0x67792cu32,8));
            if c.cond(15) { pc = 0x576c15; continue; }
            pc = 0x576bb8; continue;
            }
            0x576bb8 => {
            c.set(0,8,rm(m,0x677928u32,8));
            pc = 0x576c15; continue;
            }
            0x576bbf => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x576bc9);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.push(m, 0x576bd5);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x576bef; continue; }
            pc = 0x576bdf; continue;
            }
            0x576bdf => {
            c.set(0,8,rm(m,0x67792au32,8));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(15) { pc = 0x576c15; continue; }
            pc = 0x576be8; continue;
            }
            0x576be8 => {
            c.set(0,8,rm(m,0x67792eu32,8));
            pc = 0x576c15; continue;
            }
            0x576bef => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x576c07; continue; }
            pc = 0x576bf7; continue;
            }
            0x576bf7 => {
            c.set(0,8,rm(m,0x67792bu32,8));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(15) { pc = 0x576c15; continue; }
            pc = 0x576c00; continue;
            }
            0x576c00 => {
            c.set(0,8,rm(m,0x67792du32,8));
            pc = 0x576c15; continue;
            }
            0x576c07 => {
            c.set(0,8,rm(m,0x677929u32,8));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(15) { pc = 0x576c15; continue; }
            pc = 0x576c10; continue;
            }
            0x576c10 => {
            c.set(0,8,rm(m,0x67792fu32,8));
            pc = 0x576c15; continue;
            }
            0x576c15 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.set(7,32,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x80u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.and(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x6),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x6); wm(m,a,16,r); } }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x6));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[1],16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1]; wm(m,a,16,r); } }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa));
            c.set(2,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x576c9d);
            { let args = []; c.r[0] = host.call(m, 0x559810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            c.set(0,32,0x99999999u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[1],16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1]; wm(m,a,16,r); } }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00576e80(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x576e80;
    loop {
        match pc {
            0x576e80 => {
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e2e44u32,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x36),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x576e9c; continue; }
            pc = 0x576e98; continue;
            }
            0x576e98 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x576ea5; continue;
            }
            0x576e9c => {
            c.set(1,32,rm(m,0x6e2e48u32,32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x576ea5; continue;
            }
            0x576ea5 => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.set(0,32,0xd1b71759u32);
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x414),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[7].wrapping_add(0xc),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0xcu32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(2,32); let n = 0x9u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,c.get(1,32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x34),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x576ee2; continue; }
            pc = 0x576edf; continue;
            }
            0x576edf => {
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x576ee2; continue;
            }
            0x576ee2 => {
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x32),16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x38),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3c),16));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x3a),16));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3e),16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(3,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let b = c.get(3,32); c.div32(b); }
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(6) { pc = 0x576f4b; continue; }
            pc = 0x576f39; continue;
            }
            0x576f39 => {
            c.set(2,32,c.r[0].wrapping_add(0xffffff80));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e),8));
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let r = c.imul(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.imul(a,c.get(2,32),32); c.set(3,32,r); }
            pc = 0x576f5f; continue;
            }
            0x576f4b => {
            c.set(2,32,0x80u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(2,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e),8));
            { let a = c.get(3,32); let r = c.imul(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.imul(a,c.get(0,32),32); c.set(3,32,r); }
            pc = 0x576f5f; continue;
            }
            0x576f5f => {
            { let a = c.get(3,32); let n = 0xeu32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let a = c.get(0,8); let r = c.sub(a,c.get(3,8),8); c.set(0,8,r); }
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x40),8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x41),8));
            c.set(2,32,c.get(3,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); let r = c.add(a,c.get(3,8),8); c.set(0,8,r); }
            { let a = c.get(1,8); let r = c.add(a,c.get(0,8),8); c.set(1,8,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            { let a = rm(m,c.r[0],32); c.sub(a,0x1u32,32); }
            if c.cond(6) { pc = 0x576fbc; continue; }
            pc = 0x576f94; continue;
            }
            0x576f94 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x576fa2; continue; }
            pc = 0x576f9b; continue;
            }
            0x576f9b => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            pc = 0x576fab; continue;
            }
            0x576fa2 => {
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            pc = 0x576fab; continue;
            }
            0x576fab => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(3) { pc = 0x576fc1; continue; }
            pc = 0x576fb6; continue;
            }
            0x576fb6 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            pc = 0x576fc1; continue;
            }
            0x576fbc => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            pc = 0x576fc1; continue;
            }
            0x576fc1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(3,32,c.get(2,32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x576ffb; continue; }
            pc = 0x576fd5; continue;
            }
            0x576fd5 => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x576ffb; continue; }
            pc = 0x576fe7; continue;
            }
            0x576fe7 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x576ffb; continue; }
            pc = 0x576ff2; continue;
            }
            0x576ff2 => {
            { let a = c.get(1,8); c.sub(a,rm(m,c.r[6].wrapping_add(0x2d),8),8); }
            if c.cond(4) { pc = 0x5773a7; continue; }
            pc = 0x576ffb; continue;
            }
            0x576ffb => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x32),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x34),16),16); }
            if c.cond(5) { pc = 0x5771b0; continue; }
            pc = 0x577009; continue;
            }
            0x577009 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(7,32,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(3,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(3,32,r); }
            { let a = c.get(7,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x577027);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc314),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5771a6; continue; }
            pc = 0x57703f; continue;
            }
            0x57703f => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[1].wrapping_add(0xc318),32),32); }
            if c.cond(3) { pc = 0x5771a6; continue; }
            pc = 0x57704f; continue;
            }
            0x57704f => {
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc308),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x98),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x577073); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xeu32,32); }
            if c.cond(4) { pc = 0x577100; continue; }
            pc = 0x57707c; continue;
            }
            0x57707c => {
            { let a = c.get(0,32); c.sub(a,0x13u32,32); }
            if c.cond(4) { pc = 0x577100; continue; }
            pc = 0x577081; continue;
            }
            0x577081 => {
            c.set(1,32,rm(m,0x6e2e4cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x36),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32); c.sub(a,0x3u32,32); }
            if c.cond(5) { pc = 0x57717a; continue; }
            pc = 0x577099; continue;
            }
            0x577099 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xee98u32);
            c.push(m, 0x5770a9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5770db; continue; }
            pc = 0x5770ad; continue;
            }
            0x5770ad => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xee98u32);
            c.push(m, 0x5770bd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.push(m, 0x5770c4);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let r = c.sub(a,0x1168u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5770db);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x595b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x5770db; continue;
            }
            0x5770db => {
            c.set(2,8,rm(m,0x677929u32,8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x41),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.add(a,0x20u32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(0,32));
            c.push(m,0x12u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x57716f; continue;
            }
            0x577100 => {
            c.set(1,32,rm(m,0x6e2e4cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x36),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32); c.sub(a,0x3u32,32); }
            if c.cond(5) { pc = 0x57717a; continue; }
            pc = 0x577114; continue;
            }
            0x577114 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xea92u32);
            c.push(m, 0x577124);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x577156; continue; }
            pc = 0x577128; continue;
            }
            0x577128 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xea92u32);
            c.push(m, 0x577138);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.push(m, 0x57713f);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let r = c.sub(a,0x156eu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x577156);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x595b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x577156; continue;
            }
            0x577156 => {
            c.set(2,8,rm(m,0x677929u32,8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x12u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x57716f; continue;
            }
            0x57716f => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x57717a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            pc = 0x57717a; continue;
            }
            0x57717a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5771a6; continue; }
            pc = 0x577197; continue;
            }
            0x577197 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x3e),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3c),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5771a6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62fb90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5771a6; continue;
            }
            0x5771a6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5771b0 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(5,32,c.get(1,32));
            c.set(3,32,c.r[6].wrapping_add(0x8));
            { let a = c.get(5,32); let r = c.imul(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x80u32,32); c.set(5,32,r); }
            { let a = c.get(5,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.and(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x6),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x6); wm(m,a,16,r); } }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x6));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[1],16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1]; wm(m,a,16,r); } }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x57723a);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[5].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            c.set(0,32,0x99999999u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[1],16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1]; wm(m,a,16,r); } }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x1f),8));
            c.set(5,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x2d); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x80u32,32); c.set(5,32,r); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(5,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.and(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,16,c.get(1,8));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x6),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x6); wm(m,a,16,r); } }
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x6));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[5].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[1],16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1]; wm(m,a,16,r); } }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x577342);
            { let args = []; c.r[0] = host.call(m, 0x559790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[5].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            c.set(0,32,0x99999999u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[1],16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1]; wm(m,a,16,r); } }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa));
            pc = 0x5773a7; continue;
            }
            0x5773a7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00576d20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x576d20;
    loop {
        match pc {
            0x576d20 => {
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e2e44u32,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x36),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x576d3b; continue; }
            pc = 0x576d37; continue;
            }
            0x576d37 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x576d44; continue;
            }
            0x576d3b => {
            c.set(1,32,rm(m,0x6e2e48u32,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x576d44; continue;
            }
            0x576d44 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            c.set(7,32,c.get(2,32));
            c.push(m,c.get(3,32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x80u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.and(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[1],16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1]; wm(m,a,16,r); } }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x576dd4);
            { let args = []; c.r[0] = host.call(m, 0x559260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(3,8,c.get(0,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x576de9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(4) { pc = 0x576dfe; continue; }
            pc = 0x576dee; continue;
            }
            0x576dee => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m, 0x576dfe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x576dfe; continue;
            }
            0x576dfe => {
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x576e0a);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x6));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2d),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            c.set(0,32,0x99999999u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[1],16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1]; wm(m,a,16,r); } }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2d),8));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0062fb90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x62fb90;
    loop {
        match pc {
            0x62fb90 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x77),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x630634; continue; }
            pc = 0x62fba4; continue;
            }
            0x62fba4 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x232),16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x2c),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1004),16));
            c.set(3,32,c.get(2,32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x62fce6; continue; }
            pc = 0x62fbe4; continue;
            }
            0x62fbe4 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x62fce6; continue; }
            pc = 0x62fbef; continue;
            }
            0x62fbef => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x62fc0b; continue;
            }
            0x62fc0b => {
            c.set(2,32,c.get(1,32));
            c.set(7,32,c.get(5,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62fc2c; continue; }
            pc = 0x62fc27; continue;
            }
            0x62fc27 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x62fc2e; continue;
            }
            0x62fc2c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x62fc2e; continue;
            }
            0x62fc2e => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x62fc3c);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[0]));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62fc69; continue; }
            pc = 0x62fc64; continue;
            }
            0x62fc64 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x62fc6b; continue;
            }
            0x62fc69 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x62fc6b; continue;
            }
            0x62fc6b => {
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x62fc79);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6df270u32,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x12u32,32); }
            if c.cond(13) { pc = 0x62fcc5; continue; }
            pc = 0x62fca2; continue;
            }
            0x62fca2 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x14),32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x58),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x62fd6c; continue; }
            pc = 0x62fcc5; continue;
            }
            0x62fcc5 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1004),16),16); }
            if c.cond(2) { pc = 0x62fc0b; continue; }
            pc = 0x62fce6; continue;
            }
            0x62fce6 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x236),16));
            c.set(2,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x2e),16));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1006),16));
            c.set(3,32,c.get(2,32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x62fe5c; continue; }
            pc = 0x62fd21; continue;
            }
            0x62fd21 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x62fe5c; continue; }
            pc = 0x62fd2c; continue;
            }
            0x62fd2c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x62fd48; continue;
            }
            0x62fd48 => {
            c.set(2,32,c.get(1,32));
            c.set(7,32,c.get(5,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x410),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62fd9c; continue; }
            pc = 0x62fd67; continue;
            }
            0x62fd67 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x62fd9e; continue;
            }
            0x62fd6c => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x14),32));
            { let a = c.r[2].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            { let a = rm(m,c.r[6].wrapping_add(0x2c),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x2c); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x62fd9c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x62fd9e; continue;
            }
            0x62fd9e => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x62fdac);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[0]));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x410),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62fddc; continue; }
            pc = 0x62fdd7; continue;
            }
            0x62fdd7 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x62fdde; continue;
            }
            0x62fddc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x62fdde; continue;
            }
            0x62fdde => {
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x62fdec);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6df270u32,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x12u32,32); }
            if c.cond(13) { pc = 0x62fe3b; continue; }
            pc = 0x62fe15; continue;
            }
            0x62fe15 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x410),32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x58),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x62fee2; continue; }
            pc = 0x62fe3b; continue;
            }
            0x62fe3b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1006),16),16); }
            if c.cond(2) { pc = 0x62fd48; continue; }
            pc = 0x62fe5c; continue;
            }
            0x62fe5c => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x23a),16));
            c.set(2,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x30),16));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1008),16));
            c.set(3,32,c.get(2,32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x62ffd5; continue; }
            pc = 0x62fe97; continue;
            }
            0x62fe97 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x62ffd5; continue; }
            pc = 0x62fea2; continue;
            }
            0x62fea2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x62febe; continue;
            }
            0x62febe => {
            c.set(2,32,c.get(1,32));
            c.set(7,32,c.get(5,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x80c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62ff15; continue; }
            pc = 0x62fedd; continue;
            }
            0x62fedd => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x62ff17; continue;
            }
            0x62fee2 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x410),32));
            { let a = c.r[2].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            { let a = rm(m,c.r[6].wrapping_add(0x2e),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x2e); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x62ff15 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x62ff17; continue;
            }
            0x62ff17 => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x62ff25);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[0]));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x80c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62ff55; continue; }
            pc = 0x62ff50; continue;
            }
            0x62ff50 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x62ff57; continue;
            }
            0x62ff55 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x62ff57; continue;
            }
            0x62ff57 => {
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x62ff65);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6df270u32,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x12u32,32); }
            if c.cond(13) { pc = 0x62ffb4; continue; }
            pc = 0x62ff8e; continue;
            }
            0x62ff8e => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x80c),32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x58),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x63005b; continue; }
            pc = 0x62ffb4; continue;
            }
            0x62ffb4 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1008),16),16); }
            if c.cond(2) { pc = 0x62febe; continue; }
            pc = 0x62ffd5; continue;
            }
            0x62ffd5 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x23e),16));
            c.set(2,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x32),16));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x100a),16));
            c.set(3,32,c.get(2,32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x63014a; continue; }
            pc = 0x630010; continue;
            }
            0x630010 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x63014a; continue; }
            pc = 0x63001b; continue;
            }
            0x63001b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x630037; continue;
            }
            0x630037 => {
            c.set(2,32,c.get(1,32));
            c.set(7,32,c.get(5,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc08),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63008e; continue; }
            pc = 0x630056; continue;
            }
            0x630056 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x630090; continue;
            }
            0x63005b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x80c),32));
            { let a = c.r[2].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            { let a = rm(m,c.r[6].wrapping_add(0x30),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x30); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x63008e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x630090; continue;
            }
            0x630090 => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x63009e);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[0]));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc08),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6300ce; continue; }
            pc = 0x6300c9; continue;
            }
            0x6300c9 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x6300d0; continue;
            }
            0x6300ce => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6300d0; continue;
            }
            0x6300d0 => {
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x6300de);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6df270u32,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x12u32,32); }
            if c.cond(13) { pc = 0x630129; continue; }
            pc = 0x630107; continue;
            }
            0x630107 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xc08),32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x58),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x630197; continue; }
            pc = 0x630129; continue;
            }
            0x630129 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x100a),16),16); }
            if c.cond(2) { pc = 0x630037; continue; }
            pc = 0x63014a; continue;
            }
            0x63014a => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x18),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x21e),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x22),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x630248; continue; }
            pc = 0x63016f; continue;
            }
            0x63016f => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x630248; continue; }
            pc = 0x63017a; continue;
            }
            0x63017a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x6301ce; continue;
            }
            0x630197 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xc08),32));
            { let a = c.r[2].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            { let a = rm(m,c.r[6].wrapping_add(0x32),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x32); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x6301ca => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x6301ce; continue;
            }
            0x6301ce => {
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x7c));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x7c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6301e6; continue; }
            pc = 0x6301e1; continue;
            }
            0x6301e1 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x6301e8; continue;
            }
            0x6301e6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6301e8; continue;
            }
            0x6301e8 => {
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x6301f4);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x630204; continue; }
            pc = 0x6301ff; continue;
            }
            0x6301ff => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x630206; continue;
            }
            0x630204 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x630206; continue;
            }
            0x630206 => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x630214);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x12u32,32); }
            if c.cond(13) { pc = 0x630241; continue; }
            pc = 0x630238; continue;
            }
            0x630238 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x630295; continue; }
            pc = 0x630241; continue;
            }
            0x630241 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x18),16),16); }
            if c.cond(2) { pc = 0x6301ca; continue; }
            pc = 0x630248; continue;
            }
            0x630248 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x1a),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x222),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x24),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x630339; continue; }
            pc = 0x63026d; continue;
            }
            0x63026d => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x630339; continue; }
            pc = 0x630278; continue;
            }
            0x630278 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x6302b5; continue;
            }
            0x630295 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x7c),32));
            { let a = c.r[0].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            { let a = rm(m,c.r[6].wrapping_add(0x22),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x22); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x6302b1 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x6302b5; continue;
            }
            0x6302b5 => {
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x11c));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x11c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6302d3; continue; }
            pc = 0x6302ce; continue;
            }
            0x6302ce => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x6302d5; continue;
            }
            0x6302d3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6302d5; continue;
            }
            0x6302d5 => {
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x6302e1);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6302f1; continue; }
            pc = 0x6302ec; continue;
            }
            0x6302ec => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x6302f3; continue;
            }
            0x6302f1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6302f3; continue;
            }
            0x6302f3 => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x630301);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x12u32,32); }
            if c.cond(13) { pc = 0x63032e; continue; }
            pc = 0x630325; continue;
            }
            0x630325 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x630386; continue; }
            pc = 0x63032e; continue;
            }
            0x63032e => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x1a),16),16); }
            if c.cond(2) { pc = 0x6302b1; continue; }
            pc = 0x630339; continue;
            }
            0x630339 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x1c),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x226),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x26),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x63042d; continue; }
            pc = 0x63035e; continue;
            }
            0x63035e => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x63042d; continue; }
            pc = 0x630369; continue;
            }
            0x630369 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x6303a9; continue;
            }
            0x630386 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x11c),32));
            { let a = c.r[0].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            { let a = rm(m,c.r[6].wrapping_add(0x24),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x24); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x6303a5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x6303a9; continue;
            }
            0x6303a9 => {
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x174));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x174),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6303c7; continue; }
            pc = 0x6303c2; continue;
            }
            0x6303c2 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x6303c9; continue;
            }
            0x6303c7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6303c9; continue;
            }
            0x6303c9 => {
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x6303d5);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6303e5; continue; }
            pc = 0x6303e0; continue;
            }
            0x6303e0 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x6303e7; continue;
            }
            0x6303e5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6303e7; continue;
            }
            0x6303e7 => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x6303f5);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x12u32,32); }
            if c.cond(13) { pc = 0x630422; continue; }
            pc = 0x630419; continue;
            }
            0x630419 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x63047a; continue; }
            pc = 0x630422; continue;
            }
            0x630422 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x1c),16),16); }
            if c.cond(2) { pc = 0x6303a5; continue; }
            pc = 0x63042d; continue;
            }
            0x63042d => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x1e),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x22a),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x28),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x630521; continue; }
            pc = 0x630452; continue;
            }
            0x630452 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x630521; continue; }
            pc = 0x63045d; continue;
            }
            0x63045d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x63049d; continue;
            }
            0x63047a => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x174),32));
            { let a = c.r[0].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            { let a = rm(m,c.r[6].wrapping_add(0x26),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x26); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x630499 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x63049d; continue;
            }
            0x63049d => {
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x1bc));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x1bc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6304bb; continue; }
            pc = 0x6304b6; continue;
            }
            0x6304b6 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x6304bd; continue;
            }
            0x6304bb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6304bd; continue;
            }
            0x6304bd => {
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x6304c9);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6304d9; continue; }
            pc = 0x6304d4; continue;
            }
            0x6304d4 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x6304db; continue;
            }
            0x6304d9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6304db; continue;
            }
            0x6304db => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x6304e9);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x12u32,32); }
            if c.cond(13) { pc = 0x630516; continue; }
            pc = 0x63050d; continue;
            }
            0x63050d => {
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x63056e; continue; }
            pc = 0x630516; continue;
            }
            0x630516 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x1e),16),16); }
            if c.cond(2) { pc = 0x630499; continue; }
            pc = 0x630521; continue;
            }
            0x630521 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x20),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x22e),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2a),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x630634; continue; }
            pc = 0x630546; continue;
            }
            0x630546 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x630634; continue; }
            pc = 0x630551; continue;
            }
            0x630551 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x630591; continue;
            }
            0x63056e => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x1bc),32));
            { let a = c.r[0].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            { let a = rm(m,c.r[6].wrapping_add(0x28),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x28); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x63058d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x630591; continue;
            }
            0x630591 => {
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x1f8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x1f8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6305af; continue; }
            pc = 0x6305aa; continue;
            }
            0x6305aa => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x6305b1; continue;
            }
            0x6305af => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6305b1; continue;
            }
            0x6305b1 => {
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x6305bd);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6305cd; continue; }
            pc = 0x6305c8; continue;
            }
            0x6305c8 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x6305cf; continue;
            }
            0x6305cd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6305cf; continue;
            }
            0x6305cf => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x6305dd);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x23u32,32); }
            if c.cond(13) { pc = 0x63060a; continue; }
            pc = 0x630601; continue;
            }
            0x630601 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x63061f; continue; }
            pc = 0x63060a; continue;
            }
            0x63060a => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x20),16),16); }
            if c.cond(2) { pc = 0x63058d; continue; }
            pc = 0x630615; continue;
            }
            0x630615 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x63061f => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x1f8),32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,8,0x1u32); }
            { let a = rm(m,c.r[6].wrapping_add(0x2a),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x2a); wm(m,a,16,r); } }
            pc = 0x630634; continue;
            }
            0x630634 => {
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

pub fn f_0055ffc0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x55ffc0;
    loop {
        match pc {
            0x55ffc0 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x8),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x55ffd9; continue; }
            pc = 0x55ffc9; continue;
            }
            0x55ffc9 => {
            { let a = c.get(0,8); c.sub(a,0x13u32,8); }
            if c.cond(4) { pc = 0x55ffd9; continue; }
            pc = 0x55ffcd; continue;
            }
            0x55ffcd => {
            { let a = c.get(0,8); c.sub(a,0xbu32,8); }
            if c.cond(4) { pc = 0x55ffd9; continue; }
            pc = 0x55ffd1; continue;
            }
            0x55ffd1 => {
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[1].wrapping_add(0xe); wm(m,a,16,c.get(2,16)); }
            pc = 0x55ffd9; continue;
            }
            0x55ffd9 => {
            { let a = c.get(0,8); c.sub(a,0x8cu32,8); }
            if c.cond(5) { pc = 0x55ffe7; continue; }
            pc = 0x55ffdd; continue;
            }
            0x55ffdd => {
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[1].wrapping_add(0xe); wm(m,a,16,0x1u32); }
            pc = 0x55ffe7; continue;
            }
            0x55ffe7 => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00564810(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x564810;
    loop {
        match pc {
            0x564810 => {
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6e1734u32,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x564824);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x45); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x44); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x31); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,0x6e1734u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x56484f; continue; }
            pc = 0x56484b; continue;
            }
            0x56484b => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x564857; continue;
            }
            0x56484f => {
            c.set(0,32,rm(m,0x6e1738u32,32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            pc = 0x564857; continue;
            }
            0x564857 => {
            c.push(m,0x2u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x564862);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x564870);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x559cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x48),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x44),8));
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xc678),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xc674),32));
            c.set(5,32,c.r[1].wrapping_add(c.r[0]));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x4),32));
            c.set(1,8,rm(m,c.r[5],8));
            c.set(5,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            c.set(2,32,rm(m,c.r[2],32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[1].wrapping_add(c.r[5]),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0xfu32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.set(5,32,rm(m,c.r[2].wrapping_add(0xc678),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xc674),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[5]).wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,0x1u32); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xe),16));
            { let a = c.get(0,32); let n = 0xfu32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x12),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x564901);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x565d70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x56491b);
            { let args = []; c.r[0] = host.call(m, 0x559260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m, 0x564932);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[4].wrapping_add(0x48),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x564948; continue; }
            pc = 0x564938; continue;
            }
            0x564938 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x564948);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x564948; continue;
            }
            0x564948 => {
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x564953);
            { let args = []; c.r[0] = host.call(m, 0x559790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x564970);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x45),8));
            c.set(5,32,c.get(0,32));
            c.push(m,0x8u32);
            { let a = c.get(5,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[5].wrapping_mul(8)));
            c.set(5,32,rm(m,0x6df270u32,32));
            c.set(5,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5649a2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792fu32,8));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5649be);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x8u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5649d1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,0x67792au32,8));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5649ec);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5649ff);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,8,rm(m,0x67792fu32,8));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x564a1b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x564a2e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,0x3ab20u32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = rm(m,c.r[7].wrapping_add(0xc),32); c.div32(b); }
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x4e); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x6a); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x2e); wm(m,a,16,0xffffu32); }
            { let a = c.r[6].wrapping_add(0x6c); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0x70); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00564ac0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x564ac0;
    loop {
        match pc {
            0x564ac0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x78u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e1734u32,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x31),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x564ae0; continue; }
            pc = 0x564ad8; continue;
            }
            0x564ad8 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            pc = 0x564aef; continue;
            }
            0x564ae0 => {
            c.set(1,32,rm(m,0x6e1738u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(7,32,c.get(2,32));
            pc = 0x564aef; continue;
            }
            0x564aef => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x45),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            if c.cond(5) { pc = 0x564b19; continue; }
            pc = 0x564b0f; continue;
            }
            0x564b0f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x78u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x564b19 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(0,8)); }
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x414),32));
            { let a = c.get(3,32); c.sub(a,0x1f4u32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            if c.cond(6) { pc = 0x564b4f; continue; }
            pc = 0x564b46; continue;
            }
            0x564b46 => {
            c.set(3,32,0x1f4u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            pc = 0x564b4f; continue;
            }
            0x564b4f => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x30),8));
            c.set(0,32,c.r[6].wrapping_add(0x30));
            { let a = c.get(1,8); c.sub(a,0x2u32,8); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            if c.cond(5) { pc = 0x564b6c; continue; }
            pc = 0x564b61; continue;
            }
            0x564b61 => {
            { let a = rm(m,c.r[6].wrapping_add(0x28),8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x564b6c; continue; }
            pc = 0x564b67; continue;
            }
            0x564b67 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            pc = 0x564b7f; continue;
            }
            0x564b6c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x20),32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x564b7b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x564b7f; continue;
            }
            0x564b7f => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,8); c.sub(a,rm(m,c.r[6].wrapping_add(0x15),8),8); }
            if c.cond(4) { pc = 0x564b9e; continue; }
            pc = 0x564b8a; continue;
            }
            0x564b8a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x564b96); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(0,8)); }
            pc = 0x564b9e; continue;
            }
            0x564b9e => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0x50));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x564bb8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x564da2; continue; }
            pc = 0x564bc3; continue;
            }
            0x564bc3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x28),8));
            { let a = c.get(0,32); let r = c.sub(a,0x91u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x564d6a; continue; }
            pc = 0x564bd3; continue;
            }
            0x564bd3 => {
            { let a = c.get(0,32); let r = c.sub(a,0x20u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x564ca8; continue; }
            pc = 0x564bdc; continue;
            }
            0x564bdc => {
            c.set(3,32,0x5u32);
            pc = 0x564be1; continue;
            }
            0x564be1 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x564bea);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,0x0u32);
            c.push(m,0x13u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x564c02);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x564be1; continue; }
            pc = 0x564c05; continue;
            }
            0x564c05 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xea7eu32);
            c.push(m, 0x564c15);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x564c4f; continue; }
            pc = 0x564c19; continue;
            }
            0x564c19 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xea7eu32);
            c.push(m, 0x564c29);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.push(m, 0x564c30);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let r = c.sub(a,0x1582u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x564c4f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x595b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x564c4f; continue;
            }
            0x564c4f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x74),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x564c6f; continue; }
            pc = 0x564c56; continue;
            }
            0x564c56 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x3f8u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x564c68); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,0x0u32); }
            pc = 0x564c6f; continue;
            }
            0x564c6f => {
            c.set(0,8,rm(m,0x677929u32,8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0xb0023u32);
            c.push(m,0x12u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x564c93);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.push(m,0x91u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x564ca1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,8,c.get(0,8));
            pc = 0x564d6e; continue;
            }
            0x564ca8 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0xc1u32);
            c.push(m,0x12u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x564cca);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.push(m,0xffffff80u32);
            c.push(m,0x7d0u32);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x564cd8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x57af40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x54),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x50),16));
            c.set(7,32,c.r[6].wrapping_add(0x54));
            { let a = c.get(6,32); let r = c.add(a,0x50u32,32); c.set(6,32,r); }
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x564cf1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,8,rm(m,0x67792fu32,8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x564d05);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,16,rm(m,c.r[7],16));
            c.set(1,16,rm(m,c.r[6],16));
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x564d16);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,8,rm(m,0x67792au32,8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x564d2a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,16,rm(m,c.r[7],16));
            c.set(1,16,rm(m,c.r[6],16));
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x564d3b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,8,rm(m,0x67792fu32,8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x564d4f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,16,rm(m,c.r[7],16));
            c.set(1,16,rm(m,c.r[6],16));
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x564d60);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x78u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x564d6a => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x24),8));
            pc = 0x564d6e; continue;
            }
            0x564d6e => {
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[6].wrapping_add(0x15),8),8); }
            if c.cond(4) { pc = 0x565c17; continue; }
            pc = 0x564d77; continue;
            }
            0x564d77 => {
            c.set(2,32,c.r[4].wrapping_add(0x68));
            c.set(5,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x564d86);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[4].wrapping_add(0x74));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(3,8)); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x564d95);
            { let args = []; c.r[0] = host.call(m, 0x559790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x78u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x564da2 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            c.set(0,8,0x1u32);
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(0,8)); }
            if c.cond(4) { pc = 0x564db9; continue; }
            pc = 0x564daf; continue;
            }
            0x564daf => {
            { let a = c.get(0,8); let r = c.or(a,0xffu32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x6a); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(0,8)); }
            pc = 0x564db9; continue;
            }
            0x564db9 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            if c.cond(6) { pc = 0x5657fc; continue; }
            pc = 0x564dc6; continue;
            }
            0x564dc6 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x564dd4; continue; }
            pc = 0x564dce; continue;
            }
            0x564dce => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            pc = 0x564dd4; continue;
            }
            0x564dd4 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x50));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(5,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(7,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            { let a = c.get(5,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x564e04);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = rm(m,c.r[6].wrapping_add(0x3c),8); c.sub(a,0x3u32,8); }
            if c.cond(3) { pc = 0x564e84; continue; }
            pc = 0x564e0a; continue;
            }
            0x564e0a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = rm(m,c.r[0].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(7) { pc = 0x564e1c; continue; }
            pc = 0x564e15; continue;
            }
            0x564e15 => {
            { let a = rm(m,c.r[0].wrapping_add(0x3c),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x564e84; continue; }
            pc = 0x564e1c; continue;
            }
            0x564e1c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x564e84; continue; }
            pc = 0x564e23; continue;
            }
            0x564e23 => {
            c.push(m,0x2u32);
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x564e2d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            { let a = c.get(2,8); let r = c.add(a,0x3u32,8); c.set(2,8,r); }
            c.set(0,8,c.get(2,8));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x564e4f; continue; }
            pc = 0x564e3c; continue;
            }
            0x564e3c => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3d),8));
            c.push(m,c.get(7,32));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x564e4d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x565d70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x564e70; continue;
            }
            0x564e4f => {
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            if c.cond(5) { pc = 0x564e57; continue; }
            pc = 0x564e53; continue;
            }
            0x564e53 => {
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,0x4u32); }
            pc = 0x564e57; continue;
            }
            0x564e57 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x48),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4a),16));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[6].wrapping_add(0x42); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x564e70);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x565e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x564e70; continue;
            }
            0x564e70 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4c),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x564e84; continue; }
            pc = 0x564e79; continue;
            }
            0x564e79 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4e),16));
            { let a = c.get(1,16); let r = c.sub(a,c.get(0,16),16); c.set(1,16,r); }
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,c.get(1,16)); }
            pc = 0x564e84; continue;
            }
            0x564e84 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x40),16));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x564e99; continue; }
            pc = 0x564e8e; continue;
            }
            0x564e8e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x34),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x564ec4; continue; }
            pc = 0x564e95; continue;
            }
            0x564e95 => {
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x564ea0; continue; }
            pc = 0x564e99; continue;
            }
            0x564e99 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x34),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x564ec4; continue; }
            pc = 0x564ea0; continue;
            }
            0x564ea0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x564eb9; continue; }
            pc = 0x564eaa; continue;
            }
            0x564eaa => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x564ec4; continue; }
            pc = 0x564eb1; continue;
            }
            0x564eb1 => {
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x56501e; continue; }
            pc = 0x564eb9; continue;
            }
            0x564eb9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x38),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x56501e; continue; }
            pc = 0x564ec4; continue;
            }
            0x564ec4 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x564f4e; continue; }
            pc = 0x564ed4; continue;
            }
            0x564ed4 => {
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x565c24)); continue;
            }
            0x564edb => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3d),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x44),8));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xc674),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x564f4e; continue; }
            pc = 0x564eff; continue;
            }
            0x564eff => {
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            pc = 0x564f4b; continue;
            }
            0x564f03 => {
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            pc = 0x564f0f; continue;
            }
            0x564f0f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = rm(m,c.r[0].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x564f4e; continue; }
            pc = 0x564f1a; continue;
            }
            0x564f1a => {
            { let a = rm(m,c.r[0].wrapping_add(0x3c),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x564f4e; continue; }
            pc = 0x564f21; continue;
            }
            0x564f21 => {
            { let a = rm(m,c.r[6].wrapping_add(0x28),8); c.sub(a,0xfu32,8); }
            if c.cond(4) { pc = 0x564f4e; continue; }
            pc = 0x564f27; continue;
            }
            0x564f27 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x564f33);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5663d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(0,8)); }
            pc = 0x564f4e; continue;
            }
            0x564f39 => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            pc = 0x564f3e; continue;
            }
            0x564f3e => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3d),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x565983; continue; }
            pc = 0x564f49; continue;
            }
            0x564f49 => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            pc = 0x564f4b; continue;
            }
            0x564f4b => {
            { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,c.get(0,8)); }
            pc = 0x564f4e; continue;
            }
            0x564f4e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x4c),16); let r = c.add(a,c.get(3,16),16); { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,r); } }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x40),16));
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x4c),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4e),16));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,16); c.sub(a,c.get(2,16),16); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x564fd6; continue; }
            pc = 0x564f79; continue;
            }
            0x564f79 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x564fd6; continue; }
            pc = 0x564f80; continue;
            }
            0x564f80 => {
            c.set(7,32,c.get(5,32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,c.r[1].wrapping_add(c.r[7]));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            if c.cond(6) { pc = 0x564fc1; continue; }
            pc = 0x564f9a; continue;
            }
            0x564f9a => {
            { let a = c.get(5,16); c.and(a,c.get(5,16),16); }
            if c.cond(5) { pc = 0x564fb0; continue; }
            pc = 0x564f9f; continue;
            }
            0x564f9f => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x4c),16); let r = c.add(a,c.get(3,16),16); { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,r); } }
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,c.get(1,32)); }
            pc = 0x564fd2; continue;
            }
            0x564fb0 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,c.get(2,16)); }
            pc = 0x564fd2; continue;
            }
            0x564fc1 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x60),16));
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x4c),16); let r = c.add(a,c.get(0,16),16); { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,r); } }
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,0x0u32); }
            pc = 0x564fd2; continue;
            }
            0x564fd2 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            pc = 0x564fd6; continue;
            }
            0x564fd6 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4c),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x564ff8; continue; }
            pc = 0x564fdf; continue;
            }
            0x564fdf => {
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,0x0u32); }
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            pc = 0x564ff8; continue;
            }
            0x564ff8 => {
            { let a = rm(m,c.r[6].wrapping_add(0x3c),8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x565236; continue; }
            pc = 0x565002; continue;
            }
            0x565002 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x565019);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x565d70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x565236; continue;
            }
            0x56501e => {
            { let a = rm(m,c.r[6].wrapping_add(0x4c),16); let r = c.add(a,c.get(3,16),16); { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,r); } }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x4c),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4e),16));
            { let a = c.get(7,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x56508c; continue; }
            pc = 0x56502f; continue;
            }
            0x56502f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x56508c; continue; }
            pc = 0x565036; continue;
            }
            0x565036 => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(5,32,c.r[1].wrapping_add(c.r[7]));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x30),32));
            if c.cond(6) { pc = 0x565077; continue; }
            pc = 0x56504e; continue;
            }
            0x56504e => {
            { let a = rm(m,c.r[6].wrapping_add(0x4c),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x565066; continue; }
            pc = 0x565055; continue;
            }
            0x565055 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x4c),16); let r = c.add(a,c.get(3,16),16); { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,r); } }
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,c.get(1,32)); }
            pc = 0x565088; continue;
            }
            0x565066 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,c.get(2,16)); }
            pc = 0x565088; continue;
            }
            0x565077 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x60),16));
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x4c),16); let r = c.add(a,c.get(0,16),16); { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,r); } }
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,0x0u32); }
            pc = 0x565088; continue;
            }
            0x565088 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            pc = 0x56508c; continue;
            }
            0x56508c => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4c),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x56520a; continue; }
            pc = 0x565099; continue;
            }
            0x565099 => {
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,0x0u32); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            if c.cond(5) { pc = 0x56520a; continue; }
            pc = 0x5650bd; continue;
            }
            0x5650bd => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5650ca);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5669d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.sub(a,0x1a4u32,32); }
            if c.cond(6) { pc = 0x5650ee; continue; }
            pc = 0x5650d1; continue;
            }
            0x5650d1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5650e4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x630760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,0x14u32,16); }
            if c.cond(3) { pc = 0x56520a; continue; }
            pc = 0x5650ee; continue;
            }
            0x5650ee => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(1,8); let r = c.add(a,0x40u32,8); c.set(1,8,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x565106);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x565f50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x56512a; continue; }
            pc = 0x56510a; continue;
            }
            0x56510a => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(1,8); let r = c.sub(a,0x40u32,8); c.set(1,8,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x565122);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x565f50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x56520a; continue; }
            pc = 0x56512a; continue;
            }
            0x56512a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x34),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x58));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x565171);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x5c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x56518b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x34),16));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x38),16));
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x46),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x47),8));
            { let a = c.r[6].wrapping_add(0x4a); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x5c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5651c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x34),16));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x38),16));
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4a),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x48),16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x42); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x56520a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x565e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x56520a; continue;
            }
            0x56520a => {
            { let a = rm(m,c.r[6].wrapping_add(0x3c),8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x565236; continue; }
            pc = 0x565210; continue;
            }
            0x565210 => {
            { let a = rm(m,c.r[6].wrapping_add(0x28),8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x565236; continue; }
            pc = 0x565216; continue;
            }
            0x565216 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x38),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x565236; continue;
            }
            0x565236 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,c.r[4].wrapping_add(0x50));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x565256);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x50),32));
            c.set(7,32,c.r[6].wrapping_add(0x50));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            if c.cond(5) { pc = 0x565271; continue; }
            pc = 0x565268; continue;
            }
            0x565268 => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[7].wrapping_add(0x4),32),32); }
            if c.cond(4) { pc = 0x565779; continue; }
            pc = 0x565271; continue;
            }
            0x565271 => {
            { let a = rm(m,c.r[6].wrapping_add(0x28),8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x565779; continue; }
            pc = 0x56527b; continue;
            }
            0x56527b => {
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            if c.cond(5) { pc = 0x5652b9; continue; }
            pc = 0x56529c; continue;
            }
            0x56529c => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x54),16));
            c.set(1,16,rm(m,c.r[7],16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m, 0x5652ae);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x630760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            if c.cond(3) { pc = 0x5652b9; continue; }
            pc = 0x5652b4; continue;
            }
            0x5652b4 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x5652b9; continue;
            }
            0x5652b9 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x54),16));
            c.set(0,16,rm(m,c.r[7],16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,c.r[6].wrapping_add(0x54));
            c.push(m,0x8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5652d0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792fu32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5652e4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[5],16));
            c.set(0,16,rm(m,c.r[7],16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5652f8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792au32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x56530c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[5],16));
            c.set(0,16,rm(m,c.r[7],16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x565320);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792fu32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x565334);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[5],16));
            c.set(0,16,rm(m,c.r[7],16));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x56534a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x56535d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x565366; continue; }
            pc = 0x565361; continue;
            }
            0x565361 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x565366; continue;
            }
            0x565366 => {
            c.set(0,8,rm(m,0x67792fu32,8));
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x565381);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m, 0x565396);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x56539f; continue; }
            pc = 0x56539a; continue;
            }
            0x56539a => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x56539f; continue;
            }
            0x56539f => {
            c.set(2,8,rm(m,0x67792au32,8));
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x5653bb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5653d0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5653d9; continue; }
            pc = 0x5653d4; continue;
            }
            0x5653d4 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x5653d9; continue;
            }
            0x5653d9 => {
            c.set(1,8,rm(m,0x67792fu32,8));
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5653f5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x56540a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x565413; continue; }
            pc = 0x56540e; continue;
            }
            0x56540e => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x565413; continue;
            }
            0x565413 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x42),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x40),16));
            c.set(2,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x565431);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x56544b; continue; }
            pc = 0x565441; continue;
            }
            0x565441 => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x6c),32),32); }
            if c.cond(4) { pc = 0x5655f8; continue; }
            pc = 0x56544b; continue;
            }
            0x56544b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            { let a = c.get(0,32); let r = c.add(a,0xffffffe0u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xc0u32,32); }
            if c.cond(7) { pc = 0x565602; continue; }
            pc = 0x56545e; continue;
            }
            0x56545e => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x565c4c),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x565c38)); continue;
            }
            0x56546d => {
            c.set(0,8,rm(m,0x67792au32,8));
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x565488);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m, 0x56549d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5654a6; continue; }
            pc = 0x5654a1; continue;
            }
            0x5654a1 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x5654a6; continue;
            }
            0x5654a6 => {
            c.set(2,8,rm(m,0x67792fu32,8));
            pc = 0x5655d4; continue;
            }
            0x5654b1 => {
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            c.set(1,8,rm(m,0x67792au32,8));
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x5654d5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5654ea);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5654f3; continue; }
            pc = 0x5654ee; continue;
            }
            0x5654ee => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x5654f3; continue;
            }
            0x5654f3 => {
            c.set(0,8,rm(m,0x67792du32,8));
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x56550e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x565586; continue;
            }
            0x56551c => {
            c.set(2,8,rm(m,0x67792eu32,8));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x565540);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x565555);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x56555e; continue; }
            pc = 0x565559; continue;
            }
            0x565559 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x56555e; continue;
            }
            0x56555e => {
            c.set(1,8,rm(m,0x67792bu32,8));
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x56557a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x565586; continue;
            }
            0x565586 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m, 0x56558f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x56560e; continue; }
            pc = 0x565593; continue;
            }
            0x565593 => {
            pc = 0x565602; continue;
            }
            0x565595 => {
            c.set(0,8,rm(m,0x67792eu32,8));
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5655b0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m, 0x5655c5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5655ce; continue; }
            pc = 0x5655c9; continue;
            }
            0x5655c9 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x5655ce; continue;
            }
            0x5655ce => {
            c.set(2,8,rm(m,0x677929u32,8));
            pc = 0x5655d4; continue;
            }
            0x5655d4 => {
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x5655ea);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x565586; continue;
            }
            0x5655f8 => {
            { let a = rm(m,c.r[6].wrapping_add(0x3c),8); c.sub(a,0x4u32,8); }
            if c.cond(4) { pc = 0x5656d1; continue; }
            pc = 0x565602; continue;
            }
            0x565602 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5656c9; continue; }
            pc = 0x56560e; continue;
            }
            0x56560e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4c),16));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x565639; continue; }
            pc = 0x565635; continue;
            }
            0x565635 => {
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x4e),16),16); c.set(0,16,r); }
            pc = 0x565639; continue;
            }
            0x565639 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x56565e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5669d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = rm(m,c.r[7].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x565694; continue; }
            pc = 0x565669; continue;
            }
            0x565669 => {
            { let a = rm(m,c.r[7].wrapping_add(0x3c),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x565694; continue; }
            pc = 0x565670; continue;
            }
            0x565670 => {
            { let a = c.get(0,32); c.sub(a,0x348u32,32); }
            if c.cond(7) { pc = 0x565694; continue; }
            pc = 0x565677; continue;
            }
            0x565677 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x565694; continue; }
            pc = 0x56567b; continue;
            }
            0x56567b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x565694; continue; }
            pc = 0x565682; continue;
            }
            0x565682 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x56568e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5663d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(0,8)); }
            pc = 0x56569b; continue;
            }
            0x565694 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(1,8)); }
            pc = 0x56569b; continue;
            }
            0x56569b => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x574),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5656ac; continue; }
            pc = 0x5656a5; continue;
            }
            0x5656a5 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x30),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5656da; continue; }
            pc = 0x5656ac; continue;
            }
            0x5656ac => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4e),16));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x60),32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x5656da; continue; }
            pc = 0x5656c4; continue;
            }
            0x5656c4 => {
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            pc = 0x5656da; continue;
            }
            0x5656c9 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x50),32));
            pc = 0x5656d1; continue;
            }
            0x5656d1 => {
            { let a = c.r[7]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x5656da; continue;
            }
            0x5656da => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5656ed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,8,rm(m,0x67792fu32,8));
            c.set(0,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x565709);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,0x8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x56571c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792au32,8));
            c.set(2,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x565738);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,0x8u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x56574b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,0x67792fu32,8));
            c.set(1,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x565766);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x565779);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x565779; continue;
            }
            0x565779 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5657c3; continue; }
            pc = 0x565780; continue;
            }
            0x565780 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5657c3; continue; }
            pc = 0x565787; continue;
            }
            0x565787 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x30),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x565798; continue; }
            pc = 0x565792; continue;
            }
            0x565792 => {
            { let a = rm(m,c.r[6].wrapping_add(0x28),8); c.sub(a,0xfu32,8); }
            if c.cond(5) { pc = 0x5657d6; continue; }
            pc = 0x565798; continue;
            }
            0x565798 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x11),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5657d6; continue; }
            pc = 0x5657a3; continue;
            }
            0x5657a3 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x5657ba; continue; }
            pc = 0x5657b1; continue;
            }
            0x5657b1 => {
            { let a = c.get(1,8); let r = c.add(a,0x20u32,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(1,8)); }
            pc = 0x5657d6; continue;
            }
            0x5657ba => {
            { let a = c.get(1,8); let r = c.add(a,0xe0u32,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(1,8)); }
            pc = 0x5657d6; continue;
            }
            0x5657c3 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3e),8));
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(1,8)); }
            if c.cond(5) { pc = 0x5657d6; continue; }
            pc = 0x5657d2; continue;
            }
            0x5657d2 => {
            { let a = c.r[6].wrapping_add(0x6a); wm(m,a,8,0x0u32); }
            pc = 0x5657d6; continue;
            }
            0x5657d6 => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5657e2; continue; }
            pc = 0x5657da; continue;
            }
            0x5657da => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            pc = 0x5657e6; continue;
            }
            0x5657e2 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x12),8));
            pc = 0x5657e6; continue;
            }
            0x5657e6 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x564dc6; continue; }
            pc = 0x5657f8; continue;
            }
            0x5657f8 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x5657fc; continue;
            }
            0x5657fc => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x565834; continue; }
            pc = 0x56580f; continue;
            }
            0x56580f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x565834; continue; }
            pc = 0x56581e; continue;
            }
            0x56581e => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x24),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x565834; continue; }
            pc = 0x565829; continue;
            }
            0x565829 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x11),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x565848; continue; }
            pc = 0x565834; continue;
            }
            0x565834 => {
            c.set(0,32,c.r[4].wrapping_add(0x78));
            c.set(1,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(3,8,0x1u32);
            c.push(m, 0x565845);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x565848; continue;
            }
            0x565848 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x565876; continue; }
            pc = 0x565856; continue;
            }
            0x565856 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x24),8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x11),8));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,c.r[4].wrapping_add(0x78));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            c.push(m, 0x565873);
            { let args = []; c.r[0] = host.call(m, 0x559790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x565876; continue;
            }
            0x565876 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x565c17; continue; }
            pc = 0x565881; continue;
            }
            0x565881 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2e),16));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(2,32); c.sub(a,0x20000u32,32); }
            if c.cond(13) { pc = 0x565b24; continue; }
            pc = 0x565896; continue;
            }
            0x565896 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x6c),32));
            c.set(2,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x565b07; continue; }
            pc = 0x5658a7; continue;
            }
            0x5658a7 => {
            { let a = c.r[6].wrapping_add(0x70); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,0x677929u32,8));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0x160024u32);
            c.push(m,0x12u32);
            c.push(m, 0x5658c0);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0xcf0u32);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.sub(a,0x678u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5658dc);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x1400u32);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let r = c.sub(a,0xa00u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5658fe);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2e),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xc8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x6c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x44));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,0x6e6220u32);
            { let a = c.get(0,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.add(a,0x19u32,8); c.set(0,8,r); }
            { let a = c.r[7].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x56594f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x618d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(0x40));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x565a5e; continue; }
            pc = 0x56596e; continue;
            }
            0x56596e => {
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x565a50; continue; }
            pc = 0x565977; continue;
            }
            0x565977 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x675178),32));
            pc = 0x565a7d; continue;
            }
            0x565983 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x54),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x50),16));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,c.r[6].wrapping_add(0x54));
            c.set(5,32,c.r[6].wrapping_add(0x50));
            c.push(m,0x8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5659a0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792fu32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5659b4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[7],16));
            c.set(0,16,rm(m,c.r[5],16));
            c.push(m,0x8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5659c6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792au32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5659da);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[7],16));
            c.set(0,16,rm(m,c.r[5],16));
            c.push(m,0x8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5659ec);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,0x67792fu32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x565a00);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[7],16));
            c.set(0,16,rm(m,c.r[5],16));
            c.push(m,0x8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x565a12);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x22e),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x565a26; continue; }
            pc = 0x565a1e; continue;
            }
            0x565a1e => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(0x22e); wm(m,a,16,c.get(0,16)); }
            pc = 0x565a26; continue;
            }
            0x565a26 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x74),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x565a46; continue; }
            pc = 0x565a2d; continue;
            }
            0x565a2d => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x3f8u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x565a3f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,0x0u32); }
            pc = 0x565a46; continue;
            }
            0x565a46 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x78u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x565a50 => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,32,0x675378u32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            pc = 0x565a7d; continue;
            }
            0x565a5e => {
            c.set(3,32,c.r[0].wrapping_add(0xffffff80));
            { let a = c.get(3,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x565a6f; continue; }
            pc = 0x565a66; continue;
            }
            0x565a66 => {
            c.set(0,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x675178),32));
            pc = 0x565a7b; continue;
            }
            0x565a6f => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,32,0x675578u32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            pc = 0x565a7b; continue;
            }
            0x565a7b => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x565a7d; continue;
            }
            0x565a7d => {
            { let a = c.get(2,8); c.sub(a,0x80u32,8); }
            if c.cond(3) { pc = 0x565a9e; continue; }
            pc = 0x565a82; continue;
            }
            0x565a82 => {
            { let a = c.get(2,8); c.sub(a,0x40u32,8); }
            if c.cond(7) { pc = 0x565a90; continue; }
            pc = 0x565a87; continue;
            }
            0x565a87 => {
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x675178),32));
            pc = 0x565abd; continue;
            }
            0x565a90 => {
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,0x675378u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2],32));
            pc = 0x565abd; continue;
            }
            0x565a9e => {
            c.set(2,32,c.r[1].wrapping_add(0xffffff80));
            { let a = c.get(2,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x565aaf; continue; }
            pc = 0x565aa6; continue;
            }
            0x565aa6 => {
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x675178),32));
            pc = 0x565abb; continue;
            }
            0x565aaf => {
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,0x675578u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2],32));
            pc = 0x565abb; continue;
            }
            0x565abb => {
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            pc = 0x565abd; continue;
            }
            0x565abd => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x1f4u32,32); c.set(1,32,r); }
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.r[3].wrapping_add(0x1f4));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x10624dd3u32);
            { let a = c.r[7].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            pc = 0x565b24; continue;
            }
            0x565b07 => {
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xc8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x565b24; continue; }
            pc = 0x565b21; continue;
            }
            0x565b21 => {
            { let a = c.r[6].wrapping_add(0x6c); wm(m,a,32,c.get(1,32)); }
            pc = 0x565b24; continue;
            }
            0x565b24 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2e),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); c.sub(a,0x10000u32,32); }
            if c.cond(13) { pc = 0x565bf3; continue; }
            pc = 0x565b39; continue;
            }
            0x565b39 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,8,0x3u32);
            { let a = c.get(0,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,8); let r = c.sub(a,c.get(0,8),8); c.set(1,8,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x74),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,c.get(1,8)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x565bb2; continue; }
            pc = 0x565b52; continue;
            }
            0x565b52 => {
            c.set(2,32,rm(m,0x6e2e6cu32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x2c),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x565b65; continue; }
            pc = 0x565b61; continue;
            }
            0x565b61 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x565b6e; continue;
            }
            0x565b65 => {
            c.set(2,32,rm(m,0x6e2e70u32,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            pc = 0x565b6e; continue;
            }
            0x565b6e => {
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x565b95; continue; }
            pc = 0x565b7d; continue;
            }
            0x565b7d => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.sub(a,0x16u32,32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x565b93); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x565bb2; continue;
            }
            0x565b95 => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[2].wrapping_add(0x3f8));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x3f8),32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x565bab); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,0x0u32); }
            pc = 0x565bb2; continue;
            }
            0x565bb2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x74),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x565c17; continue; }
            pc = 0x565bb9; continue;
            }
            0x565bb9 => {
            c.set(1,8,rm(m,0x677929u32,8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(7,32); let r = c.add(a,0x1600b0u32,32); c.set(7,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.push(m,0x12u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x565be6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x78u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x565bf3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x74),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x565c17; continue; }
            pc = 0x565bfa; continue;
            }
            0x565bfa => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[2].wrapping_add(0x3f8));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x3f8),32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x565c10); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,0x0u32); }
            pc = 0x565c17; continue;
            }
            0x565c17 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x78u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00564a80(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x564a80;
    loop {
        match pc {
            0x564a80 => {
            c.set(2,32,rm(m,0x6e1734u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x31),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x564aa0; continue; }
            pc = 0x564a8f; continue;
            }
            0x564a8f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x564a9c);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x564aa0 => {
            c.set(2,32,rm(m,0x6e1738u32,32));
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x564ab5);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00566fe0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x566fe0;
    loop {
        match pc {
            0x566fe0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x31),8));
            c.set(1,32,rm(m,0x6e1734u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x566ff2; continue; }
            pc = 0x566fef; continue;
            }
            0x566fef => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x566ff2 => {
            c.set(1,32,rm(m,0x6e1738u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00566260(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x566260;
    loop {
        match pc {
            0x566260 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x25),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = rm(m,c.r[6].wrapping_add(0x8),8); c.sub(a,0xfu32,8); }
            if c.cond(5) { pc = 0x566374; continue; }
            pc = 0x56628c; continue;
            }
            0x56628c => {
            c.push(m, 0x566291);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x56630f; continue; }
            pc = 0x566295; continue;
            }
            0x566295 => {
            { let a = rm(m,c.r[7].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(7) { pc = 0x56630f; continue; }
            pc = 0x56629c; continue;
            }
            0x56629c => {
            { let a = rm(m,c.r[7].wrapping_add(0x3c),16); c.sub(a,0x0u32,16); }
            if c.cond(7) { pc = 0x56630f; continue; }
            pc = 0x5662a3; continue;
            }
            0x5662a3 => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xfffffff2),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xffffffee),16),16)&0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5662bd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = rm(m,c.r[6].wrapping_add(0x1c),8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x5662f7; continue; }
            pc = 0x5662c3; continue;
            }
            0x5662c3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5662d4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x630760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            if c.cond(6) { pc = 0x5662f7; continue; }
            pc = 0x5662da; continue;
            }
            0x5662da => {
            c.push(m,0x2u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5662e3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xfffffff2),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xffffffee),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0xffffffe0));
            c.push(m, 0x5662f5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x565e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x566374; continue;
            }
            0x5662f7 => {
            c.push(m,0xfu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x566300);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0xffffffe0));
            c.push(m, 0x56630d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5663d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x566374; continue;
            }
            0x56630f => {
            c.push(m,0x2u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x566318);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1c),8));
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            if c.cond(3) { pc = 0x566374; continue; }
            pc = 0x56631f; continue;
            }
            0x56631f => {
            { let a = c.get(0,8); let r = c.add(a,0x3u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x566344; continue; }
            pc = 0x566328; continue;
            }
            0x566328 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1d),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xfffffff2),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xffffffee),16));
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[6].wrapping_add(0x1d); wm(m,a,8,c.get(1,8)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0xffffffe0));
            c.push(m, 0x566342);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x565d70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x56636e; continue;
            }
            0x566344 => {
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            if c.cond(5) { pc = 0x56634c; continue; }
            pc = 0x566348; continue;
            }
            0x566348 => {
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,8,0x4u32); }
            pc = 0x56634c; continue;
            }
            0x56634c => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x28),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xfffffff2),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2a),16));
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,16,c.get(1,16)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xffffffee),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0xffffffe0));
            { let a = c.r[6].wrapping_add(0x22); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x56636e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x565e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x56636e; continue;
            }
            0x56636e => {
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,16,0x0u32); }
            pc = 0x566374; continue;
            }
            0x566374 => {
            { let a = rm(m,c.r[6].wrapping_add(0x8),8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x5663c8; continue; }
            pc = 0x56637a; continue;
            }
            0x56637a => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5663c8; continue; }
            pc = 0x566381; continue;
            }
            0x566381 => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xfffffff2),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xffffffee),16),16)&0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x56639b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5663ac);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x630760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            if c.cond(3) { pc = 0x5663c8; continue; }
            pc = 0x5663b2; continue;
            }
            0x5663b2 => {
            c.push(m,0xfu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5663bb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,c.r[6].wrapping_add(0xffffffe0));
            c.push(m,c.get(2,32));
            c.push(m, 0x5663c8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5663d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5663c8; continue;
            }
            0x5663c8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00565d70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x565d70;
    loop {
        match pc {
            0x565d70 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc678),32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x44),8));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc674),32));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]),8));
            c.set(7,32,c.r[1].wrapping_add(c.r[0]));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3d),8));
            c.set(3,32,rm(m,c.r[3].wrapping_add(c.r[0]).wrapping_add(0x4),32));
            { let a = c.get(2,8); let r = c.sub(a,c.get(1,8),8); c.set(2,8,r); }
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[3].wrapping_add(c.r[1]),32));
            { let a = c.get(3,32); let r = c.add(a,rm(m,c.r[7].wrapping_add(c.r[2]),32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,c.get(3,16)); }
            c.set(7,32,rm(m,0x6e51acu32,32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0xc678),32));
            c.set(7,32,rm(m,c.r[7].wrapping_add(0xc674),32));
            c.set(3,32,rm(m,c.r[3].wrapping_add(c.r[0]).wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[3].wrapping_add(c.r[2]).wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x4),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x42); wm(m,a,16,c.get(1,16)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x565e08);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x565e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
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

pub fn f_00565e10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x565e10;
    loop {
        match pc {
            0x565e10 => {
            c.set(2,32,rm(m,0x6e1734u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x31),8));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x565e24; continue; }
            pc = 0x565e20; continue;
            }
            0x565e20 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x565e2d; continue;
            }
            0x565e24 => {
            c.set(2,32,rm(m,0x6e1738u32,32));
            c.set(6,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x565e2d; continue;
            }
            0x565e2d => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x40),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x8),16),16); }
            if c.cond(3) { pc = 0x565ec6; continue; }
            pc = 0x565e3c; continue;
            }
            0x565e3c => {
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x42),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[4].wrapping_add(0xc),16),16); }
            if c.cond(3) { pc = 0x565e76; continue; }
            pc = 0x565e47; continue;
            }
            0x565e47 => {
            c.set(0,8,rm(m,0x67792fu32,8));
            { let a = c.r[1].wrapping_add(0x3e); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0xc56106ebu32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xdu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            pc = 0x565ea1; continue;
            }
            0x565e76 => {
            c.set(0,8,rm(m,0x67792du32,8));
            { let a = c.r[1].wrapping_add(0x3e); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0xc56106ebu32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xdu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            pc = 0x565ea1; continue;
            }
            0x565ea1 => {
            { let a = c.r[1].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(2,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0xd1b71759u32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x9u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x565ec6 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x42),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0xc),16),16); }
            if c.cond(3) { pc = 0x565f01; continue; }
            pc = 0x565ed1; continue;
            }
            0x565ed1 => {
            c.set(2,8,rm(m,0x677929u32,8));
            { let a = c.r[1].wrapping_add(0x3e); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0xc56106ebu32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xdu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            pc = 0x565f2c; continue;
            }
            0x565f01 => {
            c.set(0,8,rm(m,0x67792bu32,8));
            { let a = c.r[1].wrapping_add(0x3e); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0xc56106ebu32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xdu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            pc = 0x565f2c; continue;
            }
            0x565f2c => {
            { let a = c.r[1].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(2,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0xd1b71759u32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x9u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00565f50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x565f50;
    loop {
        match pc {
            0x565f50 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(3,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x565f81);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(3,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x565f97);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0xc),8));
            c.set(2,32,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x46); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x47); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,c.r[2].wrapping_add(0xffffffe0));
            { let a = c.get(0,32); c.sub(a,0xc0u32,32); }
            if c.cond(7) { pc = 0x565fff; continue; }
            pc = 0x565fb7; continue;
            }
            0x565fb7 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x566104),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x5660f4)); continue;
            }
            0x565fc6 => {
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,0x60u32);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x565ff4; continue;
            }
            0x565fd4 => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,0x80u32);
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x565ff4; continue;
            }
            0x565fe5 => {
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,0xa0u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x565ff4; continue;
            }
            0x565ff4 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x565fff);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x565fff; continue;
            }
            0x565fff => {
            { let a = c.get(3,8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x566010; continue; }
            pc = 0x566004; continue;
            }
            0x566004 => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x3u32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x4u32); }
            pc = 0x56601a; continue;
            }
            0x566010 => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x4u32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x3u32); }
            pc = 0x56601a; continue;
            }
            0x56601a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.r[4].wrapping_add(0xb); wm(m,a,8,0x0u32); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x5660dd; continue; }
            pc = 0x56602b; continue;
            }
            0x56602b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x5660ac; continue; }
            pc = 0x566045; continue;
            }
            0x566045 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x45),8));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            c.push(m, 0x56606e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5660e7; continue; }
            pc = 0x566072; continue;
            }
            0x566072 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x566083);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5661d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5660e7; continue; }
            pc = 0x566087; continue;
            }
            0x566087 => {
            c.set(0,8,rm(m,0x67792fu32,8));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5660a2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            if c.cond(2) { pc = 0x566045; continue; }
            pc = 0x5660ac; continue;
            }
            0x5660ac => {
            c.set(0,8,rm(m,0x677929u32,8));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5660c7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xb),8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0xb); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x56602b; continue; }
            pc = 0x5660dd; continue;
            }
            0x5660dd => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x5660e7 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005661d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5661d0;
    loop {
        match pc {
            0x5661d0 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc314),32));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x566259; continue; }
            pc = 0x5661e5; continue;
            }
            0x5661e5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0xc318),32));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(3) { pc = 0x566259; continue; }
            pc = 0x5661f5; continue;
            }
            0x5661f5 => {
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc30c),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x566230; continue; }
            pc = 0x56620b; continue;
            }
            0x56620b => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x98),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x566220); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0x30u32,32); }
            if c.cond(2) { pc = 0x566259; continue; }
            pc = 0x566225; continue;
            }
            0x566225 => {
            { let a = c.get(0,32); c.sub(a,0x31u32,32); }
            if c.cond(7) { pc = 0x566259; continue; }
            pc = 0x56622a; continue;
            }
            0x56622a => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            pc = 0x566230; continue;
            }
            0x566230 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc308),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x98),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x56624f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0x28u32,32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,u32::from(c.cond(4)));
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x566259 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005663d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5663d0;
    loop {
        match pc {
            0x5663d0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x2cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x7fffffffu32); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x45),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = rm(m,c.r[2].wrapping_add(0x242),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x5664f8; continue; }
            pc = 0x56640c; continue;
            }
            0x56640c => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x6c),8));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x250),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x566435);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x566460);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x56647a);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x566482);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x56649f; continue; }
            pc = 0x566489; continue;
            }
            0x566489 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x566495; continue; }
            pc = 0x56648d; continue;
            }
            0x56648d => {
            { let a = c.get(6,32); let r = c.add(a,0x11u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,0x11u32,32); c.set(3,32,r); }
            pc = 0x5664a7; continue;
            }
            0x566495 => {
            if c.cond(14) { pc = 0x56649a; continue; }
            pc = 0x566497; continue;
            }
            0x566497 => {
            { let a = c.get(6,32); let r = c.sub(a,0x11u32,32); c.set(6,32,r); }
            pc = 0x56649a; continue;
            }
            0x56649a => {
            { let a = c.get(3,32); let r = c.sub(a,0x11u32,32); c.set(3,32,r); }
            pc = 0x5664a7; continue;
            }
            0x56649f => {
            { let a = c.get(3,32); c.sub(a,0xbu32,32); }
            if c.cond(14) { pc = 0x5664a7; continue; }
            pc = 0x5664a4; continue;
            }
            0x5664a4 => {
            { let a = c.get(3,32); let r = c.sub(a,0x16u32,32); c.set(3,32,r); }
            pc = 0x5664a7; continue;
            }
            0x5664a7 => {
            c.set(1,32,c.get(3,32));
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(13) { pc = 0x566502; continue; }
            pc = 0x5664d4; continue;
            }
            0x5664d4 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x20),16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x28),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x4u32); }
            { let a = c.r[7].wrapping_add(0x66); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0x68); wm(m,a,16,c.get(1,16)); }
            pc = 0x5668d1; continue;
            }
            0x5664f8 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            pc = 0x566508; continue;
            }
            0x566502 => {
            c.set(1,32,rm(m,0x6df270u32,32));
            pc = 0x566508; continue;
            }
            0x566508 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,0x1u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x45),8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(3,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)));
            { let a = c.get(3,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(3,32); let r = c.add(a,0x1006u32,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            pc = 0x56653d; continue;
            }
            0x56653d => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[3]),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x566647; continue; }
            pc = 0x566551; continue;
            }
            0x566551 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(4) { pc = 0x56659c; continue; }
            pc = 0x566558; continue;
            }
            0x566558 => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(4) { pc = 0x56657d; continue; }
            pc = 0x56655b; continue;
            }
            0x56655b => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5665b9; continue; }
            pc = 0x56655e; continue;
            }
            0x56655e => {
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc08),32));
            pc = 0x5665b9; continue;
            }
            0x56657d => {
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x80c),32));
            pc = 0x5665b9; continue;
            }
            0x56659c => {
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x410),32));
            pc = 0x5665b9; continue;
            }
            0x5665b9 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x566627; continue; }
            pc = 0x5665c0; continue;
            }
            0x5665c0 => {
            c.set(3,16,rm(m,c.r[5].wrapping_add(0xe),16));
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(6,32,sx(c.get(3,16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
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
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x566621; continue; }
            pc = 0x566605; continue;
            }
            0x566605 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.r[7].wrapping_add(0x66); wm(m,a,16,c.get(3,16)); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x12),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[7].wrapping_add(0x68); wm(m,a,16,c.get(1,16)); }
            pc = 0x566621; continue;
            }
            0x566621 => {
            c.set(1,32,rm(m,0x6df270u32,32));
            pc = 0x566627; continue;
            }
            0x566627 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,16,c.get(0,8));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[3]),16),16); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            if c.cond(2) { pc = 0x566551; continue; }
            pc = 0x566647; continue;
            }
            0x566647 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(3,8)); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(3,32); let r = c.add(a,0x2u32,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(3,8); c.sub(a,0x4u32,8); }
            if c.cond(2) { pc = 0x56653d; continue; }
            pc = 0x566672; continue;
            }
            0x566672 => {
            { let a = rm(m,c.r[4].wrapping_add(0x18),32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(5) { pc = 0x566882; continue; }
            pc = 0x566680; continue;
            }
            0x566680 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = rm(m,c.r[3].wrapping_add(0x78),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x566764; continue; }
            pc = 0x56668e; continue;
            }
            0x56668e => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            { let a = rm(m,c.r[3].wrapping_add(c.r[1]).wrapping_add(0x1004),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x566760; continue; }
            pc = 0x5666a6; continue;
            }
            0x5666a6 => {
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x5666b4; continue;
            }
            0x5666b4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14),32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x566732; continue; }
            pc = 0x5666ce; continue;
            }
            0x5666ce => {
            c.set(3,16,rm(m,c.r[5].wrapping_add(0xe),16));
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(6,32,sx(c.get(3,16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
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
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x56672c; continue; }
            pc = 0x566713; continue;
            }
            0x566713 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x66); wm(m,a,16,c.get(3,16)); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x12),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            { let a = c.r[7].wrapping_add(0x68); wm(m,a,16,c.get(0,16)); }
            pc = 0x56672c; continue;
            }
            0x56672c => {
            c.set(1,32,rm(m,0x6df270u32,32));
            pc = 0x566732; continue;
            }
            0x566732 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,16,c.get(0,8));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[3].wrapping_add(c.r[1]).wrapping_add(0x1004),16),16); }
            if c.cond(2) { pc = 0x5666b4; continue; }
            pc = 0x566752; continue;
            }
            0x566752 => {
            { let a = rm(m,c.r[4].wrapping_add(0x18),32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(5) { pc = 0x566882; continue; }
            pc = 0x566760; continue;
            }
            0x566760 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x566764; continue;
            }
            0x566764 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x78),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x566839; continue; }
            pc = 0x56676f; continue;
            }
            0x56676f => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x8),16));
            c.set(2,16,rm(m,c.r[3].wrapping_add(0xa),16));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x56679d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x5667b7);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x5667bf);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x5667dc; continue; }
            pc = 0x5667c6; continue;
            }
            0x5667c6 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x5667d2; continue; }
            pc = 0x5667ca; continue;
            }
            0x5667ca => {
            { let a = c.get(6,32); let r = c.add(a,0x11u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,0x11u32,32); c.set(3,32,r); }
            pc = 0x5667e4; continue;
            }
            0x5667d2 => {
            if c.cond(14) { pc = 0x5667d7; continue; }
            pc = 0x5667d4; continue;
            }
            0x5667d4 => {
            { let a = c.get(6,32); let r = c.sub(a,0x11u32,32); c.set(6,32,r); }
            pc = 0x5667d7; continue;
            }
            0x5667d7 => {
            { let a = c.get(3,32); let r = c.sub(a,0x11u32,32); c.set(3,32,r); }
            pc = 0x5667e4; continue;
            }
            0x5667dc => {
            { let a = c.get(3,32); c.sub(a,0xbu32,32); }
            if c.cond(14) { pc = 0x5667e4; continue; }
            pc = 0x5667e1; continue;
            }
            0x5667e1 => {
            { let a = c.get(3,32); let r = c.sub(a,0x16u32,32); c.set(3,32,r); }
            pc = 0x5667e4; continue;
            }
            0x5667e4 => {
            c.set(1,32,c.get(3,32));
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(13) { pc = 0x566835; continue; }
            pc = 0x566811; continue;
            }
            0x566811 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x30),16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x34),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x4u32); }
            { let a = c.r[7].wrapping_add(0x66); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0x68); wm(m,a,16,c.get(1,16)); }
            pc = 0x5668d1; continue;
            }
            0x566835 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x566839; continue;
            }
            0x566839 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x8),16));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xa),16));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x566867);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x4u32); }
            pc = 0x5668d1; continue;
            }
            0x566882 => {
            { let a = rm(m,c.r[4].wrapping_add(0x12),8); c.sub(a,0x4u32,8); }
            if c.cond(4) { pc = 0x5668d1; continue; }
            pc = 0x566889; continue;
            }
            0x566889 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x566896; continue; }
            pc = 0x566891; continue;
            }
            0x566891 => {
            c.set(1,32,c.r[0].wrapping_add(0x8));
            pc = 0x566898; continue;
            }
            0x566896 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x566898; continue;
            }
            0x566898 => {
            c.set(6,32,sx(rm(m,c.r[1].wrapping_add(0x6),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5668ab; continue; }
            pc = 0x5668a6; continue;
            }
            0x5668a6 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x5668ad; continue;
            }
            0x5668ab => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5668ad; continue;
            }
            0x5668ad => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            pc = 0x5668d1; continue;
            }
            0x5668d1 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(3,32,c.get(6,32));
            if c.cond(15) { pc = 0x5668d9; continue; }
            pc = 0x5668d7; continue;
            }
            0x5668d7 => {
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            pc = 0x5668d9; continue;
            }
            0x5668d9 => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(0,32,c.get(2,32));
            if c.cond(15) { pc = 0x5668e1; continue; }
            pc = 0x5668df; continue;
            }
            0x5668df => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x5668e1; continue;
            }
            0x5668e1 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(5,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x566910; continue; }
            pc = 0x5668fe; continue;
            }
            0x5668fe => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x566909; continue; }
            pc = 0x566902; continue;
            }
            0x566902 => {
            c.set(0,8,rm(m,0x67792au32,8));
            pc = 0x566963; continue;
            }
            0x566909 => {
            c.set(0,8,rm(m,0x67792eu32,8));
            pc = 0x566963; continue;
            }
            0x566910 => {
            c.set(1,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(3,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x56693f; continue; }
            pc = 0x56692d; continue;
            }
            0x56692d => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x566938; continue; }
            pc = 0x566931; continue;
            }
            0x566931 => {
            c.set(0,8,rm(m,0x67792cu32,8));
            pc = 0x566963; continue;
            }
            0x566938 => {
            c.set(0,8,rm(m,0x677928u32,8));
            pc = 0x566963; continue;
            }
            0x56693f => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x566955; continue; }
            pc = 0x566943; continue;
            }
            0x566943 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x56694e; continue; }
            pc = 0x566947; continue;
            }
            0x566947 => {
            c.set(0,8,rm(m,0x67792bu32,8));
            pc = 0x566963; continue;
            }
            0x56694e => {
            c.set(0,8,rm(m,0x67792du32,8));
            pc = 0x566963; continue;
            }
            0x566955 => {
            c.set(0,8,rm(m,0x677929u32,8));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(15) { pc = 0x566963; continue; }
            pc = 0x56695e; continue;
            }
            0x56695e => {
            c.set(0,8,rm(m,0x67792fu32,8));
            pc = 0x566963; continue;
            }
            0x566963 => {
            { let a = rm(m,c.r[4].wrapping_add(0x18),32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(5) { pc = 0x566997; continue; }
            pc = 0x56696d; continue;
            }
            0x56696d => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x12),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x56697e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x565e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[7].wrapping_add(0x20));
            c.push(m, 0x56698d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x566997 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.r[7].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(1,8); let r = c.add(a,0x80u32,8); c.set(1,8,r); }
            c.push(m,0xfu32);
            { let a = c.r[7].wrapping_add(0x5c); wm(m,a,8,c.get(1,8)); }
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[7].wrapping_add(0x20));
            { let a = c.r[7].wrapping_add(0x3e); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x5669ba);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005669d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5669d0;
    loop {
        match pc {
            0x5669d0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x45),8));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(0,32));
            c.push(m,c.get(5,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x7fffffffu32); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x566a28);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(6,32,rm(m,0x6e51acu32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x566a62; continue; }
            pc = 0x566a3c; continue;
            }
            0x566a3c => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc318),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(7,32)); }
            if c.cond(3) { pc = 0x566a62; continue; }
            pc = 0x566a4e; continue;
            }
            0x566a4e => {
            c.set(3,32,c.get(0,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc30c),32));
            { let a = c.get(3,32); let r = c.imul(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[3].wrapping_mul(4)),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x566a71; continue; }
            pc = 0x566a62; continue;
            }
            0x566a62 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0x7fffffffu32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            0x566a71 => {
            c.set(3,32,c.r[2].wrapping_add(0xfffffffe));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = rm(m,c.r[4].wrapping_add(0x3c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x566a90; continue; }
            pc = 0x566a82; continue;
            }
            0x566a82 => {
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x3c),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x566a9f; continue; }
            pc = 0x566a90; continue;
            }
            0x566a90 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0x7fffffffu32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            0x566a9f => {
            { let a = c.get(2,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x566aa9; continue; }
            pc = 0x566aa4; continue;
            }
            0x566aa4 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            pc = 0x566aa9; continue;
            }
            0x566aa9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x566ad2; continue; }
            pc = 0x566ab4; continue;
            }
            0x566ab4 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc318),32));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x566ad2; continue; }
            pc = 0x566abe; continue;
            }
            0x566abe => {
            c.set(3,32,c.get(0,32));
            c.set(6,32,rm(m,c.r[6].wrapping_add(0xc30c),32));
            { let a = c.get(3,32); let r = c.imul(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(4)),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x566ae1; continue; }
            pc = 0x566ad2; continue;
            }
            0x566ad2 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0x7fffffffu32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            0x566ae1 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x566af6; continue; }
            pc = 0x566ae6; continue;
            }
            0x566ae6 => {
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x566af6; continue; }
            pc = 0x566aea; continue;
            }
            0x566aea => {
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x566b05; continue; }
            pc = 0x566af6; continue;
            }
            0x566af6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0x7fffffffu32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            0x566b05 => {
            { let a = rm(m,c.r[5].wrapping_add(0x242),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x566bed; continue; }
            pc = 0x566b13; continue;
            }
            0x566b13 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x250),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x6c),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x45),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x566b45);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x566b70);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58cfb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x566b8a);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x566b92);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x566baf; continue; }
            pc = 0x566b99; continue;
            }
            0x566b99 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x566ba5; continue; }
            pc = 0x566b9d; continue;
            }
            0x566b9d => {
            { let a = c.get(6,32); let r = c.add(a,0x11u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,0x11u32,32); c.set(7,32,r); }
            pc = 0x566bb7; continue;
            }
            0x566ba5 => {
            if c.cond(14) { pc = 0x566baa; continue; }
            pc = 0x566ba7; continue;
            }
            0x566ba7 => {
            { let a = c.get(6,32); let r = c.sub(a,0x11u32,32); c.set(6,32,r); }
            pc = 0x566baa; continue;
            }
            0x566baa => {
            { let a = c.get(7,32); let r = c.sub(a,0x11u32,32); c.set(7,32,r); }
            pc = 0x566bb7; continue;
            }
            0x566baf => {
            { let a = c.get(7,32); c.sub(a,0xbu32,32); }
            if c.cond(14) { pc = 0x566bb7; continue; }
            pc = 0x566bb4; continue;
            }
            0x566bb4 => {
            { let a = c.get(7,32); let r = c.sub(a,0x16u32,32); c.set(7,32,r); }
            pc = 0x566bb7; continue;
            }
            0x566bb7 => {
            c.set(1,32,c.get(7,32));
            c.set(0,32,0x2e8ba2e9u32);
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
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
            { let a = c.get(0,32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(13) { pc = 0x566bf1; continue; }
            pc = 0x566be4; continue;
            }
            0x566be4 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x566dfa; continue;
            }
            0x566bed => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x566bf1; continue;
            }
            0x566bf1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x1u32); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x45),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x3u32); }
            c.set(7,32,c.get(2,32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x1006u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x566c30; continue;
            }
            0x566c30 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.set(0,16,rm(m,c.r[0],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x566d26; continue; }
            pc = 0x566c49; continue;
            }
            0x566c49 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x566c93; continue; }
            pc = 0x566c50; continue;
            }
            0x566c50 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x566c75; continue; }
            pc = 0x566c53; continue;
            }
            0x566c53 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x566cb0; continue; }
            pc = 0x566c56; continue;
            }
            0x566c56 => {
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc08),32));
            pc = 0x566cb0; continue;
            }
            0x566c75 => {
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x80c),32));
            pc = 0x566cb0; continue;
            }
            0x566c93 => {
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x410),32));
            pc = 0x566cb0; continue;
            }
            0x566cb0 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x566cb9; continue; }
            pc = 0x566cb4; continue;
            }
            0x566cb4 => {
            c.set(0,32,c.r[5].wrapping_add(0x8));
            pc = 0x566cbb; continue;
            }
            0x566cb9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x566cbb; continue;
            }
            0x566cbb => {
            c.set(6,32,sx(rm(m,c.r[0].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(6,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x566ccc; continue; }
            pc = 0x566cc7; continue;
            }
            0x566cc7 => {
            c.set(0,32,c.r[5].wrapping_add(0x8));
            pc = 0x566cce; continue;
            }
            0x566ccc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x566cce; continue;
            }
            0x566cce => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
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
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x566d06; continue; }
            pc = 0x566d02; continue;
            }
            0x566d02 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x566d06; continue;
            }
            0x566d06 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x3c),8));
            c.set(1,32,rm(m,0x6df270u32,32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x24),16),16); }
            if c.cond(2) { pc = 0x566c49; continue; }
            pc = 0x566d26; continue;
            }
            0x566d26 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x566c30; continue; }
            pc = 0x566d49; continue;
            }
            0x566d49 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(5) { pc = 0x566dfa; continue; }
            pc = 0x566d57; continue;
            }
            0x566d57 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(3,8)); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0x1004),16));
            { let a = c.get(5,16); c.and(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x566dfa; continue; }
            pc = 0x566d71; continue;
            }
            0x566d71 => {
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(7,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)));
            pc = 0x566d83; continue;
            }
            0x566d7d => {
            c.set(1,32,rm(m,0x6df270u32,32));
            pc = 0x566d83; continue;
            }
            0x566d83 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x566d9b; continue; }
            pc = 0x566d96; continue;
            }
            0x566d96 => {
            c.set(1,32,c.r[0].wrapping_add(0x8));
            pc = 0x566d9d; continue;
            }
            0x566d9b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x566d9d; continue;
            }
            0x566d9d => {
            c.set(6,32,sx(rm(m,c.r[1].wrapping_add(0x6),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x566dae; continue; }
            pc = 0x566da9; continue;
            }
            0x566da9 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x566db0; continue;
            }
            0x566dae => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x566db0; continue;
            }
            0x566db0 => {
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x1c),32),32); c.set(0,32,r); }
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
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.get(2,32));
            c.set(2,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x566dea; continue; }
            pc = 0x566de6; continue;
            }
            0x566de6 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x566dea; continue;
            }
            0x566dea => {
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(0,8,c.get(3,8));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x566d7d; continue; }
            pc = 0x566dfa; continue;
            }
            0x566dfa => {
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x566e11);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x566e19);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00566e30(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x566e30;
    loop {
        match pc {
            0x566e30 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x31),8));
            c.set(7,32,rm(m,0x6e173cu32,32));
            c.set(0,32,c.get(3,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[7]),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x566ed2; continue; }
            pc = 0x566e58; continue;
            }
            0x566e58 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x64),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x566e75; continue; }
            pc = 0x566e5f; continue;
            }
            0x566e5f => {
            c.set(2,32,rm(m,0x6e1740u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[1].wrapping_add(c.r[2]),8));
            { let a = c.r[6].wrapping_add(0x64); wm(m,a,8,c.get(1,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x566e75 => {
            c.set(1,8,c.get(2,8));
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[6].wrapping_add(0x64); wm(m,a,8,c.get(1,8)); }
            if c.cond(4) { pc = 0x566e88; continue; }
            pc = 0x566e7e; continue;
            }
            0x566e7e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x566e88 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xee66u32);
            c.push(m, 0x566e98);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x566ed2; continue; }
            pc = 0x566e9c; continue;
            }
            0x566e9c => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xee66u32);
            c.push(m, 0x566eac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x566eb3);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let r = c.sub(a,0x119au32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x566ed2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x595b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x566ed2; continue;
            }
            0x566ed2 => {
            { let a = c.get(3,8); c.sub(a,0x11u32,8); }
            if c.cond(5) { pc = 0x566f6e; continue; }
            pc = 0x566edb; continue;
            }
            0x566edb => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x68),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x66),16));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x566ef9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5c),8));
            { let a = c.get(1,32); let r = c.add(a,0xffffff80u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x566f4c; continue; }
            pc = 0x566f16; continue;
            }
            0x566f16 => {
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x566f7c)); continue;
            }
            0x566f1d => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x45),8));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,0x6df270u32,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = rm(m,c.r[1].wrapping_add(0x242),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x566f47; continue; }
            pc = 0x566f40; continue;
            }
            0x566f40 => {
            { let a = c.get(0,32); let r = c.add(a,0x6004u32,32); c.set(0,32,r); }
            pc = 0x566f4c; continue;
            }
            0x566f47 => {
            { let a = c.get(0,32); let r = c.add(a,0x603eu32,32); c.set(0,32,r); }
            pc = 0x566f4c; continue;
            }
            0x566f4c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x3u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,0x17u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x566f68);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x45),8));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,8,c.get(1,8)); }
            pc = 0x566f6e; continue;
            }
            0x566f6e => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0056a700(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x56a700;
    loop {
        match pc {
            0x56a700 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,0x6e1854u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x56a766; continue; }
            pc = 0x56a71e; continue;
            }
            0x56a71e => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x8),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x56a73d; continue; }
            pc = 0x56a726; continue;
            }
            0x56a726 => {
            c.push(m, 0x56a72b);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x56a743; continue; }
            pc = 0x56a73d; continue;
            }
            0x56a73d => {
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,8,c.get(3,8)); }
            pc = 0x56a743; continue;
            }
            0x56a743 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x8),8));
            c.set(0,32,rm(m,0x6e1854u32,32));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x56a71e; continue; }
            pc = 0x56a766; continue;
            }
            0x56a766 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x88),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x56a776; continue; }
            pc = 0x56a772; continue;
            }
            0x56a772 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x56a77e; continue;
            }
            0x56a776 => {
            c.set(0,32,rm(m,0x6e1858u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            pc = 0x56a77e; continue;
            }
            0x56a77e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,0x0u32);
            c.set(2,32,c.get(0,32));
            c.push(m,0x0u32);
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,8); let r = c.and(a,0x1u32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x7fu32,8); c.set(0,8,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x56a7b0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40))]; c.r[0] = host.call(m, 0x55e620, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(44);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0056a7f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x56a7f0;
    loop {
        match pc {
            0x56a7f0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e1854u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x88),8));
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.push(m,c.get(6,32));
            if c.cond(2) { pc = 0x56a80c; continue; }
            pc = 0x56a808; continue;
            }
            0x56a808 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x56a815; continue;
            }
            0x56a80c => {
            c.set(1,32,rm(m,0x6e1858u32,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x56a815; continue;
            }
            0x56a815 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x6a),8));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m, 0x56a836);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x56a851; continue; }
            pc = 0x56a83a; continue;
            }
            0x56a83a => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x56a84c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x56a851 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x6a),8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(6,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x14),16),16); }
            if c.cond(5) { pc = 0x56a8cd; continue; }
            pc = 0x56a874; continue;
            }
            0x56a874 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x16),16),16); }
            if c.cond(5) { pc = 0x56a8cd; continue; }
            pc = 0x56a87e; continue;
            }
            0x56a87e => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x36),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x56a88f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,16,c.get(0,16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x44),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x66),16));
            c.set(6,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            { let b = c.get(6,32); c.div32(b); }
            pc = 0x56a8b7; continue;
            }
            0x56a8b7 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x56a8c8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x56a8cd => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x36),16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x56a911; continue; }
            pc = 0x56a8d6; continue;
            }
            0x56a8d6 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x38),16),16); }
            if c.cond(5) { pc = 0x56a911; continue; }
            pc = 0x56a8e0; continue;
            }
            0x56a8e0 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x79),8));
            c.set(1,32,c.get(3,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.push(m,0x0u32);
            if c.cond(4) { pc = 0x56a8fe; continue; }
            pc = 0x56a8eb; continue;
            }
            0x56a8eb => {
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x56a8f9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x56a8fe => {
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x56a90c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x56a911 => {
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x3e),16),16); }
            if c.cond(5) { pc = 0x56a974; continue; }
            pc = 0x56a917; continue;
            }
            0x56a917 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x40),16),16); }
            if c.cond(5) { pc = 0x56a974; continue; }
            pc = 0x56a921; continue;
            }
            0x56a921 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x56a933; continue; }
            pc = 0x56a92b; continue;
            }
            0x56a92b => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            pc = 0x56a939; continue;
            }
            0x56a933 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            pc = 0x56a939; continue;
            }
            0x56a939 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x56a940);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,16,c.get(0,16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x68),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3c),16));
            c.set(6,32,c.get(1,32));
            { let a = c.get(6,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x100u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x200u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            pc = 0x56a8b7; continue;
            }
            0x56a974 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x56a982);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,16,c.get(0,16));
            c.set(0,32,0x10u32);
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x10u32,32); }
            if c.cond(3) { pc = 0x56a997; continue; }
            pc = 0x56a995; continue;
            }
            0x56a995 => {
            c.set(0,32,c.get(7,32));
            pc = 0x56a997; continue;
            }
            0x56a997 => {
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6f),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x56a9b5; continue; }
            pc = 0x56a9a4; continue;
            }
            0x56a9a4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x66),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            pc = 0x56aa01; continue;
            }
            0x56a9b5 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x44),16));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x254),16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x100u32,32); }
            if c.cond(6) { pc = 0x56a9d7; continue; }
            pc = 0x56a9d2; continue;
            }
            0x56a9d2 => {
            c.set(1,32,0x100u32);
            pc = 0x56a9d7; continue;
            }
            0x56a9d7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[2].wrapping_add(c.r[1]));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            if c.cond(6) { pc = 0x56a9ff; continue; }
            pc = 0x56a9e8; continue;
            }
            0x56a9e8 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x66),16));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            pc = 0x56aa01; continue;
            }
            0x56a9ff => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x56aa01; continue;
            }
            0x56aa01 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.push(m,0x10u32);
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x56aa15);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
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

pub fn f_0056f0b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x56f0b0;
    loop {
        match pc {
            0x56f0b0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,0x6e2d8cu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x56f116; continue; }
            pc = 0x56f0ce; continue;
            }
            0x56f0ce => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x8),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x56f0ed; continue; }
            pc = 0x56f0d6; continue;
            }
            0x56f0d6 => {
            c.push(m, 0x56f0db);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x56f0f3; continue; }
            pc = 0x56f0ed; continue;
            }
            0x56f0ed => {
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,8,c.get(3,8)); }
            pc = 0x56f0f3; continue;
            }
            0x56f0f3 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x8),8));
            c.set(0,32,rm(m,0x6e2d8cu32,32));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x56f0ce; continue; }
            pc = 0x56f116; continue;
            }
            0x56f116 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x88),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x56f126; continue; }
            pc = 0x56f122; continue;
            }
            0x56f122 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x56f12e; continue;
            }
            0x56f126 => {
            c.set(0,32,rm(m,0x6e2d90u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            pc = 0x56f12e; continue;
            }
            0x56f12e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,0x0u32);
            c.set(2,32,c.get(0,32));
            c.push(m,0x0u32);
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,8); let r = c.and(a,0x1u32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x7fu32,8); c.set(0,8,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x56f160);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40))]; c.r[0] = host.call(m, 0x55e620, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(44);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0056f1a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x56f1a0;
    loop {
        match pc {
            0x56f1a0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e2d8cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x88),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x56f1bc; continue; }
            pc = 0x56f1b8; continue;
            }
            0x56f1b8 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x56f1c5; continue;
            }
            0x56f1bc => {
            c.set(1,32,rm(m,0x6e2d90u32,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x56f1c5; continue;
            }
            0x56f1c5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6a),8));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m, 0x56f1e6);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x56f201; continue; }
            pc = 0x56f1ea; continue;
            }
            0x56f1ea => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x56f1fc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x56f201 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6a),8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(6,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x14),16),16); }
            if c.cond(5) { pc = 0x56f27d; continue; }
            pc = 0x56f224; continue;
            }
            0x56f224 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x16),16),16); }
            if c.cond(5) { pc = 0x56f27d; continue; }
            pc = 0x56f22e; continue;
            }
            0x56f22e => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x36),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x56f23f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,16,c.get(0,16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x44),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x0u32);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x66),16));
            c.push(m,0x1u32);
            c.set(6,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(1,32,c.get(7,32));
            { let b = c.get(6,32); c.div32(b); }
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x56f278);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x56f27d => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x36),16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x56f2f2; continue; }
            pc = 0x56f286; continue;
            }
            0x56f286 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x38),16),16); }
            if c.cond(5) { pc = 0x56f2f2; continue; }
            pc = 0x56f290; continue;
            }
            0x56f290 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x79),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x56f1ea; continue; }
            pc = 0x56f29b; continue;
            }
            0x56f29b => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x34),8));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x56f2a6; continue; }
            pc = 0x56f2a2; continue;
            }
            0x56f2a2 => {
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            if c.cond(5) { pc = 0x56f2db; continue; }
            pc = 0x56f2a6; continue;
            }
            0x56f2a6 => {
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x14),8));
            c.push(m,0x0u32);
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0x8u32);
            c.push(m,0x12u32);
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x56f2cd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x590610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.push(m,0xffffff80u32);
            c.push(m,0x3e8u32);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x56f2db);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x57af40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x56f2db; continue;
            }
            0x56f2db => {
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x56f2ed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x56f2f2 => {
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x3e),16),16); }
            if c.cond(5) { pc = 0x56f366; continue; }
            pc = 0x56f2f8; continue;
            }
            0x56f2f8 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x40),16),16); }
            if c.cond(5) { pc = 0x56f366; continue; }
            pc = 0x56f302; continue;
            }
            0x56f302 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x56f314; continue; }
            pc = 0x56f30c; continue;
            }
            0x56f30c => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            pc = 0x56f31a; continue;
            }
            0x56f314 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            pc = 0x56f31a; continue;
            }
            0x56f31a => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x56f321);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,16,c.get(0,16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x68),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3c),16));
            c.set(6,32,c.get(1,32));
            { let a = c.get(6,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x100u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x200u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x0u32);
            { let b = c.get(1,32); c.div32(b); }
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x56f361);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x56f366 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x56f374);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,16,c.get(0,16));
            c.set(0,32,0x10u32);
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x10u32,32); }
            if c.cond(3) { pc = 0x56f389; continue; }
            pc = 0x56f387; continue;
            }
            0x56f387 => {
            c.set(0,32,c.get(3,32));
            pc = 0x56f389; continue;
            }
            0x56f389 => {
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6f),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x56f3a7; continue; }
            pc = 0x56f396; continue;
            }
            0x56f396 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x66),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            pc = 0x56f3f3; continue;
            }
            0x56f3a7 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x44),16));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x254),16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x100u32,32); }
            if c.cond(6) { pc = 0x56f3c9; continue; }
            pc = 0x56f3c4; continue;
            }
            0x56f3c4 => {
            c.set(1,32,0x100u32);
            pc = 0x56f3c9; continue;
            }
            0x56f3c9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[2].wrapping_add(c.r[1]));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            if c.cond(6) { pc = 0x56f3f1; continue; }
            pc = 0x56f3da; continue;
            }
            0x56f3da => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x66),16));
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            { let b = c.get(3,32); c.div32(b); }
            pc = 0x56f3f3; continue;
            }
            0x56f3f1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x56f3f3; continue;
            }
            0x56f3f3 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.push(m,0x10u32);
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x56f407);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x55e8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00572a90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x572a90;
    loop {
        match pc {
            0x572a90 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,0x6e2dd0u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x572b12; continue; }
            pc = 0x572aaf; continue;
            }
            0x572aaf => {
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            pc = 0x572ab9; continue;
            }
            0x572ab9 => {
            c.set(2,32,rm(m,0x6e2dd8u32,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2]),8));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(5) { pc = 0x572afe; continue; }
            pc = 0x572ac9; continue;
            }
            0x572ac9 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xc),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x572ae8; continue; }
            pc = 0x572ad1; continue;
            }
            0x572ad1 => {
            c.push(m, 0x572ad6);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x572aee; continue; }
            pc = 0x572ae8; continue;
            }
            0x572ae8 => {
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,8,c.get(3,8)); }
            pc = 0x572aee; continue;
            }
            0x572aee => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xc),8));
            c.set(1,32,rm(m,0x6e2dd0u32,32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(0,8)); }
            pc = 0x572afe; continue;
            }
            0x572afe => {
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x572ab9; continue; }
            pc = 0x572b11; continue;
            }
            0x572b11 => {
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x572b12; continue;
            }
            0x572b12 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x88),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x572b22; continue; }
            pc = 0x572b1e; continue;
            }
            0x572b1e => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x572b2b; continue;
            }
            0x572b22 => {
            c.set(2,32,rm(m,0x6e2dd4u32,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x572b2b; continue;
            }
            0x572b2b => {
            c.set(2,32,rm(m,0x6e2dd8u32,32));
            c.push(m,0x0u32);
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[2]),8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,8); let r = c.and(a,0x7fu32,8); c.set(0,8,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,8); let r = c.and(a,0x1u32,8); c.set(2,8,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x572b65);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40))]; c.r[0] = host.call(m, 0x55e620, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(44);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005595f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5595f0;
    loop {
        match pc {
            0x5595f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x30u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x55965d; continue; }
            pc = 0x559608; continue;
            }
            0x559608 => {
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x559617);
            { let args = []; c.r[0] = host.call(m, 0x559260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x559640; continue; }
            pc = 0x55961e; continue;
            }
            0x55961e => {
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a4ccu32,32); c.push(m, 0x55962f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x559636);
            { let args = []; c.r[0] = host.call(m, 0x5596c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x559640 => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x55964c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x559653);
            { let args = []; c.r[0] = host.call(m, 0x5596c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x55965d => {
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x55966c);
            { let args = []; c.r[0] = host.call(m, 0x559260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55968a; continue; }
            pc = 0x559673; continue;
            }
            0x559673 => {
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4ccu32,32); c.push(m, 0x559688); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x55969a; continue;
            }
            0x55968a => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x55969a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x55969a; continue;
            }
            0x55969a => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m, 0x5596aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5596b1);
            { let args = []; c.r[0] = host.call(m, 0x5596c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0055e250(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x55e250;
    loop {
        match pc {
            0x55e250 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x80u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.and(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x59),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55e29e; continue; }
            pc = 0x55e279; continue;
            }
            0x55e279 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.add(a,0xffbau32,16); { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xdu32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa));
            pc = 0x55e342; continue;
            }
            0x55e29e => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x55e2a5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(5) { pc = 0x55e2cb; continue; }
            pc = 0x55e2af; continue;
            }
            0x55e2af => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.add(a,0xfff5u32,16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xbu32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa));
            pc = 0x55e342; continue;
            }
            0x55e2cb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x82),16));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8));
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x84),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x55e342; continue; }
            pc = 0x55e30b; continue;
            }
            0x55e30b => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x82),16));
            { let a = rm(m,c.r[0],16); let r = c.add(a,c.get(1,16),16); { let a = c.r[0]; wm(m,a,16,r); } }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x84),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa));
            pc = 0x55e342; continue;
            }
            0x55e342 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x55e351);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x59),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55e37e; continue; }
            pc = 0x55e35b; continue;
            }
            0x55e35b => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.add(a,0x46u32,16); { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xfff3u32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x55e37e => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x55e385); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(5) { pc = 0x55e3ac; continue; }
            pc = 0x55e38c; continue;
            }
            0x55e38c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.add(a,0xbu32,16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,0xfff5u32,16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x55e3ac => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x82),16));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8); wm(m,a,16,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8));
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x84),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.sub(a,c.get(2,16),16); { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x55e423; continue; }
            pc = 0x55e3ec; continue;
            }
            0x55e3ec => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x82),16));
            { let a = rm(m,c.r[0],16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[0]; wm(m,a,16,r); } }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x84),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.sub(a,c.get(2,16),16); { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa));
            pc = 0x55e423; continue;
            }
            0x55e423 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0055e8c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x55e8c0;
    loop {
        match pc {
            0x55e8c0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x34u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x64),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x55e8f3; continue; }
            pc = 0x55e8d0; continue;
            }
            0x55e8d0 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55e918; continue; }
            pc = 0x55e8d7; continue;
            }
            0x55e8d7 => {
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[0].wrapping_add(0x3f8));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x3f8),32));
            { let target = rm(m,c.r[0].wrapping_add(0x4),32); c.push(m, 0x55e8f3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x55e8f3; continue;
            }
            0x55e8f3 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x5c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55e9b6; continue; }
            pc = 0x55e8ff; continue;
            }
            0x55e8ff => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55e9b1; continue; }
            pc = 0x55e90b; continue;
            }
            0x55e90b => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x4c),8));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,c.get(1,8)); }
            pc = 0x55e9b6; continue;
            }
            0x55e918 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x59),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55e99b; continue; }
            pc = 0x55e91f; continue;
            }
            0x55e91f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x40),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x55e99b; continue; }
            pc = 0x55e93a; continue;
            }
            0x55e93a => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[0].wrapping_add(0xc),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x55e974; continue; }
            pc = 0x55e952; continue;
            }
            0x55e952 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x214),32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x8c),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x55e974; continue; }
            pc = 0x55e965; continue;
            }
            0x55e965 => {
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55e974 => {
            { let a = rm(m,c.r[0].wrapping_add(0x10),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x55e99b; continue; }
            pc = 0x55e979; continue;
            }
            0x55e979 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x210),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8c),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x55e99b; continue; }
            pc = 0x55e98c; continue;
            }
            0x55e98c => {
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55e99b => {
            c.push(m,0x2u32);
            c.set(1,32,c.r[2].wrapping_add(0x20));
            c.push(m, 0x55e9a5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55e9b1 => {
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            pc = 0x55e9b6; continue;
            }
            0x55e9b6 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,0x0u32);
            c.set(1,32,c.r[6].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x55e9cd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x557340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(3,8,c.get(0,8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(3,8)); }
            if c.cond(4) { pc = 0x55ea99; continue; }
            pc = 0x55e9de; continue;
            }
            0x55e9de => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x55e9e7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55ea07; continue; }
            pc = 0x55e9eb; continue;
            }
            0x55e9eb => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x84),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x82),16),16)&0xffffffffu32);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x59),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55ea07);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x55d0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x55ea07; continue;
            }
            0x55ea07 => {
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[6].wrapping_add(0x15),8),8); }
            if c.cond(4) { pc = 0x55ea99; continue; }
            pc = 0x55ea10; continue;
            }
            0x55ea10 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(5,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(5,32),32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x86),8));
            { let a = c.get(7,32); let r = c.imul(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x34),8));
            { let a = c.get(7,32); let r = c.add(a,0x40u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            if c.cond(4) { pc = 0x55ea3f; continue; }
            pc = 0x55ea33; continue;
            }
            0x55ea33 => {
            { let a = c.get(0,8); c.sub(a,0xdu32,8); }
            if c.cond(4) { pc = 0x55ea3f; continue; }
            pc = 0x55ea37; continue;
            }
            0x55ea37 => {
            { let a = c.get(0,8); c.sub(a,0x18u32,8); }
            if c.cond(4) { pc = 0x55ea3f; continue; }
            pc = 0x55ea3b; continue;
            }
            0x55ea3b => {
            { let a = c.get(0,8); c.sub(a,0x19u32,8); }
            if c.cond(5) { pc = 0x55ea42; continue; }
            pc = 0x55ea3f; continue;
            }
            0x55ea3f => {
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            pc = 0x55ea42; continue;
            }
            0x55ea42 => {
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x55ea8e; continue; }
            pc = 0x55ea46; continue;
            }
            0x55ea46 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x59),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55ea8e; continue; }
            pc = 0x55ea4d; continue;
            }
            0x55ea4d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m, 0x55ea69);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55ea8e; continue; }
            pc = 0x55ea6d; continue;
            }
            0x55ea6d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = rm(m,c.r[1].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x55ea8e; continue; }
            pc = 0x55ea8c; continue;
            }
            0x55ea8c => {
            { let a = c.get(7,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            pc = 0x55ea8e; continue;
            }
            0x55ea8e => {
            c.set(2,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            pc = 0x55ea99; continue;
            }
            0x55ea99 => {
            c.set(0,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x86),8));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x414),32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(7,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x34),8));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x55eb13; continue; }
            pc = 0x55eac6; continue;
            }
            0x55eac6 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x59),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55eb13; continue; }
            pc = 0x55eacd; continue;
            }
            0x55eacd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m, 0x55eae9);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55eb13; continue; }
            pc = 0x55eaed; continue;
            }
            0x55eaed => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = rm(m,c.r[1].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x55eb13; continue; }
            pc = 0x55eb0c; continue;
            }
            0x55eb0c => {
            c.set(2,32,c.r[7].wrapping_add(c.r[7]));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            pc = 0x55eb13; continue;
            }
            0x55eb13 => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(3,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x55eb31);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x59),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55ec5e; continue; }
            pc = 0x55eb3c; continue;
            }
            0x55eb3c => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x55eb45); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55eb73; continue; }
            pc = 0x55eb49; continue;
            }
            0x55eb49 => {
            { let a = rm(m,c.r[6].wrapping_add(0x5a),8); c.sub(a,0x28u32,8); }
            if c.cond(6) { pc = 0x55eb73; continue; }
            pc = 0x55eb4f; continue;
            }
            0x55eb4f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m, 0x55eb6b);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55ec52; continue; }
            pc = 0x55eb73; continue;
            }
            0x55eb73 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x768),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m,c.get(0,32));
            c.push(m, 0x55eb9c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x55ec52; continue; }
            pc = 0x55eba6; continue;
            }
            0x55eba6 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x55ebb6);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(13) { pc = 0x55ec18; continue; }
            pc = 0x55ebbe; continue;
            }
            0x55ebbe => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x55ebcf);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(13) { pc = 0x55ec18; continue; }
            pc = 0x55ebd7; continue;
            }
            0x55ebd7 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55ec18; continue; }
            pc = 0x55ebde; continue;
            }
            0x55ebde => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x55ec18; continue; }
            pc = 0x55ebe5; continue;
            }
            0x55ebe5 => {
            { let a = c.get(0,8); c.sub(a,0x31u32,8); }
            if c.cond(4) { pc = 0x55ec18; continue; }
            pc = 0x55ebe9; continue;
            }
            0x55ebe9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x55ec03);
            { let args = []; c.r[0] = host.call(m, 0x5252f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0xffu32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x55ec5e; continue; }
            pc = 0x55ec18; continue;
            }
            0x55ec18 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x4),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(7,16); c.sub(a,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x776),16),16); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            if c.cond(3) { pc = 0x55ec52; continue; }
            pc = 0x55ec3c; continue;
            }
            0x55ec3c => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x55ec48);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x55eba6; continue; }
            pc = 0x55ec52; continue;
            }
            0x55ec52 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55ec5e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x34),8));
            { let a = c.get(0,8); c.sub(a,0xb1u32,8); }
            if c.cond(5) { pc = 0x55ed36; continue; }
            pc = 0x55ec84; continue;
            }
            0x55ec84 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x68),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x55ec95; continue; }
            pc = 0x55ec8d; continue;
            }
            0x55ec8d => {
            { let a = c.r[6].wrapping_add(0x68); wm(m,a,16,0xffffu32); }
            pc = 0x55ecb3; continue;
            }
            0x55ec95 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(7,32,c.get(0,32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x55ecdd; continue; }
            pc = 0x55ecaa; continue;
            }
            0x55ecaa => {
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x68); wm(m,a,16,c.get(0,16)); }
            pc = 0x55ecb3; continue;
            }
            0x55ecb3 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55ecc4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55ecd1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55ecdd => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x55ecf3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x6e),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6c),16));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x55ed09);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x70),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55ec52; continue; }
            pc = 0x55ed14; continue;
            }
            0x55ed14 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x55ed26);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.r[6].wrapping_add(0x70); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55ed36 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x58),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x55edf0; continue; }
            pc = 0x55ed41; continue;
            }
            0x55ed41 => {
            { let a = c.get(0,8); c.sub(a,0x91u32,8); }
            if c.cond(4) { pc = 0x55edb9; continue; }
            pc = 0x55ed45; continue;
            }
            0x55ed45 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x55ed4c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(4) { pc = 0x55ed58; continue; }
            pc = 0x55ed50; continue;
            }
            0x55ed50 => {
            c.set(0,8,rm(m,0x67792du32,8));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            pc = 0x55ed58; continue;
            }
            0x55ed58 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x91u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m, 0x55ed6b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xee48u32);
            c.set(3,8,c.get(0,8));
            c.push(m, 0x55ed7d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x55edbd; continue; }
            pc = 0x55ed81; continue;
            }
            0x55ed81 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xee48u32);
            c.push(m, 0x55ed91);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x55ed98);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let r = c.sub(a,0x11b8u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x55edb7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x595b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x55edbd; continue;
            }
            0x55edb9 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x10),8));
            pc = 0x55edbd; continue;
            }
            0x55edbd => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55edce);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x55ede4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55edf0 => {
            c.push(m, 0x55edf5);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55ee5e; continue; }
            pc = 0x55edf9; continue;
            }
            0x55edf9 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x55ee5e; continue; }
            pc = 0x55edfd; continue;
            }
            0x55edfd => {
            { let a = rm(m,c.r[5].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x55ee5e; continue; }
            pc = 0x55ee04; continue;
            }
            0x55ee04 => {
            { let a = rm(m,c.r[6].wrapping_add(0x7b),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x55ee5e; continue; }
            pc = 0x55ee0a; continue;
            }
            0x55ee0a => {
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x10),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[5].wrapping_add(0x36),16),16); }
            if c.cond(5) { pc = 0x55ee5e; continue; }
            pc = 0x55ee14; continue;
            }
            0x55ee14 => {
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[5].wrapping_add(0x38),16),16); }
            if c.cond(5) { pc = 0x55ee5e; continue; }
            pc = 0x55ee1e; continue;
            }
            0x55ee1e => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x59),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55ee5e; continue; }
            pc = 0x55ee25; continue;
            }
            0x55ee25 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x7a4));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x7ac),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x68),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x2c),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[2]),8); c.sub(a,0x0u32,8); }
            if c.cond(9) { pc = 0x55ec52; continue; }
            pc = 0x55ee5e; continue;
            }
            0x55ee5e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m, 0x55ee7a);
            { let args = []; c.r[0] = host.call(m, 0x633ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55eead; continue; }
            pc = 0x55ee7e; continue;
            }
            0x55ee7e => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x55f4e3; continue; }
            pc = 0x55ee86; continue;
            }
            0x55ee86 => {
            { let a = rm(m,c.r[5].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x55f4e3; continue; }
            pc = 0x55ee91; continue;
            }
            0x55ee91 => {
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x10),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[5].wrapping_add(0x3e),16),16); }
            if c.cond(5) { pc = 0x55f4e3; continue; }
            pc = 0x55ee9f; continue;
            }
            0x55ee9f => {
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[5].wrapping_add(0x40),16),16); }
            if c.cond(5) { pc = 0x55f4e3; continue; }
            pc = 0x55eead; continue;
            }
            0x55eead => {
            { let a = rm(m,c.r[6].wrapping_add(0x7b),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x55ec52; continue; }
            pc = 0x55eeb7; continue;
            }
            0x55eeb7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x7a4));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x68),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]),8); c.sub(a,0x0u32,8); }
            if c.cond(9) { pc = 0x55f4e3; continue; }
            pc = 0x55eeee; continue;
            }
            0x55eeee => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x59),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x55f4e3; continue; }
            pc = 0x55eef9; continue;
            }
            0x55eef9 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x55ef19; continue; }
            pc = 0x55eefd; continue;
            }
            0x55eefd => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x55ef0b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6c),16));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x55ef19);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x55ef19; continue;
            }
            0x55ef19 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x55efbc; continue; }
            pc = 0x55ef27; continue;
            }
            0x55ef27 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x10),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(3,8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55ef83; continue; }
            pc = 0x55ef32; continue;
            }
            0x55ef32 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55ef43);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            { let a = c.get(2,8); let r = c.sub(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(9) { pc = 0x55ef62; continue; }
            pc = 0x55ef4f; continue;
            }
            0x55ef4f => {
            { let a = c.get(0,8); let r = c.sub(a,0x20u32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x57); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x56),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x56); wm(m,a,8,c.get(0,8)); }
            pc = 0x55ef73; continue;
            }
            0x55ef62 => {
            { let a = c.get(0,8); let r = c.add(a,0x20u32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x56); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x57),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x57); wm(m,a,8,c.get(0,8)); }
            pc = 0x55ef73; continue;
            }
            0x55ef73 => {
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x55ef83);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x55ef83; continue;
            }
            0x55ef83 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x2u32);
            c.push(m, 0x55ef8e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x56),8));
            { let a = c.r[6].wrapping_add(0x55); wm(m,a,8,0x3u32); }
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            if c.cond(7) { pc = 0x55ef9f; continue; }
            pc = 0x55ef99; continue;
            }
            0x55ef99 => {
            { let a = rm(m,c.r[6].wrapping_add(0x57),8); c.sub(a,0x3u32,8); }
            if c.cond(6) { pc = 0x55efb0; continue; }
            pc = 0x55ef9f; continue;
            }
            0x55ef9f => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x55efa8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55ec52; continue; }
            pc = 0x55efb0; continue;
            }
            0x55efb0 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55efbc => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x3a),16));
            { let a = c.get(2,16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x55f387; continue; }
            pc = 0x55efcd; continue;
            }
            0x55efcd => {
            { let a = rm(m,c.r[6].wrapping_add(0x56),8); c.sub(a,0x2u32,8); }
            if c.cond(7) { pc = 0x55efd9; continue; }
            pc = 0x55efd3; continue;
            }
            0x55efd3 => {
            { let a = rm(m,c.r[6].wrapping_add(0x57),8); c.sub(a,0x2u32,8); }
            if c.cond(6) { pc = 0x55efe0; continue; }
            pc = 0x55efd9; continue;
            }
            0x55efd9 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55efe0);
            { let args = []; c.r[0] = host.call(m, 0x55d9d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x55efe0; continue;
            }
            0x55efe0 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55efe7);
            { let args = []; c.r[0] = host.call(m, 0x55d910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3d),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(3,8,u32::from(c.cond(4)));
            c.push(m, 0x55f01c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x16),16));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55f02d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55d700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(7,32,c.get(0,32));
            if c.cond(4) { pc = 0x55f113; continue; }
            pc = 0x55f037; continue;
            }
            0x55f037 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x55f047; continue; }
            pc = 0x55f03b; continue;
            }
            0x55f03b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x14),8));
            { let a = c.get(0,8); let r = c.add(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55f05f; continue; }
            pc = 0x55f047; continue;
            }
            0x55f047 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x55f05b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x55a2c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55f09f; continue; }
            pc = 0x55f05f; continue;
            }
            0x55f05f => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x20),16));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.r[6].wrapping_add(0x6c); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[6].wrapping_add(0x6e); wm(m,a,16,c.get(1,16)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55f088);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55cf60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x2u32);
            c.push(m, 0x55f093);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55f09f => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x55f113; continue; }
            pc = 0x55f0a3; continue;
            }
            0x55f0a3 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x34),8));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x55f0ae; continue; }
            pc = 0x55f0aa; continue;
            }
            0x55f0aa => {
            { let a = c.get(0,8); c.sub(a,0x6u32,8); }
            if c.cond(5) { pc = 0x55f113; continue; }
            pc = 0x55f0ae; continue;
            }
            0x55f0ae => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x34),8));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            if c.cond(4) { pc = 0x55f0d3; continue; }
            pc = 0x55f0b9; continue;
            }
            0x55f0b9 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x5u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x55f0c5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,8,c.get(0,8));
            { let a = c.get(3,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x55f0d7; continue; }
            pc = 0x55f0cc; continue;
            }
            0x55f0cc => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            pc = 0x55f0d3; continue;
            }
            0x55f0d3 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x10),8));
            pc = 0x55f0d7; continue;
            }
            0x55f0d7 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x55f0ea);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55cf60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55f0f7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x55f107);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55f113 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3d),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,32); let r = c.sub(a,0x80u32,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x55f14b; continue; }
            pc = 0x55f125; continue;
            }
            0x55f125 => {
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x55f136);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x55a2c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55f14b; continue; }
            pc = 0x55f13a; continue;
            }
            0x55f13a => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3d),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,c.get(0,8)); }
            pc = 0x55f324; continue;
            }
            0x55f14b => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x58),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(1,8,c.get(3,8));
            { let a = c.get(1,32); let r = c.sub(a,0x80u32,32); c.set(1,32,r); }
            c.set(7,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x55f18f; continue; }
            pc = 0x55f168; continue;
            }
            0x55f168 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m, 0x55f17d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x55a2c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55f18f; continue; }
            pc = 0x55f181; continue;
            }
            0x55f181 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,c.get(1,8)); }
            pc = 0x55f324; continue;
            }
            0x55f18f => {
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[6].wrapping_add(0x14),8),8); }
            if c.cond(4) { pc = 0x55f1be; continue; }
            pc = 0x55f194; continue;
            }
            0x55f194 => {
            { let a = c.get(3,8); let r = c.add(a,0x80u32,8); c.set(3,8,r); }
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x55f1a8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x55a2c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55f1ba; continue; }
            pc = 0x55f1ac; continue;
            }
            0x55f1ac => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,c.get(2,8)); }
            pc = 0x55f324; continue;
            }
            0x55f1ba => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x58),8));
            pc = 0x55f1be; continue;
            }
            0x55f1be => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3d),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(1,8,c.get(0,8));
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            { let a = c.get(1,32); let r = c.sub(a,0x80u32,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x55f1fe; continue; }
            pc = 0x55f1d4; continue;
            }
            0x55f1d4 => {
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x55f1e5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x55a2c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55f1fe; continue; }
            pc = 0x55f1e9; continue;
            }
            0x55f1e9 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3d),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            pc = 0x55f324; continue;
            }
            0x55f1fe => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x55f232; continue; }
            pc = 0x55f207; continue;
            }
            0x55f207 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x55f21c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x55a2c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55f232; continue; }
            pc = 0x55f220; continue;
            }
            0x55f220 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            pc = 0x55f324; continue;
            }
            0x55f232 => {
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[6].wrapping_add(0x14),8),8); }
            if c.cond(4) { pc = 0x55f261; continue; }
            pc = 0x55f237; continue;
            }
            0x55f237 => {
            { let a = c.get(3,8); let r = c.add(a,0x80u32,8); c.set(3,8,r); }
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x55f24b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x55a2c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55f261; continue; }
            pc = 0x55f24f; continue;
            }
            0x55f24f => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            pc = 0x55f324; continue;
            }
            0x55f261 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            c.push(m, 0x55f26c);
            { let args = []; c.r[0] = host.call(m, 0x55d9d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55f273);
            { let args = []; c.r[0] = host.call(m, 0x55d910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3d),8));
            c.push(m,0x1u32);
            c.set(7,32,c.r[6].wrapping_add(0x20));
            c.push(m,0x1u32);
            c.set(3,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x55f28c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x55a2c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55f31a; continue; }
            pc = 0x55f294; continue;
            }
            0x55f294 => {
            c.push(m, 0x55f299);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3d),8));
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let n = 0x7u32; let r = c.shl(a,n,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,c.get(1,8),8); c.set(0,8,r); }
            c.push(m,0x1u32);
            { let a = c.get(0,8); let r = c.sub(a,0x40u32,8); c.set(0,8,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x55f2b5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x55a2c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55f31a; continue; }
            pc = 0x55f2b9; continue;
            }
            0x55f2b9 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            c.push(m,0x1u32);
            { let a = c.get(0,8); let r = c.add(a,0x80u32,8); c.set(0,8,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x55f2ce);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x55a2c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55f31a; continue; }
            pc = 0x55f2d2; continue;
            }
            0x55f2d2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x48),32));
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x55f2e5; continue; }
            pc = 0x55f2dc; continue;
            }
            0x55f2dc => {
            c.set(2,32,rm(m,0x6dccf8u32,32));
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,c.get(2,32)); }
            pc = 0x55f2e5; continue;
            }
            0x55f2e5 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3d),8));
            c.push(m,0x1u32);
            { let a = c.get(0,8); let r = c.add(a,0x80u32,8); c.set(0,8,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x55f2fa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x55a2c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55f31a; continue; }
            pc = 0x55f2fe; continue;
            }
            0x55f2fe => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,c.get(5,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x55f310); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55f31a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,16,c.get(0,16)); }
            pc = 0x55f324; continue;
            }
            0x55f324 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x55f34d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55f35e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55cf60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x20),16));
            { let a = c.r[6].wrapping_add(0x6c); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x2u32);
            { let a = c.r[6].wrapping_add(0x6e); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x55f37b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55f387 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x34),8));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,8); c.sub(a,0x6u32,8); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x55f39c; continue; }
            pc = 0x55f398; continue;
            }
            0x55f398 => {
            { let a = c.get(0,8); c.sub(a,0xbu32,8); }
            if c.cond(5) { pc = 0x55f3a4; continue; }
            pc = 0x55f39c; continue;
            }
            0x55f39c => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x55f3a8; continue;
            }
            0x55f3a4 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x55f3a8; continue;
            }
            0x55f3a8 => {
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x55f3b4; continue; }
            pc = 0x55f3ac; continue;
            }
            0x55f3ac => {
            { let a = c.get(0,8); c.sub(a,0x6u32,8); }
            if c.cond(4) { pc = 0x55f3b4; continue; }
            pc = 0x55f3b0; continue;
            }
            0x55f3b0 => {
            { let a = c.get(0,8); c.sub(a,0xbu32,8); }
            if c.cond(5) { pc = 0x55f3e3; continue; }
            pc = 0x55f3b4; continue;
            }
            0x55f3b4 => {
            c.set(0,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x55f3c9; continue; }
            pc = 0x55f3c7; continue;
            }
            0x55f3c7 => {
            c.set(3,32,c.get(1,32));
            pc = 0x55f3c9; continue;
            }
            0x55f3c9 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x40),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x44),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(5,32)); }
            pc = 0x55f3e3; continue;
            }
            0x55f3e3 => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,c.get(5,32));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x55f404; continue; }
            pc = 0x55f3f0; continue;
            }
            0x55f3f0 => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x55f404; continue; }
            pc = 0x55f3fd; continue;
            }
            0x55f3fd => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x55f48a; continue;
            }
            0x55f404 => {
            c.set(0,32,c.get(7,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(3,8,0x1u32);
            c.push(m, 0x55f41d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x3a),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x55f48a; continue; }
            pc = 0x55f432; continue;
            }
            0x55f432 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x54),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55f48a; continue; }
            pc = 0x55f439; continue;
            }
            0x55f439 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x16),16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x8),8));
            c.set(2,32,c.r[4].wrapping_add(0x5c));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x55f45e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x593580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55f486; continue; }
            pc = 0x55f462; continue;
            }
            0x55f462 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55f473);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55cf60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x54),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55f486; continue; }
            pc = 0x55f47a; continue;
            }
            0x55f47a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55f486 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            pc = 0x55f48a; continue;
            }
            0x55f48a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(1,8); let r = c.or(a,c.get(3,8),8); c.set(1,8,r); }
            if c.cond(4) { pc = 0x55f4d1; continue; }
            pc = 0x55f49a; continue;
            }
            0x55f49a => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55f4ab);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x55f4c5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55f4d1 => {
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55f4e3 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x10),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55f4fe; continue; }
            pc = 0x55f4ee; continue;
            }
            0x55f4ee => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x55f4fa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            pc = 0x55f4fe; continue;
            }
            0x55f4fe => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x55f518);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58d030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7b),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            if c.cond(5) { pc = 0x55f5df; continue; }
            pc = 0x55f52b; continue;
            }
            0x55f52b => {
            c.set(3,16,rm(m,c.r[5].wrapping_add(0x10),16));
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x36),16));
            { let a = c.get(3,16); c.sub(a,c.get(7,16),16); }
            if c.cond(5) { pc = 0x55f5df; continue; }
            pc = 0x55f53c; continue;
            }
            0x55f53c => {
            { let a = rm(m,c.r[5].wrapping_add(0x12),16); c.sub(a,c.get(7,16),16); }
            if c.cond(5) { pc = 0x55f552; continue; }
            pc = 0x55f542; continue;
            }
            0x55f542 => {
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x55f552; continue; }
            pc = 0x55f54e; continue;
            }
            0x55f54e => {
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x55f58f; continue; }
            pc = 0x55f552; continue;
            }
            0x55f552 => {
            { let a = c.get(3,16); c.sub(a,c.get(7,16),16); }
            if c.cond(5) { pc = 0x55f5df; continue; }
            pc = 0x55f55b; continue;
            }
            0x55f55b => {
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[5].wrapping_add(0x38),16),16); }
            if c.cond(5) { pc = 0x55f5df; continue; }
            pc = 0x55f565; continue;
            }
            0x55f565 => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.set(7,32,rm(m,c.r[2].wrapping_add(0xc314),32));
            { let a = c.get(7,32); let r = c.sub(a,0x3u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(7) { pc = 0x55f58f; continue; }
            pc = 0x55f578; continue;
            }
            0x55f578 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(2) { pc = 0x55f58f; continue; }
            pc = 0x55f57d; continue;
            }
            0x55f57d => {
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xc318),32));
            { let a = c.get(2,32); let r = c.sub(a,0x5u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(7) { pc = 0x55f58f; continue; }
            pc = 0x55f58a; continue;
            }
            0x55f58a => {
            { let a = c.get(1,32); c.sub(a,0x4u32,32); }
            if c.cond(3) { pc = 0x55f5df; continue; }
            pc = 0x55f58f; continue;
            }
            0x55f58f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x55f59d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x55f5a4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x21e),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x55ec52; continue; }
            pc = 0x55f5b8; continue;
            }
            0x55f5b8 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x55f5bf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x21e),16); let r = c.dec(a,16); { let a = c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x21e); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(5,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x21e));
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55f5df => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x38),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x3a),16),16); }
            if c.cond(5) { pc = 0x55fcf3; continue; }
            pc = 0x55f5ed; continue;
            }
            0x55f5ed => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x55f644; continue; }
            pc = 0x55f5f1; continue;
            }
            0x55f5f1 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x60),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x55f603; continue; }
            pc = 0x55f5f8; continue;
            }
            0x55f5f8 => {
            { let a = c.r[6].wrapping_add(0x80); wm(m,a,16,0x0u32); }
            pc = 0x55f62f; continue;
            }
            0x55f603 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x55f60a);
            { let args = []; c.r[0] = host.call(m, 0x6319d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,16); c.sub(a,0x19u32,16); }
            { let a = c.r[6].wrapping_add(0x80); wm(m,a,16,c.get(0,16)); }
            if c.cond(6) { pc = 0x55f627; continue; }
            pc = 0x55f617; continue;
            }
            0x55f617 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x78),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55f627; continue; }
            pc = 0x55f61e; continue;
            }
            0x55f61e => {
            { let a = c.r[6].wrapping_add(0x80); wm(m,a,16,0x19u32); }
            pc = 0x55f627; continue;
            }
            0x55f627 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x55f62f; continue;
            }
            0x55f62f => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x55f638);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x55f648; continue;
            }
            0x55f644 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x58),32));
            pc = 0x55f648; continue;
            }
            0x55f648 => {
            { let a = rm(m,c.r[6].wrapping_add(0x7b),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x55f690; continue; }
            pc = 0x55f64e; continue;
            }
            0x55f64e => {
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x3e),16));
            { let a = rm(m,c.r[5].wrapping_add(0x10),16); c.sub(a,c.get(2,16),16); }
            if c.cond(5) { pc = 0x55f690; continue; }
            pc = 0x55f658; continue;
            }
            0x55f658 => {
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x40),16));
            { let a = rm(m,c.r[5].wrapping_add(0x12),16); c.sub(a,c.get(7,16),16); }
            if c.cond(5) { pc = 0x55f690; continue; }
            pc = 0x55f662; continue;
            }
            0x55f662 => {
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x55f690; continue; }
            pc = 0x55f66c; continue;
            }
            0x55f66c => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x55f690; continue; }
            pc = 0x55f676; continue;
            }
            0x55f676 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x55f684);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55f690 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6c),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x55f922; continue; }
            pc = 0x55f69e; continue;
            }
            0x55f69e => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x6e),16));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x55f922; continue; }
            pc = 0x55f6ac; continue;
            }
            0x55f6ac => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x59),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x55f722; continue; }
            pc = 0x55f6b3; continue;
            }
            0x55f6b3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55f6cb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x560cb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,c.get(0,8)); }
            if c.cond(4) { pc = 0x55f6d7; continue; }
            pc = 0x55f6d3; continue;
            }
            0x55f6d3 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x55f6f4; continue;
            }
            0x55f6d7 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x58),32); c.push(m, 0x55f6e0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,8,c.get(0,8));
            pc = 0x55f6e2; continue;
            }
            0x55f6e2 => {
            { let a = c.get(3,8); c.sub(a,0x1u32,8); }
            if c.cond(4) { pc = 0x55fccd; continue; }
            pc = 0x55f6eb; continue;
            }
            0x55f6eb => {
            { let a = c.get(3,8); c.sub(a,0x4u32,8); }
            if c.cond(4) { pc = 0x55fcd0; continue; }
            pc = 0x55f6f4; continue;
            }
            0x55f6f4 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x5c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55fbb7; continue; }
            pc = 0x55f700; continue;
            }
            0x55f700 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x34),8));
            { let a = c.get(0,32); let r = c.add(a,0xfffffffeu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xccu32,32); }
            if c.cond(7) { pc = 0x55fb4f; continue; }
            pc = 0x55f713; continue;
            }
            0x55f713 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x55fe08),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x55fdec)); continue;
            }
            0x55f722 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x55f729); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55f803; continue; }
            pc = 0x55f731; continue;
            }
            0x55f731 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x55f803; continue; }
            pc = 0x55f73f; continue;
            }
            0x55f73f => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55f746);
            { let args = []; c.r[0] = host.call(m, 0x579380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,8,c.get(0,8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x55f915; continue; }
            pc = 0x55f750; continue;
            }
            0x55f750 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8c),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x55f8de; continue; }
            pc = 0x55f75e; continue;
            }
            0x55f75e => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x55f77a; continue; }
            pc = 0x55f768; continue;
            }
            0x55f768 => {
            c.push(m, 0x55f76d);
            { let args = []; c.r[0] = host.call(m, 0x571ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55f77a; continue; }
            pc = 0x55f771; continue;
            }
            0x55f771 => {
            { let a = c.r[6].wrapping_add(0x80); wm(m,a,16,0x0u32); }
            pc = 0x55f77a; continue;
            }
            0x55f77a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6a),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x78),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55f8de; continue; }
            pc = 0x55f79d; continue;
            }
            0x55f79d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8c),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x90),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x5ca),16));
            { let a = rm(m,c.r[7].wrapping_add(0x10),16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x55f7d6; continue; }
            pc = 0x55f7c4; continue;
            }
            0x55f7c4 => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x5ce),16),16); }
            if c.cond(4) { pc = 0x55f8de; continue; }
            pc = 0x55f7d6; continue;
            }
            0x55f7d6 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x5ce),16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x55f7e9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x62b860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x55f7fc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x62b960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(7,32));
            pc = 0x55f8d7; continue;
            }
            0x55f803 => {
            { let a = c.get(3,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x54),16),16); }
            if c.cond(4) { pc = 0x55f820; continue; }
            pc = 0x55f80a; continue;
            }
            0x55f80a => {
            { let a = c.get(3,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x50),16),16); }
            if c.cond(7) { pc = 0x55f81c; continue; }
            pc = 0x55f811; continue;
            }
            0x55f811 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x55f818); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(4) { pc = 0x55f820; continue; }
            pc = 0x55f81c; continue;
            }
            0x55f81c => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x55f822; continue;
            }
            0x55f820 => {
            c.set(0,8,0x1u32);
            pc = 0x55f822; continue;
            }
            0x55f822 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x58),32); c.push(m, 0x55f82a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,8,c.get(0,8));
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x55f833); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(5) { pc = 0x55f8de; continue; }
            pc = 0x55f83b; continue;
            }
            0x55f83b => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x55f915; continue; }
            pc = 0x55f843; continue;
            }
            0x55f843 => {
            { let a = rm(m,c.r[5].wrapping_add(0x6f),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x55f8de; continue; }
            pc = 0x55f84d; continue;
            }
            0x55f84d => {
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x42),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[5].wrapping_add(0x44),16),16); }
            if c.cond(3) { pc = 0x55f8de; continue; }
            pc = 0x55f85b; continue;
            }
            0x55f85b => {
            { let a = rm(m,c.r[5].wrapping_add(0x3a),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x55f8de; continue; }
            pc = 0x55f862; continue;
            }
            0x55f862 => {
            { let a = rm(m,c.r[5].wrapping_add(0x3c),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x55f8de; continue; }
            pc = 0x55f869; continue;
            }
            0x55f869 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x55f87a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x62b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,0x14u32,16); }
            if c.cond(3) { pc = 0x55f8de; continue; }
            pc = 0x55f880; continue;
            }
            0x55f880 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x90),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x5ca),16));
            { let a = rm(m,c.r[5].wrapping_add(0x10),16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x55f8af; continue; }
            pc = 0x55f8a1; continue;
            }
            0x55f8a1 => {
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x5ce),16),16); }
            if c.cond(4) { pc = 0x55f8de; continue; }
            pc = 0x55f8af; continue;
            }
            0x55f8af => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x5ce),16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x55f8c2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x62b860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,16,rm(m,c.r[5].wrapping_add(0xe),16));
            c.set(2,16,rm(m,c.r[5].wrapping_add(0xc),16));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x55f8d5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x62b960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(5,32));
            pc = 0x55f8d7; continue;
            }
            0x55f8d7 => {
            c.push(m,0x1u32);
            c.push(m, 0x55f8de);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x62dd70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x55f8de; continue;
            }
            0x55f8de => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x55f915; continue; }
            pc = 0x55f8e2; continue;
            }
            0x55f8e2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55f90c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x55fff0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,c.get(0,8)); }
            pc = 0x55f6f4; continue;
            }
            0x55f915 => {
            { let a = c.get(3,8); c.sub(a,0x2u32,8); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,u32::from(c.cond(4))); }
            pc = 0x55f6e2; continue;
            }
            0x55f922 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6e),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55f936);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x560750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x55f700; continue;
            }
            0x55f93b => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55fb4f; continue; }
            pc = 0x55f947; continue;
            }
            0x55f947 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x8cu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x55f95c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5576c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x55f971; continue; }
            pc = 0x55f960; continue;
            }
            0x55f960 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x8cu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            pc = 0x55fb3a; continue;
            }
            0x55f971 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0xbu32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            pc = 0x55fb3a; continue;
            }
            0x55f97f => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55fb4f; continue; }
            pc = 0x55f98b; continue;
            }
            0x55f98b => {
            pc = 0x55fa83; continue;
            }
            0x55f990 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55fb4f; continue; }
            pc = 0x55f99c; continue;
            }
            0x55f99c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x13u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x55f9ae);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5576c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x55fa17; continue; }
            pc = 0x55f9b2; continue;
            }
            0x55f9b2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x12u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x55f9c0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5576c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x55f9d3; continue; }
            pc = 0x55f9c4; continue;
            }
            0x55f9c4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x92u32);
            c.push(m,c.get(0,32));
            pc = 0x55fadb; continue;
            }
            0x55f9d3 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0xcu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x55f9e1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5576c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            c.set(1,32,c.get(7,32));
            if c.cond(4) { pc = 0x55f9f6; continue; }
            pc = 0x55f9e7; continue;
            }
            0x55f9e7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0xecu32);
            c.push(m,c.get(2,32));
            pc = 0x55fadd; continue;
            }
            0x55f9f6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            pc = 0x55f9fd; continue;
            }
            0x55f9fd => {
            c.push(m, 0x55fa02);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,0x1u32); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,16,0x0u32); }
            pc = 0x55fb4f; continue;
            }
            0x55fa17 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x13u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x55fa25);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x55fb4b; continue;
            }
            0x55fa2a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x58),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55fa6a; continue; }
            pc = 0x55fa36; continue;
            }
            0x55fa36 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x8cu32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x55fa47);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5576c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x55fa5c; continue; }
            pc = 0x55fa4b; continue;
            }
            0x55fa4b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x8cu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            pc = 0x55fb3a; continue;
            }
            0x55fa5c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0xbu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            pc = 0x55fb3a; continue;
            }
            0x55fa6a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x2u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x55fa76);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x55fb4b; continue;
            }
            0x55fa7b => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55fa98; continue; }
            pc = 0x55fa83; continue;
            }
            0x55fa83 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0xbu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x55fa93);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x55fb4b; continue;
            }
            0x55fa98 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x13u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x55faaa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5576c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            c.set(1,32,c.get(7,32));
            if c.cond(4) { pc = 0x55fac1; continue; }
            pc = 0x55fab0; continue;
            }
            0x55fab0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x13u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x55fabc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x55fb4b; continue;
            }
            0x55fac1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x12u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x55facd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5576c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x55fb00; continue; }
            pc = 0x55fad1; continue;
            }
            0x55fad1 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x92u32);
            c.push(m,c.get(1,32));
            pc = 0x55fadb; continue;
            }
            0x55fadb => {
            c.set(1,32,c.get(7,32));
            pc = 0x55fadd; continue;
            }
            0x55fadd => {
            c.push(m, 0x55fae2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0xccu32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x55faf2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,0x1u32); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,16,0x0u32); }
            pc = 0x55fb4f; continue;
            }
            0x55fb00 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0xcu32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x55fb0e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5576c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x55fb1e; continue; }
            pc = 0x55fb12; continue;
            }
            0x55fb12 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0xecu32);
            c.push(m,c.get(0,32));
            pc = 0x55fadb; continue;
            }
            0x55fb1e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            pc = 0x55f9fd; continue;
            }
            0x55fb2c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0xeeu32);
            c.push(m,c.get(2,32));
            pc = 0x55fb3a; continue;
            }
            0x55fb3a => {
            c.push(m, 0x55fb3f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,0x1u32); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,16,0x0u32); }
            pc = 0x55fb4b; continue;
            }
            0x55fb4b => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            pc = 0x55fb4f; continue;
            }
            0x55fb4f => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x70),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55fcf3; continue; }
            pc = 0x55fb5a; continue;
            }
            0x55fb5a => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x34),8));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x55fb6d; continue; }
            pc = 0x55fb61; continue;
            }
            0x55fb61 => {
            { let a = c.get(0,8); c.sub(a,0xbu32,8); }
            if c.cond(4) { pc = 0x55fb6d; continue; }
            pc = 0x55fb65; continue;
            }
            0x55fb65 => {
            { let a = c.get(0,8); c.sub(a,0x13u32,8); }
            if c.cond(5) { pc = 0x55fcf3; continue; }
            pc = 0x55fb6d; continue;
            }
            0x55fb6d => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x55fb87; continue; }
            pc = 0x55fb71; continue;
            }
            0x55fb71 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x55fb87);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x630d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x55fb87; continue;
            }
            0x55fb87 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x59),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55fbae; continue; }
            pc = 0x55fb8e; continue;
            }
            0x55fb8e => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x79),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5a),8));
            if c.cond(5) { pc = 0x55fba9; continue; }
            pc = 0x55fb9b; continue;
            }
            0x55fb9b => {
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x70); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x5a); wm(m,a,8,c.get(0,8)); }
            pc = 0x55fcf3; continue;
            }
            0x55fba9 => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x5a); wm(m,a,8,c.get(0,8)); }
            pc = 0x55fbae; continue;
            }
            0x55fbae => {
            { let a = c.r[6].wrapping_add(0x70); wm(m,a,8,0x0u32); }
            pc = 0x55fcf3; continue;
            }
            0x55fbb7 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x55fbef; continue; }
            pc = 0x55fbbb; continue;
            }
            0x55fbbb => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x59),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55fc01; continue; }
            pc = 0x55fbc2; continue;
            }
            0x55fbc2 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7b),8));
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            if c.cond(4) { pc = 0x55fbd4; continue; }
            pc = 0x55fbc9; continue;
            }
            0x55fbc9 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x55fbd4; continue; }
            pc = 0x55fbcd; continue;
            }
            0x55fbcd => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x79),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x55fc01; continue; }
            pc = 0x55fbd4; continue;
            }
            0x55fbd4 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x50),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x54),16),16); }
            if c.cond(6) { pc = 0x55fbe4; continue; }
            pc = 0x55fbe0; continue;
            }
            0x55fbe0 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x55fc01; continue; }
            pc = 0x55fbe4; continue;
            }
            0x55fbe4 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x58),32); c.push(m, 0x55fbed); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,8,c.get(0,8));
            pc = 0x55fbef; continue;
            }
            0x55fbef => {
            { let a = c.get(3,8); c.sub(a,0x1u32,8); }
            if c.cond(4) { pc = 0x55fcb0; continue; }
            pc = 0x55fbf8; continue;
            }
            0x55fbf8 => {
            { let a = c.get(3,8); c.sub(a,0x4u32,8); }
            if c.cond(4) { pc = 0x55fcb3; continue; }
            pc = 0x55fc01; continue;
            }
            0x55fc01 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,0x1u32); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x34),8));
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,16,0x0u32); }
            { let a = c.get(0,32); let r = c.add(a,0xfffffffeu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xccu32,32); }
            if c.cond(7) { pc = 0x55fcf3; continue; }
            pc = 0x55fc20; continue;
            }
            0x55fc20 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x55fee8),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x55fed8)); continue;
            }
            0x55fc2f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x5u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x55fc3f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x55fcef; continue;
            }
            0x55fc44 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0xdu32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x55fc56);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5576c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            c.set(1,32,c.get(7,32));
            if c.cond(4) { pc = 0x55fc69; continue; }
            pc = 0x55fc5c; continue;
            }
            0x55fc5c => {
            c.push(m,0xdu32);
            c.push(m,c.get(3,32));
            c.push(m, 0x55fc64);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x55fcef; continue;
            }
            0x55fc69 => {
            c.push(m,0x12u32);
            c.push(m,c.get(3,32));
            c.push(m, 0x55fc71);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5576c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            c.set(1,32,c.get(7,32));
            if c.cond(4) { pc = 0x55fc7e; continue; }
            pc = 0x55fc77; continue;
            }
            0x55fc77 => {
            c.push(m,0x92u32);
            pc = 0x55fc83; continue;
            }
            0x55fc7e => {
            c.push(m,0xecu32);
            pc = 0x55fc83; continue;
            }
            0x55fc83 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x55fc89);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0xccu32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x55fc99);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5576b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x55fcf3; continue;
            }
            0x55fc9b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0xeeu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x55fcae);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x55fcef; continue;
            }
            0x55fcb0 => {
            { let a = c.get(3,8); c.sub(a,0x4u32,8); }
            pc = 0x55fcb3; continue;
            }
            0x55fcb3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,8,u32::from(c.cond(4)));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x55fce8; continue;
            }
            0x55fccd => {
            { let a = c.get(3,8); c.sub(a,0x4u32,8); }
            pc = 0x55fcd0; continue;
            }
            0x55fcd0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x55fce8; continue;
            }
            0x55fce8 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55fcef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x560970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x55fcef; continue;
            }
            0x55fcef => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            pc = 0x55fcf3; continue;
            }
            0x55fcf3 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55fd4c; continue; }
            pc = 0x55fd03; continue;
            }
            0x55fd03 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x34),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,8); c.sub(a,0xbu32,8); }
            if c.cond(5) { pc = 0x55fd10; continue; }
            pc = 0x55fd0e; continue;
            }
            0x55fd0e => {
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x55fd10; continue;
            }
            0x55fd10 => {
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x55fd1c; continue; }
            pc = 0x55fd14; continue;
            }
            0x55fd14 => {
            { let a = c.get(0,8); c.sub(a,0x13u32,8); }
            if c.cond(4) { pc = 0x55fd1c; continue; }
            pc = 0x55fd18; continue;
            }
            0x55fd18 => {
            { let a = c.get(0,8); c.sub(a,0xbu32,8); }
            if c.cond(5) { pc = 0x55fd4c; continue; }
            pc = 0x55fd1c; continue;
            }
            0x55fd1c => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x38),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3a),16));
            c.set(3,32,c.get(2,32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x55fd36; continue; }
            pc = 0x55fd34; continue;
            }
            0x55fd34 => {
            c.set(1,32,c.get(0,32));
            pc = 0x55fd36; continue;
            }
            0x55fd36 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x44),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x40),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            pc = 0x55fd4c; continue;
            }
            0x55fd4c => {
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x55fd83; continue; }
            pc = 0x55fd59; continue;
            }
            0x55fd59 => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x55fd83; continue; }
            pc = 0x55fd66; continue;
            }
            0x55fd66 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x10),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x55fd83; continue; }
            pc = 0x55fd71; continue;
            }
            0x55fd71 => {
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x55fd83 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x55fd94);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3c),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(1,32,c.r[6].wrapping_add(0x14));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x55fdcb; continue; }
            pc = 0x55fda7; continue;
            }
            0x55fda7 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(2,8); c.sub(a,rm(m,c.r[4].wrapping_add(0x10),8),8); }
            if c.cond(4) { pc = 0x55fdcb; continue; }
            pc = 0x55fdb0; continue;
            }
            0x55fdb0 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x34),8));
            { let a = c.get(2,8); c.sub(a,0x8eu32,8); }
            if c.cond(4) { pc = 0x55fdcb; continue; }
            pc = 0x55fdb8; continue;
            }
            0x55fdb8 => {
            { let a = c.get(2,8); c.sub(a,0xeeu32,8); }
            if c.cond(4) { pc = 0x55fdcb; continue; }
            pc = 0x55fdbd; continue;
            }
            0x55fdbd => {
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x14),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x55fdc8);
            { let args = []; c.r[0] = host.call(m, 0x55a1c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x55fdcb; continue;
            }
            0x55fdcb => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x55fddf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x55e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
