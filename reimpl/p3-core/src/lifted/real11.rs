//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

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
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5eabad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x264),32));
            c.set(1,32,c.r[6].wrapping_add(0x264));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5eabc1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
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
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
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
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn class22_update_scene_buffer_cells_and_animations(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x592000;
    loop {
        match pc {
            0x592000 => {
            { let a = c.get(4,32); let r = c.sub(a,0x40u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x38));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fild32(m,c.r[6].wrapping_add(0x510));
            c.fstp32(m,c.r[4].wrapping_add(0x34));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x34),4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            c.push(m, 0x59203d);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0xc));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x38),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x44u32,8); }
            if c.cond(11) { pc = 0x59205c; continue; }
            pc = 0x592058; continue;
            }
            0x592058 => {
            { let a = rm(m,c.r[4].wrapping_add(0xc),32); let r = c.inc(a,32); { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x59205c; continue;
            }
            0x59205c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x514),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x34));
            c.fild32(m,c.r[4].wrapping_add(0x38));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x34),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x38),4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            c.fop_st('+',0,0,false);
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            c.push(m, 0x5920a5);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x38));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x44u32,8); }
            if c.cond(11) { pc = 0x5920c3; continue; }
            pc = 0x5920c2; continue;
            }
            0x5920c2 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x5920c3; continue;
            }
            0x5920c3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0xc640),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x6e338cu32,8));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x1u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            if c.cond(15) { pc = 0x592776; continue; }
            pc = 0x5920ff; continue;
            }
            0x5920ff => {
            c.set(2,32,c.get(0,32));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc308));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            pc = 0x59210c; continue;
            }
            0x59210c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(3,32); let r = c.add(a,c.get(5,32),32); c.set(3,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0x560); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(7,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(1,32)); }
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x592195; continue; }
            pc = 0x592145; continue;
            }
            0x592145 => {
            { let a = rm(m,c.r[1].wrapping_add(0x2),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x592195; continue; }
            pc = 0x59214b; continue;
            }
            0x59214b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x2),8));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x59216d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x592183; continue; }
            pc = 0x59217b; continue;
            }
            0x59217b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            pc = 0x592185; continue;
            }
            0x592183 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x592185; continue;
            }
            0x592185 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x592195); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x592195; continue;
            }
            0x592195 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xfffffffc));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x59225a; continue; }
            pc = 0x5921ba; continue;
            }
            0x5921ba => {
            c.set(1,16,rm(m,c.r[0],16));
            { let a = c.get(1,16); c.and(a,c.get(1,16),16); }
            if c.cond(4) { pc = 0x592235; continue; }
            pc = 0x5921c2; continue;
            }
            0x5921c2 => {
            { let a = rm(m,c.r[0].wrapping_add(0x2),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x592235; continue; }
            pc = 0x5921c8; continue;
            }
            0x5921c8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x5921e2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x592235; continue; }
            pc = 0x5921eb; continue;
            }
            0x5921eb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7],16));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x2),8));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x59220d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x592223; continue; }
            pc = 0x59221b; continue;
            }
            0x59221b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            pc = 0x592225; continue;
            }
            0x592223 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x592225; continue;
            }
            0x592225 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x592235); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x592235; continue;
            }
            0x592235 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x510),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(7,32)); }
            if c.cond(2) { pc = 0x5921ba; continue; }
            pc = 0x59225a; continue;
            }
            0x59225a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.set(2,32,c.r[7].wrapping_add(0xfffffffd));
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x5923d0; continue; }
            pc = 0x59229e; continue;
            }
            0x59229e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x592326; continue; }
            pc = 0x5922b1; continue;
            }
            0x5922b1 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x592311; continue; }
            pc = 0x5922bf; continue;
            }
            0x5922bf => {
            { let a = rm(m,c.r[1].wrapping_add(0x2),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x592311; continue; }
            pc = 0x5922c5; continue;
            }
            0x5922c5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x2),8));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x5922e8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x5922fe; continue; }
            pc = 0x5922f6; continue;
            }
            0x5922f6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            pc = 0x592300; continue;
            }
            0x5922fe => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x592300; continue;
            }
            0x592300 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x592311); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x592311; continue;
            }
            0x592311 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x510),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(1,32)); }
            pc = 0x592326; continue;
            }
            0x592326 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(3) { pc = 0x592388; continue; }
            pc = 0x59233f; continue;
            }
            0x59233f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x59236d; continue; }
            pc = 0x59234d; continue;
            }
            0x59234d => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x2),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x59236d; continue; }
            pc = 0x592354; continue;
            }
            0x592354 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x59236d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x59236d; continue;
            }
            0x59236d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x510),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x59233f; continue; }
            pc = 0x592388; continue;
            }
            0x592388 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,0x1u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = rm(m,c.r[6].wrapping_add(0x55c),32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x59229e; continue; }
            pc = 0x5923c8; continue;
            }
            0x5923c8 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            pc = 0x5923d0; continue;
            }
            0x5923d0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x500),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc318),32));
            { let a = c.r[6].wrapping_add(0x4f8); wm(m,a,32,0x0u32); }
            c.set(3,32,c.r[2].wrapping_add(c.r[1]));
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x592401; continue; }
            pc = 0x5923f5; continue;
            }
            0x5923f5 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            pc = 0x592407; continue;
            }
            0x592401 => {
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            pc = 0x592407; continue;
            }
            0x592407 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x5926e9; continue; }
            pc = 0x592422; continue;
            }
            0x592422 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x59243f; continue; }
            pc = 0x592439; continue;
            }
            0x592439 => {
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            pc = 0x59243f; continue;
            }
            0x59243f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0xffu32); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x5926a3; continue; }
            pc = 0x59245d; continue;
            }
            0x59245d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x5924be; continue; }
            pc = 0x59246b; continue;
            }
            0x59246b => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x12),8));
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x2),8));
            { let a = c.get(3,8); c.sub(a,c.get(2,8),8); }
            if c.cond(3) { pc = 0x5924be; continue; }
            pc = 0x592476; continue;
            }
            0x592476 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[2].wrapping_add(0x2),8));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.get(7,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x592494); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x5924a8; continue; }
            pc = 0x5924a0; continue;
            }
            0x5924a0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            pc = 0x5924aa; continue;
            }
            0x5924a8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5924aa; continue;
            }
            0x5924aa => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x5924be); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x5924be; continue;
            }
            0x5924be => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            if c.cond(7) { pc = 0x5924e2; continue; }
            pc = 0x5924de; continue;
            }
            0x5924de => {
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(1,8)); }
            pc = 0x5924e2; continue;
            }
            0x5924e2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            if c.cond(14) { pc = 0x592676; continue; }
            pc = 0x5924f1; continue;
            }
            0x5924f1 => {
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            pc = 0x5924f5; continue;
            }
            0x5924f5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x514),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2),8));
            c.set(7,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(3,32,c.r[3].wrapping_add(c.r[2]).wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x592667; continue; }
            pc = 0x592520; continue;
            }
            0x592520 => {
            c.set(0,8,c.get(1,8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x592667; continue; }
            pc = 0x59252a; continue;
            }
            0x59252a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x592537);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x590a20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x592567; continue; }
            pc = 0x592544; continue;
            }
            0x592544 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8));
            pc = 0x592551; continue;
            }
            0x592551 => {
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x59255f; continue; }
            pc = 0x592555; continue;
            }
            0x592555 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x14u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x592551; continue; }
            pc = 0x59255d; continue;
            }
            0x59255d => {
            pc = 0x592567; continue;
            }
            0x59255f => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x592667; continue; }
            pc = 0x592567; continue;
            }
            0x592567 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4dc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5925c4; continue; }
            pc = 0x592577; continue;
            }
            0x592577 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4dc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x592593);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4dc),32));
            { let a = c.r[6].wrapping_add(0x4e0); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0xffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5925c1);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x5925c4; continue;
            }
            0x5925c4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x48),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc); wm(m,a,16,c.get(1,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x11); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xe); wm(m,a,16,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4f8); wm(m,a,32,c.get(0,32)); }
            pc = 0x592667; continue;
            }
            0x592667 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5924f5; continue; }
            pc = 0x592676; continue;
            }
            0x592676 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x510),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(0,32,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            if c.cond(2) { pc = 0x59245d; continue; }
            pc = 0x5926a3; continue;
            }
            0x5926a3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            if c.cond(2) { pc = 0x592422; continue; }
            pc = 0x5926e9; continue;
            }
            0x5926e9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x592759; continue; }
            pc = 0x592703; continue;
            }
            0x592703 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(3,8,rm(m,c.r[7].wrapping_add(0x10),8));
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,c.get(5,32));
            c.set(0,32,rm(m,c.r[5],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x592726); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x59273a; continue; }
            pc = 0x592732; continue;
            }
            0x592732 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            pc = 0x59273c; continue;
            }
            0x59273a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x59273c; continue;
            }
            0x59273c => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x59274e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x592703; continue; }
            pc = 0x592759; continue;
            }
            0x592759 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x59210c; continue; }
            pc = 0x592776; continue;
            }
            0x592776 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x5927b3; continue; }
            pc = 0x592784; continue;
            }
            0x592784 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2a0),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5927a2; continue; }
            pc = 0x59279d; continue;
            }
            0x59279d => {
            { let a = c.get(1,32); let r = c.add(a,0xffffffc4u32,32); c.set(1,32,r); }
            pc = 0x5927a4; continue;
            }
            0x5927a2 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x5927a4; continue;
            }
            0x5927a4 => {
            { let a = c.r[1].wrapping_add(0x5d); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x592784; continue; }
            pc = 0x5927b3; continue;
            }
            0x5927b3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x59285c; continue; }
            pc = 0x5927c7; continue;
            }
            0x5927c7 => {
            c.set(0,32,rm(m,0x6e51b4u32,32));
            pc = 0x5927cc; continue;
            }
            0x5927cc => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2a0),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5927ef; continue; }
            pc = 0x5927ea; continue;
            }
            0x5927ea => {
            c.set(1,32,c.r[0].wrapping_add(0xffffffc4));
            pc = 0x5927f1; continue;
            }
            0x5927ef => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x5927f1; continue;
            }
            0x5927f1 => {
            { let a = rm(m,0x6e51b4u32,16); let r = c.inc(a,16); { let a = 0x6e51b4u32; wm(m,a,16,r); } }
            c.set(0,32,rm(m,0x6e51b4u32,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x59281b; continue; }
            pc = 0x59280f; continue;
            }
            0x59280f => {
            { let a = 0x6e51b4u32; wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,0x6e51b4u32,32));
            pc = 0x59281b; continue;
            }
            0x59281b => {
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x68),16));
            { let a = c.get(7,16); c.sub(a,c.get(2,16),16); }
            if c.cond(4) { pc = 0x592841; continue; }
            pc = 0x592824; continue;
            }
            0x592824 => {
            c.push(m, 0x592829);
            { let args = []; c.r[0] = host.call(m, 0x583080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x59283a; continue; }
            pc = 0x59282d; continue;
            }
            0x59282d => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x11u32,32); }
            if c.cond(13) { pc = 0x59285a; continue; }
            pc = 0x59283a; continue;
            }
            0x59283a => {
            c.set(0,32,rm(m,0x6e51b4u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x592841; continue;
            }
            0x592841 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x5927cc; continue; }
            pc = 0x592858; continue;
            }
            0x592858 => {
            pc = 0x59285c; continue;
            }
            0x59285a => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x59285c; continue;
            }
            0x59285c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(14) { pc = 0x592899; continue; }
            pc = 0x59286a; continue;
            }
            0x59286a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2a0),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x592888; continue; }
            pc = 0x592883; continue;
            }
            0x592883 => {
            { let a = c.get(1,32); let r = c.add(a,0xffffffc4u32,32); c.set(1,32,r); }
            pc = 0x59288a; continue;
            }
            0x592888 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x59288a; continue;
            }
            0x59288a => {
            { let a = c.r[1].wrapping_add(0x68); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x59286a; continue; }
            pc = 0x592899; continue;
            }
            0x592899 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn debug_page_town_trade_balance_matrix(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x416310;
    loop {
        match pc {
            0x416310 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m,0x65bcabu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
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
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d04),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x69544cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x416383);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x3u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x41639d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc0),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x4163b6; continue; }
            pc = 0x4163af; continue;
            }
            0x4163af => {
            { let a = c.r[7].wrapping_add(0xc0); wm(m,a,16,c.get(3,16)); }
            pc = 0x4163b6; continue;
            }
            0x4163b6 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc0),16));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.push(m,0x69545cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4163d6);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x17u32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4163ed); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x695498u32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x4163fb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x35u32);
            c.push(m,0xau32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416414); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x0u32); }
            c.push(m, 0x416422);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,0x64u32);
            c.set(3,32,0x6c0bb0u32);
            pc = 0x41642c; continue;
            }
            0x41642c => {
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x3),8),8)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x2),8),8)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x1),8),8)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[0],8),8)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x6954a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x416450);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x35u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416466); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x2eu32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,0x6c0c00u32,32); }
            if c.cond(12) { pc = 0x41642c; continue; }
            pc = 0x416474; continue;
            }
            0x416474 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x53u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            if c.cond(6) { pc = 0x4165db; continue; }
            pc = 0x416491; continue;
            }
            0x416491 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x4164ac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x2u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0xau32);
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4164c8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(68);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x0u32); }
            c.push(m, 0x4164d6);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,0x64u32);
            pc = 0x4164db; continue;
            }
            0x4164db => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x0u32);
            c.set(2,16,rm(m,c.r[0].wrapping_add(0xc0),16));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x4164fb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x52ee90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,0x66bc6cu32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x4100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x416526; continue; }
            pc = 0x41651a; continue;
            }
            0x41651a => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x1c),4); c.fop('+',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            pc = 0x416526; continue;
            }
            0x416526 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x41653c; continue; }
            pc = 0x416534; continue;
            }
            0x416534 => {
            { let v = c.ld(m,0x66bc60u32,4); c.fop('*',v); }
            pc = 0x416542; continue;
            }
            0x41653c => {
            { let v = c.ld(m,0x66bc5cu32,4); c.fop('*',v); }
            pc = 0x416542; continue;
            }
            0x416542 => {
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.fstp64(m,c.r[4]);
            c.push(m,0x6954acu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x41655f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x416578); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(3,32); let r = c.add(a,0x2eu32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x4164db; continue; }
            pc = 0x416585; continue;
            }
            0x416585 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,0x66bc5cu32,4); c.fop('*',v); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.fstp64(m,c.r[4]);
            c.push(m,0x6954b4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4165a4);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x5c),32); c.push(m, 0x4165bd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x416491; continue; }
            pc = 0x4165db; continue;
            }
            0x4165db => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4165ec);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
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

pub fn hanse_content_window_show(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5bef00;
    loop {
        match pc {
            0x5bef00 => {
            { let a = c.get(4,32); let r = c.sub(a,0x48u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x5bef13);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bef1d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x60),32); c.push(m, 0x5bef2d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x5bef3d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0x64u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[6],32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[7].wrapping_add(0x64),32); c.push(m, 0x5bef70); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5bef77);
            { let args = []; c.r[0] = host.call(m, 0x462390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x1300),32));
            c.set(7,32,c.r[6].wrapping_add(0x1300));
            c.push(m,0x64u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[3].wrapping_add(0x48),32); c.push(m, 0x5bef8a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xfffffffd));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5bef9e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x3u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x5befb1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5befb9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5befc5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x13e8));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5befd3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[7],32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[3].wrapping_add(0x48),32); c.push(m, 0x5befdc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xfffffffd));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5beff3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x5bf005); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5bf011); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c2c00u32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5bf01e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0x14d0));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5bf02c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5bf038); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c2bf4u32,32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5bf045);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xd),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5bf06c; continue; }
            pc = 0x5bf052; continue;
            }
            0x5bf052 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x18),16),16); c.set(0,16,r); }
            { let a = c.get(0,32); let r = c.add(a,0x96u32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xbe); wm(m,a,16,c.get(0,16)); }
            pc = 0x5bf08a; continue;
            }
            0x5bf06c => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x5bf073); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x30),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,16); let r = c.sub(a,c.get(0,16),16); c.set(1,16,r); }
            { let a = c.get(1,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x18),16),16); c.set(1,16,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x3u32,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xbe); wm(m,a,16,c.get(1,16)); }
            pc = 0x5bf08a; continue;
            }
            0x5bf08a => {
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xbe),16));
            c.push(m,0x64u32);
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5bf0a3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5bf0b9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5bf0c5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c625cu32);
            c.set(1,32,0x6dcd28u32);
            { let a = c.r[6].wrapping_add(0xb4); wm(m,a,16,0x32u32); }
            c.push(m, 0x5bf0dd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6c2c94u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcd28u32);
            c.push(m, 0x5bf0f0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)));
            c.push(m,0x6c6260u32);
            { let a = c.get(1,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0xb4),16),16); c.set(1,16,r); }
            { let a = c.get(1,32); let r = c.add(a,0x5u32,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xb6); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,0x6dcd28u32);
            c.push(m, 0x5bf113);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0xb6),16),16); c.set(0,16,r); }
            c.set(1,32,0x6dcd28u32);
            { let a = c.r[6].wrapping_add(0xb8); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,0x6c2ca4u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5bf132);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0xb8),16),16); c.set(0,16,r); }
            { let a = c.get(0,32); let r = c.add(a,0x5u32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xba); wm(m,a,16,c.get(0,16)); }
            c.set(0,8,0xffu32);
            { let a = c.r[6].wrapping_add(0xd8); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xc324),8));
            { let a = c.r[6].wrapping_add(0xdb); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0xd9); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,16,0xffffu32); }
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x5bf17f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x550));
            { let a = c.r[6].wrapping_add(0x4d8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4d4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x5bf19a; continue;
            }
            0x5bf19a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[7],32));
            c.push(m,0x64u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x6b));
            c.set(0,32,rm(m,0x69c7f4u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcdd0u32);
            c.push(m, 0x5bf1b6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xff));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x5bf1c6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bf1ce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5bf1da); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x66u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x198u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x5bf19a; continue; }
            pc = 0x5bf1f2; continue;
            }
            0x5bf1f2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0xadc));
            c.set(0,8,rm(m,0x66de58u32,8));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5bf218);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xbc4));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de58u32,8));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5bf23e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xd94));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de58u32,8));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5bf264);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xe7c));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de58u32,8));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5bf28a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x8f0));
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5bf2b0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x9d8));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de57u32,8));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5bf2d6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x64u32);
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xffffff9c));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.add(a,0x64u32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5bf2f1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbc4),32));
            c.push(m,0x64u32);
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xffffff9c));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xffffff9c));
            c.set(1,32,c.r[6].wrapping_add(0xbc4));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5bf318); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bf320);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xbc4));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bf32e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c6268u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5bf33a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c626au32);
            c.set(1,32,c.r[6].wrapping_add(0xbc4));
            c.push(m, 0x5bf34a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5bf356); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xbc4),32));
            c.set(1,32,c.r[6].wrapping_add(0xbc4));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5bf36a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[7],32));
            c.push(m,0x64u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffce));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[3].wrapping_add(0x44),32); c.push(m, 0x5bf37e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,0x32u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x5bf398); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9d8),32));
            c.set(3,32,c.r[6].wrapping_add(0x9d8));
            c.set(2,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xffffffce));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,0x64u32);
            c.push(m,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5bf3ce); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bf3d6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bf3de);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c626cu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5bf3ea);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c626eu32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5bf3f6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5bf402); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5bf40e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0xcac));
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5bf434);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[7],32));
            c.push(m,0x64u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffd8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x13e8),32));
            c.set(1,32,c.r[6].wrapping_add(0x13e8));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5bf452); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x5bf464); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bf46c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x69c82cu32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5bf47a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5bf486); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5bf48d);
            { let args = []; c.r[0] = host.call(m, 0x5c67a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,0x64u32,32); c.set(0,32,r); }
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,c.r[0].wrapping_add(0x1e));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xfffffff3));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,c.r[4].wrapping_add(0x48));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x5bf4cd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.set(3,32,c.r[6].wrapping_add(0x124));
            c.set(0,32,c.r[2].wrapping_mul(4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5bf4ee);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5bf4fa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x4u32);
            c.set(0,32,c.r[6].wrapping_add(0x12fa));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0xad8); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x12f9); wm(m,a,8,0x0u32); }
            c.push(m, 0x5bf518);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xcu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0xd1u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(7,32,c.r[6].wrapping_add(0xf64));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bf540);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x5bf54c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5bf558); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(7,32,c.r[6].wrapping_add(0x104c));
            { let a = c.get(0,32); let r = c.add(a,0x176u32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5bf572);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5bf57e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5bf58a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1138),32));
            c.set(7,32,c.r[6].wrapping_add(0x1138));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5bf59b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x78));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5bf5d0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5bf5dc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5bf5e8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x188),32));
            c.set(0,32,0x2710u32);
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[7].wrapping_add(0x180); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x5bf605; continue; }
            pc = 0x5bf5fd; continue;
            }
            0x5bf5fd => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5bf605);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5bf605; continue;
            }
            0x5bf605 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x188),32));
            { let a = c.r[7].wrapping_add(0x184); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x5bf622; continue; }
            pc = 0x5bf619; continue;
            }
            0x5bf619 => {
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5bf622);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5bf622; continue;
            }
            0x5bf622 => {
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5bf62b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd94),32));
            c.set(1,32,c.r[6].wrapping_add(0xd94));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5bf63a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(3,32,0xfffffffeu32);
            c.set(1,32,c.get(7,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5bf648); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd94),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,c.r[6].wrapping_add(0xd94));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x5bf664); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5bf66b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5bf676); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xe7c),32));
            c.set(7,32,c.r[6].wrapping_add(0xe7c));
            c.set(0,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x2));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5bf698); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bf6a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bf6a8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c6270u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5bf6b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c6272u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5bf6c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5bf6cc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5bf6d8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x12f3); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x12f4); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x15b8); wm(m,a,8,0x0u32); }
            c.set(1,8,rm(m,0x6de4b0u32,8));
            { let a = c.r[6].wrapping_add(0x15b9); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x118); wm(m,a,16,0x0u32); }
            c.set(2,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.r[6].wrapping_add(0x15cc); wm(m,a,32,c.get(2,32)); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x15c4),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x15c0),32));
            c.set(7,32,c.get(3,32));
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5bf734; continue; }
            pc = 0x5bf722; continue;
            }
            0x5bf722 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bf72a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x5bf722; continue; }
            pc = 0x5bf734; continue;
            }
            0x5bf734 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x15c4),32));
            c.set(7,32,c.get(5,32));
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5bf74e; continue; }
            pc = 0x5bf740; continue;
            }
            0x5bf740 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5bf747);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x5bf740; continue; }
            pc = 0x5bf74e; continue;
            }
            0x5bf74e => {
            { let a = c.r[6].wrapping_add(0x15c4); wm(m,a,32,c.get(5,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x437290;
    loop {
        match pc {
            0x437290 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e17cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x60u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(0,32,c.r[4].wrapping_add(0x80));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,0x0u32); }
            c.push(m, 0x4372d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4372df);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b5380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x66e0a4u32,32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x94),32); c.push(m, 0x4372ef); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x6976e4u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,8,0x1u32); }
            c.push(m, 0x437311);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x3e80u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6976f0u32);
            c.push(m, 0x43732d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x437337);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b3ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x4e20u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697708u32);
            c.push(m, 0x437350);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x43735a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b3ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x437373);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4cd4),32));
            c.set(1,32,c.r[6].wrapping_add(0x4cd4));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x437382); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43739b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x5678),32));
            c.set(1,32,c.r[6].wrapping_add(0x5678));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4373aa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x3u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4373c3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x21c),32));
            c.set(1,32,c.r[6].wrapping_add(0x21c));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4373d2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2cu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,c.r[4].wrapping_add(0x88));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4373eb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2f4),32));
            c.set(1,32,c.r[6].wrapping_add(0x2f4));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4373fa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2du32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x437413);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3cc),32));
            c.set(1,32,c.r[6].wrapping_add(0x3cc));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x437422); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x32u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43743b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4a4),32));
            c.set(7,32,c.r[6].wrapping_add(0x4a4));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x43744c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x31u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x437465);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x57c),32));
            c.set(3,32,c.r[6].wrapping_add(0x57c));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x437476); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[5].wrapping_add(0x48),32); c.push(m, 0x43747d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,0x12cu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x437491); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,0x190u32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x4374a3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[7].wrapping_add(0x48),32); c.push(m, 0x4374aa); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,0x12cu32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x4374be); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,0x190u32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[7].wrapping_add(0x78),32); c.push(m, 0x4374d0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,0x1u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4374ed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x654),32));
            c.set(5,32,c.r[6].wrapping_add(0x654));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x4374fe); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x71u32);
            c.push(m,0x196u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x43750d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x66e10cu32,32));
            { let a = c.r[6].wrapping_add(0x714); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x66e110u32,32));
            { let a = c.r[6].wrapping_add(0x718); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x71c); wm(m,a,32,0xff404040u32); }
            c.set(2,32,rm(m,0x69af00u32,32));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x43753c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x437546); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(0xe8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0xec); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x43755f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x437567);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,0x69af04u32);
            c.set(3,32,c.r[6].wrapping_add(0x748));
            pc = 0x437572; continue;
            }
            0x437572 => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43757a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464f30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3].wrapping_add(0xffffff0c),32));
            c.set(1,32,c.r[3].wrapping_add(0xffffff0c));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x437591); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x6));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,0x196u32);
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x4375aa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4375b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4375bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xf4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x69af0cu32,32); }
            if c.cond(12) { pc = 0x437572; continue; }
            pc = 0x4375cd; continue;
            }
            0x4375cd => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x4375d6);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,8,0x2u32); }
            c.push(m, 0x4375e6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x4375ef);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x218); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43760e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x930),32));
            c.set(3,32,c.r[6].wrapping_add(0x930));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x43761f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x1acu32);
            c.push(m,0x52u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x43762d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,0x1u32);
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x43763a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x66e10cu32,32));
            { let a = c.r[6].wrapping_add(0x9f0); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x66e110u32,32));
            { let a = c.r[6].wrapping_add(0x9f4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x9f8); wm(m,a,32,0xff404040u32); }
            c.set(2,32,rm(m,0x69af10u32,32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x437669);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x69af10u32,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x130),32); c.push(m, 0x43767a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[6].wrapping_add(0xc10));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x437688); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[3].wrapping_add(0xe8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[3].wrapping_add(0xec); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4376a0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4376a8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xc30));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,0x69af14u32); }
            c.set(7,32,c.r[6].wrapping_add(0xa24));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x4376c3; continue;
            }
            0x4376c3 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4376cb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464f30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xffffff0c),32));
            c.set(1,32,c.r[7].wrapping_add(0xffffff0c));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x4376e2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x6));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,0x52u32);
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x4376f8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x437709);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x130),32); c.push(m, 0x43771d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x437729); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x437731);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x20u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xf4u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0x69af1cu32,32); }
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x4376c3; continue; }
            pc = 0x43775e; continue;
            }
            0x43775e => {
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x437777);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xef8),32));
            c.set(7,32,c.r[6].wrapping_add(0xef8));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x437788); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x66e10cu32,32));
            c.set(0,32,0x1u32);
            { let a = c.r[6].wrapping_add(0xfb8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,0x66e110u32,32));
            { let a = c.r[6].wrapping_add(0xfbc); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[7].wrapping_add(0xe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0xec); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x4377c1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4377ce); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4377da); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x226u32);
            c.push(m,0x1c3u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x4377eb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[6].wrapping_add(0x10e0));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0xfc0); wm(m,a,32,0xff404040u32); }
            c.push(m, 0x437803);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464f30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xfec));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x437811);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464f30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0x11d4));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x43781f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464f30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x12c8));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43782d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464f30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x13bc));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43783b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464f30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x437854);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c64),32));
            c.set(3,32,c.r[6].wrapping_add(0x2c64));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x437865); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x66e10cu32,32));
            c.push(m,0xa0u32);
            { let a = c.r[6].wrapping_add(0x2d24); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x66e110u32,32));
            { let a = c.r[6].wrapping_add(0x2d28); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,0x1u32);
            c.set(1,32,c.r[6].wrapping_add(0xfec));
            { let a = c.r[3].wrapping_add(0xe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[3].wrapping_add(0xec); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x2d2c); wm(m,a,32,0xff404040u32); }
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x1f5u32);
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x4378ac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,32,rm(m,c.r[5],32));
            c.push(m,0x220u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[3].wrapping_add(0x44),32); c.push(m, 0x4378b9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,0xbeu32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[3].wrapping_add(0x78),32); c.push(m, 0x4378cb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x12c8),32));
            c.push(m,0x220u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4378de); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,0xbeu32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x12c8));
            { let target = rm(m,c.r[3].wrapping_add(0x78),32); c.push(m, 0x4378f4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x13bc),32));
            c.set(3,32,c.r[6].wrapping_add(0x13bc));
            c.push(m,0x172u32);
            c.push(m,0x1f4u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x43790f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x437920); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x69b2acu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x43792e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x69af28u32,32));
            c.set(1,32,c.r[6].wrapping_add(0xfec));
            c.push(m,c.get(2,32));
            c.push(m, 0x437940);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x69b28cu32,32));
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,c.get(0,32));
            c.push(m, 0x437951);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x69a954u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x43795f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x69af54u32,32));
            c.set(1,32,c.r[6].wrapping_add(0x12c8));
            c.push(m,c.get(2,32));
            c.push(m, 0x437971);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x69af30u32,32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43797e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x437986);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x10e0));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x437994);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0xfec));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4379a2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4379aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x12c8));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4379b8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4379c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4379c9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43e550, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4379d5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4379e2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x12c8),32));
            c.set(1,32,c.r[6].wrapping_add(0x12c8));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4379f6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x437a02); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x21u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x4ca4); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0x4ca8); wm(m,a,32,0x0u32); }
            c.push(m, 0x437a2f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14b0),32));
            c.set(3,32,c.r[6].wrapping_add(0x14b0));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x437a40); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x437a59);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1908),32));
            c.set(7,32,c.r[6].wrapping_add(0x1908));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x437a6a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[6].wrapping_add(0x1588));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x437a78); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(1,32); let r = c.add(a,0x2e8u32,32); c.set(1,32,r); }
            c.push(m, 0x437a89);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x437a91);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x18bu32);
            c.push(m,0x64u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x437a9f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x66e10cu32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x437aac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2d80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x437ab4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b1c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x437abd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2e20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6cbe00u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x1a74); wm(m,a,32,0x10u32); }
            c.push(m, 0x437ad3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4cab50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1a7c); wm(m,a,32,0x1u32); }
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[7].wrapping_add(0x170); wm(m,a,32,0x64u32); }
            { let a = c.r[7].wrapping_add(0x179); wm(m,a,8,0x1u32); }
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x437afd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x437b09); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x437b22);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1788),32));
            c.set(5,32,c.r[6].wrapping_add(0x1788));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x437b33); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[6].wrapping_add(0x15a8));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x437b42); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(1,32); let r = c.add(a,0x168u32,32); c.set(1,32,r); }
            c.push(m, 0x437b53);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x437b5b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x3));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x437b75); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,0x66e10cu32,32));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x437b83);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2d80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x437b8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b1c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x437b94);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2e20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6cbe00u32);
            c.set(1,32,c.get(5,32));
            { let a = c.r[6].wrapping_add(0x18f4); wm(m,a,32,0x10u32); }
            c.push(m, 0x437baa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4cab50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(4,32)); }
            { let a = c.r[6].wrapping_add(0x18fc); wm(m,a,32,0x1u32); }
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0x170); wm(m,a,32,0x64u32); }
            { let a = c.r[5].wrapping_add(0x179); wm(m,a,8,0x1u32); }
            c.push(m, 0x437bde);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x15c8),32));
            c.set(1,32,c.r[6].wrapping_add(0x15c8));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x437bed); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,32,c.r[6].wrapping_add(0xfec));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x437c09); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x437c15); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x15c8));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x44));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x437c3a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x1eu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x7c),32); c.push(m, 0x437c66); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x66e10cu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x437c74);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2d80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[6].wrapping_add(0x1748));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x437c82); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x14b0));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x437c90);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b1c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x437c99);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2e20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697720u32);
            { let a = c.r[6].wrapping_add(0x1734); wm(m,a,32,0xfu32); }
            c.push(m, 0x437cb6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4498),32));
            c.set(1,32,c.r[6].wrapping_add(0x4498));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x437cc5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697738u32);
            c.push(m, 0x437cd8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x48ac),32));
            c.set(1,32,c.r[6].wrapping_add(0x48ac));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x437ce7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x437cee);
            { let args = []; c.r[0] = host.call(m, 0x43ebe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x437d04);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1a88),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x437d13); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1a88),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            c.set(0,32,c.r[6].wrapping_add(0x4228));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x437d29); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x66e10cu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            c.push(m, 0x437d3b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2d80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            c.push(m, 0x437d48);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2e20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1a88),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            c.push(m,0x1fcu32);
            c.push(m,0x16fu32);
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x437d61); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x484c),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            { let a = c.r[6].wrapping_add(0x1bfc); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x437d7d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x1a88));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x437d8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x437d93);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x437d9b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x437da3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x437db0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x437dbc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x437dc8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x437dd7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x437de3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x7u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x437df9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc70),32));
            c.set(7,32,c.r[6].wrapping_add(0xc70));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x437e0a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[6].wrapping_add(0xd48));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x437e18);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4ada30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x8u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x437e2e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe20),32));
            c.set(1,32,c.r[6].wrapping_add(0xe20));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x437e3d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,32,0x3u32);
            pc = 0x437e42; continue;
            }
            0x437e42 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x437e4e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xd8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x437e42; continue; }
            pc = 0x437e57; continue;
            }
            0x437e57 => {
            c.push(m,0x9u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x437e6d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2a94),32));
            c.set(5,32,c.r[6].wrapping_add(0x2a94));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x437e7e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,0x2u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x437e89);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0xau32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x437e9f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2b7c),32));
            c.set(7,32,c.r[6].wrapping_add(0x2b7c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x437eb0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x437ebb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11d4),32));
            c.set(3,32,c.r[6].wrapping_add(0x11d4));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x437ed4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1ccu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x437ee6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x437ef1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,0x1ccu32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x437f07); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let target = rm(m,c.r[0].wrapping_add(0x78),32); c.push(m, 0x437f17); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x437f27); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x437f34); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c64),32));
            c.set(1,32,c.r[6].wrapping_add(0x2c64));
            c.set(0,32,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x5));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x437f55); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[6].wrapping_add(0x1f4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x65u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,0xfffffe0cu32);
            c.set(7,32,c.r[6].wrapping_add(0x2354));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x437f78; continue;
            }
            0x437f78 => {
            c.push(m,0x5u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x437f8e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xfffff8c0),32));
            c.set(3,32,c.r[7].wrapping_add(0xfffff8c0));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x437f9f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,c.get(0,32));
            c.push(m,0x16fu32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x437fb0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x437fb8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x437fc4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x437fcd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xeu32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x88));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x437fe3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x437fea); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.push(m,0x16fu32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x437ffb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438003);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43800f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x438018); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); let r = c.add(a,0x1cu32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,8,0x0u32); }
            { let a = c.r[0]; wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            if c.cond(12) { pc = 0x437f78; continue; }
            pc = 0x438048; continue;
            }
            0x438048 => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x438050);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x2b7c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x43805e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43806b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x438077); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x438081); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43808a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,0x0u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x84),32); c.push(m, 0x4380a1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4380ae); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x2c64));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4380bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4380c8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697750u32);
            c.push(m, 0x4380db);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2d64),32));
            c.set(7,32,c.r[6].wrapping_add(0x2d64));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x4380ec); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4380f8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4864),32));
            c.set(7,32,c.r[6].wrapping_add(0x4498));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43810f);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x486c),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x2));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4850),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x4848),32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4868),32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x50));
            c.set(0,32,c.r[4].wrapping_add(0x60));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(1,32)); }
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x43814a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4848),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x438168);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x438174); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x443c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x4490));
            c.push(m, 0x438186);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x1u32); }
            c.push(m, 0x438194);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            c.push(m, 0x4381a2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x80));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4381b6);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x84),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x6cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
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
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4a3e8e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
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
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4a3ec5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
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

pub fn multiplayer_screen_enter_client_lobby(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4392e0;
    loop {
        match pc {
            0x4392e0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65e1d2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x439303);
            { let args = []; c.r[0] = host.call(m, 0x4401c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6cbb9cu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,0x28u32); }
            c.push(m, 0x43931b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x1788));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            c.push(m, 0x439335);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x439345);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6cbb9du32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x439353);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(5,32,c.r[6].wrapping_add(0x1908));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x1u32); }
            c.push(m, 0x43936d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x43937a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x190u32);
            c.set(2,32,c.r[6].wrapping_add(0x54d4));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,0x1u32); }
            c.push(m, 0x439394);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x654));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x3u32); }
            pc = 0x4393a5; continue;
            }
            0x4393a5 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x2dc),32));
            c.set(1,32,c.r[3].wrapping_add(0x2dc));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4393b9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4393c5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.add(a,0xf4u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4393a5; continue; }
            pc = 0x4393d6; continue;
            }
            0x4393d6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x15c8),32));
            c.set(1,32,c.r[6].wrapping_add(0x15c8));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4393ea); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4393f6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x439403); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x12c8),32));
            c.set(1,32,c.r[6].wrapping_add(0x12c8));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43941c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x11d4),32));
            c.set(1,32,c.r[6].wrapping_add(0x11d4));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43943a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x439443);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43e550, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x439456); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43946a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c64),32));
            c.set(7,32,c.r[6].wrapping_add(0x2c64));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x439487); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x439493); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2a94),32));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(7,32,c.r[6].wrapping_add(0x2a94));
            c.set(1,8,u32::from(c.cond(4)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4394b3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2b7c),32));
            c.set(5,32,c.r[6].wrapping_add(0x2b7c));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4394d3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4cd1),8));
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4394e2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x4cd1),8));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4394f5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4394fe);
            { let args = []; c.r[0] = host.call(m, 0x439de0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xfec),32));
            c.set(1,32,c.r[6].wrapping_add(0xfec));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x439512); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x439523); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2d64),32));
            c.set(1,32,c.r[6].wrapping_add(0x2d64));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x439537); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,0xfffffdfcu32);
            { let a = c.r[6].wrapping_add(0x4ca8); wm(m,a,32,0x2u32); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[6].wrapping_add(0x204));
            c.set(7,32,c.r[6].wrapping_add(0x1c14));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x439558; continue;
            }
            0x439558 => {
            c.set(1,8,rm(m,c.r[5],8));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x740),32));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(3,32,c.r[7].wrapping_add(0x740));
            c.set(1,8,u32::from(c.cond(7)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x439575); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[5],8));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x439588); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x439591); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43959a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x439558; continue; }
            pc = 0x4395ac; continue;
            }
            0x4395ac => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4395dd; continue; }
            pc = 0x4395b8; continue;
            }
            0x4395b8 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x43960b; continue; }
            pc = 0x4395bb; continue;
            }
            0x4395bb => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2354),32));
            c.set(1,32,c.r[6].wrapping_add(0x2354));
            c.push(m,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4395cb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c14),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c14));
            c.push(m,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4395db); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43960b; continue;
            }
            0x4395dd => {
            c.set(7,32,c.r[6].wrapping_add(0x1c14));
            c.set(3,32,0x8u32);
            pc = 0x4395e8; continue;
            }
            0x4395e8 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x740),32));
            c.set(1,32,c.r[7].wrapping_add(0x740));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4395f9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x439602); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x4395e8; continue; }
            pc = 0x43960b; continue;
            }
            0x43960b => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            c.set(7,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x439621); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x69af64u32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43962e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x1f4),8));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x439643); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x439668);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
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
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x43f1dd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
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

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "barn_content_window_update_wares_page" => barn_content_window_update_wares_page(m, host, regs),
        "class22_update_scene_buffer_cells_and_animations" => class22_update_scene_buffer_cells_and_animations(m, host, regs),
        "debug_page_town_trade_balance_matrix" => debug_page_town_trade_balance_matrix(m, host, regs),
        "hanse_content_window_show" => hanse_content_window_show(m, host, regs),
        "multiplayer_screen_load_layout" => multiplayer_screen_load_layout(m, host, regs),
        "statistics_window_prepare_town_detail_page" => statistics_window_prepare_town_detail_page(m, host, regs),
        "multiplayer_screen_enter_client_lobby" => multiplayer_screen_enter_client_lobby(m, host, regs),
        "multiplayer_screen_draw_wait_overlay" => multiplayer_screen_draw_wait_overlay(m, host, regs),
        _ => return None,
    })
}
