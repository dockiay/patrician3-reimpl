//! Generado por `reimpl/tools/ui_rt_grp.py rain` con `tools/lift.py` (no editar a mano): grupo `rain`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_RAIN: [(u32, super::core_table::CoreFn); 8] = [
    (0x575a80, f_00575a80),
    (0x575b00, f_00575b00),
    (0x5732b0, f_005732b0),
    (0x573350, f_00573350),
    (0x576250, f_00576250),
    (0x576300, f_00576300),
    (0x57fc60, f_0057fc60),
    (0x57feb0, f_0057feb0),
];
pub fn f_00575a80(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x575a80;
    loop {
        match pc {
            0x575a80 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x10),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(3,8,c.get(4,8));
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.r[1].wrapping_add(0x23); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[1].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[1].wrapping_add(0x15); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[1].wrapping_add(0x21); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[1].wrapping_add(0x22); wm(m,a,8,c.get(2,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00575b00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x575b00;
    loop {
        match pc {
            0x575b00 => {
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x414),32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x23),8),8)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x21),8),8)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x575b37); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x575b40);
            { let args = []; c.r[0] = host.call(m, 0x6592a3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(5,16,rm(m,c.r[6].wrapping_add(0xe),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x21),8),8)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x12),16));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(1,16)); }
            c.push(m, 0x575b69);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3fu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,0xd20d20d3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x23),8),8)&0xffffffffu32);
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(7,32,c.get(2,32));
            { let a = c.get(7,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m, 0x575bab);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3fu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,0xd20d20d3u32);
            { let a = c.get(1,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x21),8));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(13) { pc = 0x575bed; continue; }
            pc = 0x575beb; continue;
            }
            0x575beb => {
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            pc = 0x575bed; continue;
            }
            0x575bed => {
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x23),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(13) { pc = 0x575bf6; continue; }
            pc = 0x575bf4; continue;
            }
            0x575bf4 => {
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            pc = 0x575bf6; continue;
            }
            0x575bf6 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x575c30; continue; }
            pc = 0x575c14; continue;
            }
            0x575c14 => {
            c.set(7,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x575c30; continue; }
            pc = 0x575c26; continue;
            }
            0x575c26 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x575c30 => {
            c.set(0,16,sx(c.get(0,8),8)&0xffffu32);
            c.set(2,32,c.r[0].wrapping_mul(4));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            c.set(0,16,sx(c.get(3,8),8)&0xffffu32);
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x20),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0x20); wm(m,a,16,r); } }
            c.set(2,32,c.r[0].wrapping_mul(4));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x22),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0x22); wm(m,a,16,r); } }
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x23),8),8)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x21),8),8)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x575c79); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x575c82);
            { let args = []; c.r[0] = host.call(m, 0x6592a3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4ccu32,32); c.push(m, 0x575ca9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0xc650),32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0xc654),32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x575cc0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4c8u32,32); c.push(m, 0x575cd1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x575ce1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x575cf8; continue; }
            pc = 0x575ce5; continue;
            }
            0x575ce5 => {
            { let a = rm(m,c.r[6].wrapping_add(0x23),8); c.sub(a,0x3u32,8); }
            if c.cond(14) { pc = 0x575cf8; continue; }
            pc = 0x575ceb; continue;
            }
            0x575ceb => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x575cf8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005732b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5732b0;
    loop {
        match pc {
            0x5732b0 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6e2df0u32,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5732bf);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x18),16));
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5732d4);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0x8000ffffu32,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x5732e2; continue; }
            pc = 0x5732db; continue;
            }
            0x5732db => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xffff0000u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x5732e2; continue;
            }
            0x5732e2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x8));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20),8));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6e2df0u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(2) { pc = 0x573323; continue; }
            pc = 0x573313; continue;
            }
            0x573313 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57331d);
            { let args = []; c.r[0] = host.call(m, 0x559130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            0x573323 => {
            c.set(2,32,rm(m,0x6e2df4u32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x573335);
            { let args = []; c.r[0] = host.call(m, 0x559130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00573350(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x573350;
    loop {
        match pc {
            0x573350 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e2df0u32,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x57336c; continue; }
            pc = 0x573368; continue;
            }
            0x573368 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x573375; continue;
            }
            0x57336c => {
            c.set(1,32,rm(m,0x6e2df4u32,32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x573375; continue;
            }
            0x573375 => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.set(3,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(3,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(5,32,rm(m,c.r[2].wrapping_add(0x414),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x573390);
            { let args = []; c.r[0] = host.call(m, 0x559370, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(0,32,0xd1b71759u32);
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(5,32),32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xe),16));
            { let a = c.get(2,32); let n = 0xdu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x573405; continue; }
            pc = 0x5733c0; continue;
            }
            0x5733c0 => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5733cc);
            { let args = []; c.r[0] = host.call(m, 0x559130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e51acu32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[0].wrapping_add(0xc31c),32),32); }
            if c.cond(14) { pc = 0x573405; continue; }
            pc = 0x5733e0; continue;
            }
            0x5733e0 => {
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,16,0x0u32); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x24),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(1,16); let r = c.neg(a,16); c.set(1,16,r); }
            { let a = c.r[6].wrapping_add(0xe); wm(m,a,16,c.get(1,16)); }
            c.push(m, 0x573402);
            { let args = []; c.r[0] = host.call(m, 0x559130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x573405; continue;
            }
            0x573405 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00576250(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x576250;
    loop {
        match pc {
            0x576250 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6e2e30u32,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x57625f);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let b = c.get(7,32); c.div32(b); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x18),16));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,0x6e2e30u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x57629a; continue; }
            pc = 0x576296; continue;
            }
            0x576296 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x5762a3; continue;
            }
            0x57629a => {
            c.set(1,32,rm(m,0x6e2e34u32,32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            pc = 0x5762a3; continue;
            }
            0x5762a3 => {
            c.push(m, 0x5762a8);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let b = rm(m,c.r[1],32); c.div32(b); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x25); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,c.get(4,8));
            { let a = c.r[6].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00576300(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x576300;
    loop {
        match pc {
            0x576300 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e2e30u32,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x24),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x57631c; continue; }
            pc = 0x576318; continue;
            }
            0x576318 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x576325; continue;
            }
            0x57631c => {
            c.set(1,32,rm(m,0x6e2e34u32,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x576325; continue;
            }
            0x576325 => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x25),8));
            { let a = c.get(0,8); c.sub(a,0x80u32,8); }
            c.set(3,32,rm(m,c.r[2].wrapping_add(0x414),32));
            if c.cond(5) { pc = 0x57639f; continue; }
            pc = 0x576338; continue;
            }
            0x576338 => {
            c.set(0,32,rm(m,0x6dccf8u32,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x576518; continue; }
            pc = 0x576348; continue;
            }
            0x576348 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x1c),32); c.push(m, 0x57634f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e2e38u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x24),8));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x26),8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x576395; continue; }
            pc = 0x576371; continue;
            }
            0x576371 => {
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); let r = c.sub(a,c.get(2,8),8); c.set(0,8,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[6].wrapping_add(0x26); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(1,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x18),32); c.push(m, 0x57638d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x576395 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x57639f => {
            c.set(0,32,c.r[6].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5763b2);
            { let args = []; c.r[0] = host.call(m, 0x559370, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x25),8),8)&0xffffffffu32);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xe),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x5763ce);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0xc),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,0xd20d20d3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(7,32,c.get(2,32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x26),8),8)&0xffffffffu32);
            c.push(m,c.get(2,32));
            { let a = c.get(7,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m, 0x5763ff);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,0xd20d20d3u32);
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0xc),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x25),8));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(13) { pc = 0x576430; continue; }
            pc = 0x57642e; continue;
            }
            0x57642e => {
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            pc = 0x576430; continue;
            }
            0x576430 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x26),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(13) { pc = 0x576439; continue; }
            pc = 0x576437; continue;
            }
            0x576437 => {
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            pc = 0x576439; continue;
            }
            0x576439 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x20),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x5764d1; continue; }
            pc = 0x576458; continue;
            }
            0x576458 => {
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(3,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x20),32));
            c.set(0,8,c.get(3,8));
            c.set(1,32,rm(m,c.r[2],32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x576474; continue; }
            pc = 0x576470; continue;
            }
            0x576470 => {
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,0x0u32); }
            pc = 0x576474; continue;
            }
            0x576474 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(1,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x24),8));
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6e2e38u32,32));
            c.set(3,8,0x1u32);
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x5764d1; continue; }
            pc = 0x576495; continue;
            }
            0x576495 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x5764a0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x24),8));
            c.set(0,32,rm(m,0x6e2e38u32,32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)),32),32); c.set(1,32,r); }
            c.set(0,32,0x3ab95901u32);
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(7,32,c.get(2,32));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            c.push(m, 0x5764c1);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5764d1; continue; }
            pc = 0x5764c9; continue;
            }
            0x5764c9 => {
            { let a = c.r[6].wrapping_add(0x25); wm(m,a,8,0x80u32); }
            { let a = c.r[6].wrapping_add(0x26); wm(m,a,8,0xffu32); }
            pc = 0x5764d1; continue;
            }
            0x5764d1 => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5764f7; continue; }
            pc = 0x5764e2; continue;
            }
            0x5764e2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5764f7; continue; }
            pc = 0x5764f3; continue;
            }
            0x5764f3 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x5764f9; continue;
            }
            0x5764f7 => {
            c.set(0,8,0x1u32);
            pc = 0x5764f9; continue;
            }
            0x5764f9 => {
            { let a = c.get(0,8); let r = c.or(a,c.get(3,8),8); c.set(0,8,r); }
            if c.cond(4) { pc = 0x576518; continue; }
            pc = 0x5764fd; continue;
            }
            0x5764fd => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x57650d);
            { let args = []; c.r[0] = host.call(m, 0x559130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x576518 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0057fc60(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57fc60;
    loop {
        match pc {
            0x57fc60 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x10),16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x57fca2);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,0x6dccf8u32,32));
            { let b = rm(m,0x6e2ee0u32,32); c.div32(b); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,0x6e2ed0u32);
            c.push(m,0x0u32);
            c.push(m, 0x57fcc5);
            { let args = []; c.r[0] = host.call(m, 0x559130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e2f18u32,32));
            c.set(1,32,rm(m,0x6e2f1cu32,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            c.set(6,32,c.get(0,32));
            if c.cond(3) { pc = 0x57fde2; continue; }
            pc = 0x57fce7; continue;
            }
            0x57fce7 => {
            c.set(2,32,rm(m,0x6e2f14u32,32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x57fd06; continue; }
            pc = 0x57fcf1; continue;
            }
            0x57fcf1 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            if c.cond(6) { pc = 0x57fd06; continue; }
            pc = 0x57fcfd; continue;
            }
            0x57fcfd => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            pc = 0x57fd0b; continue;
            }
            0x57fd06 => {
            c.set(1,32,0x1u32);
            pc = 0x57fd0b; continue;
            }
            0x57fd0b => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x57fd13; continue; }
            pc = 0x57fd0f; continue;
            }
            0x57fd0f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x57fd18; continue;
            }
            0x57fd13 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            pc = 0x57fd18; continue;
            }
            0x57fd18 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x57fd24; continue; }
            pc = 0x57fd22; continue;
            }
            0x57fd22 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x57fd24; continue;
            }
            0x57fd24 => {
            c.push(m,c.get(5,32));
            c.set(2,32,c.r[0].wrapping_mul(4));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x57fd33);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,0x6e2f14u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(5,32,c.get(0,32));
            { let a = c.get(3,32); c.sub(a,c.get(6,32),32); }
            c.set(7,32,c.get(5,32));
            if c.cond(4) { pc = 0x57fd58; continue; }
            pc = 0x57fd44; continue;
            }
            0x57fd44 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x57fd4b);
            { let args = []; c.r[0] = host.call(m, 0x580100, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x57fd44; continue; }
            pc = 0x57fd58; continue;
            }
            0x57fd58 => {
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,0x6e2f10u32);
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x57fd6a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5800d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e2f18u32,32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6e2f10u32);
            c.push(m, 0x57fd80);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5800a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,0x6e2f18u32,32));
            c.set(0,32,rm(m,0x6e2f14u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6e2f10u32);
            c.push(m, 0x57fd97);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x586b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6e2f14u32,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x57fda3);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,0x6e2f14u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = 0x6e2f1cu32; wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x57fdc1; continue; }
            pc = 0x57fdbd; continue;
            }
            0x57fdbd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x57fdcb; continue;
            }
            0x57fdc1 => {
            c.set(0,32,rm(m,0x6e2f18u32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            pc = 0x57fdcb; continue;
            }
            0x57fdcb => {
            c.set(1,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            { let a = 0x6e2f14u32; wm(m,a,32,c.get(5,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x6e2f18u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            0x57fde2 => {
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            if c.cond(3) { pc = 0x57fe4d; continue; }
            pc = 0x57fdee; continue;
            }
            0x57fdee => {
            c.set(1,32,c.r[6].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6e2f10u32);
            c.push(m, 0x57fdfe);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5800a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,0x6e2f18u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(2,32,0x1u32);
            { let a = c.get(1,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,0x6e2f10u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57fe22);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5800d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e2f18u32,32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x57fe90; continue; }
            pc = 0x57fe2e; continue;
            }
            0x57fe2e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x57fe2e; continue; }
            pc = 0x57fe3b; continue;
            }
            0x57fe3b => {
            c.set(0,32,rm(m,0x6e2f18u32,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x6e2f18u32; wm(m,a,32,c.get(0,32)); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            0x57fe4d => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.add(a,0xfffffffcu32,32); c.set(0,32,r); }
            c.set(1,32,0x6e2f10u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x57fe5d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5800a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,0x6e2f18u32,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.add(a,0xfffffffcu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x57fe7a; continue; }
            pc = 0x57fe6b; continue;
            }
            0x57fe6b => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            { let a = c.get(0,32); let r = c.sub(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x57fe6b; continue; }
            pc = 0x57fe7a; continue;
            }
            0x57fe7a => {
            c.set(1,32,c.r[6].wrapping_add(0x4));
            c.set(0,32,c.get(6,32));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x57fe90; continue; }
            pc = 0x57fe83; continue;
            }
            0x57fe83 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x57fe83; continue; }
            pc = 0x57fe90; continue;
            }
            0x57fe90 => {
            c.set(0,32,rm(m,0x6e2f18u32,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x6e2f18u32; wm(m,a,32,c.get(0,32)); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0057feb0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57feb0;
    loop {
        match pc {
            0x57feb0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x28),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x57febe; continue; }
            pc = 0x57feba; continue;
            }
            0x57feba => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x57febe => {
            c.set(0,32,rm(m,0x6dccf8u32,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x57fece; continue; }
            pc = 0x57feca; continue;
            }
            0x57feca => {
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x57fece => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6e2ef0u32,32));
            c.set(0,8,c.get(2,8));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x57feed; continue; }
            pc = 0x57fee9; continue;
            }
            0x57fee9 => {
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,0x0u32); }
            pc = 0x57feed; continue;
            }
            0x57feed => {
            c.set(2,32,rm(m,0x6dccf8u32,32));
            c.set(0,32,rm(m,0x6e2ee0u32,32));
            c.set(1,32,c.r[6].wrapping_add(0x8));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0x6e2ed0u32);
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x57ff0d);
            { let args = []; c.r[0] = host.call(m, 0x559130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,8,0x1u32);
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
