//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static SHIPS: [(u32, super::core_table::CoreFn); 2] = [
    (0x00491240, ship_side_menu_draw_ship_row),
    (0x0051a3a0, ship_get_index),
];

pub fn ship_side_menu_draw_ship_row(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x491240;
    loop {
        match pc {
            0x491240 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660494u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(7,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x3864),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6cbd5cu32,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x491290);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x49129a);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3874),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4912bb);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x4912c1);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(7,32)); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x18),16));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x48));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x1u32); }
            c.push(m, 0x49130a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x490c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x18),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.get(4,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49132f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x490f80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0xff000000u32);
            c.set(3,32,c.get(0,32));
            c.push(m, 0x49133b);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x491350);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3874),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x49137b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38a8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4913a7);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x4913ad);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4913de; continue; }
            pc = 0x4913bb; continue;
            }
            0x4913bb => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4913de; continue; }
            pc = 0x4913c2; continue;
            }
            0x4913c2 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xc),32));
            pc = 0x4913e2; continue;
            }
            0x4913de => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            pc = 0x4913e2; continue;
            }
            0x4913e2 => {
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x491459; continue; }
            pc = 0x4913f2; continue;
            }
            0x4913f2 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x15c),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x491459; continue; }
            pc = 0x4913fc; continue;
            }
            0x4913fc => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x49140c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x39c),8));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6ba448u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x491454);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x49148c; continue;
            }
            0x491459 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x48));
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x18),16));
            c.set(1,32,0x6e55d0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x49146f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x612ee0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,0x2u32); }
            c.push(m, 0x49147e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x48));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.push(m, 0x49148c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x49148c; continue;
            }
            0x49148c => {
            c.push(m,0xff000000u32);
            c.push(m, 0x491496);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4914aa);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3864),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x4914d7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x491ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3854),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x491503; continue; }
            pc = 0x4914e4; continue;
            }
            0x4914e4 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x491503; continue; }
            pc = 0x4914eb; continue;
            }
            0x4914eb => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0xc),32));
            pc = 0x491507; continue;
            }
            0x491503 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x491507; continue;
            }
            0x491507 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x491510);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x491517);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x49152c);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x51eb851fu32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let b = rm(m,c.r[5].wrapping_add(0x10),32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m,0x6ba454u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x491551);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38a8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x491577);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0xff000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            c.push(m, 0x49158e);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4915a4);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4916a2; continue; }
            pc = 0x4915bb; continue;
            }
            0x4915bb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3878),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3820),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x491607; continue; }
            pc = 0x4915e4; continue;
            }
            0x4915e4 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x491607; continue; }
            pc = 0x4915eb; continue;
            }
            0x4915eb => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(7,32,c.get(2,32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0xc),32));
            pc = 0x491607; continue;
            }
            0x491607 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x491610);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x491617);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x49162c);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x34));
            { let a = c.get(5,32); let r = c.add(a,c.get(3,32),32); c.set(5,32,r); }
            c.push(m,0x6ba458u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x49164c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x38a8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[6].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x491672);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0xff000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            c.push(m, 0x491689);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x49169f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x4916a2; continue;
            }
            0x4916a2 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4916b0);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4916c1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
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

pub fn ship_get_index(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51a3a0;
    loop {
        match pc {
            0x51a3a0 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,0x2aaaaaabu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "ship_side_menu_draw_ship_row" => ship_side_menu_draw_ship_row(m, host, regs),
        "ship_get_index" => ship_get_index(m, host, regs),
        _ => return None,
    })
}
