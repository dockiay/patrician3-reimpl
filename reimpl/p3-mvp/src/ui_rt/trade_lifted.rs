//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;
pub fn op_sell_cargo_to_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x537340;
    loop {
        match pc {
            0x537340 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xe),16));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53744b; continue; }
            pc = 0x53736c; continue;
            }
            0x53736c => {
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x5373a2; continue; }
            pc = 0x53737c; continue;
            }
            0x53737c => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53738c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xe),8));
            c.set(2,32,0x1u32);
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x608),32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x53744b; continue; }
            pc = 0x5373a2; continue;
            }
            0x5373a2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(1,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x54));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x5373c2; continue; }
            pc = 0x5373b7; continue;
            }
            0x5373b7 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1]; wm(m,a,32,0x0u32); }
            pc = 0x5373c6; continue;
            }
            0x5373c2 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            pc = 0x5373c6; continue;
            }
            0x5373c6 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5373cd);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5373d6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7],8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5373e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xe),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x537403);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xe),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x537425);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f8690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xe),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = rm(m,c.r[0],32); let r = c.add(a,c.get(1,32),32); { let a = c.r[0]; wm(m,a,32,r); } }
            pc = 0x53744b; continue;
            }
            0x53744b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
pub fn op_buy_cargo_from_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x537450;
    loop {
        match pc {
            0x537450 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.r[1].wrapping_add(0x4));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(2,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(1,32,c.get(0,32));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x5375fc; continue; }
            pc = 0x537486; continue;
            }
            0x537486 => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x39),8));
            { let a = c.get(2,16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x5375fc; continue; }
            pc = 0x537494; continue;
            }
            0x537494 => {
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x5374ca; continue; }
            pc = 0x5374a4; continue;
            }
            0x5374a4 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5374b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa),8));
            c.set(2,32,0x1u32);
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x608),32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5375fc; continue; }
            pc = 0x5374ca; continue;
            }
            0x5374ca => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x39),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(5,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5374f1);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x5374fc; continue; }
            pc = 0x5374fa; continue;
            }
            0x5374fa => {
            c.set(1,32,c.get(0,32));
            pc = 0x5374fc; continue;
            }
            0x5374fc => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[6],32),32); }
            if c.cond(13) { pc = 0x537553; continue; }
            pc = 0x537500; continue;
            }
            0x537500 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x5375fc; continue; }
            pc = 0x537508; continue;
            }
            0x537508 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x537534; continue; }
            pc = 0x537518; continue;
            }
            0x537518 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            pc = 0x537551; continue;
            }
            0x537534 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            pc = 0x537551; continue;
            }
            0x537551 => {
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            pc = 0x537553; continue;
            }
            0x537553 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,8,rm(m,c.r[7],8));
            c.push(m,c.get(2,32));
            c.push(m, 0x537562);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x53757f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[3],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x5375ab; continue; }
            pc = 0x537589; continue;
            }
            0x537589 => {
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x39),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x5375a7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x52f3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5375ab; continue;
            }
            0x5375ab => {
            { let a = rm(m,c.r[6],32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x5375fc; continue; }
            pc = 0x5375b0; continue;
            }
            0x5375b0 => {
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5375cc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x517fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5375d3);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            { let a = c.r[5]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5375fc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f8840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x5375fc; continue;
            }
            0x5375fc => {
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
pub fn op_sell_office_stock_to_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53d2b0;
    loop {
        match pc {
            0x53d2b0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[6].wrapping_add(0x8),16); c.sub(a,0x18u32,16); }
            if c.cond(3) { pc = 0x53d3b1; continue; }
            pc = 0x53d2bf; continue;
            }
            0x53d2bf => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x53d3b1; continue; }
            pc = 0x53d2ca; continue;
            }
            0x53d2ca => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xe),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53d2e0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x53d3b1; continue; }
            pc = 0x53d2ea; continue;
            }
            0x53d2ea => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x53d2fe; continue; }
            pc = 0x53d2fb; continue;
            }
            0x53d2fb => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            pc = 0x53d2fe; continue;
            }
            0x53d2fe => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2c6),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53d3b1; continue; }
            pc = 0x53d314; continue;
            }
            0x53d314 => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x2c4),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x53d374; continue; }
            pc = 0x53d324; continue;
            }
            0x53d324 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xe),16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x53d33b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x2c4),16));
            c.set(3,32,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x53d351);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[0],32));
            c.push(m,0x0u32);
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xe),16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(0,32));
            c.push(m, 0x53d373);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f8690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x53d374; continue;
            }
            0x53d374 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(0,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2c6),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = rm(m,c.r[0],32); let r = c.add(a,c.get(1,32),32); { let a = c.r[0]; wm(m,a,32,r); } }
            pc = 0x53d3b1; continue;
            }
            0x53d3b1 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
pub fn op_buy_ware_for_office(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53d3c0;
    loop {
        match pc {
            0x53d3c0 => {
            c.push(m,c.get(1,32));
            { let a = rm(m,c.r[1].wrapping_add(0x8),16); c.sub(a,0x18u32,16); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.r[1].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x53d52a; continue; }
            pc = 0x53d3d3; continue;
            }
            0x53d3d3 => {
            { let a = rm(m,c.r[6],32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x53d52a; continue; }
            pc = 0x53d3dc; continue;
            }
            0x53d3dc => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53d3f2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x53d52a; continue; }
            pc = 0x53d3fc; continue;
            }
            0x53d3fc => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x2c4),16));
            c.push(m,c.get(2,32));
            c.push(m, 0x53d410);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(3,32,c.get(0,32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x53d47f; continue; }
            pc = 0x53d438; continue;
            }
            0x53d438 => {
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x53d460; continue; }
            pc = 0x53d444; continue;
            }
            0x53d444 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            pc = 0x53d47d; continue;
            }
            0x53d460 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            pc = 0x53d47d; continue;
            }
            0x53d47d => {
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            pc = 0x53d47f; continue;
            }
            0x53d47f => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x53d48e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x53d4c0; continue; }
            pc = 0x53d498; continue;
            }
            0x53d498 => {
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x53d4b7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x52f3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = rm(m,c.r[6],32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x53d52a; continue; }
            pc = 0x53d4bc; continue;
            }
            0x53d4bc => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x53d4c0; continue;
            }
            0x53d4c0 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x53d4d2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5007a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x2c6),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.push(m,0x0u32);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x2c6),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x53d52a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f8840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x53d52a; continue;
            }
            0x53d52a => {
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
pub fn fit_buy_amount_to_budget(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x52f3e0;
    loop {
        match pc {
            0x52f3e0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1],32));
            c.push(m,c.get(7,32));
            if c.cond(15) { pc = 0x52f40f; continue; }
            pc = 0x52f3f5; continue;
            }
            0x52f3f5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            { let a = c.r[1]; wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            0x52f40f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1],32));
            c.set(5,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x52f427; continue;
            }
            0x52f427 => {
            c.set(0,32,c.get(6,32));
            c.push(m,c.get(5,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x52f433);
            { let r = (((m.r32(c.r[4].wrapping_add(4)) as u64) | ((m.r32(c.r[4].wrapping_add(8)) as u64) << 32)) as i64).wrapping_mul(((m.r32(c.r[4].wrapping_add(12)) as u64) | ((m.r32(c.r[4].wrapping_add(16)) as u64) << 32)) as i64); c.r[0] = r as u32; c.r[2] = (r >> 32) as u32; }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x52f444);
            { let r = (((m.r32(c.r[4].wrapping_add(4)) as u64) | ((m.r32(c.r[4].wrapping_add(8)) as u64) << 32)) as i64).checked_div(((m.r32(c.r[4].wrapping_add(12)) as u64) | ((m.r32(c.r[4].wrapping_add(16)) as u64) << 32)) as i64).expect("división por cero del x86"); c.r[0] = r as u32; c.r[2] = (r >> 32) as u32; }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x52f470; continue; }
            pc = 0x52f454; continue;
            }
            0x52f454 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x52f48d; continue;
            }
            0x52f470 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x52f48d; continue;
            }
            0x52f48d => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x52f4c6; continue; }
            pc = 0x52f491; continue;
            }
            0x52f491 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x52f4a2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x2c),32),32); }
            if c.cond(15) { pc = 0x52f427; continue; }
            pc = 0x52f4ac; continue;
            }
            0x52f4ac => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(3,32,c.get(6,32));
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x52f4e4; continue; }
            pc = 0x52f4be; continue;
            }
            0x52f4be => {
            { let a = c.get(6,32); let r = c.add(a,0xc8u32,32); c.set(6,32,r); }
            pc = 0x52f4ea; continue;
            }
            0x52f4c6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[1]; wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            0x52f4e4 => {
            { let a = c.get(6,32); let r = c.add(a,0x7d0u32,32); c.set(6,32,r); }
            pc = 0x52f4ea; continue;
            }
            0x52f4ea => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[0],32),32); }
            if c.cond(15) { pc = 0x52f509; continue; }
            pc = 0x52f4f2; continue;
            }
            0x52f4f2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x52f503);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x2c),32),32); }
            if c.cond(12) { pc = 0x52f4ac; continue; }
            pc = 0x52f509; continue;
            }
            0x52f509 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[2]; wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
pub fn ship_add_wares_by_total_cost(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x517fe0;
    loop {
        match pc {
            0x517fe0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x5180b3; continue; }
            pc = 0x517ff3; continue;
            }
            0x517ff3 => {
            c.push(m,0x4u32);
            c.push(m, 0x517ffa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x20));
            c.set(2,32,c.r[0].wrapping_add(c.r[7]));
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            c.fstp64(m,c.r[4].wrapping_add(0x8));
            c.fld(c.ld(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xb4),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x8),8); c.fop('*',v); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.fstp64(m,c.r[4].wrapping_add(0x8));
            c.fild32(m,c.r[4].wrapping_add(0x24));
            c.fstp64(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),8));
            { let v = c.ld(m,c.r[4].wrapping_add(0x8),8); c.fop('+',v); }
            c.fstp64(m,c.r[4].wrapping_add(0x8));
            if c.cond(4) { pc = 0x51807b; continue; }
            pc = 0x518042; continue;
            }
            0x518042 => {
            { let a = c.get(1,32); c.sub(a,0x14u32,32); }
            if c.cond(3) { pc = 0x518063; continue; }
            pc = 0x518047; continue;
            }
            0x518047 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            pc = 0x518098; continue;
            }
            0x518063 => {
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x518098; continue;
            }
            0x51807b => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            pc = 0x518098; continue;
            }
            0x518098 => {
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54));
            c.fstp64(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x8),8));
            { let v = c.ld(m,c.r[4].wrapping_add(0x10),8); c.fop('/',v); }
            c.fstp32(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xb4));
            pc = 0x5180b3; continue;
            }
            0x5180b3 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
pub fn find_or_create_office(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5308a0;
    loop {
        match pc {
            0x5308a0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66346cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x530d19; continue; }
            pc = 0x5308d7; continue;
            }
            0x5308d7 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(3,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xa),16),16); }
            if c.cond(3) { pc = 0x530d19; continue; }
            pc = 0x5308e5; continue;
            }
            0x5308e5 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x784),16));
            { let a = c.get(1,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x530937; continue; }
            pc = 0x530908; continue;
            }
            0x530908 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x74),32));
            pc = 0x53090b; continue;
            }
            0x53090b => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[5].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2c4),16); c.sub(a,c.get(3,16),16); }
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            if c.cond(4) { pc = 0x53095f; continue; }
            pc = 0x53092b; continue;
            }
            0x53092b => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2ca),16));
            { let a = c.get(1,16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x53090b; continue; }
            pc = 0x530937; continue;
            }
            0x530937 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x530d19; continue; }
            pc = 0x530943; continue;
            }
            0x530943 => {
            { let a = rm(m,c.r[6].wrapping_add(0x7e),16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x530a53; continue; }
            pc = 0x53094d; continue;
            }
            0x53094d => {
            { let a = c.get(7,16); c.and(a,c.get(7,16),16); }
            { let a = c.r[6].wrapping_add(0x7e); wm(m,a,16,c.get(7,16)); }
            if c.cond(4) { pc = 0x530979; continue; }
            pc = 0x530956; continue;
            }
            0x530956 => {
            c.set(2,32,c.r[7].wrapping_add(0x20));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,c.get(2,16)); }
            pc = 0x53097f; continue;
            }
            0x53095f => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            pc = 0x530d1b; continue;
            }
            0x530979 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,0x40u32); }
            pc = 0x53097f; continue;
            }
            0x53097f => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(0,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x53099c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x5309d3; continue; }
            pc = 0x5309af; continue;
            }
            0x5309af => {
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x5309cb; continue; }
            pc = 0x5309b6; continue;
            }
            0x5309b6 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            pc = 0x5309b7; continue;
            }
            0x5309b7 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5309be);
            { let args = []; c.r[0] = host.call(m, 0x4febb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,0x44cu32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x5309b7; continue; }
            pc = 0x5309c7; continue;
            }
            0x5309c7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            pc = 0x5309cb; continue;
            }
            0x5309cb => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(5,32,c.get(0,32));
            pc = 0x5309d5; continue;
            }
            0x5309d3 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x5309d5; continue;
            }
            0x5309d5 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0xffffffffu32); }
            if c.cond(4) { pc = 0x530d19; continue; }
            pc = 0x5309e5; continue;
            }
            0x5309e5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x74),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x530a15; continue; }
            pc = 0x5309ec; continue;
            }
            0x5309ec => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x7e),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x530a09);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x74),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x530a12);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x530a15; continue;
            }
            0x530a15 => {
            { let a = c.get(7,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x8),16),16); }
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,c.get(5,32)); }
            if c.cond(3) { pc = 0x530a53; continue; }
            pc = 0x530a1e; continue;
            }
            0x530a1e => {
            c.set(0,32,c.get(7,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x74),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0x2c4); wm(m,a,16,0xffffu32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x74),32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0x2c8); wm(m,a,16,c.get(7,16)); }
            { let a = c.get(7,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x8),16),16); }
            if c.cond(2) { pc = 0x530a1e; continue; }
            pc = 0x530a53; continue;
            }
            0x530a53 => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x7e),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x74),32));
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(5,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x2c8),16));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[6].wrapping_add(0x7e); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x78),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(3,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0xc),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x8),16),16); }
            c.set(1,8,u32::from(c.cond(2)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x530ab1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4febd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x40),16));
            { let a = c.r[5].wrapping_add(0x2c4); wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x78),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[3]).wrapping_add(0xc),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[5].wrapping_add(0x2c8); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x78),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[2].wrapping_add(c.r[3]).wrapping_add(0xc); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[5].wrapping_add(0x2c6); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x784),16));
            { let a = c.r[5].wrapping_add(0x2ca); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x784); wm(m,a,16,c.get(7,16)); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[5].wrapping_add(0x2c0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = rm(m,0x6dfc14u32,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x530b2f; continue; }
            pc = 0x530b16; continue;
            }
            0x530b16 => {
            c.set(0,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x530b2f; continue; }
            pc = 0x530b1f; continue;
            }
            0x530b1f => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x28cc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2]); wm(m,a,8,0x1u32); }
            pc = 0x530b2f; continue;
            }
            0x530b2f => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x530b38);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x40),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x78),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x44),8));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,c.r[2].wrapping_add(c.r[3]));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0xdu32); }
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,0x0u32); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x19),8));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x1u32); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x39c),8));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x530b79);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x44),8));
            { let a = c.get(0,8); let r = c.and(a,0xfu32,8); c.set(0,8,r); }
            c.set(0,16,c.get(0,8));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(7,16); c.sub(a,c.get(1,16),16); }
            { let a = c.r[4].wrapping_add(0x2e); wm(m,a,16,c.get(0,16)); }
            if c.cond(3) { pc = 0x530bcf; continue; }
            pc = 0x530b98; continue;
            }
            0x530b98 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            pc = 0x530b9b; continue;
            }
            0x530b9b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2a),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(0x2a),16); let r = c.inc(a,16); { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,r); } }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(7) { pc = 0x530bcf; continue; }
            pc = 0x530bad; continue;
            }
            0x530bad => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(0,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x74),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2c8),16));
            { let a = c.get(7,16); c.sub(a,c.get(1,16),16); }
            if c.cond(2) { pc = 0x530b9b; continue; }
            pc = 0x530bcf; continue;
            }
            0x530bcf => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[2]).wrapping_add(0x784),16));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,16,0x0u32); }
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x530c20; continue; }
            pc = 0x530bea; continue;
            }
            0x530bea => {
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x2c),16));
            c.set(3,16,c.get(2,16));
            { let a = c.get(2,16); let r = c.inc(a,16); c.set(2,16,r); }
            { let a = c.get(3,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xa),16),16); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,16,c.get(2,16)); }
            if c.cond(7) { pc = 0x530c20; continue; }
            pc = 0x530bff; continue;
            }
            0x530bff => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x74),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2ca),16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(2) { pc = 0x530bea; continue; }
            pc = 0x530c20; continue;
            }
            0x530c20 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x18),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x530c58; continue; }
            pc = 0x530c2e; continue;
            }
            0x530c2e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x78),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x530c47; continue; }
            pc = 0x530c3d; continue;
            }
            0x530c3d => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x8a),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x530c58; continue; }
            pc = 0x530c47; continue;
            }
            0x530c47 => {
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,0x6dd730u32);
            c.push(m,c.get(0,32));
            c.push(m,0xfffffffeu32);
            c.push(m, 0x530c58);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x530c58; continue;
            }
            0x530c58 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x530d04; continue; }
            pc = 0x530c66; continue;
            }
            0x530c66 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc324),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x530d04; continue; }
            pc = 0x530c78; continue;
            }
            0x530c78 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x78),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x8),8); c.and(a,0x4u32,8); }
            if c.cond(5) { pc = 0x530d04; continue; }
            pc = 0x530c86; continue;
            }
            0x530c86 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[2]));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            if c.cond(5) { pc = 0x530ceb; continue; }
            pc = 0x530ca1; continue;
            }
            0x530ca1 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x2cu32);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x803),8));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x802),8));
            c.push(m,c.get(2,32));
            c.push(m, 0x530cba);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x590680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x672a48u32,8));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x803),8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x802),8));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x530ce9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x5891c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x530d04; continue;
            }
            0x530ceb => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x2du32);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x805),8));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x804),8));
            c.push(m,c.get(2,32));
            c.push(m, 0x530d04);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x590680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x530d04; continue;
            }
            0x530d04 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x530d15);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(5,32));
            pc = 0x530d1b; continue;
            }
            0x530d19 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x530d1b; continue;
            }
            0x530d1b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
pub fn office_add_ware_with_total_cost(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5007a0;
    loop {
        match pc {
            0x5007a0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.fld(c.ld(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3b8),4));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32));
            c.fstp64(m,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(6,32),32); c.set(2,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x18));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.fstp64(m,c.r[4].wrapping_add(0xc));
            c.fld(c.ld(m,c.r[4].wrapping_add(0xc),8));
            { let v = c.ld(m,c.r[4].wrapping_add(0x4),8); c.fop('*',v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.fstp64(m,c.r[4]);
            c.fild32(m,c.r[4].wrapping_add(0x1c));
            c.fstp64(m,c.r[4].wrapping_add(0x8));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x8),8));
            { let v = c.ld(m,c.r[4],8); c.fop('+',v); }
            c.fstp64(m,c.r[4]);
            if c.cond(14) { pc = 0x50080d; continue; }
            pc = 0x5007f6; continue;
            }
            0x5007f6 => {
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp64(m,c.r[4].wrapping_add(0x8));
            c.fld(c.ld(m,c.r[4],8));
            { let v = c.ld(m,c.r[4].wrapping_add(0x8),8); c.fop('/',v); }
            c.fstp32(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3b8));
            pc = 0x50080d; continue;
            }
            0x50080d => {
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
pub fn office_close_weekly_period(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5006f0;
    loop {
        match pc {
            0x5006f0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x418),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x428),32));
            { let a = c.r[6].wrapping_add(0x41c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x430),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x420),32));
            { let a = c.r[6].wrapping_add(0x42c); wm(m,a,32,c.get(2,32)); }
            c.set(2,16,rm(m,c.r[6],16));
            { let a = c.r[6].wrapping_add(0x434); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x438),32));
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[6].wrapping_add(0x43c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x424); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x440),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x444); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[6].wrapping_add(0x438); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x500774; continue; }
            pc = 0x500754; continue;
            }
            0x500754 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c6),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x500774);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x529f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x500774; continue;
            }
            0x500774 => {
            { let a = c.r[6].wrapping_add(0x418); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x420); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x428); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x430); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x438); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x440); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6]; wm(m,a,16,c.get(7,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
pub fn op_convoy_trade_wares_with_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x539300;
    loop {
        match pc {
            0x539300 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x53969f; continue; }
            pc = 0x53931e; continue;
            }
            0x53931e => {
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,0x18u32,16); }
            if c.cond(3) { pc = 0x53969f; continue; }
            pc = 0x539329; continue;
            }
            0x539329 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x53969f; continue; }
            pc = 0x53933a; continue;
            }
            0x53933a => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53934a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1c),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0xe),16),16); }
            if c.cond(5) { pc = 0x53969f; continue; }
            pc = 0x539373; continue;
            }
            0x539373 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xa),16));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x539481; continue; }
            pc = 0x5393b3; continue;
            }
            0x5393b3 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x539431; continue; }
            pc = 0x5393c3; continue;
            }
            0x5393c3 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x539431; continue; }
            pc = 0x5393c7; continue;
            }
            0x5393c7 => {
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[6],8));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[2].wrapping_add(0xc),16),16); }
            if c.cond(5) { pc = 0x53941b; continue; }
            pc = 0x5393e2; continue;
            }
            0x5393e2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x5393fc; continue; }
            pc = 0x5393ea; continue;
            }
            0x5393ea => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            pc = 0x53940c; continue;
            }
            0x5393fc => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,0x0u32); }
            pc = 0x53940c; continue;
            }
            0x53940c => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x539413);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x3c),8); let r = c.or(a,0x4u32,8); { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,r); } }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x53941b; continue;
            }
            0x53941b => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x5393c3; continue; }
            pc = 0x539431; continue;
            }
            0x539431 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x1c),16));
            c.set(1,32,0x6de3d8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x539444);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(6,32,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x1c),16));
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x53945d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f8690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(0,8); let r = c.or(a,0x4u32,8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[3].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x539481 => {
            { let a = rm(m,c.r[2],32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x53969f; continue; }
            pc = 0x53948a; continue;
            }
            0x53948a => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x5394dc; continue; }
            pc = 0x53949a; continue;
            }
            0x53949a => {
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[6],8));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[2].wrapping_add(0xc),16),16); }
            if c.cond(5) { pc = 0x5394c6; continue; }
            pc = 0x5394b5; continue;
            }
            0x5394b5 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5394bc);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5394c6; continue; }
            pc = 0x5394c4; continue;
            }
            0x5394c4 => {
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            pc = 0x5394c6; continue;
            }
            0x5394c6 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x53949a; continue; }
            pc = 0x5394dc; continue;
            }
            0x5394dc => {
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x5394e6; continue; }
            pc = 0x5394e0; continue;
            }
            0x5394e0 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.get(7,32));
            pc = 0x5394e6; continue;
            }
            0x5394e6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5394f4; continue; }
            pc = 0x5394f0; continue;
            }
            0x5394f0 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x5394f4; continue;
            }
            0x5394f4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x1u32); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xa),16));
            pc = 0x539502; continue;
            }
            0x539502 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x53969f; continue; }
            pc = 0x539516; continue;
            }
            0x539516 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            c.set(2,16,rm(m,c.r[7],8));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[0].wrapping_add(0xc),16),16); }
            if c.cond(5) { pc = 0x53968d; continue; }
            pc = 0x53953a; continue;
            }
            0x53953a => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x539541);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x539555; continue; }
            pc = 0x53954b; continue;
            }
            0x53954b => {
            c.set(6,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            pc = 0x539555; continue;
            }
            0x539555 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x53968d; continue; }
            pc = 0x53955d; continue;
            }
            0x53955d => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x1c),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x539570);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5395ee; continue; }
            pc = 0x53957c; continue;
            }
            0x53957c => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            pc = 0x539584; continue;
            }
            0x539584 => {
            { let a = c.get(0,32); let r = c.imul(a,c.get(6,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(1,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5395b2; continue; }
            pc = 0x539596; continue;
            }
            0x539596 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x5395cf; continue;
            }
            0x5395b2 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x5395cf; continue;
            }
            0x5395cf => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(6,32));
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x1c),16));
            c.set(1,32,0x6de3d8u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5395e2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x539584; continue; }
            pc = 0x5395ee; continue;
            }
            0x5395ee => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x539614; continue; }
            pc = 0x5395f8; continue;
            }
            0x5395f8 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x539631; continue;
            }
            0x539614 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x539631; continue;
            }
            0x539631 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x539689; continue; }
            pc = 0x539635; continue;
            }
            0x539635 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x539689; continue; }
            pc = 0x539639; continue;
            }
            0x539639 => {
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x1c),16));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m, 0x53964e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f8840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x539670);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x517fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x539681);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.get(0,8); let r = c.or(a,0x4u32,8); c.set(0,8,r); }
            { let a = c.r[3].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x539689; continue;
            }
            0x539689 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x53968d; continue;
            }
            0x53968d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x539502; continue; }
            pc = 0x53969f; continue;
            }
            0x53969f => {
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
pub fn op_transfer_weapons_ship_office(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x538210;
    loop {
        match pc {
            0x538210 => {
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x538679; continue; }
            pc = 0x53822c; continue;
            }
            0x53822c => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x538679; continue; }
            pc = 0x538243; continue;
            }
            0x538243 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x538679; continue; }
            pc = 0x53825f; continue;
            }
            0x53825f => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x39),8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x538679; continue; }
            pc = 0x538272; continue;
            }
            0x538272 => {
            c.set(1,16,rm(m,c.r[7],8));
            c.set(0,16,c.get(0,8));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x538288);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x538679; continue; }
            pc = 0x538292; continue;
            }
            0x538292 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.and(a,0x80000000u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x80000000u32,32); }
            if c.cond(5) { pc = 0x5384a3; continue; }
            pc = 0x5382a7; continue;
            }
            0x5382a7 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x5382de; continue; }
            pc = 0x5382b4; continue;
            }
            0x5382b4 => {
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc),32),32); }
            if c.cond(5) { pc = 0x5382de; continue; }
            pc = 0x5382d4; continue;
            }
            0x5382d4 => {
            { let a = rm(m,c.r[0].wrapping_add(0x14),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x538679; continue; }
            pc = 0x5382de; continue;
            }
            0x5382de => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            if c.cond(2) { pc = 0x538357; continue; }
            pc = 0x5382e6; continue;
            }
            0x5382e6 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x154),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x538327; continue; }
            pc = 0x5382fc; continue;
            }
            0x5382fc => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x2bc),32));
            c.push(m,0x10u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[5].wrapping_add(0x2bc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x154); wm(m,a,16,0x0u32); }
            c.push(m, 0x53831c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x538327 => {
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x10),16),16); c.set(0,16,r); }
            c.push(m,0x10u32);
            { let a = c.r[7].wrapping_add(0x154); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x2bc),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0x2bc); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x53834c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x538357 => {
            { let a = rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672cb4),16); c.sub(a,0x7d0u32,16); }
            c.set(1,32,0x18u32);
            if c.cond(3) { pc = 0x53843c; continue; }
            pc = 0x53836c; continue;
            }
            0x53836c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x13c),8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5383c0; continue; }
            pc = 0x53837d; continue;
            }
            0x53837d => {
            { let a = c.get(1,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x538390; continue; }
            pc = 0x538382; continue;
            }
            0x538382 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,8); let r = c.or(a,0x1u32,8); c.set(0,8,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x13c),8); c.sub(a,0x6u32,8); }
            if c.cond(4) { pc = 0x5383c0; continue; }
            pc = 0x538390; continue;
            }
            0x538390 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x5383c4; continue; }
            pc = 0x538397; continue;
            }
            0x538397 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(c.r[1]).wrapping_add(0x13c); wm(m,a,8,0xffu32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,16,rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672cb4),16));
            c.set(3,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x124),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x124); wm(m,a,32,c.get(3,32)); }
            pc = 0x5383c0; continue;
            }
            0x5383c0 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x53836c; continue; }
            pc = 0x5383c4; continue;
            }
            0x5383c4 => {
            c.set(1,32,0x18u32);
            pc = 0x5383c9; continue;
            }
            0x5383c9 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x13c),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x538424; continue; }
            pc = 0x5383da; continue;
            }
            0x5383da => {
            { let a = c.get(1,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x5383ee; continue; }
            pc = 0x5383df; continue;
            }
            0x5383df => {
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.or(a,0x1u32,32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[7]).wrapping_add(0x13c),8); c.sub(a,0x6u32,8); }
            if c.cond(4) { pc = 0x538424; continue; }
            pc = 0x5383ee; continue;
            }
            0x5383ee => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(2,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(6) { pc = 0x538665; continue; }
            pc = 0x5383ff; continue;
            }
            0x5383ff => {
            { let a = c.r[7].wrapping_add(c.r[1]).wrapping_add(0x13c); wm(m,a,8,0xffu32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672cb4),16));
            c.set(3,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x124),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x124); wm(m,a,32,c.get(3,32)); }
            pc = 0x538424; continue;
            }
            0x538424 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x5383c9; continue; }
            pc = 0x538428; continue;
            }
            0x538428 => {
            c.push(m,0x10u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x538431);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53843c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x13c),8));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x53848b; continue; }
            pc = 0x53844d; continue;
            }
            0x53844d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(2,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(6) { pc = 0x538665; continue; }
            pc = 0x53845e; continue;
            }
            0x53845e => {
            { let a = c.r[7].wrapping_add(c.r[1]).wrapping_add(0x13c); wm(m,a,8,0xffu32); }
            { let a = c.r[1].wrapping_add(c.r[7]).wrapping_add(0x13d); wm(m,a,8,0xffu32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672cb4),16));
            c.set(3,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x124),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x124); wm(m,a,32,c.get(3,32)); }
            pc = 0x53848b; continue;
            }
            0x53848b => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x53843c; continue; }
            pc = 0x53848f; continue;
            }
            0x53848f => {
            c.push(m,0x10u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x538498);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5384a3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            if c.cond(2) { pc = 0x538509; continue; }
            pc = 0x5384ab; continue;
            }
            0x5384ab => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5384b2);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x5384ce; continue; }
            pc = 0x5384cc; continue;
            }
            0x5384cc => {
            c.set(2,32,c.get(6,32));
            pc = 0x5384ce; continue;
            }
            0x5384ce => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x2bc),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x5384da; continue; }
            pc = 0x5384d8; continue;
            }
            0x5384d8 => {
            c.set(2,32,c.get(0,32));
            pc = 0x5384da; continue;
            }
            0x5384da => {
            { let a = rm(m,c.r[7].wrapping_add(0x154),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[7].wrapping_add(0x154); wm(m,a,16,r); } }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x2bc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.push(m,0x10u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[5].wrapping_add(0x2bc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5384fe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x538509 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672cb4),16));
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x124),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = rm(m,c.r[6].wrapping_add(0x10),32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x538525; continue; }
            pc = 0x538522; continue;
            }
            0x538522 => {
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x538525; continue;
            }
            0x538525 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x53852f);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_mul(2).wrapping_add(0x672cb4),16));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = rm(m,c.r[6].wrapping_add(0x10),32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x538544; continue; }
            pc = 0x538541; continue;
            }
            0x538541 => {
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x538544; continue;
            }
            0x538544 => {
            { let a = rm(m,c.r[3].wrapping_mul(2).wrapping_add(0x672cb4),16); c.sub(a,0x7d0u32,16); }
            if c.cond(3) { pc = 0x538604; continue; }
            pc = 0x538554; continue;
            }
            0x538554 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,0x7u32);
            pc = 0x538558; continue;
            }
            0x538558 => {
            { let a = rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x13c),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x53859b; continue; }
            pc = 0x538562; continue;
            }
            0x538562 => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[7]).wrapping_add(0x13d),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x53859b; continue; }
            pc = 0x53856b; continue;
            }
            0x53856b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x538596; continue; }
            pc = 0x538572; continue;
            }
            0x538572 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(3,16,rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672cb4),16));
            { let a = rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x124),32); let r = c.sub(a,c.get(3,32),32); { let a = c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x124); wm(m,a,32,r); } }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = c.r[7].wrapping_add(c.r[1]).wrapping_add(0x13c); wm(m,a,8,c.get(0,8)); }
            pc = 0x53859b; continue;
            }
            0x538596 => {
            c.set(1,32,0x18u32);
            pc = 0x53859b; continue;
            }
            0x53859b => {
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x538558; continue; }
            pc = 0x5385a3; continue;
            }
            0x5385a3 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x5385a5; continue;
            }
            0x5385a5 => {
            { let a = rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x13c),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x5385ea; continue; }
            pc = 0x5385af; continue;
            }
            0x5385af => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(2,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(6) { pc = 0x5385e5; continue; }
            pc = 0x5385bc; continue;
            }
            0x5385bc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672cb4),16));
            c.set(3,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x124),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x124); wm(m,a,32,c.get(3,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = c.r[7].wrapping_add(c.r[1]).wrapping_add(0x13c); wm(m,a,8,c.get(0,8)); }
            pc = 0x5385ea; continue;
            }
            0x5385e5 => {
            c.set(1,32,0x18u32);
            pc = 0x5385ea; continue;
            }
            0x5385ea => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x5385a5; continue; }
            pc = 0x5385f0; continue;
            }
            0x5385f0 => {
            c.push(m,0x10u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5385f9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x538604 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x538606; continue;
            }
            0x538606 => {
            { let a = rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x13c),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x53865d; continue; }
            pc = 0x538610; continue;
            }
            0x538610 => {
            { let a = rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x13d),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x53865d; continue; }
            pc = 0x53861a; continue;
            }
            0x53861a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(2,32,c.r[1].wrapping_add(0xffffffff));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(6) { pc = 0x538658; continue; }
            pc = 0x538627; continue;
            }
            0x538627 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_mul(2).wrapping_add(0x672cb4),16));
            c.set(3,32,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x124),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x124); wm(m,a,32,c.get(3,32)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x13c); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[0].wrapping_add(c.r[7]).wrapping_add(0x13d); wm(m,a,8,0x6u32); }
            pc = 0x53865d; continue;
            }
            0x538658 => {
            c.set(0,32,0x18u32);
            pc = 0x53865d; continue;
            }
            0x53865d => {
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x538606; continue; }
            pc = 0x538665; continue;
            }
            0x538665 => {
            c.push(m,0x10u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x53866e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x538679 => {
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
pub static TRADE: [(u32, super::core_table::CoreFn); 11] = [
(0x537340,op_sell_cargo_to_town),
(0x537450,op_buy_cargo_from_town),
(0x53d2b0,op_sell_office_stock_to_town),
(0x53d3c0,op_buy_ware_for_office),
(0x52f3e0,fit_buy_amount_to_budget),
(0x517fe0,ship_add_wares_by_total_cost),
(0x5308a0,find_or_create_office),
(0x5007a0,office_add_ware_with_total_cost),
(0x5006f0,office_close_weekly_period),
(0x539300,op_convoy_trade_wares_with_town),
(0x538210,op_transfer_weapons_ship_office),
];
