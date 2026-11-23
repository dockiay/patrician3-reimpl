//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn debug_page_merchant_ships_and_town_population(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x410b40;
    loop {
        match pc {
            0x410b40 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m,0x65b778u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(0,32,rm(m,0x6df31cu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6de4a0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4a2u32,16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,c.get(4,8));
            c.push(m,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d04),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,0x694a38u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x410bb5);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x3u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x410bcf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x410d92; continue; }
            pc = 0x410bdf; continue;
            }
            0x410bdf => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x410bef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            if c.cond(3) { pc = 0x410d45; continue; }
            pc = 0x410c0e; continue;
            }
            0x410c0e => {
            pc = 0x410c14; continue;
            }
            0x410c10 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x410c14; continue;
            }
            0x410c14 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x39),8));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[5].wrapping_add(0x10),16),16); }
            if c.cond(5) { pc = 0x410d30; continue; }
            pc = 0x410c37; continue;
            }
            0x410c37 => {
            { let a = c.get(3,16); c.sub(a,0x5u32,16); }
            if c.cond(3) { pc = 0x410d23; continue; }
            pc = 0x410c41; continue;
            }
            0x410c41 => {
            c.fild32(m,c.r[7].wrapping_add(0x10));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(7,32));
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,0x66bc5cu32,4); c.fop('*',v); }
            c.fstp64(m,c.r[4]);
            c.push(m, 0x410c5f);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.fild32(m,c.r[4].wrapping_add(0x30));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.fstp32(m,c.r[4].wrapping_add(0x30));
            { let b = rm(m,c.r[7].wrapping_add(0x14),32); c.idiv32(b); }
            c.fld(c.ld(m,c.r[4].wrapping_add(0x30),4));
            { let v = c.ld(m,0x66bc5cu32,4); c.fop('*',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x25u32);
            c.set(2,32,c.r[7].wrapping_add(0x160));
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0cb4),32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x694a54u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x410cb7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xfffffff8),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[0]).wrapping_add(0x8a));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.push(m,0x28au32);
            { let target = rm(m,c.r[0].wrapping_add(0x5c),32); c.push(m, 0x410ce1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x14),16));
            { let a = rm(m,c.r[4].wrapping_add(0x18),16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x410d23; continue; }
            pc = 0x410cec; continue;
            }
            0x410cec => {
            c.push(m,0x694a78u32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x410cfa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x26cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x410d15); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x410d23);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x410d23; continue;
            }
            0x410d23 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            pc = 0x410d30; continue;
            }
            0x410d30 => {
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            if c.cond(2) { pc = 0x410c10; continue; }
            pc = 0x410d45; continue;
            }
            0x410d45 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x478),16));
            c.set(2,16,rm(m,0x6dd8a2u32,16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6dd8a0u32,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x694a7cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x410d78);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x3u32);
            c.push(m,0x12cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x410d92); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x410d92; continue;
            }
            0x410d92 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4110a7; continue; }
            pc = 0x410da8; continue;
            }
            0x410da8 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(7,32,0x7u32);
            c.set(3,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6de4a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(0,32,sx(rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x9a4),16),16)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x2d4),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x694ab8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x410df6);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x410e0d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x694adcu32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x410e1b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x26cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x410e37); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x410e45);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694ae4u32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x410e53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x29u32);
            c.push(m,0x26cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x410e6f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x410e7d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694aecu32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x410e8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x3bu32);
            c.push(m,0x26cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x410ea7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x410eb5);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x694af0u32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x410ec3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x4du32);
            c.push(m,0x26cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x410edf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x410eed);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(3,32); let r = c.add(a,0x308u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x2d8u32,32); c.set(0,32,r); }
            c.set(7,32,0x17u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x4u32); }
            pc = 0x410f0d; continue;
            }
            0x410f0d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(2,32));
            c.push(m,0x694af8u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x410f23);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x2bcu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x410f3c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,sx(rm(m,c.r[3].wrapping_add(0xfffffff8),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,0x66bc58u32,4); c.fop('*',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x694afcu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x410f6b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x320u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x410f84); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,sx(rm(m,c.r[3],16),16)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x694b04u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x410f97);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x384u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x410fb0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.add(a,0x2u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x12u32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x410f0d; continue; }
            pc = 0x410fd0; continue;
            }
            0x410fd0 => {
            c.push(m,0x694b0cu32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x410fde);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x6u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x64u32);
            c.push(m,0x26cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x410ffa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x411008);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xc0),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,0x6df270u32,32));
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x1018),16));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x30));
            c.fstp64(m,c.r[4].wrapping_add(0x30));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x30),8));
            { let v = c.ld(m,0x66bc50u32,8); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x694b14u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41105a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x64u32);
            c.push(m,0x320u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411074); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x694b20u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41108d);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x78u32);
            c.push(m,0x26cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4110a7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x4110a7; continue;
            }
            0x4110a7 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4110b8);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_ship_cargo_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x411310;
    loop {
        match pc {
            0x411310 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m,0x65b7acu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x694ba8u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            c.push(m, 0x411366);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x122u32);
            c.push(m,0x26cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411388); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x0u32); }
            c.push(m, 0x411396);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x42),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x411403; continue; }
            pc = 0x4113a3; continue;
            }
            0x4113a3 => {
            c.set(1,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x3),8));
            c.set(7,32,c.r[0].wrapping_add(c.r[1]));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4113c5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4113d6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x694bd0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4113e6);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x134u32);
            c.push(m,0x28au32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x411403); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x411403; continue;
            }
            0x411403 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[3].wrapping_add(0x54));
            c.set(7,32,0x146u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x411415; continue;
            }
            0x411415 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[2],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x4114b0; continue; }
            pc = 0x411427; continue;
            }
            0x411427 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            c.fild32(m,c.r[4].wrapping_add(0x18));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x41145d; continue; }
            pc = 0x411435; continue;
            }
            0x411435 => {
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x66bc60u32,4); c.fop('*',v); }
            c.push(m,c.get(0,32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x694be0u32);
            c.push(m,c.get(1,32));
            pc = 0x411483; continue;
            }
            0x41145d => {
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x66bc5cu32,4); c.fop('*',v); }
            c.push(m,c.get(2,32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x694bf0u32);
            c.push(m,c.get(0,32));
            pc = 0x411483; continue;
            }
            0x411483 => {
            c.push(m, 0x411488);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x28au32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4114a1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(7,32); c.sub(a,0x18eu32,32); }
            if c.cond(13) { pc = 0x4114c9; continue; }
            pc = 0x4114a9; continue;
            }
            0x4114a9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.add(a,0x12u32,32); c.set(7,32,r); }
            pc = 0x4114b0; continue;
            }
            0x4114b0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(12) { pc = 0x411415; continue; }
            pc = 0x4114c9; continue;
            }
            0x4114c9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x138),16));
            c.push(m,c.get(0,32));
            c.push(m,0x694c00u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4114e2);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x28au32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4114fb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x50),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x41154d; continue; }
            pc = 0x411508; continue;
            }
            0x411508 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x160));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x694c04u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x411531);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x12u32,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x28au32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41154d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x41154d; continue;
            }
            0x41154d => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x52),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x41159f; continue; }
            pc = 0x41155a; continue;
            }
            0x41155a => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x160));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x694c18u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x411583);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x12u32,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x28au32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41159f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x41159f; continue;
            }
            0x41159f => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4115b0);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_facilities_and_population(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x414e50;
    loop {
        match pc {
            0x414e50 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m,0x65bbaau32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x30u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xc0),16));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,0x6de4a2u32,16));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6de4a0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,c.get(4,8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(7,32,0x14u32);
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d04),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x6951ccu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x414ee8);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2u32);
            c.push(m,0x2u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414f02); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xc0),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x414f1c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2u32);
            c.push(m,0xdcu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414f38); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x0u32); }
            c.push(m, 0x414f46);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6951dcu32);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x414f54);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = rm(m,c.r[3].wrapping_add(0x2c8),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x414f6f; continue; }
            pc = 0x414f61; continue;
            }
            0x414f61 => {
            c.push(m,0x6951e0u32);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x414f6f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x414f6f; continue;
            }
            0x414f6f => {
            { let a = rm(m,c.r[3].wrapping_add(0x2c8),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x414f86; continue; }
            pc = 0x414f78; continue;
            }
            0x414f78 => {
            c.push(m,0x6951e8u32);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x414f86);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x414f86; continue;
            }
            0x414f86 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x2c8),32));
            { let a = c.get(4,8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x414f9f; continue; }
            pc = 0x414f91; continue;
            }
            0x414f91 => {
            c.push(m,0x6951f0u32);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x414f9f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x414f9f; continue;
            }
            0x414f9f => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x2c8),32));
            { let a = c.get(4,8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x414fb8; continue; }
            pc = 0x414faa; continue;
            }
            0x414faa => {
            c.push(m,0x6951fcu32);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x414fb8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x414fb8; continue;
            }
            0x414fb8 => {
            { let a = rm(m,c.r[3].wrapping_add(0x2c8),32); c.and(a,0x4000000u32,32); }
            if c.cond(4) { pc = 0x414fd2; continue; }
            pc = 0x414fc4; continue;
            }
            0x414fc4 => {
            c.push(m,0x695208u32);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x414fd2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x414fd2; continue;
            }
            0x414fd2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x2u32);
            c.push(m,0x1a4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x414fe9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x414fe9; continue;
            }
            0x414fe9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,32,c.r[0].wrapping_add(0x84));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(0x4),16); c.sub(a,0x0u32,16); }
            if c.cond(7) { pc = 0x41500c; continue; }
            pc = 0x415003; continue;
            }
            0x415003 => {
            { let a = rm(m,c.r[3],32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x415274; continue; }
            pc = 0x41500c; continue;
            }
            0x41500c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.add(a,0x3u32,32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0a90),32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m, 0x415024);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x2u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41503c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x0u32); }
            c.push(m, 0x41504a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x4),16));
            c.push(m,c.get(0,32));
            c.push(m,0x695214u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x415060);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x7du32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415076); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.fild32(m,c.r[3]);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,0x66bc68u32,4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x69521cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41509b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4150b4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x8),16));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,0x66bc68u32,4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x695228u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4150e5);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x113u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4150fe); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xa),16));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.push(m,0x695234u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x415114);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x19au32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41512d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x6),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c2c),8));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x64),32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x430),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(15) { pc = 0x415161; continue; }
            pc = 0x415151; continue;
            }
            0x415151 => {
            c.push(m,0x695240u32);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x41515f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x415194; continue;
            }
            0x415161 => {
            c.fild32(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xc4));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fild32(m,c.r[4].wrapping_add(0x20));
            c.fstp32(m,c.r[4].wrapping_add(0x20));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x20),4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x69524cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x415191);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x415194; continue;
            }
            0x415194 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x226u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4151aa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x6),8));
            { let a = c.get(7,32); let r = c.add(a,0xfu32,32); c.set(7,32,r); }
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            if c.cond(6) { pc = 0x4151e1; continue; }
            pc = 0x4151b4; continue;
            }
            0x4151b4 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4151bb);
            { let args = []; c.r[0] = host.call(m, 0x510120, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x695258u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4151cb);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x7du32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4151e1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x4151e1; continue;
            }
            0x4151e1 => {
            { let a = rm(m,c.r[4].wrapping_add(0x14),32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x415271; continue; }
            pc = 0x4151ec; continue;
            }
            0x4151ec => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xfu32,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x814),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x810),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(2,32)); }
            c.fild64(m,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x3c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x3c),4));
            { let v = c.ld(m,0x66bc64u32,4); c.fop('/',v); }
            c.fstp64(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[0].wrapping_add(0x818),4));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.fstp64(m,c.r[4].wrapping_add(0x8));
            c.fild64(m,c.r[4].wrapping_add(0x48));
            c.fstp32(m,c.r[4].wrapping_add(0x3c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x3c),4));
            { let v = c.ld(m,0x66bc64u32,4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x695264u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x415258);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415271); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x415271; continue;
            }
            0x415271 => {
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            pc = 0x415274; continue;
            }
            0x415274 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x15u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x414fe9; continue; }
            pc = 0x415286; continue;
            }
            0x415286 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0xc0),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x415460; continue; }
            pc = 0x4152a0; continue;
            }
            0x4152a0 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x4152a2; continue;
            }
            0x4152a2 => {
            { let a = c.get(3,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x41536d; continue; }
            pc = 0x4152ab; continue;
            }
            0x4152ab => {
            pc = m.r32(c.r[3].wrapping_mul(4).wrapping_add(0x4156e4)); continue;
            }
            0x4152b2 => {
            c.push(m,0x69529cu32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x4152c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4152d8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            pc = 0x415363; continue;
            }
            0x4152e1 => {
            c.push(m,0x6952a4u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x4152ef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415307); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            pc = 0x415363; continue;
            }
            0x41530d => {
            c.push(m,0x6952acu32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x41531b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415333); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            pc = 0x415363; continue;
            }
            0x415339 => {
            c.push(m,0x6952b0u32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x415347);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x6u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41535f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            pc = 0x415363; continue;
            }
            0x415363 => {
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x0u32); }
            c.push(m, 0x41536d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x41536d; continue;
            }
            0x41536d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x2d8),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6952b8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x415388);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41539e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x300),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x30));
            c.fstp32(m,c.r[4].wrapping_add(0x30));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x30),4));
            { let v = c.ld(m,0x66bc58u32,4); c.fop('*',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x6952bcu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4153d5);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4153ee); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0x308),16),16)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.push(m,0x6952c4u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41540a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x12cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415423); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4152a2; continue; }
            pc = 0x415430; continue;
            }
            0x415430 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x2d4),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6952ccu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41544a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415460); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x415460; continue;
            }
            0x415460 => {
            c.push(m,0x6952d0u32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x41546e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x7u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x14u32);
            c.push(m,0x2a8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41548a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x0u32); }
            c.push(m, 0x415498);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6952e0u32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x4154a6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x8u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x14u32);
            c.push(m,0x316u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4154c2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x0u32); }
            c.push(m, 0x4154d0);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6952ecu32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x4154de);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x9u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x14u32);
            c.push(m,0x366u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4154fa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x0u32); }
            c.push(m, 0x415508);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6952fcu32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x415516);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0xau32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x14u32);
            c.push(m,0x3b6u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415532); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x0u32); }
            c.push(m, 0x415540);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,0x3cu32);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x415547; continue;
            }
            0x415547 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            c.set(1,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xfffff8f8u32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.get(0,32); let r = c.add(a,0x7d0u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x415570);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0xbu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x28au32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41558b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x0u32); }
            c.push(m, 0x415599);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xc0),16));
            c.set(1,32,0x6de3d8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4155b2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x52e960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.fstp64(m,c.r[4]);
            c.push(m,0x695308u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4155c7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x2e4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4155e0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,16,rm(m,c.r[2].wrapping_add(0xc0),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x4155fe);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x695310u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41560e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x316u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415627); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,16,rm(m,c.r[2].wrapping_add(0xc0),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x415645);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x695318u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x415655);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x366u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41566e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.fild32(m,c.r[4].wrapping_add(0x18));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x30));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x30),4));
            { let v = c.ld(m,c.r[7].wrapping_mul(4).wrapping_add(0x673a18),4); c.fop('*',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x695320u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x415696);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x3b6u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4156af); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x415547; continue; }
            pc = 0x4156bc; continue;
            }
            0x4156bc => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4156cd);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_world_economy_summary(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x415700;
    loop {
        match pc {
            0x415700 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m,0x65bc64u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x1c0u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(0,32,rm(m,0x6de4a0u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4a2u32,16));
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,8,c.get(4,8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d04),32));
            { let a = c.r[4].wrapping_add(0x1dc); wm(m,a,32,c.get(7,32)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,0x695328u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x415773);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x3u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41578d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x695338u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x41579b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4157b7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x4157c8);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695340u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4157d6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4157f2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x415803);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x69534cu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x415811);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0xc8u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415830); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x415841);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695354u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x41584f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x4u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x12cu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41586e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x41587f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x69535cu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x41588d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x190u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4158ac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x4158bd);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x695364u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4158cb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x6u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x1f4u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4158ea); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x4158fb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x69536cu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x415909);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x7u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x244u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415928); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x415939);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,0x35u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,0x18u32);
            c.set(7,32,c.r[4].wrapping_add(0x48));
            c.stos(m,32,true);
            c.set(1,32,c.r[4].wrapping_add(0x108));
            c.set(2,32,c.r[4].wrapping_add(0xa8));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x16c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x415981);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5312e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            if c.cond(6) { pc = 0x415b48; continue; }
            pc = 0x415994; continue;
            }
            0x415994 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x415998; continue;
            }
            0x415998 => {
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(7,32,c.r[2].wrapping_add(c.r[1]));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x4159be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x8u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4159d9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x4159ea);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x2d4),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x695374u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x415a00);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x2d4),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415a26); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(7,32,0xf5u32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[0].wrapping_add(0x300));
            { let a = c.get(0,32); let r = c.add(a,0x2d8u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            pc = 0x415a4a; continue;
            }
            0x415a4a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,c.get(0,32));
            c.push(m,0x695378u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x415a60);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[2],32));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[7].wrapping_add(0xffffffd3));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415a88); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(2,32,sx(rm(m,c.r[1],16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x24));
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x24),4));
            { let v = c.ld(m,0x66bc58u32,4); c.fop('*',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x69537cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x415aba);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415acf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            c.set(0,32,0x4u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(7,32); let r = c.add(a,0x64u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,0x285u32,32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            if c.cond(12) { pc = 0x415a4a; continue; }
            pc = 0x415b02; continue;
            }
            0x415b02 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            pc = 0x415b0b; continue;
            }
            0x415b0b => {
            c.set(2,32,rm(m,c.r[1],32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x48),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x48); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x415b0b; continue; }
            pc = 0x415b20; continue;
            }
            0x415b20 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x9f8u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(2) { pc = 0x415998; continue; }
            pc = 0x415b48; continue;
            }
            0x415b48 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x695384u32);
            c.push(m,c.get(1,32));
            { let a = c.get(3,32); let r = c.add(a,0xau32,32); c.set(3,32,r); }
            c.push(m, 0x415b5f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415b75); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(7,32,0xc8u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x415b82; continue;
            }
            0x415b82 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(2,32));
            c.push(m,0x695388u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x415b98);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415bad); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.add(a,0x64u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,0x258u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x415b82; continue; }
            pc = 0x415bc3; continue;
            }
            0x415bc3 => {
            c.push(m,0x64u32);
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x69538cu32);
            c.push(m,c.get(0,32));
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            c.push(m, 0x415bd7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415bed); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x415c52; continue; }
            pc = 0x415bf5; continue;
            }
            0x415bf5 => {
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(7,32,0xc8u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x415c02; continue;
            }
            0x415c02 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let b = rm(m,c.r[4].wrapping_add(0x20),32); c.div32(b); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x695390u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x415c27);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415c3c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.add(a,0x64u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,0x258u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x415c02; continue; }
            pc = 0x415c52; continue;
            }
            0x415c52 => {
            c.push(m,0x695394u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x415c60);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x9u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x294u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415c7f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x415c90);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x69539cu32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x415c9e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0xau32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x2eeu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415cbd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x415cce);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6953a4u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x415cdc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0xbu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x334u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415cfb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x415d0c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6953b0u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x415d1a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0xcu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x37au32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415d39); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x415d4a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6953bcu32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x415d58);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0xdu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x3c0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415d77); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x415d88);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6953c8u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x415d96);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0xeu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0x406u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415db5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x415dc6);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,0x35u32);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x415dcd; continue;
            }
            0x415dcd => {
            c.set(0,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m, 0x415dde);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0xfu32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x294u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415dfc); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,8,0x0u32); }
            c.push(m, 0x415e0d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x48),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x415e34; continue; }
            pc = 0x415e1b; continue;
            }
            0x415e1b => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x6953d4u32);
            pc = 0x415e4b; continue;
            }
            0x415e34 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x6953dcu32);
            pc = 0x415e4b; continue;
            }
            0x415e4b => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m, 0x415e55);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x2eeu32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415e6e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xa8),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x415e98; continue; }
            pc = 0x415e7f; continue;
            }
            0x415e7f => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x6953e4u32);
            pc = 0x415eaf; continue;
            }
            0x415e98 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x6953ecu32);
            pc = 0x415eaf; continue;
            }
            0x415eaf => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m, 0x415eb9);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x334u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415ed2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x168),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x415efc; continue; }
            pc = 0x415ee3; continue;
            }
            0x415ee3 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x6953f4u32);
            pc = 0x415f13; continue;
            }
            0x415efc => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x6953fcu32);
            pc = 0x415f13; continue;
            }
            0x415f13 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m, 0x415f1d);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x37au32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415f36); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x108),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x415f60; continue; }
            pc = 0x415f47; continue;
            }
            0x415f47 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x695404u32);
            pc = 0x415f77; continue;
            }
            0x415f60 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x69540cu32);
            pc = 0x415f77; continue;
            }
            0x415f77 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m, 0x415f81);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x3c0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x415f9a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xa8),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x415fdf; continue; }
            pc = 0x415faa; continue;
            }
            0x415faa => {
            c.fild32(m,c.r[4].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x108));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fild32(m,c.r[4].wrapping_add(0x24));
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x24),4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x695414u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x415fda);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x415fed; continue;
            }
            0x415fdf => {
            c.push(m,0x69541cu32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x415fed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x415fed; continue;
            }
            0x415fed => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x406u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416003); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x415dcd; continue; }
            pc = 0x416010; continue;
            }
            0x416010 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x1d4); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x416024);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1cc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_trade_potential_matrix(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x417bf0;
    loop {
        match pc {
            0x417bf0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m,0x65bdddu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x48u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xc0),16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x418102; continue; }
            pc = 0x417c2b; continue;
            }
            0x417c2b => {
            c.set(1,32,0x28u32);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0xc8u32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(3,32)); }
            c.set(7,32,0x64u32);
            if c.cond(6) { pc = 0x417ce0; continue; }
            pc = 0x417c68; continue;
            }
            0x417c68 => {
            { let a = c.get(3,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x417c9c; continue; }
            pc = 0x417c6d; continue;
            }
            0x417c6d => {
            c.set(2,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m, 0x417c7e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x10u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x417c96); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            pc = 0x417cc9; continue;
            }
            0x417c9c => {
            c.set(0,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m, 0x417cad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x20u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x417cc5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            pc = 0x417cc9; continue;
            }
            0x417cc9 => {
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x0u32); }
            c.push(m, 0x417cd3);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x417c68; continue; }
            pc = 0x417ce0; continue;
            }
            0x417ce0 => {
            c.set(7,32,0x2fu32);
            c.set(3,32,0x6c0bb0u32);
            pc = 0x417cea; continue;
            }
            0x417cea => {
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417cf9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655c88, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(7,32));
            c.push(m,0x5dcu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x417d06);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655cd4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[3],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m, 0x417d12);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x3u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[7].wrapping_add(0x1));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x5u32);
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x417d2d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,0x0u32); }
            c.push(m, 0x417d3b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x22u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,0x6c0c10u32,32); }
            if c.cond(12) { pc = 0x417cea; continue; }
            pc = 0x417d49; continue;
            }
            0x417d49 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(6) { pc = 0x41805d; continue; }
            pc = 0x417d5c; continue;
            }
            0x417d5c => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x64u32); }
            pc = 0x417d6a; continue;
            }
            0x417d66 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x417d6a; continue;
            }
            0x417d6a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(7,32,0x30u32);
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xc0),16));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x417e0d; continue; }
            pc = 0x417d8a; continue;
            }
            0x417d8a => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x417d96);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x531b70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.add(a,0xfu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            pc = 0x417da9; continue;
            }
            0x417da9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de3d8u32);
            c.set(2,16,rm(m,c.r[0].wrapping_add(0xc0),16));
            c.push(m,c.get(2,32));
            c.push(m, 0x417dd5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x52f030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x417e55; continue; }
            pc = 0x417ddb; continue;
            }
            0x417ddb => {
            { let a = c.get(3,32); c.sub(a,0x7530u32,32); }
            if c.cond(14) { pc = 0x417de8; continue; }
            pc = 0x417de3; continue;
            }
            0x417de3 => {
            c.set(3,32,0x7530u32);
            pc = 0x417de8; continue;
            }
            0x417de8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x673684),8));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x48));
            c.fstp32(m,c.r[4].wrapping_add(0x48));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x48),4));
            pc = 0x417ea6; continue;
            }
            0x417e09 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x417e0d; continue;
            }
            0x417e0d => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x417e19);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x52e960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.fstp64(m,c.r[4]);
            c.push(m,0x695628u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x417e2e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x417e47); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(7,32); let r = c.add(a,0x22u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x417e09; continue; }
            pc = 0x417e50; continue;
            }
            0x417e50 => {
            pc = 0x41803c; continue;
            }
            0x417e55 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0xc0),16));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x417e7e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x52f030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,32); c.sub(a,0x7530u32,32); }
            if c.cond(14) { pc = 0x417e8a; continue; }
            pc = 0x417e85; continue;
            }
            0x417e85 => {
            c.set(0,32,0x7530u32);
            pc = 0x417e8a; continue;
            }
            0x417e8a => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x673684),8));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x48));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            c.fstp32(m,c.r[4].wrapping_add(0x48));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x48),4));
            pc = 0x417ea6; continue;
            }
            0x417ea6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let v = c.ld(m,c.r[4].wrapping_add(0x10),4); c.fop('*',v); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            if c.cond(4) { pc = 0x417ed9; continue; }
            pc = 0x417ec0; continue;
            }
            0x417ec0 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(3,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x695630u32);
            pc = 0x417ef0; continue;
            }
            0x417ed9 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(3,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x695638u32);
            pc = 0x417ef0; continue;
            }
            0x417ef0 => {
            c.push(m,c.get(1,32));
            c.push(m, 0x417ef6);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x417f0f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,0x66bc78u32,4); c.fop('*',v); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(7,32); let r = c.add(a,0xeu32,32); c.set(7,32,r); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x695640u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x417f31);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x417f4a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x417f7d; continue; }
            pc = 0x417f5b; continue;
            }
            0x417f5b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x417f6f; continue; }
            pc = 0x417f61; continue;
            }
            0x417f61 => {
            { let a = c.get(3,32); c.sub(a,0x3e8u32,32); }
            if c.cond(13) { pc = 0x417f9b; continue; }
            pc = 0x417f69; continue;
            }
            0x417f69 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x417f9b; continue;
            }
            0x417f6f => {
            { let a = c.get(3,32); c.sub(a,0xfffffc18u32,32); }
            if c.cond(14) { pc = 0x417f9b; continue; }
            pc = 0x417f77; continue;
            }
            0x417f77 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x417f9b; continue;
            }
            0x417f7d => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x417f8b; continue; }
            pc = 0x417f81; continue;
            }
            0x417f81 => {
            { let a = c.get(3,32); c.sub(a,0x1770u32,32); }
            if c.cond(13) { pc = 0x417f9b; continue; }
            pc = 0x417f89; continue;
            }
            0x417f89 => {
            pc = 0x417f93; continue;
            }
            0x417f8b => {
            { let a = c.get(3,32); c.sub(a,0xffffe890u32,32); }
            if c.cond(14) { pc = 0x417f9b; continue; }
            pc = 0x417f93; continue;
            }
            0x417f93 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            pc = 0x417f9b; continue;
            }
            0x417f9b => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x4100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x417fcb; continue; }
            pc = 0x417fac; continue;
            }
            0x417fac => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(2,8,u32::from(c.cond(13)));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            pc = 0x417fcb; continue;
            }
            0x417fcb => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x20),4); c.fop('+',v); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x18u32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x20));
            if c.cond(12) { pc = 0x417da9; continue; }
            pc = 0x417fe5; continue;
            }
            0x417fe5 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x20),4));
            { let v = c.ld(m,0x66bc78u32,4); c.fop('*',v); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.fstp64(m,c.r[4]);
            c.push(m,0x695648u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x418004);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41801d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.fld(c.ld(m,c.r[4].wrapping_add(0x30),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x20),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(10) { pc = 0x41803c; continue; }
            pc = 0x41802c; continue;
            }
            0x41802c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            pc = 0x41803c; continue;
            }
            0x41803c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            if c.cond(2) { pc = 0x417d66; continue; }
            pc = 0x41805d; continue;
            }
            0x41805d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4180b8; continue; }
            pc = 0x418065; continue;
            }
            0x418065 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(2,32); c.sub(a,0x18u32,32); }
            if c.cond(13) { pc = 0x4180b8; continue; }
            pc = 0x41806e; continue;
            }
            0x41806e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            if c.cond(4) { pc = 0x418091; continue; }
            pc = 0x41808a; continue;
            }
            0x41808a => {
            c.push(m,0x695650u32);
            pc = 0x418096; continue;
            }
            0x418091 => {
            c.push(m,0x69565cu32);
            pc = 0x418096; continue;
            }
            0x418096 => {
            c.push(m,c.get(2,32));
            c.push(m, 0x41809c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4180b3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            pc = 0x4180b8; continue;
            }
            0x4180b8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4180f1; continue; }
            pc = 0x4180c0; continue;
            }
            0x4180c0 => {
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.push(m,0x695668u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4180d7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x258u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4180f1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x4180f1; continue;
            }
            0x4180f1 => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x418102);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x418102; continue;
            }
            0x418102 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn render_window_title(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x420c70;
    loop {
        match pc {
            0x420c70 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65c65au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x791eu32);
            c.set(1,32,0x6da820u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.push(m, 0x420ca3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x420cc1; continue; }
            pc = 0x420caa; continue;
            }
            0x420caa => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x420cc1; continue; }
            pc = 0x420cb1; continue;
            }
            0x420cb1 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[0],32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0xc),32));
            pc = 0x420cd5; continue;
            }
            0x420cc1 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            pc = 0x420cd5; continue;
            }
            0x420cd5 => {
            c.push(m,c.get(1,32));
            c.push(m, 0x420cdb);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x420ce2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,c.get(6,32));
            c.set(2,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x420cf2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x420cfd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x420d0b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[0].wrapping_add(c.r[5]).wrapping_add(0xffffff6e));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x420d1a);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x420d21);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcf20u32);
            c.push(m, 0x420d2e);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x420d34);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x66e0c8u32,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x420d40);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x420d4f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,0xdu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x420d5a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x420d68); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(c.r[0]).wrapping_add(0xa));
            c.push(m,c.get(1,32));
            c.push(m, 0x420d72);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x66e0ccu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x420d7e);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x420d8d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x420d98); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x420da6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x9));
            c.push(m,c.get(0,32));
            c.push(m, 0x420db0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x420dc4);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn main_menu_init(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42ecc0;
    loop {
        match pc {
            0x42ecc0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65d457u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            c.push(m, 0x42ecfd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x42ed04);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b5380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[5].wrapping_add(0xf); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x284); wm(m,a,32,0xffffffffu32); }
            c.set(0,32,rm(m,0x66e094u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x94),32); c.push(m, 0x42ed21); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1f40u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697118u32);
            c.push(m, 0x42ed36);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x42ed41);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4439b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1770u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69712cu32);
            c.push(m, 0x42ed56);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x42ed61);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4439b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[5].wrapping_add(0xffffffec));
            c.set(3,32,0x1u32);
            c.push(m,0x697140u32);
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x42ed81);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x42ed94);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x42ed9e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b3ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x3e8eu32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x42edad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3e9eu32);
            c.set(1,32,0x6da820u32);
            { let a = 0x6cc37cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42edc1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3e9fu32);
            c.set(1,32,0x6da820u32);
            { let a = 0x6cc380u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42edd5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xcu32);
            c.set(1,32,0x6da820u32);
            { let a = 0x6cc384u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42ede6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xeu32);
            c.set(1,32,0x6da820u32);
            { let a = c.r[6].wrapping_add(0x274); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42edf8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x278); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x42ee0f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x19c),32));
            c.set(1,32,c.r[6].wrapping_add(0x19c));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42ee1e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x69714cu32);
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6cbb60u32);
            c.push(m,0x697160u32);
            c.push(m,0x697168u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x42ee46);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42ee57; continue; }
            pc = 0x42ee4a; continue;
            }
            0x42ee4a => {
            c.push(m,0x697174u32);
            c.push(m, 0x42ee54);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bbb30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x42ee57; continue;
            }
            0x42ee57 => {
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[6].wrapping_add(0x288));
            c.push(m, 0x42ee67);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2698u32);
            c.push(m, 0x42ee71);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            if c.cond(4) { pc = 0x42ee88; continue; }
            pc = 0x42ee7f; continue;
            }
            0x42ee7f => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42ee86);
            { let args = []; c.r[0] = host.call(m, 0x496c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42ee8a; continue;
            }
            0x42ee88 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42ee8a; continue;
            }
            0x42ee8a => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6d1c7cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42eea3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42eeae); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x42eebf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6d1c90u32);
            c.push(m, 0x42eec9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x49ac90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697180u32);
            c.push(m, 0x42eedb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6d2f60u32);
            c.push(m, 0x42eee5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x49bc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x5d28u32);
            c.push(m, 0x42eeef);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3u32); }
            if c.cond(4) { pc = 0x42ef06; continue; }
            pc = 0x42eefd; continue;
            }
            0x42eefd => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42ef04);
            { let args = []; c.r[0] = host.call(m, 0x436ad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42ef08; continue;
            }
            0x42ef06 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42ef08; continue;
            }
            0x42ef08 => {
            c.push(m,0x0u32);
            { let a = 0x6cbb94u32; wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            c.push(m, 0x42ef22);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x42ef27);
            { let args = []; c.r[0] = host.call(m, 0x4406a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x42ef89, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn main_menu_create_screens_tail(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42ef89;
    loop {
        match pc {
            0x42ef89 => {
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42efdf; continue; }
            pc = 0x42ef95; continue;
            }
            0x42ef95 => {
            c.push(m,0x2c98u32);
            c.push(m, 0x42ef9f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x7u32); }
            if c.cond(4) { pc = 0x42efb6; continue; }
            pc = 0x42efad; continue;
            }
            0x42efad => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42efb4);
            { let args = []; c.r[0] = host.call(m, 0x45cac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42efb8; continue;
            }
            0x42efb6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42efb8; continue;
            }
            0x42efb8 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cbdf0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42efd1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42efdc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0xf),8); let r = c.and(a,c.get(0,8),8); { let a = c.r[5].wrapping_add(0xf); wm(m,a,8,r); } }
            pc = 0x42efdf; continue;
            }
            0x42efdf => {
            c.set(0,32,rm(m,0x6cbb7cu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42f02f; continue; }
            pc = 0x42efe8; continue;
            }
            0x42efe8 => {
            c.push(m,0x1210u32);
            c.push(m, 0x42eff2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x8u32); }
            if c.cond(4) { pc = 0x42f009; continue; }
            pc = 0x42f000; continue;
            }
            0x42f000 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42f007);
            { let args = []; c.r[0] = host.call(m, 0x4313b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f00b; continue;
            }
            0x42f009 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42f00b; continue;
            }
            0x42f00b => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cbb7cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f024);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42f02f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f02f; continue;
            }
            0x42f02f => {
            c.set(0,32,rm(m,0x6cbaa4u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42f07f; continue; }
            pc = 0x42f038; continue;
            }
            0x42f038 => {
            c.push(m,0xfa8u32);
            c.push(m, 0x42f042);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x9u32); }
            if c.cond(4) { pc = 0x42f059; continue; }
            pc = 0x42f050; continue;
            }
            0x42f050 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42f057);
            { let args = []; c.r[0] = host.call(m, 0x40d7b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f05b; continue;
            }
            0x42f059 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42f05b; continue;
            }
            0x42f05b => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cbaa4u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f074);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbaa4u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42f07f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f07f; continue;
            }
            0x42f07f => {
            c.set(0,32,rm(m,0x6d9594u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42f0cf; continue; }
            pc = 0x42f088; continue;
            }
            0x42f088 => {
            c.push(m,0xc24u32);
            c.push(m, 0x42f092);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xau32); }
            if c.cond(4) { pc = 0x42f0a9; continue; }
            pc = 0x42f0a0; continue;
            }
            0x42f0a0 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42f0a7);
            { let args = []; c.r[0] = host.call(m, 0x4ab1a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f0ab; continue;
            }
            0x42f0a9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42f0ab; continue;
            }
            0x42f0ab => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6d9594u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f0c4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d9594u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42f0cf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f0cf; continue;
            }
            0x42f0cf => {
            c.set(0,32,rm(m,0x6cbb6cu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42f11f; continue; }
            pc = 0x42f0d8; continue;
            }
            0x42f0d8 => {
            c.push(m,0xd0cu32);
            c.push(m, 0x42f0e2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xbu32); }
            if c.cond(4) { pc = 0x42f0f9; continue; }
            pc = 0x42f0f0; continue;
            }
            0x42f0f0 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42f0f7);
            { let args = []; c.r[0] = host.call(m, 0x42fbf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f0fb; continue;
            }
            0x42f0f9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42f0fb; continue;
            }
            0x42f0fb => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cbb6cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f114);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb6cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42f11f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f11f; continue;
            }
            0x42f11f => {
            c.set(0,32,rm(m,0x6cbb30u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42f16f; continue; }
            pc = 0x42f128; continue;
            }
            0x42f128 => {
            c.push(m,0x350u32);
            c.push(m, 0x42f132);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xcu32); }
            if c.cond(4) { pc = 0x42f149; continue; }
            pc = 0x42f140; continue;
            }
            0x42f140 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42f147);
            { let args = []; c.r[0] = host.call(m, 0x4215d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f14b; continue;
            }
            0x42f149 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42f14b; continue;
            }
            0x42f14b => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cbb30u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f164);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb30u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42f16f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f16f; continue;
            }
            0x42f16f => {
            c.set(0,32,rm(m,0x6d95a0u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x42f1bf; continue; }
            pc = 0x42f178; continue;
            }
            0x42f178 => {
            c.push(m,0x434u32);
            c.push(m, 0x42f182);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xdu32); }
            if c.cond(4) { pc = 0x42f199; continue; }
            pc = 0x42f190; continue;
            }
            0x42f190 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42f197);
            { let args = []; c.r[0] = host.call(m, 0x4ac5d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f19b; continue;
            }
            0x42f199 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42f19b; continue;
            }
            0x42f19b => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6d95a0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42f1b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42f1bf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42f1bf; continue;
            }
            0x42f1bf => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            c.push(m, 0x42f1cb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0x8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x42f1da);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xf),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn campaign_select_screen_process_input(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x430570;
    loop {
        match pc {
            0x430570 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65d58du32);
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,0x6dcbdbu32,8));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x4305c9; continue; }
            pc = 0x4305aa; continue;
            }
            0x4305aa => {
            { let a = 0x6dcbdbu32; wm(m,a,8,0x0u32); }
            c.set(0,32,0xfffffffeu32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4305c9 => {
            c.set(1,32,c.r[6].wrapping_add(0x5f0));
            c.push(m, 0x4305d4);
            { let args = []; c.r[0] = host.call(m, 0x431260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x23c));
            { let a = c.r[6].wrapping_add(0xca1); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4305e5);
            { let args = []; c.r[0] = host.call(m, 0x431260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xcf0),32));
            { let a = c.r[6].wrapping_add(0xca0); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xce0),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x430630; continue; }
            pc = 0x4305fb; continue;
            }
            0x4305fb => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xcf8),32),32); }
            if c.cond(15) { pc = 0x430630; continue; }
            pc = 0x430603; continue;
            }
            0x430603 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcdc),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xcec),32));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x430630; continue; }
            pc = 0x430613; continue;
            }
            0x430613 => {
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xcf4),32),32); }
            if c.cond(15) { pc = 0x430630; continue; }
            pc = 0x43061b; continue;
            }
            0x43061b => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[6].wrapping_add(0xcac),32); c.idiv32(b); }
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x994),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            pc = 0x430633; continue;
            }
            0x430630 => {
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            pc = 0x430633; continue;
            }
            0x430633 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xce8),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x430672; continue; }
            pc = 0x43063d; continue;
            }
            0x43063d => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xcf8),32),32); }
            if c.cond(15) { pc = 0x430672; continue; }
            pc = 0x430645; continue;
            }
            0x430645 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xce4),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xcec),32));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x430672; continue; }
            pc = 0x430655; continue;
            }
            0x430655 => {
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xcf4),32),32); }
            if c.cond(15) { pc = 0x430672; continue; }
            pc = 0x43065d; continue;
            }
            0x43065d => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[6].wrapping_add(0xcac),32); c.idiv32(b); }
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x994),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            pc = 0x430675; continue;
            }
            0x430672 => {
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            pc = 0x430675; continue;
            }
            0x430675 => {
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(14) { pc = 0x430757; continue; }
            pc = 0x43067e; continue;
            }
            0x43067e => {
            { let a = rm(m,c.r[6].wrapping_add(0xc9c),32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x4307e4; continue; }
            pc = 0x43068b; continue;
            }
            0x43068b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcc4),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4307e4; continue; }
            pc = 0x430699; continue;
            }
            0x430699 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xca4),32),32); }
            if c.cond(13) { pc = 0x4307e4; continue; }
            pc = 0x4306a5; continue;
            }
            0x4306a5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xcd8),32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1c),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4306c3; continue; }
            pc = 0x4306b7; continue;
            }
            0x4306b7 => {
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x1c),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x4306eb; continue; }
            pc = 0x4306c3; continue;
            }
            0x4306c3 => {
            c.set(2,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4ccu32,32); c.push(m, 0x4306dc); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x4306eb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4306eb; continue;
            }
            0x4306eb => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x244),32));
            c.set(5,32,c.r[6].wrapping_add(0x244));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let a = c.r[6].wrapping_add(0xcc4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xca1); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0xca0); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0xcc8); wm(m,a,32,0xffffffffu32); }
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43071f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x430728);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x461dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x2bcu32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x31fu32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x64u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x257u32); }
            c.push(m, 0x430757);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x430757; continue;
            }
            0x430757 => {
            { let a = c.r[6].wrapping_add(0xccc); wm(m,a,8,0x0u32); }
            pc = 0x43075e; continue;
            }
            0x43075e => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x43076f; continue; }
            pc = 0x430762; continue;
            }
            0x430762 => {
            { let a = c.r[6].wrapping_add(0xca8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xca1); wm(m,a,8,0x1u32); }
            pc = 0x43076f; continue;
            }
            0x43076f => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xca1),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xcdc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xce0),32));
            { let a = c.r[6].wrapping_add(0xce4); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0xce8); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x4307a2; continue; }
            pc = 0x430791; continue;
            }
            0x430791 => {
            c.set(0,32,c.r[6].wrapping_add(0xcec));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4307a2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4307a2; continue;
            }
            0x4307a2 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xca0),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4307bd; continue; }
            pc = 0x4307ac; continue;
            }
            0x4307ac => {
            c.set(1,32,c.r[6].wrapping_add(0xcfc));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x4307bd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4307bd; continue;
            }
            0x4307bd => {
            c.set(1,32,c.r[6].wrapping_add(0x9a4));
            c.push(m, 0x4307c8);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43080a; continue; }
            pc = 0x4307cc; continue;
            }
            0x4307cc => {
            c.set(0,32,0xfffffffeu32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4307e4 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xccc),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x430757; continue; }
            pc = 0x4307f2; continue;
            }
            0x4307f2 => {
            { let a = rm(m,c.r[6].wrapping_add(0xcc4),32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x430757; continue; }
            pc = 0x4307fe; continue;
            }
            0x4307fe => {
            { let a = c.r[6].wrapping_add(0xccc); wm(m,a,8,0x1u32); }
            pc = 0x43075e; continue;
            }
            0x43080a => {
            c.set(7,32,c.r[6].wrapping_add(0xa98));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x430817);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4308b4; continue; }
            pc = 0x43081f; continue;
            }
            0x43081f => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xccc),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4308b4; continue; }
            pc = 0x43082d; continue;
            }
            0x43082d => {
            c.set(7,32,c.r[6].wrapping_add(0xb8c));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43083a);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43089e; continue; }
            pc = 0x43083e; continue;
            }
            0x43083e => {
            c.set(2,32,rm(m,0x66e0a0u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m,0x6d1c90u32);
            c.push(m, 0x430854);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43085f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,0x6d1c90u32);
            c.push(m, 0x43086b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x49b180, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6d1c90u32);
            c.push(m, 0x430875);
            { let args = []; c.r[0] = host.call(m, 0x4b70c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x66e0bcu32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x430886);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6d1c90u32);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x430895);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x43089e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43089e; continue;
            }
            0x43089e => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4308b4 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x4308bf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x4308c8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xccc); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x4308da);
            { let args = []; c.r[0] = host.call(m, 0x4429d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            { let a = c.get(1,32); let r = c.add(a,0x164u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4308ee);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6cbb7cu32,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.add(a,0x240u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x430902);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,0x0u32);
            c.push(m, 0x43090f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4349a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbb7cu32,32));
            { let a = c.r[0].wrapping_add(0x11bc); wm(m,a,32,0x4u32); }
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcc4),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            c.push(m,c.get(2,32));
            c.push(m,0x697318u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x430947);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,0x4u32);
            { let a = c.r[0].wrapping_add(0x17c); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x430971);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m, 0x43097c);
            { let args = []; c.r[0] = host.call(m, 0x4335b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x430986);
            { let args = []; c.r[0] = host.call(m, 0x4d64b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1bu32);
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x430992);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8e70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.push(m,0x1u32);
            { let a = c.r[0].wrapping_add(0xd); wm(m,a,8,0x5u32); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xcc4),8));
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.r[2].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x17c),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dd8b4u32,32));
            c.push(m, 0x4309cd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x511270, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4309d7);
            { let args = []; c.r[0] = host.call(m, 0x54a480, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4309e8);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0xfffffffdu32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_process_input(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4324a0;
    loop {
        match pc {
            0x4324a0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65d81cu32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x6cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xba0),32));
            c.set(6,32,c.r[5].wrapping_add(0xba0));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc4),32); c.push(m, 0x4324d2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(0x11cf); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,0x6dcbdbu32,8));
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x432579; continue; }
            pc = 0x4324f8; continue;
            }
            0x4324f8 => {
            { let a = 0x6dcbdbu32; wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x1158),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x432528; continue; }
            pc = 0x43250b; continue;
            }
            0x43250b => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(6,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x43251f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(6,32));
            c.push(m, 0x432525);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x432528; continue;
            }
            0x432528 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x115c),32));
            { let a = c.r[5].wrapping_add(0x1158); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x432555; continue; }
            pc = 0x432538; continue;
            }
            0x432538 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(6,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x43254c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(6,32));
            c.push(m, 0x432552);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x432555; continue;
            }
            0x432555 => {
            { let a = c.r[5].wrapping_add(0x115c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0x11ac); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,0xfffffffeu32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x78u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x432579 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11b8),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4327a2; continue; }
            pc = 0x432589; continue;
            }
            0x432589 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11b4),32));
            c.set(1,32,c.r[5].wrapping_add(0x7ec));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4325a2; continue; }
            pc = 0x43259a; continue;
            }
            0x43259a => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4325a0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4325a9; continue;
            }
            0x4325a2 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4325a9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4325a9; continue;
            }
            0x4325a9 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(3,32,c.r[5].wrapping_add(0x510));
            pc = 0x4325b1; continue;
            }
            0x4325b1 => {
            { let a = c.get(7,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x4325d2; continue; }
            pc = 0x4325b6; continue;
            }
            0x4325b6 => {
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x4325c0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4325d2; continue; }
            pc = 0x4325c4; continue;
            }
            0x4325c4 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x11c0),32),32); }
            if c.cond(5) { pc = 0x4325e3; continue; }
            pc = 0x4325cc; continue;
            }
            0x4325cc => {
            { let a = c.r[5].wrapping_add(0x11c0); wm(m,a,32,c.get(7,32)); }
            pc = 0x4325d2; continue;
            }
            0x4325d2 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xf4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x4325b1; continue; }
            pc = 0x4325de; continue;
            }
            0x4325de => {
            pc = 0x4326a2; continue;
            }
            0x4325e3 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(6,32,c.r[5].wrapping_add(0x510));
            pc = 0x4325eb; continue;
            }
            0x4325eb => {
            { let a = c.get(3,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x43261e; continue; }
            pc = 0x4325f0; continue;
            }
            0x4325f0 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x4325fc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11c0),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,0x66e110u32,32));
            c.set(0,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x510));
            { let a = c.r[0].wrapping_add(0xc4); wm(m,a,32,c.get(1,32)); }
            pc = 0x43261e; continue;
            }
            0x43261e => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xf4u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x4325eb; continue; }
            pc = 0x43262a; continue;
            }
            0x43262a => {
            c.set(1,32,c.get(5,32));
            { let a = c.r[5].wrapping_add(0x11c0); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x432637);
            { let args = []; c.r[0] = host.call(m, 0x4343e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x11d0),32));
            c.set(6,32,c.r[5].wrapping_add(0xd28));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0x11ac),32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x432652);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x461ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11ac),32));
            c.set(1,32,c.r[5].wrapping_add(0xd20));
            c.push(m,c.get(0,32));
            c.push(m, 0x432664);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11ac),32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x11d8),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x432680); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[5].wrapping_add(0x11e0));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x432691);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[5].wrapping_add(0x1200));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4326a2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4326a2; continue;
            }
            0x4326a2 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11c0),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(14) { pc = 0x4326ee; continue; }
            pc = 0x4326ad; continue;
            }
            0x4326ad => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.push(m,0x1u32);
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x510),32));
            c.set(1,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x510));
            { let target = rm(m,c.r[0].wrapping_add(0x124),32); c.push(m, 0x4326cc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11c0),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,0x66e110u32,32));
            c.set(0,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x510));
            { let a = c.r[0].wrapping_add(0xc4); wm(m,a,32,c.get(1,32)); }
            pc = 0x4326ee; continue;
            }
            0x4326ee => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x4326f0; continue;
            }
            0x4326f0 => {
            c.set(1,32,c.r[5].wrapping_add(0xd20));
            c.push(m, 0x4326fb);
            { let args = []; c.r[0] = host.call(m, 0x431260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.r[5].wrapping_add(0x8e0));
            { let a = c.r[5].wrapping_add(0x11ce); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43270e);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4327e4; continue; }
            pc = 0x432716; continue;
            }
            0x432716 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x43271f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xd28));
            c.push(m, 0x43272a);
            { let args = []; c.r[0] = host.call(m, 0x461480, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x1158),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x432751; continue; }
            pc = 0x432734; continue;
            }
            0x432734 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(6,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x432748);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(6,32));
            c.push(m, 0x43274e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x432751; continue;
            }
            0x432751 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x115c),32));
            { let a = c.r[5].wrapping_add(0x1158); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x43277e; continue; }
            pc = 0x432761; continue;
            }
            0x432761 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(6,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x432775);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(6,32));
            c.push(m, 0x43277b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x43277e; continue;
            }
            0x43277e => {
            { let a = c.r[5].wrapping_add(0x115c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0x11ac); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,0xfffffffeu32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x78u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4327a2 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4327a9);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x11b4),32));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xfffffff8),32));
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            c.set(1,32,c.r[5].wrapping_add(0x7ec));
            if c.cond(4) { pc = 0x4327d8; continue; }
            pc = 0x4327cd; continue;
            }
            0x4327cd => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4327d3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4326f0; continue;
            }
            0x4327d8 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4327df); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4326f0; continue;
            }
            0x4327e4 => {
            c.set(6,32,c.r[5].wrapping_add(0x7ec));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4327f1);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4328a9; continue; }
            pc = 0x4327f9; continue;
            }
            0x4327f9 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x432802); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[5].wrapping_add(0x11b8),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x43281b; continue; }
            pc = 0x43280a; continue;
            }
            0x43280a => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x11b4),32));
            { let a = c.r[5].wrapping_add(0x11c4); wm(m,a,32,c.get(1,32)); }
            pc = 0x4328a9; continue;
            }
            0x43281b => {
            c.set(1,32,c.r[5].wrapping_add(0xba0));
            c.push(m, 0x432826);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x432830);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(3,32)); }
            { let a = rm(m,c.r[2].wrapping_add(0xfffffff8),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x432881; continue; }
            pc = 0x432840; continue;
            }
            0x432840 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11ac),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x1158),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0x11ac); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x432860);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11ac),32));
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43286f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x433a80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[5].wrapping_add(0x11cc); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x432895; continue; }
            pc = 0x432879; continue;
            }
            0x432879 => {
            { let a = rm(m,c.r[5].wrapping_add(0x11ac),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0x11ac); wm(m,a,32,r); } }
            pc = 0x432895; continue;
            }
            0x432881 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x11b4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43288f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x433a80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(0x11cc); wm(m,a,8,c.get(0,8)); }
            pc = 0x432895; continue;
            }
            0x432895 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4328a9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4328a9; continue;
            }
            0x4328a9 => {
            c.set(7,32,c.r[5].wrapping_add(0x9d4));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4328b6);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4329fc; continue; }
            pc = 0x4328be; continue;
            }
            0x4328be => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x4328c7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x69b164u32,32));
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4328db);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x69b160u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x94); wm(m,a,32,0x1u32); }
            c.push(m, 0x4328f9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            { let a = c.r[4].wrapping_add(0x90); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43290f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4ac700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x11c),32); c.push(m, 0x43291d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x4329fc; continue; }
            pc = 0x432926; continue;
            }
            0x432926 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x11b4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x432934);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x433cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x10c4),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11b8),32));
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x432955; continue; }
            pc = 0x432945; continue;
            }
            0x432945 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x11ac),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x11d8),32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x43296a; continue; }
            pc = 0x432955; continue;
            }
            0x432955 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x432997; continue; }
            pc = 0x43295a; continue;
            }
            0x43295a => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11ac),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x11dc),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x432997; continue; }
            pc = 0x43296a; continue;
            }
            0x43296a => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xd28),32));
            c.set(6,32,c.r[5].wrapping_add(0xd28));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x432980); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11d0),32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0x11ac),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x432995);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x461ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4329ad; continue;
            }
            0x432997 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xd28),32));
            c.set(6,32,c.r[5].wrapping_add(0xd28));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4329ad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4329ad; continue;
            }
            0x4329ad => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4329c2; continue; }
            pc = 0x4329b1; continue;
            }
            0x4329b1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11d0),32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4329c2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x461dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4329c2; continue;
            }
            0x4329c2 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[5].wrapping_add(0x11b4); wm(m,a,32,0xffffffffu32); }
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4329d8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[5].wrapping_add(0x1200));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4329e9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0x11f0));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x4329fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x4329fc; continue;
            }
            0x4329fc => {
            { let a = rm(m,c.r[5].wrapping_add(0x11c4),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x432e93; continue; }
            pc = 0x432a09; continue;
            }
            0x432a09 => {
            c.set(7,32,c.r[5].wrapping_add(0x164));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x432a17);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.r[5].wrapping_add(0x240));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x432a25);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x258u32);
            c.push(m,0x320u32);
            c.push(m,c.get(3,32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x432a3c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x432a4b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x432a5a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0xffffffffu32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x98),32); c.push(m, 0x432a66); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.get(4,32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0xffffffffu32);
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x432a8b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xffffffffu32);
            c.push(m, 0x432a92);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xfffffffdu32);
            c.set(3,32,c.get(0,32));
            c.push(m, 0x432a9b);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x432aa6);
            { let args = []; c.r[0] = host.call(m, 0x4bb360, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m, 0x432aac);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x432ab6);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x2u32);
            c.push(m, 0x432abd);
            { let args = []; c.r[0] = host.call(m, 0x4bb460, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11c4),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x1158),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m, 0x432ad9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11bc),32));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,0x2u32); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(5) { pc = 0x432b12; continue; }
            pc = 0x432aef; continue;
            }
            0x432aef => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11c0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x432b00; continue; }
            pc = 0x432af9; continue;
            }
            0x432af9 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x3u32); }
            pc = 0x432b17; continue;
            }
            0x432b00 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x6u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            pc = 0x432b17; continue;
            }
            0x432b12 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x432b17; continue;
            }
            0x432b17 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x432b33);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x432b3a);
            { let args = []; c.r[0] = host.call(m, 0x4335b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.set(3,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x432b50);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x432bd8; continue; }
            pc = 0x432b58; continue;
            }
            0x432b58 => {
            c.set(0,32,rm(m,0x6b20ecu32,32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x432b6c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6b20e8u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x94); wm(m,a,32,0x3u32); }
            c.push(m, 0x432b8a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x90); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x432b9f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4ac700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x11c),32); c.push(m, 0x432bad); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[5].wrapping_add(0x11c4); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x432bbb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x432bc3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(3,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x78u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x432bd8 => {
            { let a = rm(m,c.r[5].wrapping_add(0x11bc),32); c.sub(a,0x3u32,32); }
            if c.cond(5) { pc = 0x432efd; continue; }
            pc = 0x432be5; continue;
            }
            0x432be5 => {
            { let a = c.r[5].wrapping_add(0x11cc); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,0x4u32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x6cbb4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,8,0x5u32); }
            c.push(m, 0x432c1a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6cbb4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x432c28);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0x14u32);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x432c37);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0xac44u32); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,0x10u32); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,0x2u32); }
            c.push(m,0x6dcb70u32);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,0x6u32); }
            c.push(m, 0x432c8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6dcb74u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x432c99);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dcb90u32,32));
            c.set(2,32,rm(m,0x6dcb8cu32,32));
            c.set(1,32,0x5u32);
            c.set(6,32,0x6dcb78u32);
            c.set(7,32,c.r[4].wrapping_add(0x3c));
            c.rep_movsd(m);
            c.set(1,32,rm(m,0x6dcb94u32,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcb9cu32,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcb98u32,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dcba0u32,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcba8u32,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcba4u32,32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dcbacu32,32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcbb4u32,32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcbb0u32,32));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1f),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x432d48; continue; }
            pc = 0x432d21; continue;
            }
            0x432d21 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x432d36; continue; }
            pc = 0x432d24; continue;
            }
            0x432d24 => {
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x320u32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x258u32); }
            pc = 0x432d58; continue;
            }
            0x432d36 => {
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x500u32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x400u32); }
            pc = 0x432d58; continue;
            }
            0x432d48 => {
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x400u32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x300u32); }
            pc = 0x432d58; continue;
            }
            0x432d58 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x20),8));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,0x10u32); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x432d70; continue; }
            pc = 0x432d68; continue;
            }
            0x432d68 => {
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,0x20u32); }
            pc = 0x432d70; continue;
            }
            0x432d70 => {
            c.set(2,32,rm(m,0x6cc40cu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x432d81);
            { let args = []; c.r[0] = host.call(m, 0x4bccc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x432e27; continue; }
            pc = 0x432d8c; continue;
            }
            0x432d8c => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x432d97);
            { let args = []; c.r[0] = host.call(m, 0x429a40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x432dad); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xd4),32); c.push(m, 0x432dbb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x1f),8));
            { let a = c.r[0].wrapping_add(0x168); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x20),8));
            { let a = c.r[0].wrapping_add(0x164); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,0x7u32); }
            c.push(m, 0x432dfb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x432e0f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0xfffffffdu32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x78u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x432e27 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(0x1f); wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[2].wrapping_add(0x20); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1f),8));
            { let a = c.r[2].wrapping_add(0x168); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x20),8));
            { let a = c.r[2].wrapping_add(0x164); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,0x8u32); }
            c.push(m, 0x432e7d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x432e91);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x432e93; continue;
            }
            0x432e93 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x11cc),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x432f15; continue; }
            pc = 0x432e9d; continue;
            }
            0x432e9d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x1158),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x432ec4; continue; }
            pc = 0x432ea7; continue;
            }
            0x432ea7 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(6,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x432ebb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(6,32));
            c.push(m, 0x432ec1);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x432ec4; continue;
            }
            0x432ec4 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x115c),32));
            { let a = c.r[5].wrapping_add(0x1158); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x432ef1; continue; }
            pc = 0x432ed4; continue;
            }
            0x432ed4 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(6,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x432ee8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(6,32));
            c.push(m, 0x432eee);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x432ef1; continue;
            }
            0x432ef1 => {
            { let a = c.r[5].wrapping_add(0x115c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0x11ac); wm(m,a,32,c.get(3,32)); }
            pc = 0x432efd; continue;
            }
            0x432efd => {
            c.set(0,32,0xfffffffdu32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x78u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x432f15 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x11ce),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x432f30; continue; }
            pc = 0x432f1f; continue;
            }
            0x432f1f => {
            { let a = c.get(5,32); let r = c.add(a,0x11e0u32,32); c.set(5,32,r); }
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x432f30);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x432f30; continue;
            }
            0x432f30 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x78u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_load_game_from_cstring(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x433510;
    loop {
        match pc {
            0x433510 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d840u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11bc),32));
            c.set(2,32,0x3u32);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x433561; continue; }
            pc = 0x43353f; continue;
            }
            0x43353f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11c0),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x43354f; continue; }
            pc = 0x433549; continue;
            }
            0x433549 => {
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(2,8)); }
            pc = 0x433565; continue;
            }
            0x43354f => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x6u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            pc = 0x433565; continue;
            }
            0x433561 => {
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(1,8)); }
            pc = 0x433565; continue;
            }
            0x433565 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x433581);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x433588);
            { let args = []; c.r[0] = host.call(m, 0x4335b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.set(3,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43359b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_delete_savegame(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x433cc0;
    loop {
        match pc {
            0x433cc0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65d8fau32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x433cef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4348c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            c.push(m, 0x433d0b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x433d17);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x697428u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x433d28);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x697430u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x433d35);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd8));
            c.push(m, 0x433d3d);
            { let args = []; c.r[0] = host.call(m, 0x653705, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3u32); }
            c.push(m,c.get(2,32));
            c.push(m, 0x433d4a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x653f49, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x433d5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_save_screen_remove_list_entry(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x433d8d;
    loop {
        match pc {
            0x433d8d => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1u32); }
            c.push(m, 0x433d99);
            { let args = []; c.r[0] = host.call(m, 0x653743, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.set(7,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x11ac); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(0,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m, 0x433db6);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x8u32); }
            if c.cond(4) { pc = 0x433de1; continue; }
            pc = 0x433dc4; continue;
            }
            0x433dc4 => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(3,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x433ddc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(3,32)); }
            pc = 0x433de8; continue;
            }
            0x433de1 => {
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,0x0u32); }
            pc = 0x433de8; continue;
            }
            0x433de8 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(1,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m, 0x433e00);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x9u32); }
            if c.cond(4) { pc = 0x433e2b; continue; }
            pc = 0x433e0e; continue;
            }
            0x433e0e => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(3,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x433e26);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(3,32)); }
            pc = 0x433e32; continue;
            }
            0x433e2b => {
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,0x0u32); }
            pc = 0x433e32; continue;
            }
            0x433e32 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x433e95; continue; }
            pc = 0x433e46; continue;
            }
            0x433e46 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe8),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,c.get(1,32)); }
            pc = 0x433e51; continue;
            }
            0x433e51 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x8),32),32); }
            if c.cond(4) { pc = 0x433e86; continue; }
            pc = 0x433e56; continue;
            }
            0x433e56 => {
            c.set(7,32,c.r[0].wrapping_mul(4));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x433e6d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x115c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x433e7d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffdc),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            pc = 0x433e86; continue;
            }
            0x433e86 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x11ac),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x433e51; continue; }
            pc = 0x433e95; continue;
            }
            0x433e95 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1158),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x433ebc; continue; }
            pc = 0x433e9f; continue;
            }
            0x433e9f => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x433eb3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x433eb9);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x433ebc; continue;
            }
            0x433ebc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x115c),32));
            { let a = c.r[6].wrapping_add(0x1158); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x433eed; continue; }
            pc = 0x433ed0; continue;
            }
            0x433ed0 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x433ee4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x433eea);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x433eed; continue;
            }
            0x433eed => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe8),32));
            { let a = c.r[6].wrapping_add(0x115c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[6].wrapping_add(0x1158); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            c.push(m, 0x433f0b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x433f1a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_update_connection_mode(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43a2a0;
    loop {
        match pc {
            0x43a2a0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65e258u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x34u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(3,32,0x0u32);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x54d0),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43a431; continue; }
            pc = 0x43a2d0; continue;
            }
            0x43a2d0 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x43a6bc; continue; }
            pc = 0x43a2d7; continue;
            }
            0x43a2d7 => {
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[6].wrapping_add(0x54d0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x5670); wm(m,a,32,0x8u32); }
            c.push(m, 0x43a2f1);
            { let args = []; c.r[0] = host.call(m, 0x54aae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sbb(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            if c.cond(4) { pc = 0x43a341; continue; }
            pc = 0x43a2fe; continue;
            }
            0x43a2fe => {
            c.push(m, 0x43a303);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43a30b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6cbba0u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x43a319);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,0x6df330u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x3u32); }
            c.push(m, 0x43a330);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x5d20); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6df2f0u32; wm(m,a,32,c.get(3,32)); }
            pc = 0x43aa8d; continue;
            }
            0x43a341 => {
            { let a = c.r[6].wrapping_add(0x20d); wm(m,a,8,0x0u32); }
            c.push(m, 0x43a34d);
            { let args = []; c.r[0] = host.call(m, 0x4392e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x5668),32));
            c.push(m,0x4890u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[6].wrapping_add(0x20c); wm(m,a,8,0x0u32); }
            c.push(m, 0x43a36a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x54ac80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x43a3d7; continue; }
            pc = 0x43a372; continue;
            }
            0x43a372 => {
            c.push(m, 0x43a377);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x43a382);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6cbba1u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x43a390);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,0x6df330u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x4u32); }
            c.push(m, 0x43a3a7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            { let a = 0x6df2f0u32; wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697798u32);
            c.push(m, 0x43a3c4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43a3cb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.r[6].wrapping_add(0x5d20); wm(m,a,8,0x0u32); }
            pc = 0x43aa8d; continue;
            }
            0x43a3d7 => {
            c.set(0,32,rm(m,0x6df31cu32,32));
            c.push(m,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x4cb0); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43a3e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ef70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x57c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43a3f6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[6].wrapping_add(0x4224); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x43a422);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x4ca0); wm(m,a,32,0x2u32); }
            pc = 0x43a6bc; continue;
            }
            0x43a431 => {
            { let a = c.r[6].wrapping_add(0x54d0); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m,0x2u32);
            c.push(m, 0x43a444);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x497ee0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x43a452); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x43a6bc; continue; }
            pc = 0x43a45b; continue;
            }
            0x43a45b => {
            c.set(7,32,c.r[6].wrapping_add(0x1788));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43a468);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43a475);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0x1908));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43a486);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43a493);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43a4a7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x43f690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m,0x1u32);
            c.push(m,0x8u32);
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,0x1u32); }
            c.push(m, 0x43a4c3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x54b230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            c.set(1,32,c.get(6,32));
            if c.cond(4) { pc = 0x43a512; continue; }
            pc = 0x43a4ca; continue;
            }
            0x43a4ca => {
            c.push(m,c.get(0,32));
            c.push(m, 0x43a4d0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6cbb9fu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x43a4de);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,0x6df330u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x2u32); }
            c.push(m, 0x43a4f2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[6].wrapping_add(0x54d0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x5d20); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x1u32); }
            c.push(m, 0x43a50d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43a6ab; continue;
            }
            0x43a512 => {
            { let a = c.r[6].wrapping_add(0x4ca0); wm(m,a,32,0x2u32); }
            c.push(m, 0x43a521);
            { let args = []; c.r[0] = host.call(m, 0x438ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.push(m,0x20u32);
            { let a = c.get(0,32); let r = c.add(a,0x1b80u32,32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x4c6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x43a53a);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[2].wrapping_add(0x168));
            c.push(m, 0x43a54e);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43a556);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(1,32,c.r[0].wrapping_add(0x2e8));
            c.push(m, 0x43a566);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43a56e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2b7c),32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x46c),32));
            c.set(1,32,c.r[6].wrapping_add(0x2b7c));
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,c.get(0,8)); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43a595); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4cd1),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2a94),32));
            c.set(1,32,c.r[6].wrapping_add(0x2a94));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(2,8,u32::from(c.cond(4)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43a5b0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4cd1),8));
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x468),32));
            { let a = c.r[6].wrapping_add(0x1cc); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,c.get(0,8));
            { let a = c.r[6].wrapping_add(0x4860); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1c4); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x204); wm(m,a,8,0x2u32); }
            c.push(m, 0x43a5ea);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ea60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x1f4); wm(m,a,8,0x1u32); }
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.r[6].wrapping_add(0x1fc); wm(m,a,8,0x1u32); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x15eu32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x320u32); }
            c.set(0,32,c.r[1].wrapping_mul(4).wrapping_add(0x64));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x1cu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43a638);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697784u32);
            c.push(m, 0x43a64f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43a656);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(3,32));
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.set(0,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43a66b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x440440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43a672);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(3,32));
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69778cu32);
            c.push(m, 0x43a689);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43a690);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(3,32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697794u32);
            c.push(m, 0x43a6a4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43a6ab);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x43a6ab; continue;
            }
            0x43a6ab => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43a6bc);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43a6bc; continue;
            }
            0x43a6bc => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5674),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43a6d2; continue; }
            pc = 0x43a6c6; continue;
            }
            0x43a6c6 => {
            { let a = c.r[6].wrapping_add(0x5674); wm(m,a,8,0x0u32); }
            pc = 0x43b2b1; continue;
            }
            0x43a6d2 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x930));
            pc = 0x43a6da; continue;
            }
            0x43a6da => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x43a6e4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43a6f0; continue; }
            pc = 0x43a6e8; continue;
            }
            0x43a6e8 => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4ca0),32),32); }
            if c.cond(5) { pc = 0x43a6fe; continue; }
            pc = 0x43a6f0; continue;
            }
            0x43a6f0 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xf4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x43a6da; continue; }
            pc = 0x43a6fc; continue;
            }
            0x43a6fc => {
            pc = 0x43a740; continue;
            }
            0x43a6fe => {
            c.set(7,32,c.r[6].wrapping_add(0x654));
            c.set(5,32,0x3u32);
            pc = 0x43a709; continue;
            }
            0x43a709 => {
            c.set(0,32,rm(m,0x66e110u32,32));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[7].wrapping_add(0x3a0); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43a71d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x2dc),32));
            c.set(1,32,c.r[7].wrapping_add(0x2dc));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x124),32); c.push(m, 0x43a731); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xf4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x43a709; continue; }
            pc = 0x43a73a; continue;
            }
            0x43a73a => {
            { let a = c.r[6].wrapping_add(0x4ca0); wm(m,a,32,c.get(3,32)); }
            pc = 0x43a740; continue;
            }
            0x43a740 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d20),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43a75b; continue; }
            pc = 0x43a74a; continue;
            }
            0x43a74a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x748),32));
            c.set(1,32,c.r[6].wrapping_add(0x748));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43a75b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43a75b; continue;
            }
            0x43a75b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(14) { pc = 0x43a7c2; continue; }
            pc = 0x43a766; continue;
            }
            0x43a766 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.push(m,0x1u32);
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,0x66e110u32,32));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x930));
            { let a = c.r[0].wrapping_add(0xc4); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x930),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x930));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x43a7a7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x43a7c2; continue; }
            pc = 0x43a7b1; continue;
            }
            0x43a7b1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x748),32));
            c.set(1,32,c.r[6].wrapping_add(0x748));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43a7c2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43a7c2; continue;
            }
            0x43a7c2 => {
            c.set(5,32,c.r[6].wrapping_add(0xef8));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43a7cf);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x43b279; continue; }
            pc = 0x43a7d7; continue;
            }
            0x43a7d7 => {
            c.set(0,8,rm(m,0x6dcbdbu32,8));
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(5) { pc = 0x43b279; continue; }
            pc = 0x43a7f7; continue;
            }
            0x43a7f7 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x654));
            pc = 0x43a7ff; continue;
            }
            0x43a7ff => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x43a806);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x43a818; continue; }
            pc = 0x43a80a; continue;
            }
            0x43a80a => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xf4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x43a7ff; continue; }
            pc = 0x43a816; continue;
            }
            0x43a816 => {
            pc = 0x43a83a; continue;
            }
            0x43a818 => {
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.r[6].wrapping_add(0x4ca4); wm(m,a,32,c.get(7,32)); }
            c.push(m,0x1u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x654),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x654));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x43a83a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43a83a; continue;
            }
            0x43a83a => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(5,32),32); c.set(7,32,r); }
            if c.cond(4) { pc = 0x43aed1; continue; }
            pc = 0x43a844; continue;
            }
            0x43a844 => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(4) { pc = 0x43ad3d; continue; }
            pc = 0x43a84b; continue;
            }
            0x43a84b => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x43b261; continue; }
            pc = 0x43a852; continue;
            }
            0x43a852 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x210); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4cbc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4220); wm(m,a,32,c.get(3,32)); }
            c.set(7,32,0x4u32);
            if c.cond(4) { pc = 0x43a890; continue; }
            pc = 0x43a876; continue;
            }
            0x43a876 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43a888; continue; }
            pc = 0x43a879; continue;
            }
            0x43a879 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x43a89a; continue; }
            pc = 0x43a87c; continue;
            }
            0x43a87c => {
            { let a = 0x6df2f0u32; wm(m,a,32,0x8u32); }
            pc = 0x43a89a; continue;
            }
            0x43a888 => {
            { let a = 0x6df2f0u32; wm(m,a,32,c.get(7,32)); }
            pc = 0x43a89a; continue;
            }
            0x43a890 => {
            { let a = 0x6df2f0u32; wm(m,a,32,0x2u32); }
            pc = 0x43a89a; continue;
            }
            0x43a89a => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43a8a5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x43a8ba);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x431cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x43a8c8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(4) { pc = 0x43ad13; continue; }
            pc = 0x43a8d1; continue;
            }
            0x43a8d1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43a908; continue; }
            pc = 0x43a8db; continue;
            }
            0x43a8db => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43a8f6; continue; }
            pc = 0x43a8de; continue;
            }
            0x43a8de => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x43a91c; continue; }
            pc = 0x43a8e1; continue;
            }
            0x43a8e1 => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = 0x6df2f0u32; wm(m,a,32,0x8u32); }
            { let a = c.r[0].wrapping_add(0xd); wm(m,a,8,0x2u32); }
            pc = 0x43a91c; continue;
            }
            0x43a8f6 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = 0x6df2f0u32; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[1].wrapping_add(0xd); wm(m,a,8,0x1u32); }
            pc = 0x43a91c; continue;
            }
            0x43a908 => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = 0x6df2f0u32; wm(m,a,32,0x2u32); }
            { let a = c.r[2].wrapping_add(0xd); wm(m,a,8,0x0u32); }
            pc = 0x43a91c; continue;
            }
            0x43a91c => {
            c.set(0,32,rm(m,0x66e0a4u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x43a92d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[6].wrapping_add(0x20c); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x5664); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x43a982; continue; }
            pc = 0x43a945; continue;
            }
            0x43a945 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x43a950);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x8),8));
            { let a = c.get(1,8); let r = c.and(a,0x4u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0x4u32,8); }
            if c.cond(4) { pc = 0x43a975; continue; }
            pc = 0x43a95b; continue;
            }
            0x43a95b => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x5664),32));
            { let a = c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x54d4); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5664),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x5664); wm(m,a,32,c.get(0,32)); }
            pc = 0x43a975; continue;
            }
            0x43a975 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x43a945; continue; }
            pc = 0x43a982; continue;
            }
            0x43a982 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x43abac; continue; }
            pc = 0x43a992; continue;
            }
            0x43a992 => {
            c.push(m, 0x43a997);
            { let args = []; c.r[0] = host.call(m, 0x438ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x4cd4));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43a9a5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42e080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x43a9af); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            c.set(1,32,c.get(6,32));
            if c.cond(5) { pc = 0x43adda; continue; }
            pc = 0x43a9ba; continue;
            }
            0x43a9ba => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43a9c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43f2b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[6].wrapping_add(0x1788));
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1c4),8));
            c.set(1,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x1a4));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4860); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43a9f6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0x1908));
            c.set(2,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x184));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x43aa0b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1cc),8));
            c.set(1,32,c.get(5,32));
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x43aa1f);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43aa2c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x7u32); }
            c.push(m, 0x43aa3b);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43aa48);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0xffffffffu32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43aa5c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x43f690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,0x1u32);
            c.push(m,0x8u32);
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,0x8u32); }
            c.push(m, 0x43aa79);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x54b230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x43aab6; continue; }
            pc = 0x43aa7e; continue;
            }
            0x43aa7e => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43aa86);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43aa8d);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43aa8d; continue;
            }
            0x43aa8d => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43aa9e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x1a0au32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43aab6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43aac7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x43aad6);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x210),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xa8u32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54d4),32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43ab02);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43ab0c);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6977f0u32);
            c.push(m, 0x43ab24);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43ab2b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.set(0,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43ab41);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x440440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43ab48);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6977f8u32);
            c.push(m, 0x43ab60);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43ab67);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697800u32);
            c.push(m, 0x43ab7c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43ab83);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43ab94);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x1a0au32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43abac => {
            c.push(m, 0x43abb1);
            { let args = []; c.r[0] = host.call(m, 0x438ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43abc2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1a88),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43abd6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x13bc),32));
            c.set(7,32,c.r[6].wrapping_add(0x13bc));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43abec); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43abf5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x4ca8); wm(m,a,32,0x5u32); }
            { let a = c.r[6].wrapping_add(0x210); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x43ac0b; continue;
            }
            0x43ac0b => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x210),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5664),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x43aca8; continue; }
            pc = 0x43ac20; continue;
            }
            0x43ac20 => {
            c.set(5,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x54d4));
            pc = 0x43ac27; continue;
            }
            0x43ac27 => {
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x43ac35);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x9),8); c.and(a,0x80u32,8); }
            if c.cond(5) { pc = 0x43ac4b; continue; }
            pc = 0x43ac3b; continue;
            }
            0x43ac3b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5664),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x43ac27; continue; }
            pc = 0x43ac49; continue;
            }
            0x43ac49 => {
            pc = 0x43ac51; continue;
            }
            0x43ac4b => {
            { let a = c.r[6].wrapping_add(0x210); wm(m,a,32,c.get(7,32)); }
            pc = 0x43ac51; continue;
            }
            0x43ac51 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x5664),32),32); }
            if c.cond(13) { pc = 0x43aca8; continue; }
            pc = 0x43ac59; continue;
            }
            0x43ac59 => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43ac61);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43f2b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[3].wrapping_add(c.r[6]).wrapping_add(0x204); wm(m,a,8,0x2u32); }
            c.push(m, 0x43ac72);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x210),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0xa8u32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54d4),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x43ac9e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43ac0b; continue; }
            pc = 0x43aca8; continue;
            }
            0x43aca8 => {
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x204),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1c4),8));
            c.set(1,32,c.r[6].wrapping_add(0x1a4));
            { let a = c.get(3,8); let r = c.or(a,0x80u32,8); c.set(3,8,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x1788));
            { let a = c.r[6].wrapping_add(0x204); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4860); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43acdd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[6].wrapping_add(0x184));
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            c.push(m,c.get(2,32));
            c.push(m, 0x43acef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1cc),8));
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,0x1a0au32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43ad13 => {
            c.set(1,32,rm(m,0x66e0a4u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43ad25);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,0x1a0au32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43ad3d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43ad65; continue; }
            pc = 0x43ad47; continue;
            }
            0x43ad47 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43ad59; continue; }
            pc = 0x43ad4a; continue;
            }
            0x43ad4a => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x43ad6f; continue; }
            pc = 0x43ad4d; continue;
            }
            0x43ad4d => {
            { let a = 0x6df2f0u32; wm(m,a,32,0x8u32); }
            pc = 0x43ad6f; continue;
            }
            0x43ad59 => {
            { let a = 0x6df2f0u32; wm(m,a,32,0x4u32); }
            pc = 0x43ad6f; continue;
            }
            0x43ad65 => {
            { let a = 0x6df2f0u32; wm(m,a,32,0x2u32); }
            pc = 0x43ad6f; continue;
            }
            0x43ad6f => {
            { let a = rm(m,c.r[6].wrapping_add(0x4ca0),32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x43aeb2; continue; }
            pc = 0x43ad7c; continue;
            }
            0x43ad7c => {
            c.set(2,32,rm(m,0x6af200u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x43ad8f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x5678));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43ad9c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x422840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x11c),32); c.push(m, 0x43ada6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x43b261; continue; }
            pc = 0x43adaf; continue;
            }
            0x43adaf => {
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[6].wrapping_add(0x20d); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x20c); wm(m,a,8,0x0u32); }
            c.push(m, 0x43adc7);
            { let args = []; c.r[0] = host.call(m, 0x54aae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43adfd; continue; }
            pc = 0x43add0; continue;
            }
            0x43add0 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43add8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            pc = 0x43adda; continue;
            }
            0x43adda => {
            c.push(m, 0x43addf);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6df2f0u32; wm(m,a,32,c.get(5,32)); }
            c.set(0,32,0x1a0au32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43adfd => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x5d1c),32));
            c.push(m,0x4890u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43ae13);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x54ac80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            c.set(1,32,c.get(6,32));
            if c.cond(4) { pc = 0x43ae47; continue; }
            pc = 0x43ae19; continue;
            }
            0x43ae19 => {
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43ae22);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            { let a = 0x6df2f0u32; wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x43ae2f);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x1a0au32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43ae47 => {
            c.set(2,32,rm(m,0x6df31cu32,32));
            { let a = c.r[6].wrapping_add(0x4cb0); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x43ae58);
            { let args = []; c.r[0] = host.call(m, 0x4392e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43ae60);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ef70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x57c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43ae6e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[6].wrapping_add(0x4224); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x43ae9a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,0x1a0au32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43aeb2 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43aeb9);
            { let args = []; c.r[0] = host.call(m, 0x439680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x1a0au32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43aed1 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43aedc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x20c); wm(m,a,8,0x0u32); }
            { let a = 0x6df2f0u32; wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43af07; continue; }
            pc = 0x43aef3; continue;
            }
            0x43aef3 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43af00; continue; }
            pc = 0x43aef6; continue;
            }
            0x43aef6 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x43af0b; continue; }
            pc = 0x43aef9; continue;
            }
            0x43aef9 => {
            c.set(0,32,0x2u32);
            pc = 0x43af0f; continue;
            }
            0x43af00 => {
            c.set(0,32,0x1u32);
            pc = 0x43af0f; continue;
            }
            0x43af07 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x43af0f; continue;
            }
            0x43af0b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x43af0f; continue;
            }
            0x43af0f => {
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43af1b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x497ee0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x43af29); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(4) { pc = 0x43b198; continue; }
            pc = 0x43af32; continue;
            }
            0x43af32 => {
            c.set(0,32,rm(m,0x66e0a4u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x43af43);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43af6b; continue; }
            pc = 0x43af4d; continue;
            }
            0x43af4d => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43af5f; continue; }
            pc = 0x43af50; continue;
            }
            0x43af50 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x43af75; continue; }
            pc = 0x43af53; continue;
            }
            0x43af53 => {
            { let a = 0x6df2f0u32; wm(m,a,32,0x8u32); }
            pc = 0x43af75; continue;
            }
            0x43af5f => {
            { let a = 0x6df2f0u32; wm(m,a,32,0x4u32); }
            pc = 0x43af75; continue;
            }
            0x43af6b => {
            { let a = 0x6df2f0u32; wm(m,a,32,0x2u32); }
            pc = 0x43af75; continue;
            }
            0x43af75 => {
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m,0x20u32);
            { let a = c.get(1,32); let r = c.add(a,0x1b80u32,32); c.set(1,32,r); }
            c.set(2,32,c.r[6].wrapping_add(0x4c6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x43af90);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x1788));
            c.set(1,32,c.r[0].wrapping_add(0x168));
            c.push(m, 0x43afa9);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43afb1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(5,32,c.r[6].wrapping_add(0x1908));
            { let a = c.get(1,32); let r = c.add(a,0x2e8u32,32); c.set(1,32,r); }
            c.push(m, 0x43afc8);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43afd0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6d1c7cu32,32));
            c.set(1,32,c.r[6].wrapping_add(0x2b7c));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x46c),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,c.get(0,8)); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43aff3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4cd1),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2a94),32));
            c.set(1,32,c.r[6].wrapping_add(0x2a94));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(2,8,u32::from(c.cond(4)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43b00e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4cd1),8));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x468),32));
            { let a = c.r[6].wrapping_add(0x1cc); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,c.get(0,8));
            { let a = c.r[6].wrapping_add(0x4860); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1c4); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x204); wm(m,a,8,0x2u32); }
            c.push(m, 0x43b049);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ea60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.r[6].wrapping_add(0x1f4); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x1fc); wm(m,a,8,0x1u32); }
            if c.cond(14) { pc = 0x43b123; continue; }
            pc = 0x43b065; continue;
            }
            0x43b065 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43b06c);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43b079);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x5u32); }
            c.push(m, 0x43b088);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43b095);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x43b0a8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x43f690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,0x1u32);
            c.push(m,0x8u32);
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,0x6u32); }
            c.push(m, 0x43b0c5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x54b230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            c.set(1,32,c.get(6,32));
            if c.cond(4) { pc = 0x43b10f; continue; }
            pc = 0x43b0cc; continue;
            }
            0x43b0cc => {
            c.push(m,c.get(0,32));
            c.push(m, 0x43b0d2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43b0e3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43b0ea);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x43b0f7);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x1a0au32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43b10f => {
            c.push(m, 0x43b114);
            { let args = []; c.r[0] = host.call(m, 0x438ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x43b121);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43b17f; continue;
            }
            0x43b123 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43b12a);
            { let args = []; c.r[0] = host.call(m, 0x438ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43b13b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1a88),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43b14f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x13bc),32));
            c.set(7,32,c.r[6].wrapping_add(0x13bc));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43b165); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43b16e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x4ca8); wm(m,a,32,0x5u32); }
            { let a = c.r[6].wrapping_add(0x204); wm(m,a,8,0x82u32); }
            pc = 0x43b17f; continue;
            }
            0x43b17f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43b18d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ea60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43b196);
            { let args = []; c.r[0] = host.call(m, 0x439de0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43b1aa; continue;
            }
            0x43b198 => {
            c.set(1,32,rm(m,0x66e0a4u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43b1aa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x43b1aa; continue;
            }
            0x43b1aa => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x15eu32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x320u32); }
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_mul(4).wrapping_add(0x64));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x1cu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m, 0x43b1ea);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6977c0u32);
            c.push(m, 0x43b202);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43b209);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43b21f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x440440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43b226);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,0xffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6977c8u32);
            c.push(m, 0x43b23e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43b245);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6977d0u32);
            c.push(m, 0x43b25a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43b261);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x43b261; continue;
            }
            0x43b261 => {
            c.set(0,32,0x1a0au32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43b279 => {
            { let a = 0x6dcbdbu32; wm(m,a,8,0x0u32); }
            { let a = c.get(6,32); let r = c.add(a,0x654u32,32); c.set(6,32,r); }
            c.set(7,32,0x3u32);
            pc = 0x43b28b; continue;
            }
            0x43b28b => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43b294); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0xf4u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x43b28b; continue; }
            pc = 0x43b29d; continue;
            }
            0x43b29d => {
            { let a = 0x6df2f0u32; wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x43b2b1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43b2b1; continue;
            }
            0x43b2b1 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0xfffffffeu32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_start_game(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43c450;
    loop {
        match pc {
            0x43c450 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65e2b0u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x43c481);
            { let args = []; c.r[0] = host.call(m, 0x4429d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x43c48f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4349a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43c4ca; continue; }
            pc = 0x43c499; continue;
            }
            0x43c499 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43c4b4; continue; }
            pc = 0x43c49c; continue;
            }
            0x43c49c => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x43c4dd; continue; }
            pc = 0x43c49f; continue;
            }
            0x43c49f => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = 0x6df2f0u32; wm(m,a,32,0x8u32); }
            { let a = c.r[0].wrapping_add(0xd); wm(m,a,8,0x2u32); }
            pc = 0x43c4dd; continue;
            }
            0x43c4b4 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = 0x6df2f0u32; wm(m,a,32,0x4u32); }
            { let a = c.r[1].wrapping_add(0xd); wm(m,a,8,0x1u32); }
            pc = 0x43c4dd; continue;
            }
            0x43c4ca => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = 0x6df2f0u32; wm(m,a,32,0x2u32); }
            { let a = c.r[2].wrapping_add(0xd); wm(m,a,8,c.get(3,8)); }
            pc = 0x43c4dd; continue;
            }
            0x43c4dd => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697824u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x43c4f1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x43c4f6);
            { let args = []; c.r[0] = host.call(m, 0x42da50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let args = []; c.r[0] = host.call(m, 0x43c596, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_start_game_tail(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43c596;
    loop {
        match pc {
            0x43c596 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x1c4),8));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43c5c0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x58a4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,8,rm(m,0x6dfc14u32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6e55d0u32);
            c.push(m, 0x43c5d1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x611e50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = c.r[6].wrapping_add(0x4cd0); wm(m,a,8,0x1u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_update_hotseat_setup_tail_enter_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43e148;
    loop {
        match pc {
            0x43e148 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffb0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(0xffffffa0));
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,0xc8u32); }
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xffffffa4); wm(m,a,32,0x7du32); }
            { let a = c.r[5].wrapping_add(0xffffffa8); wm(m,a,32,0x19u32); }
            { let a = c.r[5].wrapping_add(0xffffffac); wm(m,a,32,0x0u32); }
            c.push(m, 0x43e17b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43e185);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x43e196);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,0xffu32);
            c.set(1,32,0x6e6220u32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x43e1a9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6141d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x19),8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x43e1c2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x58a4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4cd0); wm(m,a,8,0x1u32); }
            c.push(m, 0x43e1d0);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,0x1a0au32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_draw_wait_overlay(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43f1a0;
    loop {
        match pc {
            0x43f1a0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0xffffffffu32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x43f1ad);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3cc),32));
            c.set(1,32,c.r[6].wrapping_add(0x3cc));
            { let a = c.r[2]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x43f1dd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x43f1f6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4b58c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x3u32);
            c.push(m, 0x43f1fd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x43f20a);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x43f210);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dce78u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,0x6dce78u32);
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x43f225); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,0x258u32);
            c.set(1,32,rm(m,0x69af5cu32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x14u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x190u32);
            c.push(m, 0x43f24a);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dce78u32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,0x6dce78u32);
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x43f25f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,0x258u32);
            c.set(1,32,rm(m,0x69af60u32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x190u32);
            c.push(m, 0x43f284);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
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

pub fn multiplayer_screen_reset_to_connection_mode(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43fba0;
    loop {
        match pc {
            0x43fba0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x43fbad);
            { let args = []; c.r[0] = host.call(m, 0x439de0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[6].wrapping_add(0x57c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43fbbb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xef8),32));
            c.set(1,32,c.r[6].wrapping_add(0xef8));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x43fbcc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x930));
            c.set(3,32,0x3u32);
            pc = 0x43fbd7; continue;
            }
            0x43fbd7 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43fbe3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43fbec); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x124),32); c.push(m, 0x43fbf8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xfffffd24),32));
            c.set(5,32,c.r[7].wrapping_add(0xfffffd24));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43fc0e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43fc18); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xf4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x43fbd7; continue; }
            pc = 0x43fc21; continue;
            }
            0x43fc21 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d20),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43fc4d; continue; }
            pc = 0x43fc2b; continue;
            }
            0x43fc2b => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x930),32));
            c.set(1,32,c.r[6].wrapping_add(0x930));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43fc3c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa24),32));
            c.set(1,32,c.r[6].wrapping_add(0xa24));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43fc4d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43fc4d; continue;
            }
            0x43fc4d => {
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43fc56);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43e550, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x43fc8c; continue; }
            pc = 0x43fc62; continue;
            }
            0x43fc62 => {
            c.set(3,32,c.r[6].wrapping_add(0x4248));
            pc = 0x43fc68; continue;
            }
            0x43fc68 => {
            c.set(7,32,rm(m,c.r[3],32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x43fc7e; continue; }
            pc = 0x43fc6e; continue;
            }
            0x43fc6e => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43fc75);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x43fc7b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x43fc7e; continue;
            }
            0x43fc7e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x43fc68; continue; }
            pc = 0x43fc8c; continue;
            }
            0x43fc8c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4498),32));
            c.set(1,32,c.r[6].wrapping_add(0x4498));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x443c); wm(m,a,32,c.get(7,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43fca7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1a88),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            c.push(m,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43fcba); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xfec),32));
            c.set(1,32,c.r[6].wrapping_add(0xfec));
            c.push(m,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43fccd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x15c8),32));
            c.set(1,32,c.r[6].wrapping_add(0x15c8));
            c.push(m,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43fce0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4ca8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x4ca0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4cb0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x5670); wm(m,a,32,0x8u32); }
            c.push(m, 0x43fd06);
            { let args = []; c.r[0] = host.call(m, 0x4401c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_main_window_run(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x463d20;
    loop {
        match pc {
            0x463d20 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65ef0du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x54u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1e4u32);
            c.set(5,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,0x1u32); }
            c.push(m, 0x463d4d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x463d67; continue; }
            pc = 0x463d5e; continue;
            }
            0x463d5e => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x463d65);
            { let args = []; c.r[0] = host.call(m, 0x4419a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x463d69; continue;
            }
            0x463d67 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x463d69; continue;
            }
            0x463d69 => {
            c.set(1,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0xffffffffu32); }
            { let a = 0x6cbdb8u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x463d7d);
            { let args = []; c.r[0] = host.call(m, 0x441cd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x463d88);
            { let args = []; c.r[0] = host.call(m, 0x441e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x698a28u32);
            c.push(m, 0x463d9a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x44),32));
            c.set(1,32,c.r[5].wrapping_add(0x44));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x463da3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x464230; continue; }
            pc = 0x463dab; continue;
            }
            0x463dab => {
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x463db5);
            { let args = []; c.r[0] = host.call(m, 0x54a310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.r[5].wrapping_add(0x44));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x463dbf);
            { let args = []; c.r[0] = host.call(m, 0x42f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x2fc),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x463df1; continue; }
            pc = 0x463dc9; continue;
            }
            0x463dc9 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x463dd3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffbu32,32); }
            if c.cond(4) { pc = 0x464245; continue; }
            pc = 0x463ddc; continue;
            }
            0x463ddc => {
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x463dec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x463e7a; continue;
            }
            0x463df1 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x463e34; continue; }
            pc = 0x463df6; continue;
            }
            0x463df6 => {
            c.set(1,32,c.r[5].wrapping_add(0x308));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x463e06);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,0x1u32); }
            c.set(1,32,c.r[0].wrapping_add(0x2e8));
            c.push(m, 0x463e23);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x463e34);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x463e34; continue;
            }
            0x463e34 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x304),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x300),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x2fc),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x463e54);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4381e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x463e62); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(4) { pc = 0x463e7a; continue; }
            pc = 0x463e67; continue;
            }
            0x463e67 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x463e71); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffbu32,32); }
            if c.cond(4) { pc = 0x464245; continue; }
            pc = 0x463e7a; continue;
            }
            0x463e7a => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,0x2u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(6,32)); }
            c.push(m,0x6cbb4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,8,0x3u32); }
            c.push(m, 0x463ea3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6cbb4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x463eb1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x14u32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x463ebe);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0xac44u32); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0x10u32); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(6,32)); }
            c.push(m,0x6dcb70u32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,0x4u32); }
            c.push(m, 0x463f0b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6dcb74u32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x463f19);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcb8cu32,32));
            c.set(0,32,rm(m,0x6dcb90u32,32));
            c.set(1,32,0x5u32);
            c.set(6,32,0x6dcb78u32);
            c.set(7,32,c.r[4].wrapping_add(0x24));
            c.rep_movsd(m);
            c.set(1,32,rm(m,0x6dcb94u32,32));
            c.set(6,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcb98u32,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcb9cu32,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dcba0u32,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcba4u32,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcba8u32,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dcbacu32,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcbb0u32,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcbb4u32,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x168),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x164),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x46405c; continue; }
            pc = 0x463fbb; continue;
            }
            0x463fbb => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x463fd1; continue; }
            pc = 0x463fbf; continue;
            }
            0x463fbf => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x320u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x258u32); }
            pc = 0x463ffd; continue;
            }
            0x463fd1 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x463fe8; continue; }
            pc = 0x463fd6; continue;
            }
            0x463fd6 => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x400u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x300u32); }
            pc = 0x463ffd; continue;
            }
            0x463fe8 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x463ffd; continue; }
            pc = 0x463fed; continue;
            }
            0x463fed => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x500u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x400u32); }
            pc = 0x463ffd; continue;
            }
            0x463ffd => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x164),32));
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x464011; continue; }
            pc = 0x464007; continue;
            }
            0x464007 => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x10u32); }
            pc = 0x46401e; continue;
            }
            0x464011 => {
            { let a = c.get(6,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x46401e; continue; }
            pc = 0x464016; continue;
            }
            0x464016 => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x20u32); }
            pc = 0x46401e; continue;
            }
            0x46401e => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x1c),32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x46402c);
            { let args = []; c.r[0] = host.call(m, 0x4bccc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x46405c; continue; }
            pc = 0x464033; continue;
            }
            0x464033 => {
            c.push(m,c.get(3,32));
            c.push(m,0x698a40u32);
            c.push(m,0x698a68u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x464045);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x651b5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[2].wrapping_add(0x168); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[0].wrapping_add(0x164); wm(m,a,32,c.get(3,32)); }
            pc = 0x46405c; continue;
            }
            0x46405c => {
            c.set(6,32,c.r[5].wrapping_add(0x44));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x464066);
            { let args = []; c.r[0] = host.call(m, 0x42f990, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x46406d);
            { let args = []; c.r[0] = host.call(m, 0x6534f6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x168),8));
            { let a = c.r[0].wrapping_add(0x1f); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x164),8));
            { let a = c.r[0].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x180),8));
            { let a = c.r[0].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x184),8));
            { let a = c.r[0].wrapping_add(0x25); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x178),8));
            { let a = c.r[0].wrapping_add(0x22); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x174),8));
            { let a = c.r[0].wrapping_add(0x23); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x170),8));
            { let a = c.r[0].wrapping_add(0x21); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x16c),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x24); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x21),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x464124);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442d90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x23),8));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x46413b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442e90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x188),32));
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[1].wrapping_add(0x1dd); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x464164);
            { let args = []; c.r[0] = host.call(m, 0x429a40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x46417a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x464188); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x464193);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x4641a1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffbu32,32); }
            if c.cond(5) { pc = 0x4641aa; continue; }
            pc = 0x4641a6; continue;
            }
            0x4641a6 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(3,8)); }
            pc = 0x4641aa; continue;
            }
            0x4641aa => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x4641b5);
            { let args = []; c.r[0] = host.call(m, 0x427960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x4641c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4641d7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(0x2fc); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0x5u32); }
            c.push(m, 0x4641ee);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4641ff);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x13),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x463dab; continue; }
            pc = 0x464209; continue;
            }
            0x464209 => {
            { let a = rm(m,0x6cbb94u32,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x464230; continue; }
            pc = 0x464211; continue;
            }
            0x464211 => {
            c.push(m,c.get(3,32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x698ab8u32);
            c.push(m, 0x464225);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m, 0x464230);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x464230; continue;
            }
            0x464230 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x60u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x464245 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x46424c);
            { let args = []; c.r[0] = host.call(m, 0x42f990, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x464230; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_copy_alloc(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x464530;
    loop {
        match pc {
            0x464530 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65ef30u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(0,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.or(a,0x1fu32,32); c.set(7,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            { let a = c.get(7,32); c.sub(a,0xfffffffdu32,32); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(6,32)); }
            if c.cond(6) { pc = 0x464565; continue; }
            pc = 0x464563; continue;
            }
            0x464563 => {
            c.set(7,32,c.get(0,32));
            pc = 0x464565; continue;
            }
            0x464565 => {
            c.set(0,32,c.r[7].wrapping_add(0x2));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x464575; continue; }
            pc = 0x464573; continue;
            }
            0x464573 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x464575; continue;
            }
            0x464575 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x46457b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4645aa, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_copy_alloc_tail(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4645aa;
    loop {
        match pc {
            0x4645aa => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x4645c9; continue; }
            pc = 0x4645b1; continue;
            }
            0x4645b1 => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x4645b7; continue; }
            pc = 0x4645b5; continue;
            }
            0x4645b5 => {
            c.set(0,32,c.get(7,32));
            pc = 0x4645b7; continue;
            }
            0x4645b7 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4645c6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4645c9; continue;
            }
            0x4645c9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4645ef; continue; }
            pc = 0x4645d3; continue;
            }
            0x4645d3 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4645e6; continue; }
            pc = 0x4645db; continue;
            }
            0x4645db => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4645e6; continue; }
            pc = 0x4645e0; continue;
            }
            0x4645e0 => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x4645ef; continue;
            }
            0x4645e6 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4645ec);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4645ef; continue;
            }
            0x4645ef => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            c.set(0,32,c.r[2].wrapping_add(0x1));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            if c.cond(7) { pc = 0x46460c; continue; }
            pc = 0x46460a; continue;
            }
            0x46460a => {
            c.set(7,32,c.get(3,32));
            pc = 0x46460c; continue;
            }
            0x46460c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn event_window_show_event(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x469af0;
    loop {
        match pc {
            0x469af0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65f1e0u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xa4u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x380),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x469b35; continue; }
            pc = 0x469b26; continue;
            }
            0x469b26 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c0),32));
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x469b3a; continue; }
            pc = 0x469b35; continue;
            }
            0x469b35 => {
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,0x0u32); }
            pc = 0x469b3a; continue;
            }
            0x469b3a => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = rm(m,c.r[0].wrapping_add(0x26),8); c.sub(a,0x1u32,8); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.r[6].wrapping_add(0x380); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3ea),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc4),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0xc8),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xcc),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x2u32); }
            if c.cond(5) { pc = 0x469bc1; continue; }
            pc = 0x469b7d; continue;
            }
            0x469b7d => {
            c.set(0,32,c.r[4].wrapping_add(0xd0));
            c.set(1,32,c.r[6].wrapping_add(0x374));
            c.push(m,c.get(0,32));
            c.push(m, 0x469b90);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x469b9a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0xd4));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x378));
            c.push(m, 0x469bad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x3c0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3de); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[6].wrapping_add(0x3e0); wm(m,a,16,c.get(7,16)); }
            pc = 0x469bc1; continue;
            }
            0x469bc1 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,0x25u32,32); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,0x1au32,32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,0x19u32,32); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.and(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,0x10u32,32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,0xfu32,32); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.and(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x380),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x46a2a2; continue; }
            pc = 0x469c06; continue;
            }
            0x469c06 => {
            { let a = c.get(3,32); c.sub(a,0x24u32,32); }
            if c.cond(7) { pc = 0x46a1fa; continue; }
            pc = 0x469c0f; continue;
            }
            0x469c0f => {
            pc = m.r32(c.r[3].wrapping_mul(4).wrapping_add(0x46ab50)); continue;
            }
            0x469c16 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x469c27);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x469c43; continue; }
            pc = 0x469c2e; continue;
            }
            0x469c2e => {
            c.set(1,32,rm(m,0x6a3bd0u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x469c3e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469c43 => {
            c.set(2,32,rm(m,0x6a3c34u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x469c53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469c58 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x469c69);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x469c84; continue; }
            pc = 0x469c70; continue;
            }
            0x469c70 => {
            c.set(0,32,rm(m,0x6a3bd4u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x469c7f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469c84 => {
            c.set(1,32,rm(m,0x6a3c38u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x469c94);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469c99 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x469caa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            if c.cond(4) { pc = 0x469cc6; continue; }
            pc = 0x469cb5; continue;
            }
            0x469cb5 => {
            c.set(2,32,rm(m,0x6a3bd8u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x469cc1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469cc6 => {
            c.set(0,32,rm(m,0x6a3c3cu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x469cd1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469cd6 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x469ce7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x469d03; continue; }
            pc = 0x469cee; continue;
            }
            0x469cee => {
            c.set(1,32,rm(m,0x6a3be0u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x469cfe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469d03 => {
            c.set(2,32,rm(m,0x6a3c44u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x469d13);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469d18 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x469d29);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x469d44; continue; }
            pc = 0x469d30; continue;
            }
            0x469d30 => {
            c.set(0,32,rm(m,0x6a3bdcu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x469d3f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469d44 => {
            c.set(1,32,rm(m,0x6a3c40u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x469d54);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469d59 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x469d6a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            if c.cond(4) { pc = 0x469d86; continue; }
            pc = 0x469d75; continue;
            }
            0x469d75 => {
            c.set(2,32,rm(m,0x6a3be4u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x469d81);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469d86 => {
            c.set(0,32,rm(m,0x6a3c48u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x469d91);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469d96 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x469da7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x469df6; continue; }
            pc = 0x469dae; continue;
            }
            0x469dae => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(0,32));
            c.push(m, 0x469dd0);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6aab84u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x69ee48),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x469dee);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x46a1fa; continue;
            }
            0x469df6 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(0,32));
            c.push(m, 0x469e18);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6aab88u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x69ee48),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x469e36);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x46a1fa; continue;
            }
            0x469e3e => {
            c.set(0,32,rm(m,0x6a3becu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x469e4d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469e52 => {
            c.set(1,32,rm(m,0x6a3c08u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x469e62);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469e67 => {
            c.set(0,32,rm(m,0x6a3c04u32,32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(2,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x46a1f2; continue;
            }
            0x469e85 => {
            c.set(2,32,rm(m,0x6a3bf0u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x469e95);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469e9a => {
            c.set(0,32,rm(m,0x6a3bf4u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x469ea9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469eae => {
            c.set(1,32,rm(m,0x6a3bf8u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x469ebe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469ec3 => {
            c.set(2,32,rm(m,0x6a3bfcu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x469ed3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469ed8 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x469f28; continue; }
            pc = 0x469ee7; continue;
            }
            0x469ee7 => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x469eff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x469f0e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6a3c00u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x469f20);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x46a1fa; continue;
            }
            0x469f28 => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x469f33);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(7,32));
            c.set(5,32,c.get(0,32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x469f4f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x469f5e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x469f66);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x469f6e);
            { let args = []; c.r[0] = host.call(m, 0x4f6420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6aab8cu32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x469f80);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x46a1fa; continue;
            }
            0x469f88 => {
            c.set(0,32,rm(m,0x6a3c0cu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x469f97);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469f9c => {
            { let a = c.get(5,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x469fba; continue; }
            pc = 0x469fa5; continue;
            }
            0x469fa5 => {
            c.set(1,32,rm(m,0x6a3c10u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x469fb5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469fba => {
            c.set(2,32,rm(m,0x6a3c14u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x469fca);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x469fcf => {
            c.set(1,32,rm(m,0x6a3c18u32,32));
            pc = 0x46a1de; continue;
            }
            0x469fda => {
            c.set(1,32,rm(m,0x6a3c1cu32,32));
            pc = 0x46a1de; continue;
            }
            0x469fe5 => {
            c.set(1,32,rm(m,0x6a3c20u32,32));
            pc = 0x46a1de; continue;
            }
            0x469ff0 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x46a001);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6a3c24u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x46a1f2; continue;
            }
            0x46a012 => {
            c.set(0,32,rm(m,0x6a3c28u32,32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(2,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x46a1f2; continue;
            }
            0x46a030 => {
            c.set(2,32,c.r[4].wrapping_add(0xd0));
            c.set(1,32,c.r[6].wrapping_add(0x374));
            c.push(m,c.get(2,32));
            c.push(m, 0x46a043);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x46a04d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0xd4));
            c.set(1,32,c.r[6].wrapping_add(0x378));
            c.push(m,c.get(0,32));
            c.push(m, 0x46a060);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x46a065 => {
            c.set(2,32,rm(m,0x6a3c54u32,32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x46a1f2; continue;
            }
            0x46a084 => {
            c.set(1,32,rm(m,0x6a3c58u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x46a094);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x46a099 => {
            c.set(0,32,rm(m,0x6aab70u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(2,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x46a1f2; continue;
            }
            0x46a0ba => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,rm(m,0x6aab74u32,32));
            c.set(1,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x46a0ec);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x46a1fa; continue;
            }
            0x46a0f4 => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x46a10d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x46a11c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6aab78u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x46a12e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x46a1fa; continue;
            }
            0x46a136 => {
            c.set(1,32,rm(m,0x6aab7cu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x46a146);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x46a14b => {
            c.set(0,32,rm(m,0x6aab80u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(2,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x46a1f2; continue;
            }
            0x46a16c => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(2,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6aab68),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x46a183);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x46a185 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x46a1a5; continue; }
            pc = 0x46a194; continue;
            }
            0x46a194 => {
            c.set(0,32,rm(m,0x6aab94u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x46a1a3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a1fa; continue;
            }
            0x46a1a5 => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x46a1b0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x46a1b9);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x46a1c1);
            { let args = []; c.r[0] = host.call(m, 0x4f6420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6aab90u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x46a1d3);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x46a1fa; continue;
            }
            0x46a1d8 => {
            c.set(1,32,rm(m,0x6aab98u32,32));
            pc = 0x46a1de; continue;
            }
            0x46a1de => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(0,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x46a1f2; continue;
            }
            0x46a1f2 => {
            c.push(m, 0x46a1f7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x46a1fa; continue;
            }
            0x46a1fa => {
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x46a20b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x46a216);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42b4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xc8),32); c.push(m, 0x46a220); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x46a22e; continue; }
            pc = 0x46a224; continue;
            }
            0x46a224 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x46a22e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x46a22e; continue;
            }
            0x46a22e => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(6,32); let r = c.or(a,0xffffffffu32,32); c.set(6,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0xd),8); c.sub(a,0x3u32,8); }
            if c.cond(2) { pc = 0x46a271; continue; }
            pc = 0x46a23d; continue;
            }
            0x46a23d => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x46a246);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xc8u32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x46a271);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a271; continue;
            }
            0x46a271 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x1u32); }
            c.push(m, 0x46a282);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0xd0));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x0u32); }
            c.push(m, 0x46a296);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,32,c.get(6,32)); }
            pc = 0x46ab26; continue;
            }
            0x46a2a2 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x17),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[6].wrapping_add(0x380); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3ea),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x46a5ab; continue; }
            pc = 0x46a2bf; continue;
            }
            0x46a2bf => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3c4),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3cc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3c8),32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3d0),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3d4),32));
            c.set(1,32,c.r[0].wrapping_mul(4).wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(1,32));
            c.push(m, 0x46a304);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2]).wrapping_add(0x2));
            c.push(m,c.get(0,32));
            c.push(m, 0x46a318);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1]).wrapping_add(0x2));
            c.push(m,c.get(2,32));
            c.push(m, 0x46a32c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(0,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m, 0x46a344);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x3u32); }
            if c.cond(4) { pc = 0x46a37d; continue; }
            pc = 0x46a357; continue;
            }
            0x46a357 => {
            c.push(m,0x64f253u32);
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m,0x4103c0u32);
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x46a373);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            pc = 0x46a385; continue;
            }
            0x46a37d => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            pc = 0x46a385; continue;
            }
            0x46a385 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x2u32); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(2,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(2,32));
            c.push(m, 0x46a3a1);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x4u32); }
            if c.cond(4) { pc = 0x46a3d6; continue; }
            pc = 0x46a3b4; continue;
            }
            0x46a3b4 => {
            c.push(m,0x64f253u32);
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m,0x4103c0u32);
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x46a3d0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x44),32));
            pc = 0x46a3d8; continue;
            }
            0x46a3d6 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x46a3d8; continue;
            }
            0x46a3d8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3c4),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x2u32); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x46a3fb);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3c8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x46a415);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3cc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x46a42f);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3d0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x46a44a);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3d4),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x46a461);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(0,16,rm(m,c.r[4].wrapping_add(0xcc),16));
            { let a = c.r[3].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(5,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(2,32,c.r[4].wrapping_add(0xd0));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m, 0x46a4b0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(1,32,c.r[4].wrapping_add(0xd4));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m, 0x46a4c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[6].wrapping_add(0x3c4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x3c8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3cc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x3d0); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x3d4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x3d8); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x46a50a; continue; }
            pc = 0x46a501; continue;
            }
            0x46a501 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x46a507);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x46a50a; continue;
            }
            0x46a50a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x46a51b; continue; }
            pc = 0x46a512; continue;
            }
            0x46a512 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x46a518);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x46a51b; continue;
            }
            0x46a51b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x46a52c; continue; }
            pc = 0x46a523; continue;
            }
            0x46a523 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x46a529);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x46a52c; continue;
            }
            0x46a52c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x46a551; continue; }
            pc = 0x46a534; continue;
            }
            0x46a534 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(6,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x46a548);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(6,32));
            c.push(m, 0x46a54e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x46a551; continue;
            }
            0x46a551 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x46a576; continue; }
            pc = 0x46a559; continue;
            }
            0x46a559 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(6,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x46a56d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(6,32));
            c.push(m, 0x46a573);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x46a576; continue;
            }
            0x46a576 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x1u32); }
            c.push(m, 0x46a587);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0xd0));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x0u32); }
            c.push(m, 0x46a59b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,32,0xffffffffu32); }
            pc = 0x46ab26; continue;
            }
            0x46a5ab => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c0),32));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x6u32); }
            { let a = c.get(0,32); c.sub(a,0x25u32,32); }
            if c.cond(5) { pc = 0x46a8ba; continue; }
            pc = 0x46a5cf; continue;
            }
            0x46a5cf => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xc8),32); c.push(m, 0x46a5d9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x46a5e7; continue; }
            pc = 0x46a5dd; continue;
            }
            0x46a5dd => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x46a5e7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x46a5e7; continue;
            }
            0x46a5e7 => {
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(1,32));
            c.push(m,0x698d10u32);
            c.set(1,32,0x6cc710u32);
            c.push(m, 0x46a5fb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,0x6cc710u32);
            c.push(m,c.get(2,32));
            c.push(m,0x698d00u32);
            c.set(3,8,c.get(0,8));
            c.push(m, 0x46a611);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(6,32); let r = c.or(a,0xffffffffu32,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x46a7dc; continue; }
            pc = 0x46a61c; continue;
            }
            0x46a61c => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x46a625);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,0x10u32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[4].wrapping_add(0x74));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x8u32); }
            c.stos(m,32,true);
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x66a574u32,32); c.push(m, 0x46a663); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,0x44u32); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0xa4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0xa2); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x9c); wm(m,a,32,c.get(3,32)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xd),8));
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x46a6ce; continue; }
            pc = 0x46a6a7; continue;
            }
            0x46a6a7 => {
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xc8u32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x46a6ce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a6ce; continue;
            }
            0x46a6ce => {
            c.set(0,32,rm(m,0x6cc780u32,32));
            c.push(m,0x698d1cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x46a6de);
            { let args = []; c.r[0] = host.call(m, 0x63ad50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x46a6f0; continue; }
            pc = 0x46a6e9; continue;
            }
            0x46a6e9 => {
            c.push(m,0x698d28u32);
            pc = 0x46a6f5; continue;
            }
            0x46a6f0 => {
            c.push(m,0x698d30u32);
            pc = 0x46a6f5; continue;
            }
            0x46a6f5 => {
            c.push(m, 0x46a6fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,0x698d38u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x46a718);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x60));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,0x20u32);
            c.push(m,0x1u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a2b4u32,32); c.push(m, 0x46a739); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(44);
            c.set(2,32,rm(m,0x6cc40cu32,32));
            c.set(7,32,rm(m,0x66a53cu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            { let target = c.get(7,32); c.push(m, 0x46a749); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a2b8u32,32); c.push(m, 0x46a755); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cc40cu32,32));
            c.push(m,0x5u32);
            c.push(m,c.get(1,32));
            { let target = c.get(7,32); c.push(m, 0x46a760); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = rm(m,c.r[2].wrapping_add(0xd),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x46a792; continue; }
            pc = 0x46a76b; continue;
            }
            0x46a76b => {
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xc8u32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x46a792);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a792; continue;
            }
            0x46a792 => {
            { let target = rm(m,0x66a574u32,32); c.push(m, 0x46a798); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x46a7ac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42ba60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dccf0u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x7u32); }
            { let a = 0x6dccf0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x46a7c9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x6u32); }
            c.push(m, 0x46a7da);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x46a81e; continue;
            }
            0x46a7dc => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = rm(m,c.r[1].wrapping_add(0xd),8); c.sub(a,0x3u32,8); }
            if c.cond(2) { pc = 0x46a81c; continue; }
            pc = 0x46a7e8; continue;
            }
            0x46a7e8 => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x46a7f1);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[4].wrapping_add(0x24));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xc8u32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x46a81a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a81e; continue;
            }
            0x46a81c => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x46a81e; continue;
            }
            0x46a81e => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x46a82f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x1a),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x46a83c; continue; }
            pc = 0x46a834; continue;
            }
            0x46a834 => {
            c.set(0,32,rm(m,0x6a3be8u32,32));
            c.push(m,c.get(0,32));
            pc = 0x46a843; continue;
            }
            0x46a83c => {
            c.set(1,32,rm(m,0x6a3c4cu32,32));
            c.push(m,c.get(1,32));
            pc = 0x46a843; continue;
            }
            0x46a843 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x46a84c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x46a85d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x46a868);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42b4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x5u32); }
            c.push(m, 0x46a879);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x2u32); }
            c.push(m, 0x46a88a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x1u32); }
            c.push(m, 0x46a89b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0xd0));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x46a8ae);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,32,c.get(6,32)); }
            pc = 0x46ab26; continue;
            }
            0x46a8ba => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0xd),8); c.sub(a,0x3u32,8); }
            if c.cond(2) { pc = 0x46a913; continue; }
            pc = 0x46a8c8; continue;
            }
            0x46a8c8 => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x46a8d1);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xc8u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x46a8fc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xcau32); }
            c.push(m, 0x46a913);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x46a913; continue;
            }
            0x46a913 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,0x1au32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,0x19u32,32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.or(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,0x10u32,32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.or(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,0xfu32,32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.or(a,c.get(1,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x46aa25; continue; }
            pc = 0x46a93f; continue;
            }
            0x46a93f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[6].wrapping_add(0xbc));
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(5,32));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x46a965);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x1a4));
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x46a98b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x46a997);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x46a9a3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x64u32);
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xffffffd8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x46a9bc); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.sub(a,0x5u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x46a9da); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,0x64u32);
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xffffffd8));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x5));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x46aa06); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6a3404u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x46aa14);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6a3400u32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x46aa22);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            pc = 0x46aa25; continue;
            }
            0x46aa25 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x380),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x46aaac; continue; }
            pc = 0x46aa2f; continue;
            }
            0x46aa2f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3b8),32));
            c.push(m,0x7d0u32);
            c.push(m,0x0u32);
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(4)),32));
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a554u32,32); c.push(m, 0x46aa46); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[6].wrapping_add(0x388));
            { let a = c.r[6].wrapping_add(0x384); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a558u32,32); c.push(m, 0x46aa5a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x46aa6a; continue; }
            pc = 0x46aa61; continue;
            }
            0x46aa61 => {
            { let a = c.r[6].wrapping_add(0x380); wm(m,a,8,0x0u32); }
            pc = 0x46aaac; continue;
            }
            0x46aa6a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x384),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a55cu32,32); c.push(m, 0x46aa7b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x46aa8b; continue; }
            pc = 0x46aa82; continue;
            }
            0x46aa82 => {
            { let a = c.r[6].wrapping_add(0x380); wm(m,a,8,0x0u32); }
            pc = 0x46aaac; continue;
            }
            0x46aa8b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x390),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38c),32));
            c.push(m,0x1000050u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x46aaa3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb2a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x3b4); wm(m,a,32,c.get(0,32)); }
            pc = 0x46aaac; continue;
            }
            0x46aaac => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x3e8),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c0),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3ec));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x46aacd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x46aad8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x442460, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x5u32); }
            c.push(m, 0x46aae9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x2u32); }
            c.push(m, 0x46aafa);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x1u32); }
            c.push(m, 0x46ab0b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0xd0));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,8,0x0u32); }
            c.push(m, 0x46ab1f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,32,c.get(7,32)); }
            pc = 0x46ab26; continue;
            }
            0x46ab26 => {
            c.set(1,32,c.r[4].wrapping_add(0xd4));
            c.push(m, 0x46ab32);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xb4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xb0u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn draw_interior_window_ornaments(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x46b960;
    loop {
        match pc {
            0x46b960 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0xffffffffu32);
            c.push(m, 0x46b96e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,0x6da820u32);
            c.push(m,0x7925u32);
            c.push(m, 0x46b980);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x46b9ae; continue; }
            pc = 0x46b98b; continue;
            }
            0x46b98b => {
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x46b9ae; continue; }
            pc = 0x46b992; continue;
            }
            0x46b992 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(3,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x46b9b6; continue;
            }
            0x46b9ae => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x46b9b6; continue;
            }
            0x46b9b6 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x46b9bc);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x46b9d4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66e100u32,8));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x46b9e6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x46b9f4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[5]).wrapping_add(0xffffff74));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x46ba07);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x14),32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x46ba3a; continue; }
            pc = 0x46ba14; continue;
            }
            0x46ba14 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x46ba3a; continue; }
            pc = 0x46ba1b; continue;
            }
            0x46ba1b => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x46ba3a; continue;
            }
            0x46ba3a => {
            c.push(m,c.get(7,32));
            c.push(m, 0x46ba40);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x46ba54); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66e100u32,8));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x98));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x46ba6b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x46ba79); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[5]).wrapping_add(0xffffff74));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x46ba8c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,0x6da820u32);
            c.push(m,0x7926u32);
            c.push(m, 0x46ba9e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x46bacc; continue; }
            pc = 0x46baa9; continue;
            }
            0x46baa9 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x46bacc; continue; }
            pc = 0x46bab0; continue;
            }
            0x46bab0 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(7,32,c.get(1,32));
            c.set(3,32,rm(m,c.r[2],32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            pc = 0x46bacc; continue;
            }
            0x46bacc => {
            c.push(m,c.get(7,32));
            c.push(m, 0x46bad2);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x46bae6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66e100u32,8));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x8));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x46bafa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x46bb08); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[0].wrapping_add(c.r[5]).wrapping_add(0xffffff74));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x46bb1b);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x14),32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x46bb4a; continue; }
            pc = 0x46bb28; continue;
            }
            0x46bb28 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x46bb4a; continue; }
            pc = 0x46bb2f; continue;
            }
            0x46bb2f => {
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[1],32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x46bb4a; continue;
            }
            0x46bb4a => {
            c.push(m,c.get(7,32));
            c.push(m, 0x46bb50);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x46bb64); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66e100u32,8));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x8));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x46bb78); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x46bb86); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x84));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x46bb95);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn statistics_window_build_account_summary_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4a2f60;
    loop {
        match pc {
            0x4a2f60 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x214),32));
            c.set(6,32,c.r[5].wrapping_add(0x214));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4a2f86); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4a2f92); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.r[5].wrapping_add(0x3e4));
            c.set(3,32,0x4u32);
            pc = 0x4a2f9d; continue;
            }
            0x4a2f9d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3a0),32));
            c.set(7,32,c.r[6].wrapping_add(0x3a0));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4a2fb0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4a2fbc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4a2fc5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4a2fd1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x4a2f9d; continue; }
            pc = 0x4a2fda; continue;
            }
            0x4a2fda => {
            c.set(6,32,c.r[5].wrapping_add(0xc28));
            c.set(7,32,0x4u32);
            pc = 0x4a2fe5; continue;
            }
            0x4a2fe5 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4a2fee); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4a2ffa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x4a2fe5; continue; }
            pc = 0x4a3003; continue;
            }
            0x4a3003 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x2080),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4a3020; continue; }
            pc = 0x4a300d; continue;
            }
            0x4a300d => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4a3013);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0x2084); wm(m,a,32,0x0u32); }
            pc = 0x4a3020; continue;
            }
            0x4a3020 => {
            c.push(m,0x2a0u32);
            c.push(m, 0x4a302a);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0x210); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4a3b9f; continue; }
            pc = 0x4a303b; continue;
            }
            0x4a303b => {
            c.push(m,0x2a0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4a3048);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,0x6dfcacu32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4a3b9f; continue; }
            pc = 0x4a3059; continue;
            }
            0x4a3059 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            pc = 0x4a305e; continue;
            }
            0x4a305e => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3],16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4a3079; continue; }
            pc = 0x4a3067; continue;
            }
            0x4a3067 => {
            c.set(3,32,rm(m,c.r[3].wrapping_add(0x230c),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4a305e; continue; }
            pc = 0x4a3071; continue;
            }
            0x4a3071 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            0x4a3079 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xa4),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x210),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4a359b; continue; }
            pc = 0x4a308d; continue;
            }
            0x4a308d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x2),16));
            c.set(2,32,c.r[0].wrapping_mul(8));
            c.set(5,32,c.r[0].wrapping_add(0x3));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(7,32,c.r[0].wrapping_add(0x1));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0x1fu32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0x1fu32,32); c.set(7,32,r); }
            c.fld(c.ld(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x8),4));
            c.fstp64(m,c.r[4]);
            c.push(m,0x6bad18u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a30c0);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[5].wrapping_mul(8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.fld(c.ld(m,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x8),4));
            c.fstp64(m,c.r[4]);
            c.push(m,0x6bad20u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a30e4);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            c.set(0,32,c.get(5,32));
            if c.cond(4) { pc = 0x4a310a; continue; }
            pc = 0x4a30f2; continue;
            }
            0x4a30f2 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x30),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4a30f2; continue; }
            pc = 0x4a310a; continue;
            }
            0x4a310a => {
            c.push(m,c.get(1,32));
            c.push(m,0x6bad28u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3116);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            c.set(0,32,c.get(5,32));
            if c.cond(4) { pc = 0x4a313c; continue; }
            pc = 0x4a3124; continue;
            }
            0x4a3124 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x34),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4a3124; continue; }
            pc = 0x4a313c; continue;
            }
            0x4a313c => {
            c.push(m,c.get(1,32));
            c.push(m,0x6bad2cu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3148);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a317c; continue; }
            pc = 0x4a315c; continue;
            }
            0x4a315c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a3160; continue;
            }
            0x4a3160 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x28),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4a3160; continue; }
            pc = 0x4a3178; continue;
            }
            0x4a3178 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a317c; continue;
            }
            0x4a317c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bad30u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a318c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a31c0; continue; }
            pc = 0x4a31a0; continue;
            }
            0x4a31a0 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let v = c.ld(m,c.r[1].wrapping_add(c.r[3]).wrapping_add(0x1c),4); c.fop('+',v); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            if c.cond(5) { pc = 0x4a31a0; continue; }
            pc = 0x4a31c0; continue;
            }
            0x4a31c0 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x6bad34u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a31d5);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.push(m, 0x4a31e1);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a321c; continue; }
            pc = 0x4a31fc; continue;
            }
            0x4a31fc => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a3200; continue;
            }
            0x4a3200 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x44),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4a3200; continue; }
            pc = 0x4a3218; continue;
            }
            0x4a3218 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a321c; continue;
            }
            0x4a321c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bad3cu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a322c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a326e; continue; }
            pc = 0x4a324e; continue;
            }
            0x4a324e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a3252; continue;
            }
            0x4a3252 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x38),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4a3252; continue; }
            pc = 0x4a326a; continue;
            }
            0x4a326a => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a326e; continue;
            }
            0x4a326e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bad40u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a327e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a32c0; continue; }
            pc = 0x4a32a0; continue;
            }
            0x4a32a0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a32a4; continue;
            }
            0x4a32a4 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x3c),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4a32a4; continue; }
            pc = 0x4a32bc; continue;
            }
            0x4a32bc => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a32c0; continue;
            }
            0x4a32c0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bad44u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a32d0);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x6bad48u32);
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4a32ed);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a3327; continue; }
            pc = 0x4a3301; continue;
            }
            0x4a3301 => {
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[1].wrapping_add(0x10),32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4a3301; continue; }
            pc = 0x4a3327; continue;
            }
            0x4a3327 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bad4cu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3337);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a3373; continue; }
            pc = 0x4a3353; continue;
            }
            0x4a3353 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a3357; continue;
            }
            0x4a3357 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x2c),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4a3357; continue; }
            pc = 0x4a336f; continue;
            }
            0x4a336f => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a3373; continue;
            }
            0x4a3373 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bad50u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3383);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a33c5; continue; }
            pc = 0x4a33a5; continue;
            }
            0x4a33a5 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let v = c.ld(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x20),4); c.fop('+',v); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            if c.cond(5) { pc = 0x4a33a5; continue; }
            pc = 0x4a33c5; continue;
            }
            0x4a33c5 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x6bad54u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a33da);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.push(m, 0x4a33e6);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a3421; continue; }
            pc = 0x4a3401; continue;
            }
            0x4a3401 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a3405; continue;
            }
            0x4a3405 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x24),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4a3405; continue; }
            pc = 0x4a341d; continue;
            }
            0x4a341d => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a3421; continue;
            }
            0x4a3421 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bad5cu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3431);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a3473; continue; }
            pc = 0x4a3453; continue;
            }
            0x4a3453 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a3457; continue;
            }
            0x4a3457 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x48),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4a3457; continue; }
            pc = 0x4a346f; continue;
            }
            0x4a346f => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a3473; continue;
            }
            0x4a3473 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bad60u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3483);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(0,32,c.get(5,32));
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            if c.cond(4) { pc = 0x4a34c9; continue; }
            pc = 0x4a34a9; continue;
            }
            0x4a34a9 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[3]));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x4c),32));
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[1].wrapping_add(0x40),32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4a34a9; continue; }
            pc = 0x4a34c9; continue;
            }
            0x4a34c9 => {
            c.push(m,c.get(5,32));
            c.push(m,0x6bad64u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a34d5);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,0x6bad68u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a34ea);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x6bad6cu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a34ff);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4a0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(3,32,c.get(0,32));
            c.set(1,8,c.get(4,8));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,0x6de4a2u32,16));
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[3].wrapping_add(0xffffffe2));
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            c.set(5,32,c.get(1,32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x4a3560; continue; }
            pc = 0x4a3535; continue;
            }
            0x4a3535 => {
            c.set(1,32,c.get(7,32));
            pc = 0x4a3537; continue;
            }
            0x4a3537 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x4a354a; continue; }
            pc = 0x4a353b; continue;
            }
            0x4a353b => {
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[5].wrapping_mul(2).wrapping_add(0x672d94),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            pc = 0x4a3553; continue;
            }
            0x4a354a => {
            { let a = c.get(0,32); let r = c.add(a,0x1fu32,32); c.set(0,32,r); }
            c.set(5,32,0xbu32);
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            pc = 0x4a3553; continue;
            }
            0x4a3553 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(12) { pc = 0x4a3537; continue; }
            pc = 0x4a3558; continue;
            }
            0x4a3558 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x4a3560; continue;
            }
            0x4a3560 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x1u32,32); }
            if c.cond(13) { pc = 0x4a3591; continue; }
            pc = 0x4a356b; continue;
            }
            0x4a356b => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4a3580; continue; }
            pc = 0x4a356f; continue;
            }
            0x4a356f => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672d92),16));
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            pc = 0x4a3b2a; continue;
            }
            0x4a3580 => {
            { let a = c.get(3,32); let r = c.add(a,0x1fu32,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xcu32); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            pc = 0x4a3b2a; continue;
            }
            0x4a3591 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x4a3b2a; continue;
            }
            0x4a359b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,32,0x18u32);
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x5),8));
            c.set(0,32,c.r[1].wrapping_add(0x1));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            { let a = c.get(7,32); c.sub(a,0x1u32,32); }
            c.set(5,32,c.get(2,32));
            if c.cond(5) { pc = 0x4a35b7; continue; }
            pc = 0x4a35b2; continue;
            }
            0x4a35b2 => {
            c.set(0,32,c.r[5].wrapping_add(0x15));
            pc = 0x4a35c2; continue;
            }
            0x4a35b7 => {
            { let a = c.get(7,32); c.sub(a,0x2u32,32); }
            c.set(0,32,c.r[5].wrapping_add(0x12));
            if c.cond(4) { pc = 0x4a35c2; continue; }
            pc = 0x4a35bf; continue;
            }
            0x4a35bf => {
            c.set(0,32,c.r[5].wrapping_add(0xc));
            pc = 0x4a35c2; continue;
            }
            0x4a35c2 => {
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(7,32,0x18u32);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(7,32,c.get(2,32));
            c.set(2,32,c.r[1].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.fld(c.ld(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x1888),4));
            c.fstp64(m,c.r[4]);
            c.push(m,0x6bad70u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a35f0);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_mul(8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.fld(c.ld(m,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x1888),4));
            c.fstp64(m,c.r[4]);
            c.push(m,0x6bad78u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3617);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(0,32,c.get(7,32));
            if c.cond(4) { pc = 0x4a3644; continue; }
            pc = 0x4a3625; continue;
            }
            0x4a3625 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x18b0),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4a3640; continue; }
            pc = 0x4a363e; continue;
            }
            0x4a363e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a3640; continue;
            }
            0x4a3640 => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a3625; continue; }
            pc = 0x4a3644; continue;
            }
            0x4a3644 => {
            c.push(m,c.get(1,32));
            c.push(m,0x6bad80u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3650);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(0,32,c.get(7,32));
            if c.cond(4) { pc = 0x4a367d; continue; }
            pc = 0x4a365e; continue;
            }
            0x4a365e => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x18b4),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4a3679; continue; }
            pc = 0x4a3677; continue;
            }
            0x4a3677 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a3679; continue;
            }
            0x4a3679 => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a365e; continue; }
            pc = 0x4a367d; continue;
            }
            0x4a367d => {
            c.push(m,c.get(1,32));
            c.push(m,0x6bad84u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3689);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a36c4; continue; }
            pc = 0x4a369d; continue;
            }
            0x4a369d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a36a1; continue;
            }
            0x4a36a1 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x18a8),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4a36bc; continue; }
            pc = 0x4a36ba; continue;
            }
            0x4a36ba => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a36bc; continue;
            }
            0x4a36bc => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a36a1; continue; }
            pc = 0x4a36c0; continue;
            }
            0x4a36c0 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a36c4; continue;
            }
            0x4a36c4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bad88u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a36d4);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a370f; continue; }
            pc = 0x4a36e8; continue;
            }
            0x4a36e8 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let v = c.ld(m,c.r[1].wrapping_add(c.r[3]).wrapping_add(0x189c),4); c.fop('+',v); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            if c.cond(12) { pc = 0x4a370b; continue; }
            pc = 0x4a3709; continue;
            }
            0x4a3709 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a370b; continue;
            }
            0x4a370b => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a36e8; continue; }
            pc = 0x4a370f; continue;
            }
            0x4a370f => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x6bad8cu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3724);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.push(m, 0x4a3730);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a3772; continue; }
            pc = 0x4a374b; continue;
            }
            0x4a374b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a374f; continue;
            }
            0x4a374f => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x18c4),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4a376a; continue; }
            pc = 0x4a3768; continue;
            }
            0x4a3768 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a376a; continue;
            }
            0x4a376a => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a374f; continue; }
            pc = 0x4a376e; continue;
            }
            0x4a376e => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a3772; continue;
            }
            0x4a3772 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bad94u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3782);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a37cb; continue; }
            pc = 0x4a37a4; continue;
            }
            0x4a37a4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a37a8; continue;
            }
            0x4a37a8 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x18b8),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4a37c3; continue; }
            pc = 0x4a37c1; continue;
            }
            0x4a37c1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a37c3; continue;
            }
            0x4a37c3 => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a37a8; continue; }
            pc = 0x4a37c7; continue;
            }
            0x4a37c7 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a37cb; continue;
            }
            0x4a37cb => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bad98u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a37db);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a3824; continue; }
            pc = 0x4a37fd; continue;
            }
            0x4a37fd => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a3801; continue;
            }
            0x4a3801 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x18bc),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4a381c; continue; }
            pc = 0x4a381a; continue;
            }
            0x4a381a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a381c; continue;
            }
            0x4a381c => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a3801; continue; }
            pc = 0x4a3820; continue;
            }
            0x4a3820 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a3824; continue;
            }
            0x4a3824 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bad9cu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3834);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x6bada0u32);
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4a3851);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a3895; continue; }
            pc = 0x4a3865; continue;
            }
            0x4a3865 => {
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x1894),32));
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[1].wrapping_add(0x1890),32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x4a3891; continue; }
            pc = 0x4a388f; continue;
            }
            0x4a388f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a3891; continue;
            }
            0x4a3891 => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a3865; continue; }
            pc = 0x4a3895; continue;
            }
            0x4a3895 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bada4u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a38a5);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a38e8; continue; }
            pc = 0x4a38c1; continue;
            }
            0x4a38c1 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a38c5; continue;
            }
            0x4a38c5 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x18ac),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4a38e0; continue; }
            pc = 0x4a38de; continue;
            }
            0x4a38de => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a38e0; continue;
            }
            0x4a38e0 => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a38c5; continue; }
            pc = 0x4a38e4; continue;
            }
            0x4a38e4 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a38e8; continue;
            }
            0x4a38e8 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6bada8u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a38f8);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a3941; continue; }
            pc = 0x4a391a; continue;
            }
            0x4a391a => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let v = c.ld(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x18a0),4); c.fop('+',v); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            if c.cond(12) { pc = 0x4a393d; continue; }
            pc = 0x4a393b; continue;
            }
            0x4a393b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a393d; continue;
            }
            0x4a393d => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a391a; continue; }
            pc = 0x4a3941; continue;
            }
            0x4a3941 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x6badacu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3956);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.push(m, 0x4a3962);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a39a4; continue; }
            pc = 0x4a397d; continue;
            }
            0x4a397d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a3981; continue;
            }
            0x4a3981 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x18a4),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4a399c; continue; }
            pc = 0x4a399a; continue;
            }
            0x4a399a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a399c; continue;
            }
            0x4a399c => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a3981; continue; }
            pc = 0x4a39a0; continue;
            }
            0x4a39a0 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a39a4; continue;
            }
            0x4a39a4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6badb4u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a39b4);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4a39fd; continue; }
            pc = 0x4a39d6; continue;
            }
            0x4a39d6 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4a39da; continue;
            }
            0x4a39da => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x18c8),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4a39f5; continue; }
            pc = 0x4a39f3; continue;
            }
            0x4a39f3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a39f5; continue;
            }
            0x4a39f5 => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a39da; continue; }
            pc = 0x4a39f9; continue;
            }
            0x4a39f9 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a39fd; continue;
            }
            0x4a39fd => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6badb8u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3a0d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            if c.cond(4) { pc = 0x4a3a5d; continue; }
            pc = 0x4a3a33; continue;
            }
            0x4a3a33 => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[3]));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x18cc),32));
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[1].wrapping_add(0x18c0),32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x4a3a59; continue; }
            pc = 0x4a3a57; continue;
            }
            0x4a3a57 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4a3a59; continue;
            }
            0x4a3a59 => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4a3a33; continue; }
            pc = 0x4a3a5d; continue;
            }
            0x4a3a5d => {
            c.push(m,c.get(7,32));
            c.push(m,0x6badbcu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3a69);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,0x6badc0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3a7e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x6badc4u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3a93);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4a0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            { let a = c.get(4,8); c.and(a,c.get(4,8),8); }
            if c.cond(6) { pc = 0x4a3aba; continue; }
            pc = 0x4a3aaa; continue;
            }
            0x4a3aaa => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,8,c.get(4,8));
            c.set(3,16,rm(m,c.r[1].wrapping_mul(2).wrapping_add(0x672d92),16));
            pc = 0x4a3abf; continue;
            }
            0x4a3aba => {
            c.set(3,32,0x1fu32);
            pc = 0x4a3abf; continue;
            }
            0x4a3abf => {
            { let a = c.get(4,8); c.and(a,c.get(4,8),8); }
            if c.cond(4) { pc = 0x4a3ad6; continue; }
            pc = 0x4a3ac3; continue;
            }
            0x4a3ac3 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,0x6de4a2u32,16));
            c.set(2,8,c.get(4,8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            pc = 0x4a3ae8; continue;
            }
            0x4a3ad6 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xcu32); }
            c.set(7,16,rm(m,0x6de4a2u32,16));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            pc = 0x4a3ae8; continue;
            }
            0x4a3ae8 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xa4),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4a3b00; continue; }
            pc = 0x4a3af7; continue;
            }
            0x4a3af7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,c.r[1].wrapping_add(0xfffffffe));
            pc = 0x4a3b15; continue;
            }
            0x4a3b00 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x4a3b0e; continue; }
            pc = 0x4a3b05; continue;
            }
            0x4a3b05 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,c.r[2].wrapping_add(0xfffffffb));
            pc = 0x4a3b15; continue;
            }
            0x4a3b0e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,c.r[0].wrapping_add(0xfffffff5));
            pc = 0x4a3b15; continue;
            }
            0x4a3b15 => {
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            if c.cond(13) { pc = 0x4a3b26; continue; }
            pc = 0x4a3b1a; continue;
            }
            0x4a3b1a => {
            c.set(1,32,c.r[7].wrapping_add(0xffffffff));
            { let a = c.get(5,32); let r = c.add(a,0xcu32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a3b2a; continue;
            }
            0x4a3b26 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            pc = 0x4a3b2a; continue;
            }
            0x4a3b2a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0d00),32));
            c.set(2,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6c0d00),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6c2240u32,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(0,32));
            { let a = c.get(2,32); let r = c.add(a,0x20cu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4a3b61);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6c0d00),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6c2244u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3b7f);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(0,32,rm(m,0x6c2244u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d00),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4a3b9c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            pc = 0x4a3b9f; continue;
            }
            0x4a3b9f => {
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

pub fn statistics_window_build_town_population_list_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4a3bb0;
    loop {
        match pc {
            0x4a3bb0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0xb8u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6dfca0u32,32));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,c.r[3].wrapping_add(0x3e4));
            c.set(5,32,0x4u32);
            pc = 0x4a3bda; continue;
            }
            0x4a3bda => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3a0),32));
            c.set(7,32,c.r[6].wrapping_add(0x3a0));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4a3bed); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4a3bf9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4a3c02); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4a3c0e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x4a3bda; continue; }
            pc = 0x4a3c17; continue;
            }
            0x4a3c17 => {
            c.set(6,32,c.r[3].wrapping_add(0xc28));
            c.set(7,32,0x4u32);
            pc = 0x4a3c22; continue;
            }
            0x4a3c22 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4a3c2b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4a3c37); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x4a3c22; continue; }
            pc = 0x4a3c40; continue;
            }
            0x4a3c40 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x214),32));
            c.set(6,32,c.r[3].wrapping_add(0x214));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4a3c53); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4a3c5f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0xbe),16));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(8).wrapping_add(0x6cc834),32));
            c.set(1,32,c.r[3].wrapping_add(0x20c));
            c.push(m,c.get(2,32));
            c.push(m, 0x4a3c7b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4a3c89);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[3].wrapping_add(0x210); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x4a3e51; continue; }
            pc = 0x4a3ca0; continue;
            }
            0x4a3ca0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4a3cf0; continue; }
            pc = 0x4a3cad; continue;
            }
            0x4a3cad => {
            c.set(7,32,rm(m,c.r[1],32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x4a3d13; continue; }
            pc = 0x4a3cb5; continue;
            }
            0x4a3cb5 => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.add(a,0x1e8u32,32); c.set(1,32,r); }
            pc = 0x4a3cbe; continue;
            }
            0x4a3cbe => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xfffffff8),32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0xfffffffc),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x4a3cdf; continue; }
            pc = 0x4a3cd7; continue;
            }
            0x4a3cd7 => {
            { let a = c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x28); wm(m,a,32,0x0u32); }
            pc = 0x4a3cdf; continue;
            }
            0x4a3cdf => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x204u32,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x4a3cbe; continue; }
            pc = 0x4a3cea; continue;
            }
            0x4a3cea => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4a3d13; continue;
            }
            0x4a3cf0 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x4a3d13; continue; }
            pc = 0x4a3cf6; continue;
            }
            0x4a3cf6 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(2,32); let r = c.add(a,0x2d4u32,32); c.set(2,32,r); }
            pc = 0x4a3d02; continue;
            }
            0x4a3d02 => {
            c.set(6,32,rm(m,c.r[2],32));
            { let a = c.get(2,32); let r = c.add(a,0x9f8u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x28); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4a3d02; continue; }
            pc = 0x4a3d13; continue;
            }
            0x4a3d13 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(6) { pc = 0x4a3dd4; continue; }
            pc = 0x4a3d23; continue;
            }
            0x4a3d23 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4a3d36; continue; }
            pc = 0x4a3d2d; continue;
            }
            0x4a3d2d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x2c),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4a3d2d; continue; }
            pc = 0x4a3d36; continue;
            }
            0x4a3d36 => {
            c.set(7,32,c.r[6].wrapping_add(0x1));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4a3d89; continue; }
            pc = 0x4a3d3d; continue;
            }
            0x4a3d3d => {
            c.set(1,32,c.r[6].wrapping_mul(4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x4a3d48; continue;
            }
            0x4a3d48 => {
            c.set(3,32,c.r[7].wrapping_mul(4));
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[3]).wrapping_add(0x28),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4a3d80; continue; }
            pc = 0x4a3d57; continue;
            }
            0x4a3d57 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4a3d6e);
            { let args = []; c.r[0] = host.call(m, 0x639cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,0x6de4b0u32,32));
            if c.cond(13) { pc = 0x4a3d80; continue; }
            pc = 0x4a3d7a; continue;
            }
            0x4a3d7a => {
            c.set(6,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x4a3d80; continue;
            }
            0x4a3d80 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4a3d48; continue; }
            pc = 0x4a3d85; continue;
            }
            0x4a3d85 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4a3d89; continue;
            }
            0x4a3d89 => {
            c.fild32(m,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x28));
            { let a = c.r[5]; wm(m,a,32,c.get(6,32)); }
            c.set(6,32,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x28));
            c.set(2,32,rm(m,0x6c1e90u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            c.fstp64(m,c.r[4]);
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4a3db2);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x3cu32,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[6]; wm(m,a,32,0xffffffffu32); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(2) { pc = 0x4a3d23; continue; }
            pc = 0x4a3dd4; continue;
            }
            0x4a3dd4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x20a),16));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4a3dff); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[3].wrapping_add(0x262c));
            c.push(m, 0x4a3e11);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(0x29f0); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x4a3e51; continue; }
            pc = 0x4a3e25; continue;
            }
            0x4a3e25 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(5,32,rm(m,c.r[3].wrapping_add(0x29f0),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x2d4),32));
            { let a = c.get(1,32); let r = c.add(a,0x9f8u32,32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(0x29f0); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4a3e25; continue; }
            pc = 0x4a3e51; continue;
            }
            0x4a3e51 => {
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

pub fn statistics_window_prepare_town_detail_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4a3e60;
    loop {
        match pc {
            0x4a3e60 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661081u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x214),32));
            c.set(6,32,c.r[3].wrapping_add(0x214));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4a3e8e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4a3e9a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.r[3].wrapping_add(0x3e4));
            c.set(5,32,0x4u32);
            c.push(m,c.get(7,32));
            pc = 0x4a3ea6; continue;
            }
            0x4a3ea6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3a0),32));
            c.set(7,32,c.r[6].wrapping_add(0x3a0));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4a3eb9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4a3ec5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4a3ece); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4a3eda); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x4a3ea6; continue; }
            pc = 0x4a3ee3; continue;
            }
            0x4a3ee3 => {
            c.set(6,32,c.r[3].wrapping_add(0xc28));
            c.set(7,32,0x4u32);
            pc = 0x4a3eee; continue;
            }
            0x4a3eee => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4a3ef7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4a3f03); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x4a3eee; continue; }
            pc = 0x4a3f0c; continue;
            }
            0x4a3f0c => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x2080),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(4) { pc = 0x4a3f28; continue; }
            pc = 0x4a3f19; continue;
            }
            0x4a3f19 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4a3f1f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[3].wrapping_add(0x2084); wm(m,a,32,c.get(6,32)); }
            pc = 0x4a3f28; continue;
            }
            0x4a3f28 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x2440),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xbe),16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(8).wrapping_add(0x6cc834),32));
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4a3f5e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.r[3].wrapping_add(0x20c));
            c.push(m,c.get(0,32));
            c.push(m, 0x4a3f71);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4a3f82);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
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

pub fn class75_load_texture_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b3ea0;
    loop {
        match pc {
            0x4b3ea0 => {
            c.push(m,0xffffffffu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0x661ac2u32);
            c.push(m,c.get(0,32));
            c.set(0,32,0x1014u32);
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m, 0x4b3ebf);
            { let args = []; c.r[0] = host.call(m, 0x63b920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x102c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x102c); wm(m,a,8,0x1u32); }
            if c.cond(14) { pc = 0x4b3f26; continue; }
            pc = 0x4b3eea; continue;
            }
            0x4b3eea => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            pc = 0x4b3eed; continue;
            }
            0x4b3eed => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1038),32));
            c.set(3,32,rm(m,c.r[2],32));
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4b3f04; continue; }
            pc = 0x4b3efa; continue;
            }
            0x4b3efa => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4b3eed; continue; }
            pc = 0x4b3f02; continue;
            }
            0x4b3f02 => {
            pc = 0x4b3f2d; continue;
            }
            0x4b3f04 => {
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b3f2d; continue; }
            pc = 0x4b3f09; continue;
            }
            0x4b3f09 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b41a2; continue; }
            pc = 0x4b3f11; continue;
            }
            0x4b3f11 => {
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4b41a2; continue; }
            pc = 0x4b3f19; continue;
            }
            0x4b3f19 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[0],32); let r = c.inc(a,32); { let a = c.r[0]; wm(m,a,32,r); } }
            pc = 0x4b41a2; continue;
            }
            0x4b3f26 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1038),32));
            pc = 0x4b3f2d; continue;
            }
            0x4b3f2d => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x4b3f60; continue; }
            pc = 0x4b3f31; continue;
            }
            0x4b3f31 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x102c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4b3f42);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1034));
            { let a = c.r[4].wrapping_add(0x102c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4b3f59);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x4b41cc; continue;
            }
            0x4b3f60 => {
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b41a2; continue; }
            pc = 0x4b3f69; continue;
            }
            0x4b3f69 => {
            c.set(2,32,rm(m,0x6bb910u32,32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b3f7b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1040),32));
            c.set(2,32,rm(m,0x6bb91cu32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4b3f9e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1034),32));
            c.set(2,32,rm(m,0x6bb938u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,0x6bb928u32,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dab58u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4b3fd2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1034),32));
            c.set(0,32,rm(m,0x6bb948u32,32));
            c.set(1,32,rm(m,0x6bb940u32,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4b4000);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x4b41a2; continue; }
            pc = 0x4b4008; continue;
            }
            0x4b4008 => {
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x4b4011);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0xffu32);
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4b4023);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m,0x6bb94cu32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4b4034);
            { let args = []; c.r[0] = host.call(m, 0x4b2b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4b4040);
            { let args = []; c.r[0] = host.call(m, 0x639f83, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(3,32,0x1u32);
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4b408b; continue; }
            pc = 0x4b4053; continue;
            }
            0x4b4053 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4b408b; continue; }
            pc = 0x4b4057; continue;
            }
            0x4b4057 => {
            c.push(m,0x0u32);
            c.push(m,0x6bb950u32);
            c.push(m,0x0u32);
            c.push(m, 0x4b4065);
            { let args = []; c.r[0] = host.call(m, 0x4b2b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4b4081; continue; }
            pc = 0x4b406e; continue;
            }
            0x4b406e => {
            c.push(m,0xau32);
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4b4078);
            { let args = []; c.r[0] = host.call(m, 0x639f83, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(c.r[3].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x4b4082; continue;
            }
            0x4b4081 => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            pc = 0x4b4082; continue;
            }
            0x4b4082 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4b4053; continue; }
            pc = 0x4b408b; continue;
            }
            0x4b408b => {
            c.push(m,0x1u32);
            c.push(m,0x6bb954u32);
            c.push(m,0x6dab58u32);
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.push(m, 0x4b409e);
            { let args = []; c.r[0] = host.call(m, 0x4b2b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b40ab);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            if c.cond(14) { pc = 0x4b4125; continue; }
            pc = 0x4b40be; continue;
            }
            0x4b40be => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4b4125; continue; }
            pc = 0x4b40c2; continue;
            }
            0x4b40c2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(7,32));
            c.push(m, 0x4b40d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4b40df);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b3c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4b40f6; continue; }
            pc = 0x4b40e3; continue;
            }
            0x4b40e3 => {
            c.push(m,0x1u32);
            c.push(m,0x6bb958u32);
            c.push(m,0x0u32);
            c.push(m, 0x4b40f1);
            { let args = []; c.r[0] = host.call(m, 0x4b2b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(7,32,c.get(0,32));
            pc = 0x4b40f6; continue;
            }
            0x4b40f6 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4b410e; continue; }
            pc = 0x4b40fa; continue;
            }
            0x4b40fa => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4b410e; continue; }
            pc = 0x4b40fe; continue;
            }
            0x4b40fe => {
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b4109);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x4b4114; continue;
            }
            0x4b410e => {
            c.set(3,8,0x1u32);
            c.set(7,32,c.r[4].wrapping_add(0x24));
            pc = 0x4b4114; continue;
            }
            0x4b4114 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x4b40be; continue; }
            pc = 0x4b4125; continue;
            }
            0x4b4125 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(3,32,c.r[7].wrapping_mul(4));
            c.push(m,c.get(3,32));
            c.push(m, 0x4b4136);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x4b413e);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x4b4151);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4b4162);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b416b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4b4174);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1060),32));
            { let a = c.r[5].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[3].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,0x1u32); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4b419f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            pc = 0x4b41a2; continue;
            }
            0x4b41a2 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x102c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4b41b3);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1034));
            { let a = c.r[4].wrapping_add(0x102c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4b41ca);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,0x1u32);
            pc = 0x4b41cc; continue;
            }
            0x4b41cc => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1024),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1020u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn orig(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b58c0;
    loop {
        match pc {
            0x4b58c0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,c.get(4,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(6,32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4b58ee);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4b18f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xc8),32); c.push(m, 0x4b58f8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4b5a45; continue; }
            pc = 0x4b5900; continue;
            }
            0x4b5900 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4b5906);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x60),32); c.push(m, 0x4b5919); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc0),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x4b59d8; continue; }
            pc = 0x4b592d; continue;
            }
            0x4b592d => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            pc = 0x4b5933; continue;
            }
            0x4b5933 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x94),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4b59be; continue; }
            pc = 0x4b5941; continue;
            }
            0x4b5941 => {
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[7].wrapping_add(0xd4),32),32); }
            if c.cond(4) { pc = 0x4b59be; continue; }
            pc = 0x4b5949; continue;
            }
            0x4b5949 => {
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(6,32,c.get(0,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,c.get(4,32));
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            c.set(3,32,rm(m,c.r[1],32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            { let target = rm(m,c.r[3].wrapping_add(0x9c),32); c.push(m, 0x4b5983); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dcb94u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b59a5; continue; }
            pc = 0x4b598c; continue;
            }
            0x4b598c => {
            c.set(0,32,rm(m,0x6dcce4u32,32));
            c.set(1,32,rm(m,0x6dcce0u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4b59a2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4bb800, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x4b59a5; continue;
            }
            0x4b59a5 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x94),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[2]),32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xdc),32); c.push(m, 0x4b59ba); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x4b59be; continue;
            }
            0x4b59be => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc0),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x10u32,32); c.set(6,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            if c.cond(12) { pc = 0x4b5933; continue; }
            pc = 0x4b59d8; continue;
            }
            0x4b59d8 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xd4),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4b5a2a; continue; }
            pc = 0x4b59e3; continue;
            }
            0x4b59e3 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x94),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[1].wrapping_add(0x4));
            c.set(1,32,rm(m,c.r[1],32));
            c.set(6,32,c.get(2,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,c.get(4,32));
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            c.set(3,32,rm(m,c.r[1],32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            { let target = rm(m,c.r[3].wrapping_add(0x9c),32); c.push(m, 0x4b5a2a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4b5a2a; continue;
            }
            0x4b5a2a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4b5a3b; continue; }
            pc = 0x4b5a32; continue;
            }
            0x4b5a32 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4b5a38);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4b5a3b; continue;
            }
            0x4b5a3b => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xdc),32); c.push(m, 0x4b5a45); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4b5a45; continue;
            }
            0x4b5a45 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_save_to_file_failure_tail(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d5e9c;
    loop {
        match pc {
            0x4d5e9c => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffc4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x4d5ea8);
            { let args = []; c.r[0] = host.call(m, 0x653b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            c.push(m, 0x4d5eb4);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4d5ec3);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn custom_map_save_release_buffers_tail(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x544d79;
    loop {
        match pc {
            0x544d79 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x2u32); }
            c.push(m, 0x544d88);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffdc),32));
            c.push(m,c.get(3,32));
            c.push(m, 0x544d91);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x544da1; continue; }
            pc = 0x544d98; continue;
            }
            0x544d98 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x544d9e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x544da1; continue;
            }
            0x544da1 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffd8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x544dad);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            c.push(m, 0x544db9);
            { let args = []; c.r[0] = host.call(m, 0x653b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0x8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x544dc8);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn operations_save_game_cleanup(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x547650;
    loop {
        match pc {
            0x547650 => {
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x4u32); }
            c.push(m, 0x54765d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x547663);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x54767d; continue;
            }
            0x547668 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x547675; continue; }
            pc = 0x54766c; continue;
            }
            0x54766c => {
            c.push(m,c.get(7,32));
            c.push(m, 0x547672);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x547675; continue;
            }
            0x547675 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            c.push(m, 0x54767d);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x54767d; continue;
            }
            0x54767d => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffb8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3u32); }
            c.push(m, 0x547689);
            { let args = []; c.r[0] = host.call(m, 0x653b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            c.push(m, 0x547695);
            { let args = []; c.r[0] = host.call(m, 0x653b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x5476a1);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            c.push(m, 0x5476ad);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5476bc);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_effect_sprite_draw_mask_layer(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x579ec0;
    loop {
        match pc {
            0x579ec0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e2e6cu32,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2c),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x579eda; continue; }
            pc = 0x579ed6; continue;
            }
            0x579ed6 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x579ee3; continue;
            }
            0x579eda => {
            c.set(1,32,rm(m,0x6e2e70u32,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x579ee3; continue;
            }
            0x579ee3 => {
            c.set(2,32,rm(m,0x6e2e74u32,32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[2]),8));
            { let a = c.get(0,8); c.sub(a,0x20u32,8); }
            { let a = c.r[4].wrapping_add(0xb); wm(m,a,8,c.get(0,8)); }
            if c.cond(4) { pc = 0x579f89; continue; }
            pc = 0x579ef8; continue;
            }
            0x579ef8 => {
            { let a = c.get(0,8); c.sub(a,0x24u32,8); }
            if c.cond(4) { pc = 0x579f89; continue; }
            pc = 0x579f00; continue;
            }
            0x579f00 => {
            { let a = c.get(0,8); c.sub(a,0x21u32,8); }
            if c.cond(4) { pc = 0x579f89; continue; }
            pc = 0x579f08; continue;
            }
            0x579f08 => {
            { let a = c.get(0,8); c.sub(a,0x22u32,8); }
            if c.cond(4) { pc = 0x579f89; continue; }
            pc = 0x579f0c; continue;
            }
            0x579f0c => {
            { let a = c.get(0,8); c.sub(a,0x25u32,8); }
            if c.cond(4) { pc = 0x579f89; continue; }
            pc = 0x579f10; continue;
            }
            0x579f10 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x30),8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x579f50; continue; }
            pc = 0x579f17; continue;
            }
            0x579f17 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0x80808081u32);
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x18u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.or(a,0xffffffu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x579f4d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x579f50; continue;
            }
            0x579f50 => {
            c.set(1,32,c.r[7].wrapping_add(0x8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x579f5a);
            { let args = []; c.r[0] = host.call(m, 0x559520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x30),8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x57a12e; continue; }
            pc = 0x579f68; continue;
            }
            0x579f68 => {
            c.push(m,0x64ffffffu32);
            c.push(m, 0x579f72);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x579f81);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x559a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x579f89 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x30),8));
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0x80808081u32);
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x18u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x579fbb);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x14),8));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x80u32,32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x15),8));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(6,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xc650),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc654),32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2e),16));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x57a01b; continue; }
            pc = 0x57a004; continue;
            }
            0x57a004 => {
            c.set(1,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x8),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,r); } }
            pc = 0x57a01b; continue;
            }
            0x57a01b => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.sub(a,0x24u32,8); }
            if c.cond(5) { pc = 0x57a04d; continue; }
            pc = 0x57a023; continue;
            }
            0x57a023 => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x15),8));
            c.set(0,32,0x66666667u32);
            c.set(2,32,c.r[1].wrapping_add(c.r[1]).wrapping_add(0xffffffd0));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0xa),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x15),8));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x8),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,r); } }
            pc = 0x57a056; continue;
            }
            0x57a04d => {
            { let a = c.get(0,8); c.sub(a,0x81u32,8); }
            if c.cond(5) { pc = 0x57a056; continue; }
            pc = 0x57a051; continue;
            }
            0x57a051 => {
            { let a = rm(m,c.r[6].wrapping_add(0xa),16); let r = c.add(a,0xfff5u32,16); { let a = c.r[6].wrapping_add(0xa); wm(m,a,16,r); } }
            pc = 0x57a056; continue;
            }
            0x57a056 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x34),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x57a05f);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x57a07b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x57a099); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x8),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6],16));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x57a0ba);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2e),16));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x57a0dd; continue; }
            pc = 0x57a0c6; continue;
            }
            0x57a0c6 => {
            c.set(1,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.set(0,32,0x99999999u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x8),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,r); } }
            pc = 0x57a0dd; continue;
            }
            0x57a0dd => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.sub(a,0x24u32,8); }
            if c.cond(5) { pc = 0x57a116; continue; }
            pc = 0x57a0e5; continue;
            }
            0x57a0e5 => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x15),8));
            c.set(2,32,0x18u32);
            c.set(0,32,0x66666667u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0xa),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x15),8));
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x8),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,r); } }
            pc = 0x57a11f; continue;
            }
            0x57a116 => {
            { let a = c.get(0,8); c.sub(a,0x81u32,8); }
            if c.cond(5) { pc = 0x57a11f; continue; }
            pc = 0x57a11a; continue;
            }
            0x57a11a => {
            { let a = rm(m,c.r[6].wrapping_add(0xa),16); let r = c.add(a,0xbu32,16); { let a = c.r[6].wrapping_add(0xa); wm(m,a,16,r); } }
            pc = 0x57a11f; continue;
            }
            0x57a11f => {
            c.push(m,0x64ffffffu32);
            c.push(m, 0x57a129);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x57a12e; continue;
            }
            0x57a12e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x57a13a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x559a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_view_update_weather_particles(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x58aa30;
    loop {
        match pc {
            0x58aa30 => {
            { let a = c.get(4,32); let r = c.sub(a,0x64u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc7b8),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58ad1d; continue; }
            pc = 0x58aa46; continue;
            }
            0x58aa46 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc7bc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc7cc),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x58ad1d; continue; }
            pc = 0x58aa5e; continue;
            }
            0x58aa5e => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xc5f8),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc5fc),32));
            c.set(2,32,c.get(5,32));
            c.push(m,c.get(3,32));
            { let a = c.get(2,32); let r = c.or(a,c.get(7,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x58ac09; continue; }
            pc = 0x58aa75; continue;
            }
            0x58aa75 => {
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x58ac09; continue; }
            pc = 0x58aa7d; continue;
            }
            0x58aa7d => {
            c.set(0,32,0x66666667u32);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc7c0),32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x58aaaa; continue; }
            pc = 0x58aa99; continue;
            }
            0x58aa99 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,32,c.get(2,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[3]));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x58aaaa; continue; }
            pc = 0x58aaa6; continue;
            }
            0x58aaa6 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            pc = 0x58aaaa; continue;
            }
            0x58aaaa => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc650),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc654),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(7,32));
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(1,32)); }
            { let target = rm(m,0x66a4c8u32,32); c.push(m, 0x58aaf9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x58));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4e8u32,32); c.push(m, 0x58ab0e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a4d0u32,32); c.push(m, 0x58ab19); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58ab28; continue; }
            pc = 0x58ab1d; continue;
            }
            0x58ab1d => {
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            pc = 0x58ac02; continue;
            }
            0x58ab28 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.set(7,32,rm(m,0x66a4f8u32,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = c.get(7,32); c.push(m, 0x58ab6f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = c.get(7,32); c.push(m, 0x58abb0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(5,32,c.r[7].wrapping_add(c.r[0]));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m, 0x58abf4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x58ad30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x58ac02; continue;
            }
            0x58ac02 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58ac09);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x58ad30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x58ac09; continue;
            }
            0x58ac09 => {
            c.set(2,32,rm(m,0x6dccf8u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc7c8),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(2) { pc = 0x58ad1d; continue; }
            pc = 0x58ac1e; continue;
            }
            0x58ac1e => {
            c.push(m, 0x58ac23);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc7c4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(0,32,c.get(1,32));
            c.set(5,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            { let a = c.get(0,32); let r = c.and(a,0x7fu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[6].wrapping_add(0xc7c8); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x9c4),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc650),32));
            { let a = c.get(0,8); c.and(a,0x4u32,8); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc654),32));
            c.set(2,32,c.r[0].wrapping_add(0x1));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            if c.cond(4) { pc = 0x58ac94; continue; }
            pc = 0x58ac7b; continue;
            }
            0x58ac7b => {
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x58ac8b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            pc = 0x58acfa; continue;
            }
            0x58ac94 => {
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x58aca4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58acb2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x58ad30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,0x1u32);
            c.set(1,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xffffffff));
            c.set(2,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58accf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x58ad30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,0x1u32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(7,32)); }
            c.push(m,c.get(0,32));
            pc = 0x58acfa; continue;
            }
            0x58acfa => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58ad01);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x58ad30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc5fc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc5f8),32));
            { let a = c.get(1,32); let r = c.or(a,c.get(0,32),32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x58ad1d; continue; }
            pc = 0x58ad11; continue;
            }
            0x58ad11 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc7bc),32));
            { let a = c.r[6].wrapping_add(0xc7c0); wm(m,a,32,c.get(2,32)); }
            pc = 0x58ad1d; continue;
            }
            0x58ad1d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x64u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tavern_content_window_draw(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5cdc60;
    loop {
        match pc {
            0x5cdc60 => {
            { let a = c.get(4,32); let r = c.sub(a,0x38u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x5cdc9f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5ce4b6; continue; }
            pc = 0x5cdca7; continue;
            }
            0x5cdca7 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.push(m, 0x5cdcb1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb7c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5ce4b6; continue; }
            pc = 0x5cdcbc; continue;
            }
            0x5cdcbc => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5cdcc6);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cdcd1);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cdcdc);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bf4),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x11u32,32); }
            if c.cond(7) { pc = 0x5ce3cb; continue; }
            pc = 0x5cdcee; continue;
            }
            0x5cdcee => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x5ce4e0),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x5ce4c0)); continue;
            }
            0x5cdcfd => {
            c.push(m,0x7920u32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5cdd0c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cdd20);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cddab; continue; }
            pc = 0x5cdd30; continue;
            }
            0x5cdd30 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,0x66e100u32,8));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1f),8));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cdd6d);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,c.get(4,32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cdda9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, 0x4673f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(36);
            pc = 0x5cdde8; continue;
            }
            0x5cddab => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5cddb4);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x66e100u32,8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5cdde5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x5cdde8; continue;
            }
            0x5cdde8 => {
            c.set(1,32,rm(m,0x6cc7f4u32,32));
            c.push(m, 0x5cddf3);
            { let args = []; c.r[0] = host.call(m, 0x46b6a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x16fc));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cde00);
            { let args = []; c.r[0] = host.call(m, 0x4ad900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5ce3cb; continue; }
            pc = 0x5cde08; continue;
            }
            0x5cde08 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5cde30); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5ce3cb; continue;
            }
            0x5cde35 => {
            c.push(m,0x7921u32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5cde44);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cde58);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5ce383; continue; }
            pc = 0x5cde6c; continue;
            }
            0x5cde6c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,0x66e100u32,8));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1f),8));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cdea9);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,0x1fu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            pc = 0x5ce34b; continue;
            }
            0x5cdeb9 => {
            c.push(m,0x7922u32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5cdec8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5cdedc);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cdf81; continue; }
            pc = 0x5cdef0; continue;
            }
            0x5cdef0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1f),8));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(3,8,rm(m,0x66e100u32,8));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(5,32),32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5cdf2d);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,0x2bu32);
            { let a = c.get(5,32); let r = c.add(a,0x8u32,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(5,32)); }
            c.push(m,c.get(1,32));
            c.set(2,32,0x8u32);
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,c.get(4,32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(0,32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5cdf7c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, 0x4673f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(36);
            pc = 0x5ce3c0; continue;
            }
            0x5cdf81 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5cdf8a);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66e100u32,8));
            c.push(m,c.get(2,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            pc = 0x5ce3a4; continue;
            }
            0x5cdfa6 => {
            c.push(m,0x791fu32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5cdfb5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5cdfc9);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x24),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5ce06b; continue; }
            pc = 0x5cdfdc; continue;
            }
            0x5cdfdc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,0x66e100u32,8));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1f),8));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5ce019);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(5,32); let r = c.add(a,0xdu32,32); c.set(5,32,r); }
            c.set(0,32,0x32u32);
            c.set(1,32,c.r[2].wrapping_add(0x32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(5,32)); }
            c.push(m,c.get(1,32));
            c.set(2,32,0xdu32);
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,c.get(4,32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5ce069);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, 0x4673f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(36);
            pc = 0x5ce0a8; continue;
            }
            0x5ce06b => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5ce074);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x66e100u32,8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5ce0a5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x5ce0a8; continue;
            }
            0x5ce0a8 => {
            c.set(1,32,rm(m,0x6cc7f4u32,32));
            c.push(m, 0x5ce0b3);
            { let args = []; c.r[0] = host.call(m, 0x46b6a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x1624));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5ce0c0);
            { let args = []; c.r[0] = host.call(m, 0x4ad900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5ce3cb; continue; }
            pc = 0x5ce0c8; continue;
            }
            0x5ce0c8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5ce0f0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5ce3cb; continue;
            }
            0x5ce0f5 => {
            c.push(m,0x7923u32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5ce104);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ce118);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5ce383; continue; }
            pc = 0x5ce12c; continue;
            }
            0x5ce12c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,0x66e100u32,8));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,c.get(2,32));
            pc = 0x5ce32a; continue;
            }
            0x5ce158 => {
            c.push(m,0x797bu32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5ce167);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5ce17b);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5ce383; continue; }
            pc = 0x5ce18f; continue;
            }
            0x5ce18f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1f),8));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(3,8,rm(m,0x66e100u32,8));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(5,32),32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5ce1cc);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,0x67u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,c.get(4,32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(2,32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5ce20d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, 0x4673f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(36);
            pc = 0x5ce3c0; continue;
            }
            0x5ce212 => {
            c.push(m,0x797cu32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5ce221);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5ce235);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5ce383; continue; }
            pc = 0x5ce249; continue;
            }
            0x5ce249 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1f),8));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(3,8,rm(m,0x66e100u32,8));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(5,32),32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5ce286);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,0x67u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,c.get(4,32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(2,32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5ce2c7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, 0x4673f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(36);
            pc = 0x5ce3c0; continue;
            }
            0x5ce2cc => {
            c.push(m,0x7923u32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5ce2db);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5ce2ef);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5ce383; continue; }
            pc = 0x5ce303; continue;
            }
            0x5ce303 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,0x66e100u32,8));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            { let a = c.get(3,32); let r = c.add(a,c.get(5,32),32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            pc = 0x5ce32a; continue;
            }
            0x5ce32a => {
            c.set(0,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1f),8));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ce340);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,0x67u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            pc = 0x5ce34b; continue;
            }
            0x5ce34b => {
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,c.get(4,32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(2,32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5ce381);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, 0x4673f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(36);
            pc = 0x5ce3c0; continue;
            }
            0x5ce383 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ce38c);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,8,rm(m,0x66e100u32,8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x5ce3a4; continue;
            }
            0x5ce3a4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5ce3bd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x5ce3c0; continue;
            }
            0x5ce3c0 => {
            c.set(1,32,rm(m,0x6cc7f4u32,32));
            c.push(m, 0x5ce3cb);
            { let args = []; c.r[0] = host.call(m, 0x46b6a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce3cb; continue;
            }
            0x5ce3cb => {
            { let a = rm(m,c.r[6].wrapping_add(0x1bf4),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(14) { pc = 0x5ce3e0; continue; }
            pc = 0x5ce3d4; continue;
            }
            0x5ce3d4 => {
            c.set(1,32,rm(m,0x6cc7f4u32,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5ce3e0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46b960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5ce3e0; continue;
            }
            0x5ce3e0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bf4),32));
            { let a = c.get(0,32); c.sub(a,0x11u32,32); }
            if c.cond(7) { pc = 0x5ce49f; continue; }
            pc = 0x5ce3ef; continue;
            }
            0x5ce3ef => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x5ce4f4)); continue;
            }
            0x5ce3f6 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce3fd);
            { let args = []; c.r[0] = host.call(m, 0x5cf830, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce402 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce409);
            { let args = []; c.r[0] = host.call(m, 0x5d0100, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce40e => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce415);
            { let args = []; c.r[0] = host.call(m, 0x5d0330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce41a => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce421);
            { let args = []; c.r[0] = host.call(m, 0x5d09a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce423 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce42a);
            { let args = []; c.r[0] = host.call(m, 0x5d0f00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce42c => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce433);
            { let args = []; c.r[0] = host.call(m, 0x5d10c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce435 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce43c);
            { let args = []; c.r[0] = host.call(m, 0x5d12a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce43e => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce445);
            { let args = []; c.r[0] = host.call(m, 0x5d14a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce447 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce44e);
            { let args = []; c.r[0] = host.call(m, 0x5d1e70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce450 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce457);
            { let args = []; c.r[0] = host.call(m, 0x5d21a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce459 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce460);
            { let args = []; c.r[0] = host.call(m, 0x5d22e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce462 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce469);
            { let args = []; c.r[0] = host.call(m, 0x5d2bc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce46b => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce472);
            { let args = []; c.r[0] = host.call(m, 0x5d2dc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce474 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce47b);
            { let args = []; c.r[0] = host.call(m, 0x5d36a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce47d => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce484);
            { let args = []; c.r[0] = host.call(m, 0x5d3990, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce486 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce48d);
            { let args = []; c.r[0] = host.call(m, 0x5d3eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce48f => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce496);
            { let args = []; c.r[0] = host.call(m, 0x5d7fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce498 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ce49f);
            { let args = []; c.r[0] = host.call(m, 0x5d8090, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ce49f; continue;
            }
            0x5ce49f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ce4a9);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5ce4b3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x5ce4b6; continue;
            }
            0x5ce4b6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tavern_content_window_draw_weapons_dealer_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5d14a0;
    loop {
        match pc {
            0x5d14a0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x666d2cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x2cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x69d8bcu32,32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(4,32)); }
            { let a = c.get(7,32); let r = c.add(a,0x46u32,32); c.set(7,32,r); }
            c.set(3,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffe2));
            c.set(1,32,c.get(4,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5d14e7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x1608));
            c.push(m, 0x5d14f2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x420c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,0x6dcdd0u32);
            c.push(m, 0x5d14fc);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1502);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            c.push(m, 0x5d1509);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x69d92cu32,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x0u32); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5d1528);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x69d930u32,32));
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5d1539);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x69d934u32,32));
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5d1549);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1be4),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1ca6),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[1],16); c.sub(a,c.get(2,16),16); }
            c.set(1,32,c.get(0,32));
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.or(a,c.get(1,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x5d1dd0; continue; }
            pc = 0x5d1570; continue;
            }
            0x5d1570 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.push(m,0x3u32);
            if c.cond(4) { pc = 0x5d15d3; continue; }
            pc = 0x5d1576; continue;
            }
            0x5d1576 => {
            c.push(m, 0x5d157b);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            c.set(7,32,c.r[6].wrapping_add(0xa8c));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(7,32));
            c.set(3,32,c.r[6].wrapping_add(0x9a4));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5d1595); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5d159e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7],32));
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5d15ae); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x5d15bc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x699874u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5d15ca);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x38),32));
            pc = 0x5d1a22; continue;
            }
            0x5d15d3 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x1c30),32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(5,32),32); c.set(3,32,r); }
            c.push(m, 0x5d15ec);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            c.set(1,32,c.r[6].wrapping_add(0xa8c));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x9a4));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5d1604); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5d160d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0xa8c));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5d1621); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x5d1633); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[3].wrapping_add(0xe),8));
            { let a = c.get(5,32); let r = c.sub(a,0x14u32,32); c.set(5,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0cc4),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x5d164e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c66c4u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x5d165c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[3].wrapping_add(0x160));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d166c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5d167c);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            c.set(1,32,c.r[6].wrapping_add(0xa8c));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x14u32,32); c.set(5,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5d1691); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5d169c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[6].wrapping_add(0xa8c));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5d16b5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c1c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5d16d0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d16de);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,0x6da820u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c14),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1700);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d170e);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c10),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5d1730);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d173e);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5d175f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5d176a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c1c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.push(m, 0x5d1789);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d179b);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d17a8);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x5d17af);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5d17c4);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x5d17cb);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x5d17d5);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x10),32));
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            c.push(m,0x6c66c8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x5d17f9);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1816);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m, 0x5d1820);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c14),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5d1840);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1852);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d185f);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x5d1866);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5d187b);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x5d1885);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x18),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[3].wrapping_add(0x14),32); c.idiv32(b); }
            c.set(2,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(0,32));
            c.push(m,0x6c66ccu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5d18a5);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d18c2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m, 0x5d18cc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c10),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.push(m, 0x5d18ec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d18fe);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d190b);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x5d1912);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5d1927);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x5d1931);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x40),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6c66d0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x5d1947);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x42),16));
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x5d1a05; continue; }
            pc = 0x5d195b; continue;
            }
            0x5d195b => {
            c.push(m,0x1u32);
            c.push(m, 0x5d1962);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcd28u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1974);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c20),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da820u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5d1991);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d19a3);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5d19bb);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x5d19c2);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5d19dd);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x5d19e7);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.push(m,c.get(0,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5d19f9);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x5d1a00);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            pc = 0x5d1a25; continue;
            }
            0x5d1a05 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1a22);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5d1a22; continue;
            }
            0x5d1a22 => {
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5d1a25; continue;
            }
            0x5d1a25 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x264),32));
            c.set(1,32,c.r[6].wrapping_add(0x264));
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x5d1a3e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.push(m,0x2u32);
            { let a = c.get(5,32); let r = c.sub(a,0x14u32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x5d1a4e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcd28u32);
            c.push(m, 0x5d1a5b);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1a61);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x69d938u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1b8c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x0u32);
            c.push(m, 0x5d1a78);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1a80);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x5d1a87);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6998e4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.r[6].wrapping_add(0x1b8c));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5d1aa1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5d1ab3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.push(m,0x0u32);
            c.set(1,32,c.r[6].wrapping_add(0x1b8c));
            { let a = c.get(5,32); let r = c.add(a,c.get(7,32),32); c.set(5,32,r); }
            c.push(m, 0x5d1ac4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1ad2);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x699894u32,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x264),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(5,32,c.r[6].wrapping_add(0x264));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5d1af2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5d1b03); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x5d1b0f);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6998a0u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x604),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(5,32,c.r[6].wrapping_add(0x604));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5d1b2a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5d1b39); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x5d1b41);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1ca6),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(7,32,c.r[6].wrapping_add(0x1b8c));
            if c.cond(4) { pc = 0x5d1ba6; continue; }
            pc = 0x5d1b54; continue;
            }
            0x5d1b54 => {
            c.set(1,32,rm(m,0x699874u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5d1b69);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5d1b7a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,0x3u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5d1b8d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,0x3u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5d1ba0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            pc = 0x5d1bf5; continue;
            }
            0x5d1ba6 => {
            c.set(0,32,rm(m,0x6998acu32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5d1bba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5d1bcb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,0x3u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5d1bde);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,0x3u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5d1bf1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            pc = 0x5d1bf5; continue;
            }
            0x5d1bf5 => {
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x5d1bfd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcdd0u32);
            c.push(m, 0x5d1c0a);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1c10);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[6].wrapping_add(0x604));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0xffffe454u32);
            { let a = c.get(3,32); let r = c.add(a,0xa4u32,32); c.set(3,32,r); }
            c.set(5,32,c.r[6].wrapping_add(0x1c00));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x6c0c00u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            pc = 0x5d1c44; continue;
            }
            0x5d1c44 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffc60),32));
            c.set(1,32,c.r[0].wrapping_add(0xfffffc60));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x5d1c57); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.push(m,0x2u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x5d1c64);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5d1c78);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x1c),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1c82);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x44),32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m,0x6c66d4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1c95);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x5d1c9c);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5d1cae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.set(3,32,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            c.push(m, 0x5d1cbe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m, 0x5d1ccf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.add(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5d1cd9);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1ca6),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5d1d21; continue; }
            pc = 0x5d1ce6; continue;
            }
            0x5d1ce6 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x1bfc),16));
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5d1d01);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[5]),32));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            pc = 0x5d1d3c; continue;
            }
            0x5d1d21 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,0x66666667u32);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[2],32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            pc = 0x5d1d3c; continue;
            }
            0x5d1d3c => {
            c.set(1,32,c.get(2,32));
            { let a = c.r[5]; wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6c66d8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5d1d51);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5d1d67);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,0x3u32);
            c.set(3,32,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            c.push(m, 0x5d1d77);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3u32);
            c.set(1,32,c.get(7,32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m, 0x5d1d82);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5d1d8d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x5d1d95);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0xe8u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x6c0c10u32,32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            if c.cond(12) { pc = 0x5d1c44; continue; }
            pc = 0x5d1dce; continue;
            }
            0x5d1dce => {
            pc = 0x5d1e43; continue;
            }
            0x5d1dd0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x9a4u32,32); c.set(6,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x5d1dec); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.set(6,32,c.get(0,32));
            c.push(m, 0x5d1df5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x6c66dcu32);
            c.push(m, 0x5d1e06);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x69d8dcu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x5d1e16);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c66deu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x5d1e24);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5d1e30);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x69da58u32,32));
            { let a = c.get(6,32); let r = c.add(a,0x14u32,32); c.set(6,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5d1e40);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x5d1e43; continue;
            }
            0x5d1e43 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5d1e54);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn trading_office_content_window_draw_goods_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5da020;
    loop {
        match pc {
            0x5da020 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m,0x66725eu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x2cu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,0x3u32);
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x5da05e);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x5da068);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.push(m,0xff000000u32);
            { let a = c.get(3,32); let r = c.add(a,0x64u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x32u32,32); c.set(6,32,r); }
            c.push(m, 0x5da07e);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x2u32);
            c.push(m, 0x5da085);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcd28u32);
            c.push(m, 0x5da092);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5da098);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x699880u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5da0a6);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6a0808u32,32));
            c.set(0,32,c.r[3].wrapping_add(0x50));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5da0b7);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6a0b40u32,32));
            c.set(2,32,c.r[3].wrapping_add(0xa0));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5da0cb);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6a0ba0u32,32));
            c.set(1,32,c.r[3].wrapping_add(0xf0));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5da0de);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcdd0u32);
            c.push(m, 0x5da0eb);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5da0f1);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(5,32,c.r[7].wrapping_add(0x6ec));
            pc = 0x5da104; continue;
            }
            0x5da100 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x5da104; continue;
            }
            0x5da104 => {
            c.fild32(m,c.r[5].wrapping_add(0xffffff60));
            { let a = c.get(6,32); let r = c.add(a,0x14u32,32); c.set(6,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x20));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x20),4));
            { let v = c.ld(m,c.r[5],4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(10) { pc = 0x5da12c; continue; }
            pc = 0x5da11f; continue;
            }
            0x5da11f => {
            c.set(2,32,0xff960000u32);
            c.set(1,8,0x1u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x5da13a; continue;
            }
            0x5da12c => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xff000000u32); }
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x5da13a; continue;
            }
            0x5da13a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0xed78),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x5da154; continue; }
            pc = 0x5da148; continue;
            }
            0x5da148 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xff960000u32); }
            c.set(0,8,0x1u32);
            pc = 0x5da15e; continue;
            }
            0x5da154 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xff000000u32); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x5da15e; continue;
            }
            0x5da15e => {
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x5da171; continue; }
            pc = 0x5da166; continue;
            }
            0x5da166 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5da171; continue; }
            pc = 0x5da16a; continue;
            }
            0x5da16a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(1,32));
            pc = 0x5da172; continue;
            }
            0x5da171 => {
            c.push(m,c.get(2,32));
            pc = 0x5da172; continue;
            }
            0x5da172 => {
            c.push(m, 0x5da177);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x698538),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5da195);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x698538),8));
            c.set(7,32,rm(m,0x6cc380u32,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5da1b6; continue; }
            pc = 0x5da1b0; continue;
            }
            0x5da1b0 => {
            c.set(7,32,rm(m,0x6cc384u32,32));
            pc = 0x5da1b6; continue;
            }
            0x5da1b6 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5da1c4);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5da1cd);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5da1d7);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff60),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5da222; continue; }
            pc = 0x5da1e4; continue;
            }
            0x5da1e4 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x6c68bcu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x5da1f4);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(7,32,c.r[3].wrapping_add(0x3c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5da203);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x5da20a);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5da21f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            pc = 0x5da222; continue;
            }
            0x5da222 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5da22c);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x698538),8));
            c.fld(c.ld(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x69c),4));
            { let v = c.ld(m,0x66bc6cu32,4); c.fcomp(v); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x69c));
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x44u32,8); }
            if c.cond(11) { pc = 0x5da261; continue; }
            pc = 0x5da25a; continue;
            }
            0x5da25a => {
            c.set(0,32,0x1u32);
            pc = 0x5da263; continue;
            }
            0x5da261 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5da263; continue;
            }
            0x5da263 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x5da2b9; continue; }
            pc = 0x5da271; continue;
            }
            0x5da271 => {
            c.fld(c.ld(m,c.r[1],4));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.fstp64(m,c.r[4]);
            c.push(m,0x6c68c0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5da288);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(7,32,c.r[3].wrapping_add(0x8c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5da29a);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x5da2a1);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5da2b6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            pc = 0x5da2b9; continue;
            }
            0x5da2b9 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5da2c3);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fld(c.ld(m,c.r[5],4));
            { let v = c.ld(m,0x66bc6cu32,4); c.fcomp(v); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x44u32,8); }
            if c.cond(11) { pc = 0x5da31f; continue; }
            pc = 0x5da2d6; continue;
            }
            0x5da2d6 => {
            c.fld(c.ld(m,c.r[5],4));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.fstp64(m,c.r[4]);
            c.push(m,0x6c68c8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5da2ee);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(7,32,c.r[3].wrapping_add(0xdc));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5da300);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x5da307);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5da31c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            pc = 0x5da31f; continue;
            }
            0x5da31f => {
            c.push(m,0xff000000u32);
            c.push(m, 0x5da329);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x5da100; continue; }
            pc = 0x5da341; continue;
            }
            0x5da341 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5da352);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
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

pub fn trading_office_window_draw_autotrade_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5dac80;
    loop {
        match pc {
            0x5dac80 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x667294u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x3u32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x0u32); }
            c.push(m, 0x5dacb6);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x5dacc0);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,0x55u32,32); c.set(0,32,r); }
            c.push(m,0xff000000u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(6,32); let r = c.add(a,0x32u32,32); c.set(6,32,r); }
            c.push(m, 0x5dacda);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x2u32);
            c.push(m, 0x5dace1);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x9840),32));
            c.set(3,32,c.r[7].wrapping_add(0x9840));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xc8),32); c.push(m, 0x5dacf8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5dafac; continue; }
            pc = 0x5dad00; continue;
            }
            0x5dad00 => {
            c.set(1,32,0x6dcd28u32);
            c.push(m, 0x5dad0a);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5dad10);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x699880u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5dad21);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x5dad28);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6a0808u32,32));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5dad3a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5dad45); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.push(m, 0x5dad4d);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x69b330u32,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xba00),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(3,32,c.r[7].wrapping_add(0xba00));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5dad68); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5dad73); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.push(m, 0x5dad7b);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x2u32);
            c.push(m, 0x5dad82);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x5dad89);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,0x6da820u32);
            c.push(m,0x7d0bu32);
            c.push(m, 0x5dad9b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5dadab);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5dadb4);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(5,32,c.r[7].wrapping_add(0x861c));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5dadd0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5dade7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5dadf7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcdd0u32);
            c.push(m, 0x5dae04);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5dae0a);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x5dae14);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x4fbc));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(5,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x5dae2b; continue;
            }
            0x5dae2b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x14u32,32); c.set(6,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x698538),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5dae51);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x1220u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xc8),32); c.push(m, 0x5dae66); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5dae82; continue; }
            pc = 0x5dae6a; continue;
            }
            0x5dae6a => {
            c.set(0,32,rm(m,0x69b31cu32,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x3d9c),32));
            c.set(1,32,c.r[7].wrapping_add(0x3d9c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5dae80); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5daea9; continue;
            }
            0x5dae82 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x5dae8e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5daeb5; continue; }
            pc = 0x5dae92; continue;
            }
            0x5dae92 => {
            c.set(2,32,rm(m,0x69b320u32,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x3d9c),32));
            c.set(1,32,c.r[7].wrapping_add(0x3d9c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5daea9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5daea9; continue;
            }
            0x5daea9 => {
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5daeb2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5daeb5; continue;
            }
            0x5daeb5 => {
            c.push(m,0xffffffffu32);
            c.push(m, 0x5daebc);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(3,32,rm(m,0x6cc380u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5daed9; continue; }
            pc = 0x5daed3; continue;
            }
            0x5daed3 => {
            c.set(3,32,rm(m,0x6cc384u32,32));
            pc = 0x5daed9; continue;
            }
            0x5daed9 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5daee7);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5daef0);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x195c),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[7].wrapping_add(0x195c));
            c.push(m,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5daf0d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.sub(a,0x1bu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5daf1a);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x5daf28); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5daf77; continue; }
            pc = 0x5daf2c; continue;
            }
            0x5daf2c => {
            c.set(1,32,rm(m,0x6cc37cu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5daf40);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6cc37cu32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5daf4e);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[7].wrapping_add(0x2b7c));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5daf67); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.sub(a,0x11u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5daf74);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x5daf77; continue;
            }
            0x5daf77 => {
            c.push(m,0xff000000u32);
            c.push(m, 0x5daf81);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xe8u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x190u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            if c.cond(12) { pc = 0x5dae2b; continue; }
            pc = 0x5dafaa; continue;
            }
            0x5dafaa => {
            pc = 0x5dafe9; continue;
            }
            0x5dafac => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,0xff000000u32);
            { let a = c.get(0,32); let r = c.sub(a,0x64u32,32); c.set(0,32,r); }
            c.push(m,0x14u32);
            { let a = c.get(1,32); let r = c.add(a,0x1b8u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6a0c98u32,32));
            { let a = c.get(2,32); let r = c.add(a,0xffffffceu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5dafde);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[7].wrapping_add(0xeca0));
            c.push(m, 0x5dafe9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x420a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            pc = 0x5dafe9; continue;
            }
            0x5dafe9 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5daffa);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn barn_content_window_update_wares_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5eaae0;
    loop {
        match pc {
            0x5eaae0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6679a8u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2da4),8));
            c.set(2,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x0u32); }
            c.push(m, 0x5eab29);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x5eab91; continue; }
            pc = 0x5eab31; continue;
            }
            0x5eab31 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2e0c),8));
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5eab4e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5eab91; continue; }
            pc = 0x5eab52; continue;
            }
            0x5eab52 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e1d),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5eab91; continue; }
            pc = 0x5eab5c; continue;
            }
            0x5eab5c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x51c),32));
            c.set(1,32,c.r[6].wrapping_add(0x51c));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x5eab6b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x18),16),16); c.set(0,16,r); }
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            { let a = c.r[1].wrapping_add(0xfc); wm(m,a,8,0x0u32); }
            { let a = c.r[1].wrapping_add(0xec); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x2e1d); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x2e28); wm(m,a,8,0x1u32); }
            pc = 0x5eab91; continue;
            }
            0x5eab91 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2e1d),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(1,32,c.r[6].wrapping_add(0x17c));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5eabad); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x264),32));
            c.set(1,32,c.r[6].wrapping_add(0x264));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5eabc1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x34c),32));
            c.set(1,32,c.r[6].wrapping_add(0x34c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5eabd5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x434),32));
            c.set(1,32,c.r[6].wrapping_add(0x434));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5eabe9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x295c));
            c.set(5,32,0x4u32);
            pc = 0x5eabf4; continue;
            }
            0x5eabf4 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e1d),8));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5eac05); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x5eabf4; continue; }
            pc = 0x5eac0e; continue;
            }
            0x5eac0e => {
            c.set(7,32,c.r[6].wrapping_add(0x173c));
            c.set(3,32,0x14u32);
            pc = 0x5eac19; continue;
            }
            0x5eac19 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e1d),8));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xffffede0),32));
            c.set(1,32,c.r[7].wrapping_add(0xffffede0));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5eac32); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e1d),8));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5eac43); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5eac19; continue; }
            pc = 0x5eac4c; continue;
            }
            0x5eac4c => {
            c.set(7,32,c.r[6].wrapping_add(0x295c));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            pc = 0x5eac58; continue;
            }
            0x5eac58 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x5eac62); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5eac98; continue; }
            pc = 0x5eac66; continue;
            }
            0x5eac66 => {
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[6].wrapping_add(0x2d80),8),8); }
            if c.cond(4) { pc = 0x5eac98; continue; }
            pc = 0x5eac6e; continue;
            }
            0x5eac6e => {
            c.set(7,32,c.r[6].wrapping_add(0x295c));
            c.set(5,32,0x4u32);
            pc = 0x5eac79; continue;
            }
            0x5eac79 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x124),32); c.push(m, 0x5eac85); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x5eac79; continue; }
            pc = 0x5eac8e; continue;
            }
            0x5eac8e => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[6].wrapping_add(0x2d80); wm(m,a,8,c.get(3,8)); }
            pc = 0x5eac98; continue;
            }
            0x5eac98 => {
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,8); c.sub(a,0x4u32,8); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            if c.cond(2) { pc = 0x5eac58; continue; }
            pc = 0x5eaca9; continue;
            }
            0x5eaca9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2d80),8));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x295c),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x295c));
            { let target = rm(m,c.r[0].wrapping_add(0x124),32); c.push(m, 0x5eacd3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x2da5));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2d80),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[6].wrapping_add(0x173c));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x672c14u32); }
            c.set(3,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x67a18c),32));
            c.set(7,32,c.r[6].wrapping_add(0x2dbc));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x14u32); }
            pc = 0x5ead0c; continue;
            }
            0x5ead0c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[2].wrapping_add(0xffffede0));
            c.push(m, 0x5ead1b);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5ead93; continue; }
            pc = 0x5ead1f; continue;
            }
            0x5ead1f => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x5ead27; continue; }
            pc = 0x5ead25; continue;
            }
            0x5ead25 => {
            c.set(3,32,c.get(0,32));
            pc = 0x5ead27; continue;
            }
            0x5ead27 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = rm(m,c.r[0],8); c.sub(a,0x0u32,8); }
            c.set(1,32,0x64u32);
            if c.cond(4) { pc = 0x5ead52; continue; }
            pc = 0x5ead35; continue;
            }
            0x5ead35 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2e2c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x5ead48; continue; }
            pc = 0x5ead41; continue;
            }
            0x5ead41 => {
            c.set(3,32,0x64u32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            pc = 0x5ead48; continue;
            }
            0x5ead48 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x2e2c); wm(m,a,32,c.get(0,32)); }
            pc = 0x5ead7c; continue;
            }
            0x5ead52 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x2e2c),32));
            c.set(2,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x5ead6f; continue; }
            pc = 0x5ead63; continue;
            }
            0x5ead63 => {
            c.set(0,32,0xcccccccdu32);
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(3,32,c.get(2,32));
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shr(a,n,32); c.set(3,32,r); }
            pc = 0x5ead6f; continue;
            }
            0x5ead6f => {
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(1,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.r[6].wrapping_add(0x2e2c); wm(m,a,32,c.get(1,32)); }
            pc = 0x5ead7c; continue;
            }
            0x5ead7c => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(2,32)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); let r = c.add(a,c.get(3,8),8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x2e28); wm(m,a,8,0x1u32); }
            pc = 0x5ead93; continue;
            }
            0x5ead93 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m, 0x5ead9c);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            if c.cond(4) { pc = 0x5eadeb; continue; }
            pc = 0x5eada4; continue;
            }
            0x5eada4 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x5eadae; continue; }
            pc = 0x5eadac; continue;
            }
            0x5eadac => {
            c.set(3,32,c.get(1,32));
            pc = 0x5eadae; continue;
            }
            0x5eadae => {
            c.set(5,32,rm(m,c.r[7],32));
            { let a = c.get(5,32); let r = c.add(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(5,32)); }
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.get(2,8); let r = c.sub(a,c.get(3,8),8); c.set(2,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = rm(m,c.r[2],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x5eadcb; continue; }
            pc = 0x5eadc3; continue;
            }
            0x5eadc3 => {
            { let a = rm(m,c.r[6].wrapping_add(0x2e2c),32); let r = c.sub(a,c.get(3,32),32); { let a = c.r[6].wrapping_add(0x2e2c); wm(m,a,32,r); } }
            pc = 0x5eade4; continue;
            }
            0x5eadcb => {
            c.set(1,32,c.get(3,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2e2c),32));
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x2e2c); wm(m,a,32,c.get(2,32)); }
            pc = 0x5eade4; continue;
            }
            0x5eade4 => {
            { let a = c.r[6].wrapping_add(0x2e28); wm(m,a,8,0x1u32); }
            pc = 0x5eadeb; continue;
            }
            0x5eadeb => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0xe8u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5ead0c; continue; }
            pc = 0x5eae19; continue;
            }
            0x5eae19 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2da4),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x5eae36);
            { let args = []; c.r[0] = host.call(m, 0x525630, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[6].wrapping_add(0x2e0c); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x2e16); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x2e18); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x672c14u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x673a18u32); }
            c.set(7,32,c.r[6].wrapping_add(0x2da5));
            c.set(5,32,c.r[6].wrapping_add(0x173c));
            c.set(3,32,c.r[6].wrapping_add(0x2dbc));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x14u32); }
            pc = 0x5eae82; continue;
            }
            0x5eae82 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,8,rm(m,c.r[2],8));
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xfffff8f8u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x7d0u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x5eaec5; continue; }
            pc = 0x5eaea4; continue;
            }
            0x5eaea4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[0],32));
            { let b = c.get(1,32); c.div32(b); }
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7],8));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5eaec1; continue; }
            pc = 0x5eaeba; continue;
            }
            0x5eaeba => {
            { let a = c.r[6].wrapping_add(0x2e28); wm(m,a,8,0x1u32); }
            pc = 0x5eaec1; continue;
            }
            0x5eaec1 => {
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            pc = 0x5eaecb; continue;
            }
            0x5eaec5 => {
            { let a = c.r[3]; wm(m,a,32,0x0u32); }
            pc = 0x5eaecb; continue;
            }
            0x5eaecb => {
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7],8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x5eaee1; continue; }
            pc = 0x5eaed5; continue;
            }
            0x5eaed5 => {
            c.set(2,8,c.get(0,8));
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            { let a = c.r[3]; wm(m,a,32,0x0u32); }
            pc = 0x5eaee5; continue;
            }
            0x5eaee1 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            pc = 0x5eaee5; continue;
            }
            0x5eaee5 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffede0),32));
            c.set(1,32,c.r[5].wrapping_add(0xffffede0));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(2,8,u32::from(c.cond(5)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5eaefc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[7],8));
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,8,u32::from(c.cond(5)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5eaf0c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7],8));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e16),8));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,8); let r = c.add(a,c.get(2,8),8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x2e16); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.fild64(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x24),4));
            { let v = c.ld(m,c.r[1],4); c.fop('*',v); }
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            c.push(m, 0x5eaf5f);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2e18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,0x4u32);
            { let a = c.get(5,32); let r = c.add(a,0xe8u32,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x2e18); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5eae82; continue; }
            pc = 0x5eafa7; continue;
            }
            0x5eafa7 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e16),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(7,32,c.r[6].wrapping_add(0x17c));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5eafc4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5eafcb);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(4) { pc = 0x5eafe2; continue; }
            pc = 0x5eafd2; continue;
            }
            0x5eafd2 => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5eafdb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5ea280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5eafe2);
            { let args = []; c.r[0] = host.call(m, 0x5eb010, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5eafe2; continue;
            }
            0x5eafe2 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5eaff3);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn barn_content_window_update_dispatch_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5eb010;
    loop {
        match pc {
            0x5eb010 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6679bau32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2da4),8));
            c.set(2,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            c.push(m, 0x5eb059);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e1d),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5eb071; continue; }
            pc = 0x5eb063; continue;
            }
            0x5eb063 => {
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5eb06c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5ea280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5eb383; continue;
            }
            0x5eb071 => {
            c.set(7,32,c.r[6].wrapping_add(0x173c));
            c.set(3,32,0x14u32);
            pc = 0x5eb07c; continue;
            }
            0x5eb07c => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xffffede0),32));
            c.set(1,32,c.r[7].wrapping_add(0xffffede0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5eb090); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5eb09c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5eb07c; continue; }
            pc = 0x5eb0a5; continue;
            }
            0x5eb0a5 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e1c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5eb104; continue; }
            pc = 0x5eb0af; continue;
            }
            0x5eb0af => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(1,32,c.r[6].wrapping_add(0x17c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5eb0c3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x264),32));
            c.set(1,32,c.r[6].wrapping_add(0x264));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5eb0d7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x34c),32));
            c.set(1,32,c.r[6].wrapping_add(0x34c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5eb0eb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x434),32));
            c.set(1,32,c.r[6].wrapping_add(0x434));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5eb0ff); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5eb383; continue;
            }
            0x5eb104 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(3,32,c.r[6].wrapping_add(0x17c));
            c.push(m,c.get(5,32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5eb11b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5eb12c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2e10),32));
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(1,8,u32::from(c.cond(13)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5eb141); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x264),32));
            c.set(1,32,c.r[6].wrapping_add(0x264));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5eb155); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x34c),32));
            c.set(7,32,c.r[6].wrapping_add(0x34c));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5eb16b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x434),32));
            c.set(5,32,c.r[6].wrapping_add(0x434));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5eb181); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2e18),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e16),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x5eb1a5; continue; }
            pc = 0x5eb19c; continue;
            }
            0x5eb19c => {
            { let a = c.r[6].wrapping_add(0x2e15); wm(m,a,8,0x0u32); }
            pc = 0x5eb1c2; continue;
            }
            0x5eb1a5 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(7) { pc = 0x5eb1bb; continue; }
            pc = 0x5eb1b2; continue;
            }
            0x5eb1b2 => {
            { let a = c.r[6].wrapping_add(0x2e15); wm(m,a,8,0x1u32); }
            pc = 0x5eb1c2; continue;
            }
            0x5eb1bb => {
            { let a = c.r[6].wrapping_add(0x2e15); wm(m,a,8,0x2u32); }
            pc = 0x5eb1c2; continue;
            }
            0x5eb1c2 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e14),8));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5eb1d5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e14),8));
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            c.set(1,32,c.get(5,32));
            c.set(0,8,u32::from(c.cond(2)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5eb1e9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5eb1f0);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5eb20d; continue; }
            pc = 0x5eb1f4; continue;
            }
            0x5eb1f4 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5eb206; continue; }
            pc = 0x5eb1fe; continue;
            }
            0x5eb1fe => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x2e14); wm(m,a,8,c.get(0,8)); }
            pc = 0x5eb206; continue;
            }
            0x5eb206 => {
            { let a = c.r[6].wrapping_add(0x2e28); wm(m,a,8,0x1u32); }
            pc = 0x5eb20d; continue;
            }
            0x5eb20d => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5eb214);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let v = c.pop(m); c.set(5,32,v); }
            if c.cond(4) { pc = 0x5eb232; continue; }
            pc = 0x5eb219; continue;
            }
            0x5eb219 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e14),8));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(3) { pc = 0x5eb22b; continue; }
            pc = 0x5eb223; continue;
            }
            0x5eb223 => {
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x2e14); wm(m,a,8,c.get(0,8)); }
            pc = 0x5eb22b; continue;
            }
            0x5eb22b => {
            { let a = c.r[6].wrapping_add(0x2e28); wm(m,a,8,0x1u32); }
            pc = 0x5eb232; continue;
            }
            0x5eb232 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2e14),8));
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x5eb299; continue; }
            pc = 0x5eb23f; continue;
            }
            0x5eb23f => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x5eb272; continue; }
            pc = 0x5eb242; continue;
            }
            0x5eb242 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x5eb2c3; continue; }
            pc = 0x5eb245; continue;
            }
            0x5eb245 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x66666667u32);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2e16),8));
            { let a = c.get(1,32); let r = c.add(a,0x9u32,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x2e10); wm(m,a,32,c.get(2,32)); }
            pc = 0x5eb2c3; continue;
            }
            0x5eb272 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x66666667u32);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2e16),8));
            { let a = c.get(1,32); let r = c.add(a,0x9u32,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            pc = 0x5eb2bd; continue;
            }
            0x5eb299 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x66666667u32);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2e16),8));
            { let a = c.get(1,32); let r = c.add(a,0x9u32,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x5eb2bd; continue;
            }
            0x5eb2bd => {
            { let a = c.r[6].wrapping_add(0x2e10); wm(m,a,32,c.get(1,32)); }
            pc = 0x5eb2c3; continue;
            }
            0x5eb2c3 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5eb2ca);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5eb321; continue; }
            pc = 0x5eb2ce; continue;
            }
            0x5eb2ce => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5eb2d5);
            { let args = []; c.r[0] = host.call(m, 0x5eb3c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x2e28); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x2e1c); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6e5508u32,32));
            c.push(m, 0x5eb2ee);
            { let args = []; c.r[0] = host.call(m, 0x5aa600, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2e0c),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2da4),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5eb306);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x533490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[6].wrapping_add(0x2e17); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6cc7e0u32,32));
            { let a = c.r[0].wrapping_add(0xfc); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0xec); wm(m,a,16,0x1c0u32); }
            pc = 0x5eb321; continue;
            }
            0x5eb321 => {
            c.set(1,32,c.r[6].wrapping_add(0x264));
            c.push(m, 0x5eb32c);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5eb347; continue; }
            pc = 0x5eb330; continue;
            }
            0x5eb330 => {
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5eb339);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5ea280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x2e28); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x2e14); wm(m,a,8,0x0u32); }
            pc = 0x5eb347; continue;
            }
            0x5eb347 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5eb358);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2e24),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5eb383; continue; }
            pc = 0x5eb364; continue;
            }
            0x5eb364 => {
            { let a = c.r[6].wrapping_add(0x2e28); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5eb37b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[6].wrapping_add(0x2e24); wm(m,a,32,c.get(1,32)); }
            pc = 0x5eb383; continue;
            }
            0x5eb383 => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5eb394);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tutorial_window_place_and_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x637e20;
    loop {
        match pc {
            0x637e20 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x637e34); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x290),32));
            c.set(7,32,c.r[6].wrapping_add(0xb0));
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(3,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x637e55); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(4));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x284),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x94));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x637e81);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x462520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x637e88); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x280),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x637ea2; continue; }
            pc = 0x637e9d; continue;
            }
            0x637e9d => {
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            pc = 0x637ea5; continue;
            }
            0x637ea2 => {
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x637ea5; continue;
            }
            0x637ea5 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,0x6dcce4u32,32));
            c.set(0,32,0x92492493u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(5,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(3,32,rm(m,c.r[7],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[3].wrapping_add(0x48),32); c.push(m, 0x637ee0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x637ef8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7],32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x637f09); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x637f28); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x198),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(7,32,c.r[6].wrapping_add(0x198));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[3].wrapping_add(0x44),32); c.push(m, 0x637f4e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x637f5f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x637f7a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x13c),32); c.push(m, 0x637f84); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x637f8f);
            { let args = []; c.r[0] = host.call(m, 0x4438a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x637fc4; continue; }
            pc = 0x637f97; continue;
            }
            0x637f97 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[6].wrapping_add(0x2a0); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,0x6dccf8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0]).wrapping_add(0x1));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x298),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x29c); wm(m,a,32,c.get(0,32)); }
            pc = 0x637fc4; continue;
            }
            0x637fc4 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x637fcf);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x637fe4; continue; }
            pc = 0x637fd7; continue;
            }
            0x637fd7 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.push(m, 0x637fe4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x593910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x637fe4; continue;
            }
            0x637fe4 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x637fef);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x637ffe; continue; }
            pc = 0x637ff3; continue;
            }
            0x637ff3 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x637ffe);
            { let args = []; c.r[0] = host.call(m, 0x453ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x637ffe; continue;
            }
            0x637ffe => {
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "debug_page_merchant_ships_and_town_population" => debug_page_merchant_ships_and_town_population(m, host, regs),
        "debug_page_ship_cargo_list" => debug_page_ship_cargo_list(m, host, regs),
        "debug_page_town_facilities_and_population" => debug_page_town_facilities_and_population(m, host, regs),
        "debug_page_world_economy_summary" => debug_page_world_economy_summary(m, host, regs),
        "debug_page_town_trade_potential_matrix" => debug_page_town_trade_potential_matrix(m, host, regs),
        "render_window_title" => render_window_title(m, host, regs),
        "main_menu_init" => main_menu_init(m, host, regs),
        "main_menu_create_screens_tail" => main_menu_create_screens_tail(m, host, regs),
        "campaign_select_screen_process_input" => campaign_select_screen_process_input(m, host, regs),
        "load_save_screen_process_input" => load_save_screen_process_input(m, host, regs),
        "load_save_screen_load_game_from_cstring" => load_save_screen_load_game_from_cstring(m, host, regs),
        "load_save_screen_delete_savegame" => load_save_screen_delete_savegame(m, host, regs),
        "load_save_screen_remove_list_entry" => load_save_screen_remove_list_entry(m, host, regs),
        "multiplayer_screen_update_connection_mode" => multiplayer_screen_update_connection_mode(m, host, regs),
        "multiplayer_screen_start_game" => multiplayer_screen_start_game(m, host, regs),
        "multiplayer_screen_start_game_tail" => multiplayer_screen_start_game_tail(m, host, regs),
        "multiplayer_screen_update_hotseat_setup_tail_enter_town" => multiplayer_screen_update_hotseat_setup_tail_enter_town(m, host, regs),
        "multiplayer_screen_draw_wait_overlay" => multiplayer_screen_draw_wait_overlay(m, host, regs),
        "multiplayer_screen_reset_to_connection_mode" => multiplayer_screen_reset_to_connection_mode(m, host, regs),
        "game_main_window_run" => game_main_window_run(m, host, regs),
        "std_string_copy_alloc" => std_string_copy_alloc(m, host, regs),
        "std_string_copy_alloc_tail" => std_string_copy_alloc_tail(m, host, regs),
        "event_window_show_event" => event_window_show_event(m, host, regs),
        "draw_interior_window_ornaments" => draw_interior_window_ornaments(m, host, regs),
        "statistics_window_build_account_summary_page" => statistics_window_build_account_summary_page(m, host, regs),
        "statistics_window_build_town_population_list_page" => statistics_window_build_town_population_list_page(m, host, regs),
        "statistics_window_prepare_town_detail_page" => statistics_window_prepare_town_detail_page(m, host, regs),
        "class75_load_texture_list" => class75_load_texture_list(m, host, regs),
        "orig" => orig(m, host, regs),
        "route_save_to_file_failure_tail" => route_save_to_file_failure_tail(m, host, regs),
        "custom_map_save_release_buffers_tail" => custom_map_save_release_buffers_tail(m, host, regs),
        "operations_save_game_cleanup" => operations_save_game_cleanup(m, host, regs),
        "town_effect_sprite_draw_mask_layer" => town_effect_sprite_draw_mask_layer(m, host, regs),
        "town_view_update_weather_particles" => town_view_update_weather_particles(m, host, regs),
        "tavern_content_window_draw" => tavern_content_window_draw(m, host, regs),
        "tavern_content_window_draw_weapons_dealer_page" => tavern_content_window_draw_weapons_dealer_page(m, host, regs),
        "trading_office_content_window_draw_goods_page" => trading_office_content_window_draw_goods_page(m, host, regs),
        "trading_office_window_draw_autotrade_page" => trading_office_window_draw_autotrade_page(m, host, regs),
        "barn_content_window_update_wares_page" => barn_content_window_update_wares_page(m, host, regs),
        "barn_content_window_update_dispatch_page" => barn_content_window_update_dispatch_page(m, host, regs),
        "tutorial_window_place_and_layout" => tutorial_window_place_and_layout(m, host, regs),
        _ => return None,
    })
}
