//! Generado por `reimpl/tools/ui_rt_grp.py sail` con `tools/lift.py` (no editar a mano): grupo `sail`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_SAIL: [(u32, super::core_table::CoreFn); 1] = [
    (0x45af00, f_0045af00),
];
pub fn f_0045af00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45af00;
    loop {
        match pc {
            0x45af00 => {
            { let a = c.get(4,32); let r = c.sub(a,0x2cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b978; continue; }
            pc = 0x45af15; continue;
            }
            0x45af15 => {
            { let a = rm(m,c.r[6].wrapping_add(0x292c),32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b978; continue; }
            pc = 0x45af21; continue;
            }
            0x45af21 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x45afcf; continue; }
            pc = 0x45af2f; continue;
            }
            0x45af2f => {
            c.set(0,32,rm(m,0x6cbb10u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x5a4),32));
            c.set(0,32,rm(m,0x66bdc0u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45afb9; continue; }
            pc = 0x45af43; continue;
            }
            0x45af43 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,rm(m,c.r[2],32));
            c.set(3,16,rm(m,c.r[0],16));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.set(7,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x45af6b);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x68u32); }
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x45af9e; continue; }
            pc = 0x45af80; continue;
            }
            0x45af80 => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            pc = 0x45afa2; continue;
            }
            0x45af9e => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            pc = 0x45afa2; continue;
            }
            0x45afa2 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x1u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x45afb9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45afb9; continue;
            }
            0x45afb9 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b8a8; continue; }
            pc = 0x45afc9; continue;
            }
            0x45afc9 => {
            c.push(m,c.get(0,32));
            pc = 0x45b8a0; continue;
            }
            0x45afcf => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,0xcu32,32); }
            if c.cond(7) { pc = 0x45b978; continue; }
            pc = 0x45afe4; continue;
            }
            0x45afe4 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x45b980)); continue;
            }
            0x45afeb => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x5a4),32));
            c.set(1,32,rm(m,0x66bdc0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x45b0f6; continue; }
            pc = 0x45b005; continue;
            }
            0x45b005 => {
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x10),32));
            c.set(0,32,c.r[2].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.push(m, 0x45b018);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x446d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x45b023);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x12),16); c.sub(a,c.get(3,16),16); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x10),16); c.sub(a,c.get(3,16),16); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x45b0a5; continue; }
            pc = 0x45b044; continue;
            }
            0x45b044 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x4),16); c.sub(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x45b0f4; continue; }
            pc = 0x45b050; continue;
            }
            0x45b050 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,0x28u32,32); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(2)),16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            if c.cond(13) { pc = 0x45b06b; continue; }
            pc = 0x45b065; continue;
            }
            0x45b065 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            pc = 0x45b083; continue;
            }
            0x45b06b => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffu32); }
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x10),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            pc = 0x45b083; continue;
            }
            0x45b083 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x45b092);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x45b050; continue; }
            pc = 0x45b0a3; continue;
            }
            0x45b0a3 => {
            pc = 0x45b0f4; continue;
            }
            0x45b0a5 => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x45b0f4; continue; }
            pc = 0x45b0ac; continue;
            }
            0x45b0ac => {
            c.set(5,32,0xbu32);
            pc = 0x45b0b1; continue;
            }
            0x45b0b1 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x45b0ba);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(2)),16));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x45b0e3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x45b0b1; continue; }
            pc = 0x45b0f4; continue;
            }
            0x45b0f4 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x45b0f6; continue;
            }
            0x45b0f6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x45b10a);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(0,32,rm(m,c.r[0],32));
            c.set(2,32,c.r[7].wrapping_add(c.r[7]));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x45b125);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b13d; continue; }
            pc = 0x45b134; continue;
            }
            0x45b134 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b13a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b13d; continue;
            }
            0x45b13d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[2]; wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b15b; continue; }
            pc = 0x45b152; continue;
            }
            0x45b152 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b158);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b15b; continue;
            }
            0x45b15b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x45b1cc; continue; }
            pc = 0x45b177; continue;
            }
            0x45b177 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[1].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x45b18e);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(0,32,rm(m,0x66bdc0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x5a4),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45b1b4; continue; }
            pc = 0x45b1a6; continue;
            }
            0x45b1a6 => {
            c.push(m,0xffu32);
            pc = 0x45b1ab; continue;
            }
            0x45b1ab => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45b1b4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x454b80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x45b1b4; continue;
            }
            0x45b1b4 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45b1bb);
            { let args = []; c.r[0] = host.call(m, 0x456f80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m, 0x45b1c1);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x45b1cc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b1df; continue; }
            pc = 0x45b1d6; continue;
            }
            0x45b1d6 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b1dc);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b1df; continue;
            }
            0x45b1df => {
            { let a = c.r[6].wrapping_add(0x2880); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,0x6cbb10u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x5a4),32));
            c.set(0,32,rm(m,0x66bdc0u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45b84d; continue; }
            pc = 0x45b1fd; continue;
            }
            0x45b1fd => {
            c.push(m,0xffu32);
            pc = 0x45b844; continue;
            }
            0x45b207 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(5,32)); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x45b261; continue; }
            pc = 0x45b219; continue;
            }
            0x45b219 => {
            c.set(7,32,rm(m,c.r[2],32));
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            pc = 0x45b221; continue;
            }
            0x45b221 => {
            c.set(1,16,rm(m,c.r[7],16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(c.r[5]).wrapping_add(0x8),16));
            c.set(5,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16),16); }
            if c.cond(4) { pc = 0x45b25f; continue; }
            pc = 0x45b24b; continue;
            }
            0x45b24b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x45b221; continue; }
            pc = 0x45b25f; continue;
            }
            0x45b25f => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x45b261; continue;
            }
            0x45b261 => {
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16); c.sub(a,0x4u32,16); }
            if c.cond(5) { pc = 0x45b8f1; continue; }
            pc = 0x45b273; continue;
            }
            0x45b273 => {
            c.set(0,32,rm(m,0x6cbb10u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x5a4),32));
            c.set(0,32,rm(m,0x66bdc0u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45b382; continue; }
            pc = 0x45b28b; continue;
            }
            0x45b28b => {
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            { let a = c.get(2,32); let r = c.add(a,0x10u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[2],32));
            c.push(m,c.get(2,32));
            c.push(m, 0x45b29d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x446d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(5,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,0x76u32,16); }
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,16); c.sub(a,0x44u32,16); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x45b302; continue; }
            pc = 0x45b2bf; continue;
            }
            0x45b2bf => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x12),16));
            { let a = c.get(0,16); c.sub(a,0x5b8u32,16); }
            if c.cond(14) { pc = 0x45b302; continue; }
            pc = 0x45b2c9; continue;
            }
            0x45b2c9 => {
            { let a = c.get(0,16); c.sub(a,0x695u32,16); }
            if c.cond(13) { pc = 0x45b302; continue; }
            pc = 0x45b2cf; continue;
            }
            0x45b2cf => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x8),16));
            c.set(0,32,rm(m,0x6e5544u32,32));
            { let a = c.r[0].wrapping_add(0x94); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,0x6e5544u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x45b300); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x45b380; continue;
            }
            0x45b302 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x45b30b);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x10),16); c.sub(a,c.get(3,16),16); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x12),16); c.sub(a,c.get(3,16),16); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x45b35d; continue; }
            pc = 0x45b32c; continue;
            }
            0x45b32c => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(5,32); c.sub(a,0x28u32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x45b343; continue; }
            pc = 0x45b33d; continue;
            }
            0x45b33d => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            pc = 0x45b371; continue;
            }
            0x45b343 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffu32); }
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x10),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            pc = 0x45b371; continue;
            }
            0x45b35d => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xbu32); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x45b371; continue;
            }
            0x45b371 => {
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x45b380);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45b380; continue;
            }
            0x45b380 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x45b382; continue;
            }
            0x45b382 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b86d; continue; }
            pc = 0x45b392; continue;
            }
            0x45b392 => {
            c.push(m,c.get(0,32));
            pc = 0x45b865; continue;
            }
            0x45b398 => {
            c.set(0,32,rm(m,0x6cbb10u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x5a4),32));
            c.set(0,32,rm(m,0x66bdc0u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45b573; continue; }
            pc = 0x45b3b0; continue;
            }
            0x45b3b0 => {
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.set(0,32,c.r[2].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            c.push(m,c.get(2,32));
            c.push(m, 0x45b3c2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x446d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45b3cf);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x12),16); c.sub(a,c.get(3,16),16); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x10),16); c.sub(a,c.get(3,16),16); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x45b523; continue; }
            pc = 0x45b3f2; continue;
            }
            0x45b3f2 => {
            { let a = rm(m,c.r[7].wrapping_add(0x4),16); c.sub(a,0x0u32,16); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x0u32); }
            if c.cond(6) { pc = 0x45b573; continue; }
            pc = 0x45b405; continue;
            }
            0x45b405 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            c.set(5,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)),16));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xe),8));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x45b475; continue; }
            pc = 0x45b434; continue;
            }
            0x45b434 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xa),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x45b475; continue; }
            pc = 0x45b452; continue;
            }
            0x45b452 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            { let a = c.get(1,8); c.sub(a,c.get(2,8),8); }
            if c.cond(6) { pc = 0x45b468; continue; }
            pc = 0x45b466; continue;
            }
            0x45b466 => {
            c.set(2,8,c.get(1,8));
            pc = 0x45b468; continue;
            }
            0x45b468 => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x45b452; continue; }
            pc = 0x45b475; continue;
            }
            0x45b475 => {
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x10),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(7,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,16,c.get(2,8));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45b4a2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x45bcf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x45b4b0; continue; }
            pc = 0x45b4a6; continue;
            }
            0x45b4a6 => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45b4ae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45b9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45b504; continue;
            }
            0x45b4b0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(2)),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); c.sub(a,0x28u32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x45b4dd; continue; }
            pc = 0x45b4d7; continue;
            }
            0x45b4d7 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x45b4f5; continue;
            }
            0x45b4dd => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffu32); }
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x10),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            pc = 0x45b4f5; continue;
            }
            0x45b4f5 => {
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x45b504);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x45b504; continue;
            }
            0x45b504 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x45b405; continue; }
            pc = 0x45b521; continue;
            }
            0x45b521 => {
            pc = 0x45b573; continue;
            }
            0x45b523 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x4),16); c.sub(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x45b573; continue; }
            pc = 0x45b52b; continue;
            }
            0x45b52b => {
            c.set(7,32,0xbu32);
            pc = 0x45b530; continue;
            }
            0x45b530 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x45b539);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2],32));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(2,32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[5].wrapping_mul(2)),16));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x45b562);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x45b530; continue; }
            pc = 0x45b573; continue;
            }
            0x45b573 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x45b587);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,c.r[7].wrapping_add(c.r[7]));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x45b5a2);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b5bc; continue; }
            pc = 0x45b5b3; continue;
            }
            0x45b5b3 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b5b9);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b5bc; continue;
            }
            0x45b5bc => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b5da; continue; }
            pc = 0x45b5d1; continue;
            }
            0x45b5d1 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b5d7);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b5da; continue;
            }
            0x45b5da => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(15) { pc = 0x45b177; continue; }
            pc = 0x45b5fa; continue;
            }
            0x45b5fa => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b60d; continue; }
            pc = 0x45b604; continue;
            }
            0x45b604 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b60a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b60d; continue;
            }
            0x45b60d => {
            { let a = c.r[6].wrapping_add(0x2880); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,0x6cbb10u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x5a4),32));
            c.set(0,32,rm(m,0x66bdc0u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45b84d; continue; }
            pc = 0x45b62b; continue;
            }
            0x45b62b => {
            c.push(m,0xffu32);
            pc = 0x45b844; continue;
            }
            0x45b635 => {
            c.set(0,32,rm(m,0x6cbb10u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x5a4),32));
            c.set(0,32,rm(m,0x66bdc0u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45b744; continue; }
            pc = 0x45b64d; continue;
            }
            0x45b64d => {
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.set(0,32,c.r[2].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            c.push(m,c.get(2,32));
            c.push(m, 0x45b65f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x446d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x45b66e);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x12),16); c.sub(a,c.get(3,16),16); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x10),16); c.sub(a,c.get(3,16),16); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x45b6f3; continue; }
            pc = 0x45b68f; continue;
            }
            0x45b68f => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),16); c.sub(a,c.get(7,16),16); }
            if c.cond(6) { pc = 0x45b750; continue; }
            pc = 0x45b699; continue;
            }
            0x45b699 => {
            c.set(3,32,0xffu32);
            pc = 0x45b69e; continue;
            }
            0x45b69e => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,0x28u32,32); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(2)),16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x45b6bd; continue; }
            pc = 0x45b6b7; continue;
            }
            0x45b6b7 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            pc = 0x45b6d1; continue;
            }
            0x45b6bd => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x10),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            pc = 0x45b6d1; continue;
            }
            0x45b6d1 => {
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x45b6e0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x45b69e; continue; }
            pc = 0x45b6f1; continue;
            }
            0x45b6f1 => {
            pc = 0x45b750; continue;
            }
            0x45b6f3 => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x45b750; continue; }
            pc = 0x45b6fa; continue;
            }
            0x45b6fa => {
            c.set(5,32,0xbu32);
            pc = 0x45b6ff; continue;
            }
            0x45b6ff => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x45b708);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2],32));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(2,32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)),16));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x45b731);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x45b6ff; continue; }
            pc = 0x45b742; continue;
            }
            0x45b742 => {
            pc = 0x45b74c; continue;
            }
            0x45b744 => {
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0xffu32); }
            pc = 0x45b74c; continue;
            }
            0x45b74c => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x40),32));
            pc = 0x45b750; continue;
            }
            0x45b750 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x45b764);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,c.r[7].wrapping_add(c.r[7]));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x45b77f);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x45b797; continue; }
            pc = 0x45b78e; continue;
            }
            0x45b78e => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b794);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b797; continue;
            }
            0x45b797 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[1]; wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x45b7b9; continue; }
            pc = 0x45b7b0; continue;
            }
            0x45b7b0 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b7b6);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b7b9; continue;
            }
            0x45b7b9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x45b812; continue; }
            pc = 0x45b7d9; continue;
            }
            0x45b7d9 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[1].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x45b7f0);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(0,32,rm(m,0x66bdc0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x5a4),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45b1b4; continue; }
            pc = 0x45b80c; continue;
            }
            0x45b80c => {
            c.push(m,c.get(5,32));
            pc = 0x45b1ab; continue;
            }
            0x45b812 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x45b825; continue; }
            pc = 0x45b81c; continue;
            }
            0x45b81c => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b822);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b825; continue;
            }
            0x45b825 => {
            { let a = c.r[6].wrapping_add(0x2880); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,0x6cbb10u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x5a4),32));
            c.set(0,32,rm(m,0x66bdc0u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45b84d; continue; }
            pc = 0x45b843; continue;
            }
            0x45b843 => {
            c.push(m,c.get(5,32));
            pc = 0x45b844; continue;
            }
            0x45b844 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45b84d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x454b80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x45b84d; continue;
            }
            0x45b84d => {
            c.push(m,c.get(3,32));
            c.push(m, 0x45b853);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x45b85e => {
            c.set(2,32,rm(m,c.r[2],32));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b86d; continue; }
            pc = 0x45b864; continue;
            }
            0x45b864 => {
            c.push(m,c.get(2,32));
            pc = 0x45b865; continue;
            }
            0x45b865 => {
            c.push(m, 0x45b86a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b86d; continue;
            }
            0x45b86d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[2]; wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b88b; continue; }
            pc = 0x45b882; continue;
            }
            0x45b882 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b888);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b88b; continue;
            }
            0x45b88b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            pc = 0x45b927; continue;
            }
            0x45b899 => {
            c.set(2,32,rm(m,c.r[2],32));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b8a8; continue; }
            pc = 0x45b89f; continue;
            }
            0x45b89f => {
            c.push(m,c.get(2,32));
            pc = 0x45b8a0; continue;
            }
            0x45b8a0 => {
            c.push(m, 0x45b8a5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b8a8; continue;
            }
            0x45b8a8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b8c6; continue; }
            pc = 0x45b8bd; continue;
            }
            0x45b8bd => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b8c3);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b8c6; continue;
            }
            0x45b8c6 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x45b959; continue; }
            pc = 0x45b8da; continue;
            }
            0x45b8da => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_mul(4).wrapping_add(0xffffffec));
            c.set(0,32,c.r[1].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x45b947; continue;
            }
            0x45b8f1 => {
            c.set(2,32,rm(m,c.r[2],32));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b900; continue; }
            pc = 0x45b8f7; continue;
            }
            0x45b8f7 => {
            c.push(m,c.get(2,32));
            c.push(m, 0x45b8fd);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b900; continue;
            }
            0x45b900 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b91e; continue; }
            pc = 0x45b915; continue;
            }
            0x45b915 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b91b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b91e; continue;
            }
            0x45b91e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            pc = 0x45b927; continue;
            }
            0x45b927 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x292c),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x45b959; continue; }
            pc = 0x45b932; continue;
            }
            0x45b932 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[2].wrapping_mul(4).wrapping_add(0xffffffec));
            c.set(2,32,c.r[1].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            pc = 0x45b947; continue;
            }
            0x45b947 => {
            c.push(m,c.get(1,32));
            c.push(m, 0x45b94d);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m, 0x45b957);
            { let args = []; c.r[0] = host.call(m, 0x456f80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x45b972; continue;
            }
            0x45b959 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x45b96c; continue; }
            pc = 0x45b963; continue;
            }
            0x45b963 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x45b969);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x45b96c; continue;
            }
            0x45b96c => {
            { let a = c.r[6].wrapping_add(0x2880); wm(m,a,32,c.get(5,32)); }
            pc = 0x45b972; continue;
            }
            0x45b972 => {
            { let a = rm(m,c.r[6].wrapping_add(0x292c),32); let r = c.dec(a,32); { let a = c.r[6].wrapping_add(0x292c); wm(m,a,32,r); } }
            pc = 0x45b978; continue;
            }
            0x45b978 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
