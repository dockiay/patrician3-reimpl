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

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "barn_content_window_update_wares_page" => barn_content_window_update_wares_page(m, host, regs),
        _ => return None,
    })
}
