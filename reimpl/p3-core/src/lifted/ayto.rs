//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn schedule_prep_alderman_missions(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5326e0;
    loop {
        match pc {
            0x5326e0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x4cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x5326f6);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m,0x8u32);
            c.push(m, 0x532707);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4ec5f4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x86u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x53271e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x18u32);
            c.set(2,32,c.r[4].wrapping_add(0x48));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x53272c);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x0u32); }
            pc = 0x532734; continue;
            }
            0x532734 => {
            { let a = c.r[4].wrapping_add(0x51); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x52); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x53); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x532bf4; continue; }
            pc = 0x532755; continue;
            }
            0x532755 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x532c3c)); continue;
            }
            0x53275c => {
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,0x1au32,32); }
            if c.cond(3) { pc = 0x532c1f; continue; }
            pc = 0x532766; continue;
            }
            0x532766 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2),16));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.add(a,0x19u32,32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,0xcu32,32); }
            if c.cond(6) { pc = 0x53279f; continue; }
            pc = 0x532785; continue;
            }
            0x532785 => {
            c.set(2,32,c.r[1].wrapping_add(0xfffffff3));
            c.set(0,32,0xaaaaaaabu32);
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x532793; continue;
            }
            0x532793 => {
            { let a = c.get(6,32); let r = c.add(a,0x16d00u32,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0xcu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x532793; continue; }
            pc = 0x53279f; continue;
            }
            0x53279f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x54));
            c.set(0,16,rm(m,c.r[1].wrapping_mul(2).wrapping_add(0x672d78),16));
            c.set(1,32,c.r[4].wrapping_add(0x58));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x5327c3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x532e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.r[4].wrapping_add(0x5a); wm(m,a,8,0xffu32); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672d50),8));
            { let a = c.r[4].wrapping_add(0x59); wm(m,a,8,c.get(1,8)); }
            pc = 0x532bf4; continue;
            }
            0x5327e0 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x55); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            pc = 0x5327f7; continue;
            }
            0x5327f7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,32,0xffu32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(3,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x5328ac; continue; }
            pc = 0x53280c; continue;
            }
            0x53280c => {
            c.set(5,32,rm(m,c.r[5].wrapping_add(0x68),32));
            c.set(6,32,c.r[5].wrapping_add(0x76c));
            pc = 0x532815; continue;
            }
            0x532815 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.and(a,0x8100u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x100u32,32); }
            if c.cond(5) { pc = 0x532843; continue; }
            pc = 0x532825; continue;
            }
            0x532825 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x18),8));
            { let a = rm(m,c.r[7].wrapping_add(0x672bf8),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x532839; continue; }
            pc = 0x532835; continue;
            }
            0x532835 => {
            c.set(3,32,c.get(0,32));
            pc = 0x532843; continue;
            }
            0x532839 => {
            { let a = rm(m,c.r[7].wrapping_add(0x672bf9),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x532843; continue; }
            pc = 0x532841; continue;
            }
            0x532841 => {
            c.set(2,32,c.get(0,32));
            pc = 0x532843; continue;
            }
            0x532843 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x9f8u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x532815; continue; }
            pc = 0x532852; continue;
            }
            0x532852 => {
            { let a = c.get(3,32); c.sub(a,0xffu32,32); }
            if c.cond(4) { pc = 0x5328a8; continue; }
            pc = 0x53285a; continue;
            }
            0x53285a => {
            { let a = c.get(2,32); c.sub(a,0xffu32,32); }
            if c.cond(4) { pc = 0x5328a8; continue; }
            pc = 0x532862; continue;
            }
            0x532862 => {
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2d4),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2d4),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x532888; continue; }
            pc = 0x532884; continue;
            }
            0x532884 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            pc = 0x53288c; continue;
            }
            0x532888 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            pc = 0x53288c; continue;
            }
            0x53288c => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); }
            if c.cond(2) { pc = 0x5328a8; continue; }
            pc = 0x532892; continue;
            }
            0x532892 => {
            { let a = c.r[4].wrapping_add(0x55); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x672bfa),8));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x56); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x5328a8; continue;
            }
            0x5328a8 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5328ac; continue;
            }
            0x5328ac => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x1cu32,32); }
            if c.cond(2) { pc = 0x5327f7; continue; }
            pc = 0x5328b8; continue;
            }
            0x5328b8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x532c1f; continue; }
            pc = 0x5328ce; continue;
            }
            0x5328ce => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x55),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x532c1f; continue; }
            pc = 0x5328e0; continue;
            }
            0x5328e0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x56),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,0xdu32,32); }
            if c.cond(2) { pc = 0x53290f; continue; }
            pc = 0x532907; continue;
            }
            0x532907 => {
            { let a = c.get(0,32); let r = c.add(a,0x16d00u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0xcu32,32); c.set(1,32,r); }
            pc = 0x53290f; continue;
            }
            0x53290f => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x4u32);
            c.set(2,16,rm(m,c.r[1].wrapping_mul(2).wrapping_add(0x672d78),16));
            c.push(m,0x0u32);
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x5f); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x60));
            c.push(m,c.get(0,32));
            c.push(m, 0x532935);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x532bf4; continue;
            }
            0x53293d => {
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,16,0xffffu32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x3e8u32); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x53294e; continue;
            }
            0x53294e => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x6dd7ac),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5329d0; continue; }
            pc = 0x532958; continue;
            }
            0x532958 => {
            c.set(6,32,rm(m,0x6dd894u32,32));
            c.set(7,32,0xffffu32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.and(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x16),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x5329b9; continue; }
            pc = 0x532975; continue;
            }
            0x532975 => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(5,16,rm(m,c.r[1].wrapping_add(0x42),16));
            { let a = c.get(5,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x5329a3; continue; }
            pc = 0x532990; continue;
            }
            0x532990 => {
            c.set(5,16,rm(m,c.r[1].wrapping_add(0x15d),8));
            { let a = c.get(5,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x5329a3; continue; }
            pc = 0x5329a1; continue;
            }
            0x5329a1 => {
            c.set(7,32,c.get(0,32));
            pc = 0x5329a3; continue;
            }
            0x5329a3 => {
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x120),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x532975; continue; }
            pc = 0x5329b5; continue;
            }
            0x5329b5 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5329b9; continue;
            }
            0x5329b9 => {
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); }
            if c.cond(6) { pc = 0x5329d0; continue; }
            pc = 0x5329bf; continue;
            }
            0x5329bf => {
            { let a = c.get(7,32); c.sub(a,0xffffu32,32); }
            if c.cond(4) { pc = 0x5329d0; continue; }
            pc = 0x5329c7; continue;
            }
            0x5329c7 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,16,c.get(7,16)); }
            pc = 0x5329d0; continue;
            }
            0x5329d0 => {
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x14u32,32); }
            if c.cond(2) { pc = 0x53294e; continue; }
            pc = 0x5329dc; continue;
            }
            0x5329dc => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x54),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x532c1f; continue; }
            pc = 0x5329ee; continue;
            }
            0x5329ee => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2),16));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.add(a,0x3u32,32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,0xdu32,32); }
            if c.cond(2) { pc = 0x532a15; continue; }
            pc = 0x532a0d; continue;
            }
            0x532a0d => {
            { let a = c.get(0,32); let r = c.add(a,0x16d00u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0xcu32,32); c.set(1,32,r); }
            pc = 0x532a15; continue;
            }
            0x532a15 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_mul(2).wrapping_add(0x672d78),16));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6dd7a0u32,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x42),16));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x56); wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x532a69);
            { let args = []; c.r[0] = host.call(m, 0x51a470, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(3) { pc = 0x532bf4; continue; }
            pc = 0x532a7e; continue;
            }
            0x532a7e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,8); let r = c.add(a,0x2u32,8); c.set(0,8,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            { let a = c.r[1].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            pc = 0x532bf4; continue;
            }
            0x532a99 => {
            c.set(6,32,0x2u32);
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0xffu32); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x532aa5; continue;
            }
            0x532aa5 => {
            c.set(1,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x532af0; continue; }
            pc = 0x532ab0; continue;
            }
            0x532ab0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xc),16));
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x532af0; continue; }
            pc = 0x532ac8; continue;
            }
            0x532ac8 => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x4),16));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x532ac8; continue; }
            pc = 0x532ae2; continue;
            }
            0x532ae2 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(6) { pc = 0x532af0; continue; }
            pc = 0x532aea; continue;
            }
            0x532aea => {
            c.set(6,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(3,8)); }
            pc = 0x532af0; continue;
            }
            0x532af0 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x5u32,32); }
            if c.cond(2) { pc = 0x532aa5; continue; }
            pc = 0x532af6; continue;
            }
            0x532af6 => {
            { let a = rm(m,c.r[4].wrapping_add(0x54),8); c.sub(a,0x5u32,8); }
            if c.cond(3) { pc = 0x532c1f; continue; }
            pc = 0x532b01; continue;
            }
            0x532b01 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2),16));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.add(a,0x7u32,32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,0xdu32,32); }
            if c.cond(2) { pc = 0x532b28; continue; }
            pc = 0x532b20; continue;
            }
            0x532b20 => {
            { let a = c.get(0,32); let r = c.add(a,0x16d00u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0xcu32,32); c.set(1,32,r); }
            pc = 0x532b28; continue;
            }
            0x532b28 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_mul(2).wrapping_add(0x672d78),16));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xe),8));
            { let a = c.r[4].wrapping_add(0x56); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x55); wm(m,a,8,c.get(2,8)); }
            pc = 0x532bf4; continue;
            }
            0x532b5c => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(3,8); let r = c.or(a,0xffu32,8); c.set(3,8,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(6,32,0x7d1u32);
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x56); wm(m,a,8,c.get(3,8)); }
            if c.cond(6) { pc = 0x532c1f; continue; }
            pc = 0x532b75; continue;
            }
            0x532b75 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x68),32));
            { let a = c.get(1,32); let r = c.add(a,0x2d4u32,32); c.set(1,32,r); }
            pc = 0x532b7e; continue;
            }
            0x532b7e => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x498),32));
            { let a = c.get(6,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x532b97; continue; }
            pc = 0x532b89; continue;
            }
            0x532b89 => {
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x532b97; continue; }
            pc = 0x532b8f; continue;
            }
            0x532b8f => {
            c.set(3,8,c.get(0,8));
            c.set(6,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x56); wm(m,a,8,c.get(3,8)); }
            pc = 0x532b97; continue;
            }
            0x532b97 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x9f8u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x532b7e; continue; }
            pc = 0x532ba2; continue;
            }
            0x532ba2 => {
            { let a = c.get(3,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x532c1f; continue; }
            pc = 0x532ba7; continue;
            }
            0x532ba7 => {
            { let a = c.get(6,32); let r = c.add(a,0x7d0u32,32); c.set(6,32,r); }
            c.set(0,32,0x10624dd3u32);
            { let b = c.get(6,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,8,rm(m,0x6de4a1u32,8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[1].wrapping_mul(2).wrapping_add(0x672d78),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6de4a2u32,16));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(2,32)); }
            pc = 0x532bf4; continue;
            }
            0x532bf4 => {
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m,0x10u32);
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x85u32); }
            c.push(m, 0x532c0d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x532c1f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x532c1f; continue;
            }
            0x532c1f => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x50),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x532734; continue; }
            pc = 0x532c31; continue;
            }
            0x532c31 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x4cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_get_task(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d8db0;
    loop {
        match pc {
            0x4d8db0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x4d8dc3; continue; }
            pc = 0x4d8dbe; continue;
            }
            0x4d8dbe => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4d8dc3 => {
            c.set(1,32,rm(m,c.r[1],32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_collect_open_alderman_missions(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ec5f4;
    loop {
        match pc {
            0x4ec5f4 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.push(m,c.get(6,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            if c.cond(6) { pc = 0x4ec679; continue; }
            pc = 0x4ec60f; continue;
            }
            0x4ec60f => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4ec667; continue; }
            pc = 0x4ec61d; continue;
            }
            0x4ec61d => {
            c.set(6,32,rm(m,c.r[1],32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x6),16); c.sub(a,0x32u32,16); }
            if c.cond(5) { pc = 0x4ec654; continue; }
            pc = 0x4ec62e; continue;
            }
            0x4ec62e => {
            c.set(3,16,rm(m,c.r[6].wrapping_add(0xd),8));
            { let a = c.get(3,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x4ec654; continue; }
            pc = 0x4ec63c; continue;
            }
            0x4ec63c => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.sub(a,0x5u32,8); }
            if c.cond(3) { pc = 0x4ec654; continue; }
            pc = 0x4ec642; continue;
            }
            0x4ec642 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[7]; wm(m,a,16,c.get(0,16)); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            if c.cond(4) { pc = 0x4ec679; continue; }
            pc = 0x4ec654; continue;
            }
            0x4ec654 => {
            c.set(6,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4ec61d; continue; }
            pc = 0x4ec667; continue;
            }
            0x4ec667 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x4ec679; continue; }
            pc = 0x4ec66b; continue;
            }
            0x4ec66b => {
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.stos(m,32,true);
            { let a = c.get(1,32); let r = c.adc(a,c.get(1,32),32); c.set(1,32,r); }
            c.stos(m,16,true);
            pc = 0x4ec679; continue;
            }
            0x4ec679 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
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

pub fn ship_get_pirate_band_index(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51a470;
    loop {
        match pc {
            0x51a470 => {
            c.push(m,c.get(1,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x158),8));
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            if c.cond(3) { pc = 0x51a48e; continue; }
            pc = 0x51a47b; continue;
            }
            0x51a47b => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x51a4b5; continue; }
            pc = 0x51a48e; continue;
            }
            0x51a48e => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.r[4]; wm(m,a,8,c.get(0,8)); }
            pc = 0x51a494; continue;
            }
            0x51a494 => {
            c.set(2,32,rm(m,c.r[4],32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x51a4b5; continue; }
            pc = 0x51a4a9; continue;
            }
            0x51a4a9 => {
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            { let a = c.r[4]; wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x51a494; continue; }
            pc = 0x51a4b3; continue;
            }
            0x51a4b3 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x51a4b5; continue;
            }
            0x51a4b5 => {
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn determine_new_settlement(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x532e30;
    loop {
        match pc {
            0x532e30 => {
            { let a = c.get(4,32); let r = c.sub(a,0xe0u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(0,32,0x7fffffffu32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x50u32);
            c.set(0,32,c.r[4].wrapping_add(0xa4));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0xbu32); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x3u32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x532e8b);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x50u32);
            c.set(1,32,c.r[4].wrapping_add(0x60));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x532e98);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x532f84; continue; }
            pc = 0x532ea6; continue;
            }
            0x532ea6 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x68),32));
            c.set(7,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x532ec8; continue;
            }
            0x532ec6 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x532ec8; continue;
            }
            0x532ec8 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x532ecc; continue;
            }
            0x532ecc => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x66c),32));
            c.set(3,32,rm(m,c.r[0],32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(c.r[1]).wrapping_add(0xa0),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(c.r[1]).wrapping_add(0xa0); wm(m,a,32,c.get(6,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x308),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x5c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(c.r[1]).wrapping_add(0x50),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,0x50u32,32); }
            { let a = c.r[4].wrapping_add(c.r[1]).wrapping_add(0x4c); wm(m,a,32,c.get(6,32)); }
            if c.cond(12) { pc = 0x532ecc; continue; }
            pc = 0x532f06; continue;
            }
            0x532f06 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x784),16));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x532f57; continue; }
            pc = 0x532f17; continue;
            }
            0x532f17 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x74),32));
            pc = 0x532f1a; continue;
            }
            0x532f1a => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(0x64));
            pc = 0x532f2e; continue;
            }
            0x532f2e => {
            c.set(3,32,rm(m,c.r[1].wrapping_add(0xffffffa0),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0xa0),32); let r = c.add(a,c.get(3,32),32); { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0xa0); wm(m,a,32,r); } }
            c.set(3,32,rm(m,c.r[1].wrapping_add(0xfffffffc),32));
            { let a = rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x50),32); let r = c.add(a,c.get(3,32),32); { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x50); wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x50u32,32); }
            if c.cond(12) { pc = 0x532f2e; continue; }
            pc = 0x532f4a; continue;
            }
            0x532f4a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x2ca),16));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x532f1a; continue; }
            pc = 0x532f57; continue;
            }
            0x532f57 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let r = c.add(a,0x9f8u32,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x9f8u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x532ec6; continue; }
            pc = 0x532f82; continue;
            }
            0x532f82 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x532f84; continue;
            }
            0x532f84 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x532f86; continue;
            }
            0x532f86 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x50),32));
            { let a = c.get(1,32); c.sub(a,0x400u32,32); }
            if c.cond(12) { pc = 0x532fcc; continue; }
            pc = 0x532f92; continue;
            }
            0x532f92 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0xa0),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,0x3u32);
            pc = 0x532fa4; continue;
            }
            0x532fa4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x18),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x532fb3; continue; }
            pc = 0x532fac; continue;
            }
            0x532fac => {
            c.set(1,32,0x1u32);
            pc = 0x532fc7; continue;
            }
            0x532fb3 => {
            { let a = c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            pc = 0x532fc7; continue;
            }
            0x532fc7 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(15) { pc = 0x532fa4; continue; }
            pc = 0x532fcc; continue;
            }
            0x532fcc => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x532f86; continue; }
            pc = 0x532fd2; continue;
            }
            0x532fd2 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xf4),32));
            c.set(5,32,0x3u32);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(3,32)); }
            pc = 0x532fe2; continue;
            }
            0x532fe2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x34),32));
            c.set(0,32,0x1u32);
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x672c88),8));
            { let a = c.get(1,32); let r = c.sub(a,0x3u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.and(a,0xffffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x533007; continue; }
            pc = 0x533001; continue;
            }
            0x533001 => {
            { let a = c.get(1,32); let r = c.or(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            pc = 0x53300c; continue;
            }
            0x533007 => {
            c.set(5,32,0x4u32);
            pc = 0x53300c; continue;
            }
            0x53300c => {
            { let a = c.get(2,32); c.sub(a,0xau32,32); }
            if c.cond(5) { pc = 0x533017; continue; }
            pc = 0x533011; continue;
            }
            0x533011 => {
            { let a = rm(m,c.r[6],32); let r = c.or(a,0x40020u32,32); { let a = c.r[6]; wm(m,a,32,r); } }
            pc = 0x533017; continue;
            }
            0x533017 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x532fe2; continue; }
            pc = 0x53301c; continue;
            }
            0x53301c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); c.sub(a,0x13u32,32); }
            if c.cond(7) { pc = 0x533062; continue; }
            pc = 0x533025; continue;
            }
            0x533025 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x5331f0),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x5331d8)); continue;
            }
            0x533034 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x53306a; continue;
            }
            0x53303a => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x1u32); }
            pc = 0x53306a; continue;
            }
            0x533044 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x2u32); }
            pc = 0x53306a; continue;
            }
            0x53304e => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x3u32); }
            pc = 0x53306a; continue;
            }
            0x533058 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x4u32); }
            pc = 0x53306a; continue;
            }
            0x533062 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x5u32); }
            pc = 0x53306a; continue;
            }
            0x53306a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf8),32));
            c.push(m,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[2]; wm(m,a,8,0xffu32); }
            c.push(m, 0x533082);
            { let args = []; c.r[0] = host.call(m, 0x515a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,8,c.get(0,8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            if c.cond(12) { pc = 0x5331ca; continue; }
            pc = 0x533093; continue;
            }
            0x533093 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x533141; continue; }
            pc = 0x53309f; continue;
            }
            0x53309f => {
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(6,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x6ddbc0),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5331af; continue; }
            pc = 0x5330c3; continue;
            }
            0x5330c3 => {
            c.push(m,0x18u32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,0x7fu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x5330d1);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x10),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            if c.cond(6) { pc = 0x53318c; continue; }
            pc = 0x5330e7; continue;
            }
            0x5330e7 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x6ddbb0),32));
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x6ddbb4),32));
            c.set(1,32,0xffffffe8u32);
            c.set(7,32,c.r[0].wrapping_add(0x18));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            pc = 0x533101; continue;
            }
            0x533101 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.get(5,32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x6ddbb0),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[1].wrapping_add(0x6ddbb4),32),32); c.set(0,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,0x4u32);
            pc = 0x53312f; continue;
            }
            0x53312f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1c),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x533173; continue; }
            pc = 0x533137; continue;
            }
            0x533137 => {
            { let a = c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x533176; continue;
            }
            0x533141 => {
            c.set(2,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(6,32,c.r[3].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6ddbc0),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5330c3; continue; }
            pc = 0x53315e; continue;
            }
            0x53315e => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            pc = 0x5330c3; continue;
            }
            0x533173 => {
            { let a = c.get(1,32); let r = c.or(a,0xffffffffu32,32); c.set(1,32,r); }
            pc = 0x533176; continue;
            }
            0x533176 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(9) { pc = 0x53312f; continue; }
            pc = 0x533179; continue;
            }
            0x533179 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x533101; continue; }
            pc = 0x53318c; continue;
            }
            0x53318c => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x533190; continue;
            }
            0x533190 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x5u32,32); }
            if c.cond(12) { pc = 0x533190; continue; }
            pc = 0x53319c; continue;
            }
            0x53319c => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); }
            if c.cond(14) { pc = 0x5331af; continue; }
            pc = 0x5331a2; continue;
            }
            0x5331a2 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf8),32));
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            pc = 0x5331af; continue;
            }
            0x5331af => {
            c.set(6,32,c.get(3,32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x5331b8);
            { let args = []; c.r[0] = host.call(m, 0x515a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x533093; continue; }
            pc = 0x5331ca; continue;
            }
            0x5331ca => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xe0u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn operation_packet_clear(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x543da0;
    loop {
        match pc {
            0x543da0 => {
            { let a = c.r[1].wrapping_add(0xe); wm(m,a,16,0x0u32); }
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,0xffffu32); }
            c.set(0,32,c.r[1].wrapping_add(0x10));
            c.set(1,32,0x34u32);
            pc = 0x543db4; continue;
            }
            0x543db4 => {
            { let a = c.r[0]; wm(m,a,32,0xffffffffu32); }
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x543db4; continue; }
            pc = 0x543dc0; continue;
            }
            0x543dc0 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "schedule_prep_alderman_missions" => schedule_prep_alderman_missions(m, host, regs),
        "scheduled_tasks_get_task" => scheduled_tasks_get_task(m, host, regs),
        "scheduled_tasks_collect_open_alderman_missions" => scheduled_tasks_collect_open_alderman_missions(m, host, regs),
        "ship_get_pirate_band_index" => ship_get_pirate_band_index(m, host, regs),
        "determine_new_settlement" => determine_new_settlement(m, host, regs),
        "operation_packet_clear" => operation_packet_clear(m, host, regs),
        _ => return None,
    })
}
