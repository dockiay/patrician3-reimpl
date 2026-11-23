//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn tavern_side_menu_refresh_available_pages(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a6e60;
    loop {
        match pc {
            0x5a6e60 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x6652f4u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(7,32,c.r[6].wrapping_add(0xb0c));
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x5a6e8e);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5a6ea1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb04),32));
            { let a = c.r[6].wrapping_add(0xb08); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5a6ec9);
            { let args = []; c.r[0] = host.call(m, 0x526880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5a6f6d; continue; }
            pc = 0x5a6ed1; continue;
            }
            0x5a6ed1 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xb08),32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb04),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xb08); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[7]; wm(m,a,8,0x1u32); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x5a6f0c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5268d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1b),32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5a6f2f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x5a6f39);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(7,32,c.r[6].wrapping_add(0xac));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5a6f4b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x130),32); c.push(m, 0x5a6f5a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5a6f6b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5a6fa1; continue;
            }
            0x5a6f6d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb04),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5a6f89);
            { let args = []; c.r[0] = host.call(m, 0x5265e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5a6fa1; continue; }
            pc = 0x5a6f8d; continue;
            }
            0x5a6f8d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb08),32));
            { let a = c.r[6].wrapping_add(0xb0d); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xb08); wm(m,a,32,c.get(0,32)); }
            pc = 0x5a6fa1; continue;
            }
            0x5a6fa1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb04),32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5a6faf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f6ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x5a6fc7; continue; }
            pc = 0x5a6fb3; continue;
            }
            0x5a6fb3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb08),32));
            { let a = c.r[6].wrapping_add(0xb0e); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xb08); wm(m,a,32,c.get(0,32)); }
            pc = 0x5a6fc7; continue;
            }
            0x5a6fc7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb04),32));
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x5a6fea);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5269a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6dd892u32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5a700b; continue; }
            pc = 0x5a6ff7; continue;
            }
            0x5a6ff7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb08),32));
            { let a = c.r[6].wrapping_add(0xb0f); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xb08); wm(m,a,32,c.get(0,32)); }
            pc = 0x5a700b; continue;
            }
            0x5a700b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xb04),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x82c),8); c.sub(a,0x14u32,8); }
            if c.cond(3) { pc = 0x5a7040; continue; }
            pc = 0x5a702c; continue;
            }
            0x5a702c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xb08),32));
            { let a = c.r[6].wrapping_add(0xb10); wm(m,a,8,0x1u32); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xb08); wm(m,a,32,c.get(1,32)); }
            pc = 0x5a7040; continue;
            }
            0x5a7040 => {
            c.set(1,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m, 0x5a704d);
            { let args = []; c.r[0] = host.call(m, 0x5261a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5a7065; continue; }
            pc = 0x5a7051; continue;
            }
            0x5a7051 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb08),32));
            { let a = c.r[6].wrapping_add(0xb11); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xb08); wm(m,a,32,c.get(0,32)); }
            pc = 0x5a7065; continue;
            }
            0x5a7065 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb04),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x5a7080);
            { let args = []; c.r[0] = host.call(m, 0x526280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5a7098; continue; }
            pc = 0x5a7084; continue;
            }
            0x5a7084 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb08),32));
            { let a = c.r[6].wrapping_add(0xb12); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xb08); wm(m,a,32,c.get(0,32)); }
            pc = 0x5a7098; continue;
            }
            0x5a7098 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb04),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5a70b4);
            { let args = []; c.r[0] = host.call(m, 0x526120, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5a70cc; continue; }
            pc = 0x5a70b8; continue;
            }
            0x5a70b8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb08),32));
            { let a = c.r[6].wrapping_add(0xb13); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xb08); wm(m,a,32,c.get(0,32)); }
            pc = 0x5a70cc; continue;
            }
            0x5a70cc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb04),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5a70e8);
            { let args = []; c.r[0] = host.call(m, 0x5261d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6dd892u32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5a7109; continue; }
            pc = 0x5a70f5; continue;
            }
            0x5a70f5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb08),32));
            { let a = c.r[6].wrapping_add(0xb14); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xb08); wm(m,a,32,c.get(0,32)); }
            pc = 0x5a7109; continue;
            }
            0x5a7109 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xb1c),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd736u32,16),16); }
            if c.cond(3) { pc = 0x5a71f8; continue; }
            pc = 0x5a711d; continue;
            }
            0x5a711d => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xb04),16));
            c.set(1,32,0x6dd730u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5a7130);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d7900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xb1c),16),16); }
            if c.cond(5) { pc = 0x5a71b2; continue; }
            pc = 0x5a7139; continue;
            }
            0x5a7139 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,16,rm(m,0x6dd736u32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x5a715a; continue; }
            pc = 0x5a714b; continue;
            }
            0x5a714b => {
            c.set(1,32,rm(m,0x6dd730u32,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(7,32,c.get(0,32));
            pc = 0x5a715c; continue;
            }
            0x5a715a => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x5a715c; continue;
            }
            0x5a715c => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x5a7170);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x6),16); c.sub(a,0x1bu32,16); }
            if c.cond(5) { pc = 0x5a71b2; continue; }
            pc = 0x5a7177; continue;
            }
            0x5a7177 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xc),8));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5a719c; continue; }
            pc = 0x5a718f; continue;
            }
            0x5a718f => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5a71b2; continue; }
            pc = 0x5a719c; continue;
            }
            0x5a719c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb08),32));
            { let a = c.r[6].wrapping_add(0xb15); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xb08); wm(m,a,32,c.get(0,32)); }
            pc = 0x5a71f8; continue;
            }
            0x5a71b2 => {
            c.set(1,32,rm(m,0x6e5574u32,32));
            c.set(3,32,0x9u32);
            { let a = rm(m,c.r[1].wrapping_add(0x1bf4),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x5a71ca; continue; }
            pc = 0x5a71c5; continue;
            }
            0x5a71c5 => {
            c.push(m, 0x5a71ca);
            { let args = []; c.r[0] = host.call(m, 0x5d7fa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5a71ca; continue;
            }
            0x5a71ca => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8d4),32));
            c.set(1,32,c.r[6].wrapping_add(0x8d4));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x5a71de); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xb16),8));
            { let a = c.r[6].wrapping_add(0xb1c); wm(m,a,16,0xffffu32); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x5a71f8; continue; }
            pc = 0x5a71f1; continue;
            }
            0x5a71f1 => {
            { let a = c.r[6].wrapping_add(0xb16); wm(m,a,8,0xffu32); }
            pc = 0x5a71f8; continue;
            }
            0x5a71f8 => {
            c.set(0,16,rm(m,0x6dd736u32,16));
            { let a = rm(m,c.r[6].wrapping_add(0xb1c),16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x5a730c; continue; }
            pc = 0x5a720b; continue;
            }
            0x5a720b => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5a721c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xb04),16));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xa),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x5a7233);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d7900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,c.get(0,32));
            c.set(0,16,rm(m,0x6dd736u32,16));
            { let a = c.get(5,16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x5a72e8; continue; }
            pc = 0x5a7244; continue;
            }
            0x5a7244 => {
            c.set(7,32,c.get(5,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,16,c.get(0,16));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x5a7263; continue; }
            pc = 0x5a7255; continue;
            }
            0x5a7255 => {
            c.set(0,32,rm(m,0x6dd730u32,32));
            c.set(3,32,c.get(7,32));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            pc = 0x5a7265; continue;
            }
            0x5a7263 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x5a7265; continue;
            }
            0x5a7265 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x5a7279);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x6),16); c.sub(a,0x1bu32,16); }
            if c.cond(5) { pc = 0x5a72a5; continue; }
            pc = 0x5a7280; continue;
            }
            0x5a7280 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xc),8));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5a72e8; continue; }
            pc = 0x5a7298; continue;
            }
            0x5a7298 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5a72e8; continue; }
            pc = 0x5a72a5; continue;
            }
            0x5a72a5 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6dd736u32,16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(7) { pc = 0x5a72be; continue; }
            pc = 0x5a72b2; continue;
            }
            0x5a72b2 => {
            c.set(0,32,rm(m,0x6dd730u32,32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            pc = 0x5a72c0; continue;
            }
            0x5a72be => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x5a72c0; continue;
            }
            0x5a72c0 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xb04),16));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x6),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x5a72d7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d7900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,c.get(0,32));
            c.set(0,16,rm(m,0x6dd736u32,16));
            { let a = c.get(5,16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x5a7244; continue; }
            pc = 0x5a72e8; continue;
            }
            0x5a72e8 => {
            { let a = c.r[6].wrapping_add(0xb1c); wm(m,a,16,c.get(5,16)); }
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd736u32,16),16); }
            if c.cond(3) { pc = 0x5a730c; continue; }
            pc = 0x5a72f8; continue;
            }
            0x5a72f8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb08),32));
            { let a = c.r[6].wrapping_add(0xb15); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xb08); wm(m,a,32,c.get(0,32)); }
            pc = 0x5a730c; continue;
            }
            0x5a730c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "tavern_side_menu_refresh_available_pages" => tavern_side_menu_refresh_available_pages(m, host, regs),
        _ => return None,
    })
}
