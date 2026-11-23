//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn merchant_debug_dialog_show_selected_ship(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x41f510;
    loop {
        match pc {
            0x41f510 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m,0x65c5b5u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x1a4),32));
            c.set(6,32,rm(m,0x66a4a0u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(5,32));
            c.push(m,0x188u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(5,32)); }
            { let target = c.get(6,32); c.push(m, 0x41f559); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x168),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(5,32));
            c.push(m,0x184u32);
            c.push(m,c.get(2,32));
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            { let target = c.get(6,32); c.push(m, 0x41f56f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x41f945; continue; }
            pc = 0x41f577; continue;
            }
            0x41f577 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x1a4),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m,0x199u32);
            c.push(m,c.get(0,32));
            { let target = c.get(6,32); c.push(m, 0x41f587); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x41f945; continue; }
            pc = 0x41f59b; continue;
            }
            0x41f59b => {
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x41f5c8; continue; }
            pc = 0x41f5b8; continue;
            }
            0x41f5b8 => {
            c.push(m,0x695d80u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x41f5c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x41f5e2; continue;
            }
            0x41f5c8 => {
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,0x695d84u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41f5df);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x41f5e2; continue;
            }
            0x41f5e2 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0x1d8));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x134),16));
            c.push(m,c.get(1,32));
            c.push(m,0x695d88u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41f5fd);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x38),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x41f64e; continue; }
            pc = 0x41f60f; continue;
            }
            0x41f60f => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x13b),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x41f64e; continue; }
            pc = 0x41f61b; continue;
            }
            0x41f61b => {
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x37),8));
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[7].wrapping_add(0x1c8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.push(m,0x695da4u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41f649);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x41f66f; continue;
            }
            0x41f64e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0x1c8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x37),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.push(m,0x695d94u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41f66c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x41f66f; continue;
            }
            0x41f66f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0x1f0));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x13a),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.push(m,0x695db0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41f690);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[7].wrapping_add(0x1dc));
            c.push(m,0x695dc0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41f6b6);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x136),8));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x41f6ca; continue; }
            pc = 0x41f6c3; continue;
            }
            0x41f6c3 => {
            c.push(m,0x695dd8u32);
            pc = 0x41f6f9; continue;
            }
            0x41f6ca => {
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x41f6d5; continue; }
            pc = 0x41f6ce; continue;
            }
            0x41f6ce => {
            c.push(m,0x695de8u32);
            pc = 0x41f6f9; continue;
            }
            0x41f6d5 => {
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x41f6f4; continue; }
            pc = 0x41f6d9; continue;
            }
            0x41f6d9 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x120),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.push(m,0x695dfcu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41f6ef);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x41f702; continue;
            }
            0x41f6f4 => {
            c.push(m,0x695e10u32);
            pc = 0x41f6f9; continue;
            }
            0x41f6f9 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x41f702);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x41f702; continue;
            }
            0x41f702 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x136),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[7].wrapping_add(0x1e0));
            c.push(m,0x695e28u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41f721);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x41f77b; continue; }
            pc = 0x41f731; continue;
            }
            0x41f731 => {
            c.set(1,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x3),8));
            c.set(3,32,c.r[0].wrapping_add(c.r[1]));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x41f753);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x2),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41f764);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[7].wrapping_add(0x1e4));
            c.push(m,0x695e30u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x41f776);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x41f786; continue;
            }
            0x41f77b => {
            c.set(1,32,c.r[7].wrapping_add(0x1e4));
            c.push(m, 0x41f786);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x41f786; continue;
            }
            0x41f786 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x130),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x41f7ca; continue; }
            pc = 0x41f792; continue;
            }
            0x41f792 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0x1e8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.fild32(m,c.r[4].wrapping_add(0x20));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[6].wrapping_add(0x12c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x28),4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x695e38u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41f7c5);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x41f7da; continue;
            }
            0x41f7ca => {
            c.push(m,0x695e58u32);
            c.set(1,32,c.r[7].wrapping_add(0x1e8));
            c.push(m, 0x41f7da);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x41f7da; continue;
            }
            0x41f7da => {
            c.set(3,32,c.r[6].wrapping_add(0x54));
            pc = 0x41f7dd; continue;
            }
            0x41f7dd => {
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x41f87a; continue; }
            pc = 0x41f7eb; continue;
            }
            0x41f7eb => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            c.fild32(m,c.r[4].wrapping_add(0x1c));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x41f821; continue; }
            pc = 0x41f7f9; continue;
            }
            0x41f7f9 => {
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.set(0,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,0x66bc60u32,4); c.fop('*',v); }
            c.push(m,c.get(0,32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x695e6cu32);
            c.push(m,c.get(1,32));
            pc = 0x41f847; continue;
            }
            0x41f821 => {
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.set(2,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,0x66bc5cu32,4); c.fop('*',v); }
            c.push(m,c.get(2,32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x695e7cu32);
            c.push(m,c.get(0,32));
            pc = 0x41f847; continue;
            }
            0x41f847 => {
            c.push(m, 0x41f84c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fild32(m,c.r[3]);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x168),32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('+',v); }
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x180u32);
            c.push(m,c.get(2,32));
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            { let target = rm(m,0x66a4a0u32,32); c.push(m, 0x41f87a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x41f87a; continue;
            }
            0x41f87a => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x41f7dd; continue; }
            pc = 0x41f887; continue;
            }
            0x41f887 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x40),16));
            c.push(m,c.get(0,32));
            c.push(m,0x695e8cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x41f89d);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x168),32));
            c.set(3,32,rm(m,0x66a4a0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x180u32);
            c.push(m,c.get(0,32));
            { let target = c.get(3,32); c.push(m, 0x41f8bb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x66c1e8u32,4); c.fop('*',v); }
            c.fild32(m,c.r[6].wrapping_add(0x10));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            { let v = c.ld(m,c.r[4].wrapping_add(0x24),4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,0x695e9cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x41f8e5);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x168),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x181u32);
            c.push(m,c.get(0,32));
            { let target = c.get(3,32); c.push(m, 0x41f8fd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x15e),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x41f90f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x15e),16));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,0x695eb8u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41f929);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x168),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x181u32);
            c.push(m,c.get(2,32));
            { let target = c.get(3,32); c.push(m, 0x41f941); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x0u32);
            pc = 0x41f97d; continue;
            }
            0x41f945 => {
            c.set(1,32,c.r[7].wrapping_add(0x1d8));
            c.push(m, 0x41f950);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x1c8));
            c.push(m, 0x41f95b);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x1dc));
            c.push(m, 0x41f966);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x1e4));
            c.push(m, 0x41f971);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x1e0));
            c.push(m, 0x41f97c);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(5,32));
            pc = 0x41f97d; continue;
            }
            0x41f97d => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x41f984);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65248b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x41f995);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
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

pub fn ship_convoy_picker_dialog_fill_ship_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4206c0;
    loop {
        match pc {
            0x4206c0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m,0x65c62cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x62),16));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4206f5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x1u32); }
            if c.cond(3) { pc = 0x4208bf; continue; }
            pc = 0x42072a; continue;
            }
            0x42072a => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(2) { pc = 0x42074f; continue; }
            pc = 0x420745; continue;
            }
            0x420745 => {
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x4208a5; continue; }
            pc = 0x42074f; continue;
            }
            0x42074f => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x39),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x42077a; continue; }
            pc = 0x42075e; continue;
            }
            0x42075e => {
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,0x69603cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x420775);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x42078c; continue;
            }
            0x42077a => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x696044u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x420789);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x42078c; continue;
            }
            0x42078c => {
            c.fild32(m,c.r[7].wrapping_add(0x10));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x24),4));
            { let v = c.ld(m,0x66bc5cu32,4); c.fop('*',v); }
            c.set(0,32,c.r[7].wrapping_add(0x160));
            c.fstp64(m,c.r[4]);
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0cb4),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,0x69604cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4207cd);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x136),8));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4207e8; continue; }
            pc = 0x4207da; continue;
            }
            0x4207da => {
            c.push(m,0x69606cu32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x4207e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4207e8; continue;
            }
            0x4207e8 => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x42),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x420803; continue; }
            pc = 0x4207f5; continue;
            }
            0x4207f5 => {
            c.push(m,0x696070u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x420803);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x420803; continue;
            }
            0x420803 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x420843; continue; }
            pc = 0x420810; continue;
            }
            0x420810 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x420839; continue; }
            pc = 0x420832; continue;
            }
            0x420832 => {
            c.push(m,0x696074u32);
            pc = 0x42083e; continue;
            }
            0x420839 => {
            c.push(m,0x696080u32);
            pc = 0x42083e; continue;
            }
            0x42083e => {
            c.push(m, 0x420843);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x420843; continue;
            }
            0x420843 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x80),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x143u32);
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a4a0u32,32); c.push(m, 0x42085c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x4208a5; continue; }
            pc = 0x420862; continue;
            }
            0x420862 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x80),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m,0x151u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4a0u32,32); c.push(m, 0x420876); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x60),16));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x420891; continue; }
            pc = 0x420882; continue;
            }
            0x420882 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x80),32));
            c.push(m,c.get(5,32));
            c.push(m,0x14eu32);
            c.push(m,c.get(2,32));
            pc = 0x42089f; continue;
            }
            0x420891 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x80),32));
            c.push(m,0x0u32);
            c.push(m,0x14eu32);
            c.push(m,c.get(0,32));
            pc = 0x42089f; continue;
            }
            0x42089f => {
            { let target = rm(m,0x66a4a0u32,32); c.push(m, 0x4208a5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x4208a5; continue;
            }
            0x4208a5 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x42072a; continue; }
            pc = 0x4208bf; continue;
            }
            0x4208bf => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4208cd);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4208de);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
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

pub fn money_lender_bank_dialog_draw_take_loan_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5b36e0;
    loop {
        match pc {
            0x5b36e0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m,0x665a9cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x18),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[3].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x32u32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(6,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffce));
            c.set(0,32,rm(m,0x69bfe8u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x5b373c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[3].wrapping_add(0xfa4));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x5b374b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x420c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m, 0x5b3752);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xfe4),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            c.set(1,32,0x6dcd28u32);
            if c.cond(12) { pc = 0x5b3797; continue; }
            pc = 0x5b3765; continue;
            }
            0x5b3765 => {
            c.push(m, 0x5b376a);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5b3770);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x69c108u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5b377e);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x69c10cu32,32));
            { let a = c.get(6,32); let r = c.add(a,0x14u32,32); c.set(6,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5b378f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x5b3b01; continue;
            }
            0x5b3797 => {
            c.push(m, 0x5b379c);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5b37a2);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xfd0),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x5b37b7; continue; }
            pc = 0x5b37af; continue;
            }
            0x5b37af => {
            c.set(0,32,rm(m,0x69c010u32,32));
            c.push(m,c.get(0,32));
            pc = 0x5b37be; continue;
            }
            0x5b37b7 => {
            c.set(1,32,rm(m,0x69c114u32,32));
            c.push(m,c.get(1,32));
            pc = 0x5b37be; continue;
            }
            0x5b37be => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5b37c5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcdd0u32);
            c.push(m, 0x5b37d2);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5b37d8);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cc37cu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5b37ec);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cc37cu32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5b37fb);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xfd0),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x1eu32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5b3a3a; continue; }
            pc = 0x5b380f; continue;
            }
            0x5b380f => {
            c.set(0,32,c.r[3].wrapping_add(0xf66));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x5b3819; continue;
            }
            0x5b3819 => {
            c.push(m,0xff000000u32);
            c.push(m, 0x5b3823);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xfd4),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5b383f; continue; }
            pc = 0x5b3830; continue;
            }
            0x5b3830 => {
            c.set(1,32,rm(m,0x66e104u32,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5b383c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5b383f; continue;
            }
            0x5b383f => {
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[3].wrapping_add(0xfd8),32),32); }
            if c.cond(5) { pc = 0x5b3856; continue; }
            pc = 0x5b3847; continue;
            }
            0x5b3847 => {
            c.set(2,32,rm(m,0x66e108u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5b3853);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5b3856; continue;
            }
            0x5b3856 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xfffffffe),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6c5cd0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5b3870);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcdd0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5b3882);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5b3892);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,0x69c018u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(0,8,rm(m,c.r[1],8));
            c.set(2,32,rm(m,0x69c014u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,0x6c5cd4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5b38b8);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[2].wrapping_add(c.r[7]).wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m, 0x5b38d2);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x5b38d9);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x2));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5b38f6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.push(m,0xff000000u32);
            c.push(m, 0x5b3903);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xfd4),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5b391e; continue; }
            pc = 0x5b3910; continue;
            }
            0x5b3910 => {
            c.set(0,32,rm(m,0x66e104u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5b391b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5b391e; continue;
            }
            0x5b391e => {
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[3].wrapping_add(0xfd8),32),32); }
            if c.cond(5) { pc = 0x5b3935; continue; }
            pc = 0x5b3926; continue;
            }
            0x5b3926 => {
            c.set(1,32,rm(m,0x66e108u32,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5b3932);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5b3935; continue;
            }
            0x5b3935 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,0x69c01cu32,32));
            { let a = c.get(6,32); let r = c.add(a,0x14u32,32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xfffffff6),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x6c5ce0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5b3956);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5b3962);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x5b3969);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcdd0u32);
            c.push(m, 0x5b397b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x2));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5b3994);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x5b399e);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xfd4),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5b39b9; continue; }
            pc = 0x5b39ab; continue;
            }
            0x5b39ab => {
            c.set(0,32,rm(m,0x66e104u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5b39b6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5b39b9; continue;
            }
            0x5b39b9 => {
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[3].wrapping_add(0xfd8),32),32); }
            if c.cond(5) { pc = 0x5b39d0; continue; }
            pc = 0x5b39c1; continue;
            }
            0x5b39c1 => {
            c.set(1,32,rm(m,0x66e108u32,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5b39cd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5b39d0; continue;
            }
            0x5b39d0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x69c020u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xfffffffa),8));
            { let a = c.get(6,32); let r = c.add(a,0x14u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.fild32(m,c.r[4].wrapping_add(0x20));
            c.fstp64(m,c.r[4].wrapping_add(0x20));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x20),8));
            { let v = c.ld(m,0x66d750u32,8); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m,c.get(1,32));
            c.push(m,0x6c5ce8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5b3a0e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5b3a1a);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xfd0),32));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x1eu32,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x5b3819; continue; }
            pc = 0x5b3a3a; continue;
            }
            0x5b3a3a => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xfd0),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x5b3a6c; continue; }
            pc = 0x5b3a45; continue;
            }
            0x5b3a45 => {
            c.set(1,32,rm(m,0x69c0a0u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5b3a53);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x69c0a4u32,32));
            { let a = c.get(6,32); let r = c.add(a,0x14u32,32); c.set(6,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5b3a64);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x5b3b01; continue;
            }
            0x5b3a6c => {
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x5b3b01; continue; }
            pc = 0x5b3a75; continue;
            }
            0x5b3a75 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5b3a85);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5b3aa2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5b3aba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.set(1,32,rm(m,0x69c0a8u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5b3ad2);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x2c),32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x8cu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,0xff000000u32);
            c.push(m,0x14u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5b3af8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m, 0x5b3b01);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x420a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            pc = 0x5b3b01; continue;
            }
            0x5b3b01 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5b3b12);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
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

pub fn shipyard_parchment_dialog_update_repair_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5f8f50;
    loop {
        match pc {
            0x5f8f50 => {
            { let a = c.get(4,32); let r = c.sub(a,0x2cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(7,16,rm(m,c.r[0],16));
            if c.cond(4) { pc = 0x5f8f70; continue; }
            pc = 0x5f8f67; continue;
            }
            0x5f8f67 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5f8f6d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5f8f70; continue;
            }
            0x5f8f70 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcb8),32));
            { let a = c.r[6].wrapping_add(0xcd4); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(3,32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x5f8f91);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x504ac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0xcd4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,16); c.sub(a,rm(m,c.r[0],16),16); }
            if c.cond(4) { pc = 0x5f8fe2; continue; }
            pc = 0x5f8f9c; continue;
            }
            0x5f8f9c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5f8fcb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,16); c.sub(a,c.get(3,16),16); }
            if c.cond(5) { pc = 0x5f8fe2; continue; }
            pc = 0x5f8fd0; continue;
            }
            0x5f8fd0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(1,32)); }
            pc = 0x5f8fe2; continue;
            }
            0x5f8fe2 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[6].wrapping_add(0x1b8));
            c.set(5,16,rm(m,c.r[2],16));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x5f8ffb);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5f9096; continue; }
            pc = 0x5f9003; continue;
            }
            0x5f9003 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[6].wrapping_add(0xce8); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0xce4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            if c.cond(14) { pc = 0x5f9055; continue; }
            pc = 0x5f9026; continue;
            }
            0x5f9026 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            c.set(2,32,c.r[5].wrapping_add(0x2));
            pc = 0x5f902f; continue;
            }
            0x5f902f => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[2],16));
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5f9040; continue; }
            pc = 0x5f9038; continue;
            }
            0x5f9038 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x5f902f; continue; }
            pc = 0x5f9040; continue;
            }
            0x5f9040 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x5f9055; continue; }
            pc = 0x5f9046; continue;
            }
            0x5f9046 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(2)),16));
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(1,32)); }
            pc = 0x5f9067; continue;
            }
            0x5f9055 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(0,32)); }
            pc = 0x5f9067; continue;
            }
            0x5f9067 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5f9096);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5f9096; continue;
            }
            0x5f9096 => {
            c.set(1,32,c.r[6].wrapping_add(0x2a0));
            c.push(m, 0x5f90a1);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5f913e; continue; }
            pc = 0x5f90a9; continue;
            }
            0x5f90a9 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0xce8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xce4); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x5f90e9; continue; }
            pc = 0x5f90cf; continue;
            }
            0x5f90cf => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            pc = 0x5f90d8; continue;
            }
            0x5f90d8 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[1],16));
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x5f90e9; continue; }
            pc = 0x5f90e1; continue;
            }
            0x5f90e1 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x5f90d8; continue; }
            pc = 0x5f90e9; continue;
            }
            0x5f90e9 => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            if c.cond(13) { pc = 0x5f9103; continue; }
            pc = 0x5f90f4; continue;
            }
            0x5f90f4 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x4),16));
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(1,32)); }
            pc = 0x5f910f; continue;
            }
            0x5f9103 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x2),16));
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(0,32)); }
            pc = 0x5f910f; continue;
            }
            0x5f910f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5f913e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5f913e; continue;
            }
            0x5f913e => {
            c.set(1,32,c.r[6].wrapping_add(0x558));
            c.push(m, 0x5f9149);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5f9235; continue; }
            pc = 0x5f9151; continue;
            }
            0x5f9151 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x5f915a);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0xcdf),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x5f9188; continue; }
            pc = 0x5f9162; continue;
            }
            0x5f9162 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x1du32); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x8),16));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            pc = 0x5f919a; continue;
            }
            0x5f9188 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x3u32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            pc = 0x5f919a; continue;
            }
            0x5f919a => {
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5f91a9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5f91d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            c.set(1,16,rm(m,c.r[7],16));
            { let a = c.get(1,16); c.sub(a,0x1u32,16); }
            if c.cond(5) { pc = 0x5f91ef; continue; }
            pc = 0x5f91e7; continue;
            }
            0x5f91e7 => {
            { let a = c.r[6].wrapping_add(0xce1); wm(m,a,8,c.get(3,8)); }
            pc = 0x5f9235; continue;
            }
            0x5f91ef => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            if c.cond(14) { pc = 0x5f9215; continue; }
            pc = 0x5f91f9; continue;
            }
            0x5f91f9 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            c.set(2,32,c.r[7].wrapping_add(0x2));
            pc = 0x5f9202; continue;
            }
            0x5f9202 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[2],16));
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x5f9213; continue; }
            pc = 0x5f920b; continue;
            }
            0x5f920b => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x5f9202; continue; }
            pc = 0x5f9213; continue;
            }
            0x5f9213 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x5f9215; continue;
            }
            0x5f9215 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x5f9229; continue; }
            pc = 0x5f921a; continue;
            }
            0x5f921a => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x4),16));
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(1,32)); }
            pc = 0x5f9235; continue;
            }
            0x5f9229 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x2),16));
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(2,32)); }
            pc = 0x5f9235; continue;
            }
            0x5f9235 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),16); c.sub(a,c.get(3,16),16); }
            if c.cond(5) { pc = 0x5f92c1; continue; }
            pc = 0x5f9240; continue;
            }
            0x5f9240 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x558),32));
            c.set(7,32,c.r[6].wrapping_add(0x558));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x5f9254); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5f94da; continue; }
            pc = 0x5f925c; continue;
            }
            0x5f925c => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5f9267); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1b8),32));
            c.set(1,32,c.r[6].wrapping_add(0x1b8));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5f927a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2a0),32));
            c.set(1,32,c.r[6].wrapping_add(0x2a0));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5f928d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5f92bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5f94da; continue;
            }
            0x5f92c1 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x558),32));
            c.set(1,32,c.r[6].wrapping_add(0x558));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5f92d5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1b8),32));
            c.set(1,32,c.r[6].wrapping_add(0x1b8));
            { let a = rm(m,c.r[0],16); c.sub(a,0x1u32,16); }
            if c.cond(6) { pc = 0x5f92f9; continue; }
            pc = 0x5f92ed; continue;
            }
            0x5f92ed => {
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5f92f5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            pc = 0x5f9301; continue;
            }
            0x5f92f9 => {
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5f9300); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            pc = 0x5f9301; continue;
            }
            0x5f9301 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2a0),32));
            c.set(1,32,c.r[6].wrapping_add(0x2a0));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5f9313); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x5f933d; continue; }
            pc = 0x5f9334; continue;
            }
            0x5f9334 => {
            { let a = c.r[6].wrapping_add(0xcdf); wm(m,a,8,0x1u32); }
            pc = 0x5f9343; continue;
            }
            0x5f933d => {
            { let a = c.r[6].wrapping_add(0xcdf); wm(m,a,8,c.get(3,8)); }
            pc = 0x5f9343; continue;
            }
            0x5f9343 => {
            { let a = rm(m,c.r[6].wrapping_add(0xcdf),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x5f9424; continue; }
            pc = 0x5f934f; continue;
            }
            0x5f934f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xcf4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xd04); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xcec); wm(m,a,16,c.get(3,16)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16));
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x5f93fe; continue; }
            pc = 0x5f938d; continue;
            }
            0x5f938d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcf4),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x40),16));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xcf4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[7].wrapping_add(0x14),32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xd04),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let a = c.r[6].wrapping_add(0xd04); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0xcec),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0xcec); wm(m,a,16,r); } }
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x5f938d; continue; }
            pc = 0x5f93fe; continue;
            }
            0x5f93fe => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcf4),32));
            { let a = c.r[6].wrapping_add(0xd00); wm(m,a,32,c.get(5,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0xcf4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd04),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0xd04); wm(m,a,32,c.get(0,32)); }
            pc = 0x5f9461; continue;
            }
            0x5f9424 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x40),16));
            { let a = c.r[6].wrapping_add(0xcf4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[7].wrapping_add(0x14),32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0xd04); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x10),32));
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xcec); wm(m,a,16,c.get(2,16)); }
            pc = 0x5f9461; continue;
            }
            0x5f9461 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcb8),32));
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x5f947d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x52a950, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xcfc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcb8),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,0x1u32);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5f94a2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x52aad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[6].wrapping_add(0xcf8); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5f94b9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcfc),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,c.r[6].wrapping_add(0x558));
            if c.cond(13) { pc = 0x5f94d3; continue; }
            pc = 0x5f94cb; continue;
            }
            0x5f94cb => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5f94d1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5f94da; continue;
            }
            0x5f94d3 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5f94da); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5f94da; continue;
            }
            0x5f94da => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xce8),32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let v = c.pop(m); c.set(5,32,v); }
            if c.cond(4) { pc = 0x5f951c; continue; }
            pc = 0x5f94e8; continue;
            }
            0x5f94e8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5f951c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5f951c; continue;
            }
            0x5f951c => {
            c.set(0,32,rm(m,0x6dccf0u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xce4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.fild64(m,c.r[4].wrapping_add(0xc));
            c.fstp64(m,c.r[4].wrapping_add(0xc));
            c.fld(c.ld(m,c.r[4].wrapping_add(0xc),8));
            { let v = c.ld(m,0x67a8c8u32,8); c.fop('/',v); }
            { let v = c.ld(m,0x67a8c0u32,8); c.fop('*',v); }
            c.push(m, 0x5f954e);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xffu32,32); }
            if c.cond(14) { pc = 0x5f955b; continue; }
            pc = 0x5f9555; continue;
            }
            0x5f9555 => {
            { let a = c.r[6].wrapping_add(0xce8); wm(m,a,32,c.get(7,32)); }
            pc = 0x5f955b; continue;
            }
            0x5f955b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn shipyard_parchment_dialog_update_rename_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5fabe0;
    loop {
        match pc {
            0x5fabe0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x668162u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x44u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(7,16,rm(m,c.r[0],16));
            if c.cond(4) { pc = 0x5fac15; continue; }
            pc = 0x5fac0c; continue;
            }
            0x5fac0c => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5fac12);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5fac15; continue;
            }
            0x5fac15 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcb8),32));
            { let a = c.r[6].wrapping_add(0xcd4); wm(m,a,32,0x0u32); }
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x5fac3b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x504ac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0xcd4); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[0],16); c.sub(a,0x1u32,16); }
            if c.cond(6) { pc = 0x5fac72; continue; }
            pc = 0x5fac47; continue;
            }
            0x5fac47 => {
            { let a = c.get(7,16); c.and(a,c.get(7,16),16); }
            if c.cond(5) { pc = 0x5fac58; continue; }
            pc = 0x5fac4c; continue;
            }
            0x5fac4c => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(2,32)); }
            pc = 0x5fac58; continue;
            }
            0x5fac58 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1b8),32));
            c.set(7,32,c.r[6].wrapping_add(0x1b8));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5fac6e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            pc = 0x5fac8a; continue;
            }
            0x5fac72 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1b8),32));
            c.set(7,32,c.r[6].wrapping_add(0x1b8));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5fac88); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            pc = 0x5fac8a; continue;
            }
            0x5fac8a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2a0),32));
            c.set(1,32,c.r[6].wrapping_add(0x2a0));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5fac9c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5faca4);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5fad6e; continue; }
            pc = 0x5facac; continue;
            }
            0x5facac => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            { let a = c.r[6].wrapping_add(0xce8); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0xce4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5facfa; continue; }
            pc = 0x5facd4; continue;
            }
            0x5facd4 => {
            c.set(2,32,c.r[7].wrapping_add(0x2));
            pc = 0x5facd7; continue;
            }
            0x5facd7 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[2],16));
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5face8; continue; }
            pc = 0x5face0; continue;
            }
            0x5face0 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x5facd7; continue; }
            pc = 0x5face8; continue;
            }
            0x5face8 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x5facfa; continue; }
            pc = 0x5facec; continue;
            }
            0x5facec => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(2)),16));
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(2,32)); }
            pc = 0x5fad06; continue;
            }
            0x5facfa => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)),16));
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(1,32)); }
            pc = 0x5fad06; continue;
            }
            0x5fad06 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            c.set(3,8,0x1u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x160));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x5fad2a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x6e8));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,0x0u32); }
            c.push(m, 0x5fad44);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5fad55);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[6].wrapping_add(0xce0); wm(m,a,8,0x0u32); }
            c.push(m, 0x5fad67);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5fad6e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b7920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5fad6e; continue;
            }
            0x5fad6e => {
            c.set(1,32,c.r[6].wrapping_add(0x2a0));
            c.push(m, 0x5fad79);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5fae46; continue; }
            pc = 0x5fad81; continue;
            }
            0x5fad81 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            { let a = c.r[6].wrapping_add(0xce8); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0xce4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5fadbd; continue; }
            pc = 0x5fada9; continue;
            }
            0x5fada9 => {
            c.set(2,32,c.r[7].wrapping_add(0x2));
            pc = 0x5fadac; continue;
            }
            0x5fadac => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[2],16));
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5fadbd; continue; }
            pc = 0x5fadb5; continue;
            }
            0x5fadb5 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x5fadac; continue; }
            pc = 0x5fadbd; continue;
            }
            0x5fadbd => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x5fadd1; continue; }
            pc = 0x5fadc2; continue;
            }
            0x5fadc2 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x4),16));
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(2,32)); }
            pc = 0x5faddd; continue;
            }
            0x5fadd1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2),16));
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(0,32)); }
            pc = 0x5faddd; continue;
            }
            0x5faddd => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(3,8,0x1u32);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x160));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m, 0x5fae02);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.set(7,32,c.r[6].wrapping_add(0x6e8));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,0x1u32); }
            c.push(m, 0x5fae1c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5fae2d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[6].wrapping_add(0xce0); wm(m,a,8,0x0u32); }
            c.push(m, 0x5fae3f);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5fae46);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b7920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5fae46; continue;
            }
            0x5fae46 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(5,32,c.r[6].wrapping_add(0x6e8));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x40),16));
            { let a = c.r[6].wrapping_add(0xcf4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[7].wrapping_add(0x14),32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0xd04); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x10),32));
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xcec); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x5faea4);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(7,32); let r = c.add(a,0x160u32,32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5faeb3);
            { let args = []; c.r[0] = host.call(m, 0x639cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x558));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,32,c.get(7,32));
            if c.cond(5) { pc = 0x5faeca; continue; }
            pc = 0x5faec2; continue;
            }
            0x5faec2 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5faec8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5faed1; continue;
            }
            0x5faeca => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5faed1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5faed1; continue;
            }
            0x5faed1 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5faed8);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5faefc; continue; }
            pc = 0x5faedc; continue;
            }
            0x5faedc => {
            c.set(0,8,rm(m,0x6dcbcdu32,8));
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = 0x6dcbcdu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x5fb052; continue; }
            pc = 0x5faefc; continue;
            }
            0x5faefc => {
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m, 0x5faf05);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xcu32);
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5faf13);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5faf1d);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x5faf27);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,0x2u32); }
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(1,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x5faf67; continue; }
            pc = 0x5faf44; continue;
            }
            0x5faf44 => {
            c.set(2,32,c.r[4].wrapping_add(0x24));
            { let a = c.get(5,32); let r = c.sub(a,c.get(2,32),32); c.set(5,32,r); }
            pc = 0x5faf4a; continue;
            }
            0x5faf4a => {
            c.set(1,32,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x24));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[5]),8));
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0xcu32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5faf4a; continue; }
            pc = 0x5faf67; continue;
            }
            0x5faf67 => {
            c.set(3,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(0,32));
            c.push(m,0x6c7580u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x2du32); }
            c.push(m, 0x5faf87);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5fafa3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x5fb021; continue; }
            pc = 0x5fafa7; continue;
            }
            0x5fafa7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(1,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x5fafd7; continue; }
            pc = 0x5fafb5; continue;
            }
            0x5fafb5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,c.r[3].wrapping_add(c.r[2]));
            pc = 0x5fafbc; continue;
            }
            0x5fafbc => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[5]),8));
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x24); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0xcu32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5fafbc; continue; }
            pc = 0x5fafd7; continue;
            }
            0x5fafd7 => {
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0xcu32,32); }
            if c.cond(13) { pc = 0x5fafe5; continue; }
            pc = 0x5fafe0; continue;
            }
            0x5fafe0 => {
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x24); wm(m,a,8,0x0u32); }
            pc = 0x5fafe5; continue;
            }
            0x5fafe5 => {
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(0,32));
            c.push(m,0x6c7584u32);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x2eu32); }
            c.push(m, 0x5fb001);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc84),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5fb01d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(15) { pc = 0x5fafa7; continue; }
            pc = 0x5fb021; continue;
            }
            0x5fb021 => {
            c.set(0,32,c.r[6].wrapping_add(0x6e8));
            c.set(3,8,0x1u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[6].wrapping_add(0xce0); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x5fb03a);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5fb041);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b7920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5fb052);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5fb052; continue;
            }
            0x5fb052 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xce8),32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let v = c.pop(m); c.set(5,32,v); }
            if c.cond(4) { pc = 0x5fb062; continue; }
            pc = 0x5fb060; continue;
            }
            0x5fb060 => {
            c.set(3,8,0x1u32);
            pc = 0x5fb062; continue;
            }
            0x5fb062 => {
            c.set(1,32,rm(m,0x6dccf0u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xce4),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.fild64(m,c.r[4].wrapping_add(0x10));
            c.fstp64(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),8));
            { let v = c.ld(m,0x67a8c8u32,8); c.fop('/',v); }
            { let v = c.ld(m,0x67a8c0u32,8); c.fop('*',v); }
            c.push(m, 0x5fb099);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xffu32,32); }
            if c.cond(14) { pc = 0x5fb0a6; continue; }
            pc = 0x5fb0a0; continue;
            }
            0x5fb0a0 => {
            { let a = c.r[6].wrapping_add(0xce8); wm(m,a,32,c.get(7,32)); }
            pc = 0x5fb0a6; continue;
            }
            0x5fb0a6 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x5fb137; continue; }
            pc = 0x5fb0ae; continue;
            }
            0x5fb0ae => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc94),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5fb0bf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x5fb0e9; continue; }
            pc = 0x5fb0c6; continue;
            }
            0x5fb0c6 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5fb0e9; continue; }
            pc = 0x5fb0cd; continue;
            }
            0x5fb0cd => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x5fb0ed; continue;
            }
            0x5fb0e9 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x5fb0ed; continue;
            }
            0x5fb0ed => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(7,32); let r = c.add(a,0x3cu32,32); c.set(7,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x5fb137);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5fb137; continue;
            }
            0x5fb137 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x50u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn hanse_side_menu_update(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a3c00;
    loop {
        match pc {
            0x5a3c00 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7ea),8));
            { let a = c.r[4].wrapping_add(0xf); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x5a3c17);
            { let args = []; c.r[0] = host.call(m, 0x4415d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(7,32,rm(m,0x66d4fcu32,32));
            c.set(2,32,rm(m,0x66d504u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,0x6e5558u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[2].wrapping_add(0x12fe),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(0,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5a3cd4; continue; }
            pc = 0x5a3c50; continue;
            }
            0x5a3c50 => {
            c.push(m,0x0u32);
            c.push(m, 0x5a3c57);
            { let args = []; c.r[0] = host.call(m, 0x462280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a3c61);
            { let args = []; c.r[0] = host.call(m, 0x5a37c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m, 0x5a3c6c);
            { let args = []; c.r[0] = host.call(m, 0x466320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x6u32);
            c.push(m, 0x5a3c79);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x466430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5a3c88);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x5u32);
            c.push(m, 0x5a3c97);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x6u32);
            c.push(m, 0x5a3ca6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[6].wrapping_add(0x7ed); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1c5),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5a3cc6; continue; }
            pc = 0x5a3cbd; continue;
            }
            0x5a3cbd => {
            c.push(m,0x6u32);
            c.push(m,0x1u32);
            c.push(m, 0x5a3cc6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4436e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a3cc6; continue;
            }
            0x5a3cc6 => {
            c.set(1,32,rm(m,0x6e5558u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x5a3cd4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5a3cd4; continue;
            }
            0x5a3cd4 => {
            c.set(1,32,rm(m,0x66d504u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5a3fd1; continue; }
            pc = 0x5a3ce8; continue;
            }
            0x5a3ce8 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a3cef);
            { let args = []; c.r[0] = host.call(m, 0x5a37c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5a3d14; continue; }
            pc = 0x5a3cf3; continue;
            }
            0x5a3cf3 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.set(7,32,c.r[6].wrapping_add(0xac));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x5a3d04); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5a3d0c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a3d14);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5a38e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a3d14; continue;
            }
            0x5a3d14 => {
            c.push(m,c.get(5,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(5,32,c.r[6].wrapping_add(0xac));
            pc = 0x5a3d1d; continue;
            }
            0x5a3d1d => {
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x5a3d28); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5a3d5a; continue; }
            pc = 0x5a3d2c; continue;
            }
            0x5a3d2c => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7ed),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5a3d48; continue; }
            pc = 0x5a3d36; continue;
            }
            0x5a3d36 => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m, 0x5a3d41);
            { let args = []; c.r[0] = host.call(m, 0x4665d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x7ed); wm(m,a,8,0x0u32); }
            pc = 0x5a3d48; continue;
            }
            0x5a3d48 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7ea),8));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5a3d68; continue; }
            pc = 0x5a3d54; continue;
            }
            0x5a3d54 => {
            { let a = c.r[6].wrapping_add(0x7ea); wm(m,a,8,c.get(3,8)); }
            pc = 0x5a3d5a; continue;
            }
            0x5a3d5a => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0xe8u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); c.sub(a,0x7u32,32); }
            if c.cond(12) { pc = 0x5a3d1d; continue; }
            pc = 0x5a3d66; continue;
            }
            0x5a3d66 => {
            pc = 0x5a3d8e; continue;
            }
            0x5a3d68 => {
            c.set(7,32,c.r[6].wrapping_add(0xac));
            c.set(5,32,0x7u32);
            pc = 0x5a3d73; continue;
            }
            0x5a3d73 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x5a3d7f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x5a3d73; continue; }
            pc = 0x5a3d88; continue;
            }
            0x5a3d88 => {
            { let a = c.r[6].wrapping_add(0x7ea); wm(m,a,8,c.get(3,8)); }
            pc = 0x5a3d8e; continue;
            }
            0x5a3d8e => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7ea),8));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); c.sub(a,0x7u32,8); }
            if c.cond(3) { pc = 0x5a3dc0; continue; }
            pc = 0x5a3d99; continue;
            }
            0x5a3d99 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0xac),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0xac));
            { let target = rm(m,c.r[0].wrapping_add(0x124),32); c.push(m, 0x5a3dc0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5a3dc0; continue;
            }
            0x5a3dc0 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7ea),8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0xf),8));
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5a3de7; continue; }
            pc = 0x5a3dce; continue;
            }
            0x5a3dce => {
            c.set(1,32,rm(m,0x6e5558u32,32));
            { let a = c.get(0,8); c.sub(a,0x6u32,8); }
            if c.cond(5) { pc = 0x5a3ddc; continue; }
            pc = 0x5a3dd8; continue;
            }
            0x5a3dd8 => {
            c.push(m,0x8u32);
            pc = 0x5a3de2; continue;
            }
            0x5a3ddc => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            pc = 0x5a3de2; continue;
            }
            0x5a3de2 => {
            c.push(m, 0x5a3de7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5c1080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a3de7; continue;
            }
            0x5a3de7 => {
            c.set(1,32,rm(m,0x6e5558u32,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x12fe),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5a3e03; continue; }
            pc = 0x5a3df7; continue;
            }
            0x5a3df7 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5a3e00);
            { let args = []; c.r[0] = host.call(m, 0x440f70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x5a3e03; continue;
            }
            0x5a3e03 => {
            c.push(m, 0x5a3e08);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,0x66cb34u32,4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,0x677fa8u32,8); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x4100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x5a3e72; continue; }
            pc = 0x5a3e35; continue;
            }
            0x5a3e35 => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xe8),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xd0),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x2),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5a3e5c; continue; }
            pc = 0x5a3e51; continue;
            }
            0x5a3e51 => {
            c.push(m,0x1u32);
            c.push(m,0x2u32);
            c.push(m, 0x5a3e5a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4667a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a3e65; continue;
            }
            0x5a3e5c => {
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m, 0x5a3e65);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a3e65; continue;
            }
            0x5a3e65 => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x1u32);
            c.push(m, 0x5a3e72);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x466610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a3e72; continue;
            }
            0x5a3e72 => {
            c.push(m, 0x5a3e77);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,0x66cb34u32,4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,0x677fa8u32,8); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x4100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x5a3ee1; continue; }
            pc = 0x5a3ea4; continue;
            }
            0x5a3ea4 => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xe8),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xd0),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x5a3ecb; continue; }
            pc = 0x5a3ec0; continue;
            }
            0x5a3ec0 => {
            c.push(m,0x2u32);
            c.push(m,0x1u32);
            c.push(m, 0x5a3ec9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4667a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a3ed4; continue;
            }
            0x5a3ecb => {
            c.push(m,0x0u32);
            c.push(m,0x2u32);
            c.push(m, 0x5a3ed4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a3ed4; continue;
            }
            0x5a3ed4 => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x2u32);
            c.push(m, 0x5a3ee1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x466610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a3ee1; continue;
            }
            0x5a3ee1 => {
            c.push(m, 0x5a3ee6);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,0x66cb34u32,4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,0x677fa8u32,8); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x4100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x5a3f50; continue; }
            pc = 0x5a3f13; continue;
            }
            0x5a3f13 => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xe8),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xd0),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x4),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5a3f3a; continue; }
            pc = 0x5a3f2f; continue;
            }
            0x5a3f2f => {
            c.push(m,0x3u32);
            c.push(m,0x4u32);
            c.push(m, 0x5a3f38);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4667a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a3f43; continue;
            }
            0x5a3f3a => {
            c.push(m,0x0u32);
            c.push(m,0x3u32);
            c.push(m, 0x5a3f43);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a3f43; continue;
            }
            0x5a3f43 => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x3u32);
            c.push(m, 0x5a3f50);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x466610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a3f50; continue;
            }
            0x5a3f50 => {
            c.push(m, 0x5a3f55);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,0x66cb34u32,4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,0x677fa8u32,8); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x4100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x5a3fd1; continue; }
            pc = 0x5a3f82; continue;
            }
            0x5a3f82 => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xe8),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xd0),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x5a3fbb; continue; }
            pc = 0x5a3f9e; continue;
            }
            0x5a3f9e => {
            c.push(m,0x4u32);
            c.push(m,0x3u32);
            c.push(m, 0x5a3fa7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4667a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x4u32);
            c.push(m, 0x5a3fb4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x466610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            0x5a3fbb => {
            c.push(m,0x0u32);
            c.push(m,0x4u32);
            c.push(m, 0x5a3fc4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x4u32);
            c.push(m, 0x5a3fd1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x466610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a3fd1; continue;
            }
            0x5a3fd1 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "merchant_debug_dialog_show_selected_ship" => merchant_debug_dialog_show_selected_ship(m, host, regs),
        "ship_convoy_picker_dialog_fill_ship_list" => ship_convoy_picker_dialog_fill_ship_list(m, host, regs),
        "money_lender_bank_dialog_draw_take_loan_page" => money_lender_bank_dialog_draw_take_loan_page(m, host, regs),
        "shipyard_parchment_dialog_update_repair_page" => shipyard_parchment_dialog_update_repair_page(m, host, regs),
        "shipyard_parchment_dialog_update_rename_page" => shipyard_parchment_dialog_update_rename_page(m, host, regs),
        "hanse_side_menu_update" => hanse_side_menu_update(m, host, regs),
        _ => return None,
    })
}
