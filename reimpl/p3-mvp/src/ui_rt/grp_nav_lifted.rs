//! Generado por `reimpl/tools/ui_rt_grp.py nav` con `tools/lift.py` (no editar a mano): grupo `nav`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_NAV: [(u32, super::core_table::CoreFn); 2] = [
    (0x5803b0, f_005803b0),
    (0x580520, f_00580520),
];
pub fn f_005803b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5803b0;
    loop {
        match pc {
            0x5803b0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            c.set(1,32,rm(m,0x6e2f28u32,32));
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x16c),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x58044e; continue; }
            pc = 0x5803dc; continue;
            }
            0x5803dc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5803eb; continue; }
            pc = 0x5803e2; continue;
            }
            0x5803e2 => {
            c.set(2,32,rm(m,0x6e2f2cu32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x5803eb; continue;
            }
            0x5803eb => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x580403; continue; }
            pc = 0x5803f1; continue;
            }
            0x5803f1 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x5803fe; continue; }
            pc = 0x5803f4; continue;
            }
            0x5803f4 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x58043d; continue; }
            pc = 0x5803f7; continue;
            }
            0x5803f7 => {
            { let a = c.get(3,8); c.sub(a,0x1u32,8); }
            if c.cond(6) { pc = 0x58043d; continue; }
            pc = 0x5803fc; continue;
            }
            0x5803fc => {
            pc = 0x580408; continue;
            }
            0x5803fe => {
            { let a = c.get(3,8); c.sub(a,0x2u32,8); }
            pc = 0x580406; continue;
            }
            0x580403 => {
            { let a = c.get(3,8); c.sub(a,0x1u32,8); }
            pc = 0x580406; continue;
            }
            0x580406 => {
            if c.cond(5) { pc = 0x58043d; continue; }
            pc = 0x580408; continue;
            }
            0x580408 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x8),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x580426; continue; }
            pc = 0x580410; continue;
            }
            0x580410 => {
            c.push(m, 0x580415);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x58042d; continue; }
            pc = 0x580426; continue;
            }
            0x580426 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0xc),8));
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            pc = 0x58042d; continue;
            }
            0x58042d => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x8),8));
            c.set(1,32,rm(m,0x6e2f28u32,32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            pc = 0x58043d; continue;
            }
            0x58043d => {
            { let a = rm(m,c.r[4].wrapping_add(0xc),8); let r = c.inc(a,8); { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,r); } }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5803e2; continue; }
            pc = 0x58044e; continue;
            }
            0x58044e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58046c);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0x8000003fu32,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x580478; continue; }
            pc = 0x580473; continue;
            }
            0x580473 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffc0u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x580478; continue;
            }
            0x580478 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xffffffe0));
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x580487);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x2cu32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffea));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20),8));
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.sub(a,rm(m,0x6e2f28u32,32),32); }
            if c.cond(2) { pc = 0x5804aa; continue; }
            pc = 0x5804a6; continue;
            }
            0x5804a6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5804b3; continue;
            }
            0x5804aa => {
            c.set(2,32,rm(m,0x6e2f2cu32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x5804b3; continue;
            }
            0x5804b3 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x30),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(6,32); let r = c.add(a,0x8u32,32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5804e8);
            { let args = []; c.r[0] = host.call(m, 0x559260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x5804fb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00580520(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x580520;
    loop {
        match pc {
            0x580520 => {
            { let a = c.get(4,32); let r = c.sub(a,0x48u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e2f28u32,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x580542; continue; }
            pc = 0x580538; continue;
            }
            0x580538 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            pc = 0x58054f; continue;
            }
            0x580542 => {
            c.set(1,32,rm(m,0x6e2f2cu32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            pc = 0x58054f; continue;
            }
            0x58054f => {
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0x414),32));
            { let a = c.get(5,32); c.sub(a,0x2710u32,32); }
            c.set(7,32,c.get(5,32));
            if c.cond(2) { pc = 0x580569; continue; }
            pc = 0x580564; continue;
            }
            0x580564 => {
            c.set(7,32,0x2710u32);
            pc = 0x580569; continue;
            }
            0x580569 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x580577);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x20u32,32); }
            if c.cond(15) { pc = 0x580629; continue; }
            pc = 0x580583; continue;
            }
            0x580583 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x580591);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x16u32,32); }
            if c.cond(15) { pc = 0x580629; continue; }
            pc = 0x58059d; continue;
            }
            0x58059d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x24),32));
            c.set(3,32,0x2710u32);
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5805ba);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x24),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x68db8badu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xcu32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5805f8);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(1,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x68db8badu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xcu32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            pc = 0x58065b; continue;
            }
            0x580629 => {
            c.push(m, 0x58062e);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0x8000003fu32,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x58063a; continue; }
            pc = 0x580635; continue;
            }
            0x580635 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffc0u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x58063a; continue;
            }
            0x58063a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffe0));
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x580649);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x2cu32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffea));
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            pc = 0x58065b; continue;
            }
            0x58065b => {
            { let a = c.get(5,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(0,32,0xd1b71759u32);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x24),32));
            { let b = c.get(5,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,32); let n = 0xdu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.imul(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.imul(a,c.get(2,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(2,32,c.get(7,32));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5806a8; continue; }
            pc = 0x58068b; continue;
            }
            0x58068b => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,c.get(5,32));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5806a8; continue; }
            pc = 0x580698; continue;
            }
            0x580698 => {
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5806a8 => {
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,8,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x5806c3);
            { let args = []; c.r[0] = host.call(m, 0x559260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(2,32); let r = c.add(a,0x1eu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.add(a,0x1eu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x96u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x34),16));
            { let a = c.get(2,32); let r = c.add(a,0x96u32,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xe),16));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.get(1,32));
            c.set(7,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            if c.cond(14) { pc = 0x58071d; continue; }
            pc = 0x580715; continue;
            }
            0x580715 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xe); wm(m,a,16,c.get(0,16)); }
            pc = 0x580728; continue;
            }
            0x58071d => {
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(13) { pc = 0x58072a; continue; }
            pc = 0x580722; continue;
            }
            0x580722 => {
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xe); wm(m,a,16,c.get(1,16)); }
            pc = 0x580728; continue;
            }
            0x580728 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x58072a; continue;
            }
            0x58072a => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x36),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x12),16));
            c.set(2,32,c.get(1,32));
            c.set(7,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x580749; continue; }
            pc = 0x580741; continue;
            }
            0x580741 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,c.get(0,16)); }
            pc = 0x580754; continue;
            }
            0x580749 => {
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(13) { pc = 0x580756; continue; }
            pc = 0x58074e; continue;
            }
            0x58074e => {
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,c.get(1,16)); }
            pc = 0x580754; continue;
            }
            0x580754 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x580756; continue;
            }
            0x580756 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x58076c);
            { let args = []; c.r[0] = host.call(m, 0x559260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(0,32); let r = c.add(a,0x1eu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x1eu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x96u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x96u32,32); c.set(0,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x5807f2; continue; }
            pc = 0x5807a4; continue;
            }
            0x5807a4 => {
            c.set(6,32,rm(m,0x66a4ccu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = c.get(6,32); c.push(m, 0x5807bb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x5807cb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            { let a = rm(m,c.r[2].wrapping_add(0x16c),32); c.sub(a,0x2u32,32); }
            if c.cond(12) { pc = 0x580840; continue; }
            pc = 0x5807da; continue;
            }
            0x5807da => {
            c.set(0,32,c.r[4].wrapping_add(0x48));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = c.get(6,32); c.push(m, 0x5807eb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            pc = 0x580835; continue;
            }
            0x5807f2 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x580802);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            c.push(m, 0x580812);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            { let a = rm(m,c.r[0].wrapping_add(0x16c),32); c.sub(a,0x2u32,32); }
            if c.cond(12) { pc = 0x580840; continue; }
            pc = 0x580820; continue;
            }
            0x580820 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x580830);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x48));
            c.push(m,c.get(2,32));
            pc = 0x580835; continue;
            }
            0x580835 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x580840);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x580840; continue;
            }
            0x580840 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
