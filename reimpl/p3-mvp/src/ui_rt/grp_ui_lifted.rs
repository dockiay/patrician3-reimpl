//! Generado por `reimpl/tools/ui_rt_grp.py ui` con `tools/lift.py` (no editar a mano): grupo `ui`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_UI: [(u32, super::core_table::CoreFn); 44] = [
    (0x42cd90, f_0042cd90),
    (0x441ab0, f_00441ab0),
    (0x442130, f_00442130),
    (0x442280, f_00442280),
    (0x442460, f_00442460),
    (0x4425d0, f_004425d0),
    (0x442690, f_00442690),
    (0x442a10, f_00442a10),
    (0x442d10, f_00442d10),
    (0x443000, f_00443000),
    (0x4430d0, f_004430d0),
    (0x443110, f_00443110),
    (0x4438d0, f_004438d0),
    (0x446a00, f_00446a00),
    (0x44d400, f_0044d400),
    (0x451eb0, f_00451eb0),
    (0x4538e0, f_004538e0),
    (0x459790, f_00459790),
    (0x45a5f0, f_0045a5f0),
    (0x460540, f_00460540),
    (0x46eb50, f_0046eb50),
    (0x46f4c0, f_0046f4c0),
    (0x4c4130, f_004c4130),
    (0x4c4540, f_004c4540),
    (0x4c45a0, f_004c45a0),
    (0x4cdb20, f_004cdb20),
    (0x550550, f_00550550),
    (0x5506d0, f_005506d0),
    (0x5708f0, f_005708f0),
    (0x570ba0, f_00570ba0),
    (0x57af40, f_0057af40),
    (0x580b00, f_00580b00),
    (0x580bd0, f_00580bd0),
    (0x580d20, f_00580d20),
    (0x58f3c0, f_0058f3c0),
    (0x58f440, f_0058f440),
    (0x595b00, f_00595b00),
    (0x59bc90, f_0059bc90),
    (0x59c410, f_0059c410),
    (0x5a7350, f_005a7350),
    (0x441ac0, f_00441ac0),
    (0x4c4460, f_004c4460),
    (0x4c4620, f_004c4620),
    (0x57ff20, f_0057ff20),
];
pub fn f_0042cd90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42cd90;
    loop {
        match pc {
            0x42cd90 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x42ce40; continue; }
            pc = 0x42cdab; continue;
            }
            0x42cdab => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6de4b8),8));
            c.push(m,0x0u32);
            c.set(0,32,c.get(1,32));
            c.push(m,0x0u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6ddbc2),8));
            c.set(2,32,c.r[0].wrapping_add(0x6ddbb8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x42cdde);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x42cde9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4426c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(6,32,c.get(0,32));
            c.push(m, 0x42cdf0);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            c.fild32(m,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffec78));
            { let a = c.r[7].wrapping_add(0x28c4); wm(m,a,32,c.get(1,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0xc));
            c.fld(c.ld(m,c.r[4].wrapping_add(0xc),4));
            { let v = c.ld(m,0x66cb34u32,4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0xc));
            c.fld(c.ld(m,c.r[4].wrapping_add(0xc),4));
            { let v = c.ld(m,0x66cb3cu32,4); c.fop('*',v); }
            c.push(m, 0x42ce2b);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x3a98));
            { let a = c.r[7].wrapping_add(0x2884); wm(m,a,32,c.get(0,32)); }
            pc = 0x42ce40; continue;
            }
            0x42ce40 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00441ab0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x441ab0;
    loop {
        match pc {
            0x441ab0 => {
            c.push(m, 0x441ab5);
            { let args = []; c.r[0] = host.call(m, 0x441ac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let target = rm(m,0x66a638u32,32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00442130(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x442130;
    loop {
        match pc {
            0x442130 => {
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x44213f; continue; }
            pc = 0x44213a; continue;
            }
            0x44213a => {
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x44213f => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,0x66a5c8u32,32));
            c.push(m,c.get(6,32));
            { let a = c.r[7].wrapping_add(0x1b4); wm(m,a,16,0x0u32); }
            c.set(6,32,c.r[7].wrapping_add(0x18));
            c.set(3,32,0x14u32);
            pc = 0x442159; continue;
            }
            0x442159 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x44215e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(5) { pc = 0x44216a; continue; }
            pc = 0x442163; continue;
            }
            0x442163 => {
            { let a = rm(m,c.r[7].wrapping_add(0x1b4),16); let r = c.inc(a,16); { let a = c.r[7].wrapping_add(0x1b4); wm(m,a,16,r); } }
            pc = 0x44216a; continue;
            }
            0x44216a => {
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x442159; continue; }
            pc = 0x442170; continue;
            }
            0x442170 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x1b4),16));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00442280(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x442280;
    loop {
        match pc {
            0x442280 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[6].wrapping_add(0x68),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x442451; continue; }
            pc = 0x442293; continue;
            }
            0x442293 => {
            c.push(m, 0x442298);
            { let args = []; c.r[0] = host.call(m, 0x442180, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x442451; continue; }
            pc = 0x4422b8; continue;
            }
            0x4422b8 => {
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x4422d5; continue; }
            pc = 0x4422bf; continue;
            }
            0x4422bf => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a61cu32,32); c.push(m, 0x4422c8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a620u32,32); c.push(m, 0x4422d2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x4422d5; continue;
            }
            0x4422d5 => {
            { let a = rm(m,c.r[6].wrapping_add(0x1bc),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x442451; continue; }
            pc = 0x4422e1; continue;
            }
            0x4422e1 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1de),8));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(1,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x442451; continue; }
            pc = 0x4422fa; continue;
            }
            0x4422fa => {
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            c.push(m,0x3e800u32);
            if c.cond(4) { pc = 0x44231c; continue; }
            pc = 0x442306; continue;
            }
            0x442306 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(7,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            pc = 0x442325; continue;
            }
            0x44231c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x442325; continue;
            }
            0x442325 => {
            { let target = rm(m,0x66a5ccu32,32); c.push(m, 0x44232b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x442451; continue; }
            pc = 0x442338; continue;
            }
            0x442338 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bc),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x442346; continue; }
            pc = 0x442342; continue;
            }
            0x442342 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x442350; continue;
            }
            0x442346 => {
            { let a = c.get(0,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x442350; continue; }
            pc = 0x44234b; continue;
            }
            0x44234b => {
            c.set(0,32,0x7fu32);
            pc = 0x442350; continue;
            }
            0x442350 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x442359); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[6].wrapping_add(0x1de),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4423de; continue; }
            pc = 0x442361; continue;
            }
            0x442361 => {
            c.set(0,32,rm(m,0x66d640u32,32));
            c.set(7,32,rm(m,0x66a5b4u32,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            pc = 0x442374; continue;
            }
            0x442374 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x6c),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x442384); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x442374; continue; }
            pc = 0x442388; continue;
            }
            0x442388 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(7,32,rm(m,0x66a5b8u32,32));
            c.set(5,32,rm(m,0x66a5bcu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4423b8; continue; }
            pc = 0x44239a; continue;
            }
            0x44239a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x6c),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            { let target = c.get(7,32); c.push(m, 0x4423a6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x4423b8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x4423b8; continue;
            }
            0x4423b8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4423de; continue; }
            pc = 0x4423bf; continue;
            }
            0x4423bf => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x6c),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            { let target = c.get(7,32); c.push(m, 0x4423cb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x4423de); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x4423de; continue;
            }
            0x4423de => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x44242d; continue; }
            pc = 0x4423e6; continue;
            }
            0x4423e6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x442400; continue; }
            pc = 0x4423ed; continue;
            }
            0x4423ed => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a61cu32,32); c.push(m, 0x4423f6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a620u32,32); c.push(m, 0x442400); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x442400; continue;
            }
            0x442400 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1da),8));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x1d4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1db); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x1dc); wm(m,a,8,c.get(3,8)); }
            pc = 0x44242d; continue;
            }
            0x44242d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0x1e0); wm(m,a,8,0x1u32); }
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a5c0u32,32); c.push(m, 0x44243e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0x1de),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x442451; continue; }
            pc = 0x442446; continue;
            }
            0x442446 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5c4u32,32); c.push(m, 0x442451); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x442451; continue;
            }
            0x442451 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00442460(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x442460;
    loop {
        match pc {
            0x442460 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65e56du32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4425a7; continue; }
            pc = 0x44248e; continue;
            }
            0x44248e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4425a7; continue; }
            pc = 0x44249c; continue;
            }
            0x44249c => {
            c.set(1,32,sx(rm(m,c.r[4].wrapping_add(0x24),8),8)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x4424c8; continue; }
            pc = 0x4424c3; continue;
            }
            0x4424c3 => {
            c.push(m, 0x4424c8);
            { let args = []; c.r[0] = host.call(m, 0x59c010, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4424c8; continue;
            }
            0x4424c8 => {
            c.set(3,32,0xfffffe60u32);
            c.set(7,32,c.r[6].wrapping_add(0x1a0));
            c.set(5,32,c.r[6].wrapping_add(0x18));
            { let a = c.get(3,32); let r = c.sub(a,c.get(6,32),32); c.set(3,32,r); }
            pc = 0x4424d8; continue;
            }
            0x4424d8 => {
            c.set(1,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5c8u32,32); c.push(m, 0x4424e2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(5) { pc = 0x4424fb; continue; }
            pc = 0x4424e7; continue;
            }
            0x4424e7 => {
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a5acu32,32); c.push(m, 0x4424f1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a610u32,32); c.push(m, 0x4424fb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4424fb; continue;
            }
            0x4424fb => {
            { let a = c.r[7].wrapping_add(0xffffffec); wm(m,a,8,0x0u32); }
            { let a = c.r[7]; wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[7]));
            { let a = c.get(1,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x4424d8; continue; }
            pc = 0x44250e; continue;
            }
            0x44250e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,32,rm(m,0x66a61cu32,32));
            c.set(3,32,rm(m,0x66a620u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x442531; continue; }
            pc = 0x442523; continue;
            }
            0x442523 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x442528); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            { let target = c.get(3,32); c.push(m, 0x44252e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            pc = 0x442531; continue;
            }
            0x442531 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x44253d; continue; }
            pc = 0x442538; continue;
            }
            0x442538 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x44253d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x44253d; continue;
            }
            0x44253d => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x442548; continue; }
            pc = 0x442543; continue;
            }
            0x442543 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x442548); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x442548; continue;
            }
            0x442548 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x44255d; continue; }
            pc = 0x44254f; continue;
            }
            0x44254f => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x442554); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            { let target = c.get(3,32); c.push(m, 0x44255a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            pc = 0x44255d; continue;
            }
            0x44255d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.push(m,0x3e800u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a5ccu32,32); c.push(m, 0x442571); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x4425a7; continue; }
            pc = 0x442578; continue;
            }
            0x442578 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(13) { pc = 0x442584; continue; }
            pc = 0x442580; continue;
            }
            0x442580 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x44258e; continue;
            }
            0x442584 => {
            { let a = c.get(1,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x44258e; continue; }
            pc = 0x442589; continue;
            }
            0x442589 => {
            c.set(1,32,0x7fu32);
            pc = 0x44258e; continue;
            }
            0x44258e => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x442596); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5c0u32,32); c.push(m, 0x4425a0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1df); wm(m,a,8,0x1u32); }
            pc = 0x4425a7; continue;
            }
            0x4425a7 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4425b8);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004425d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4425d0;
    loop {
        match pc {
            0x4425d0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442687; continue; }
            pc = 0x4425de; continue;
            }
            0x4425de => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x66a61cu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442603; continue; }
            pc = 0x4425ed; continue;
            }
            0x4425ed => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x4425f2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a620u32,32); c.push(m, 0x4425fc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            pc = 0x442603; continue;
            }
            0x442603 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(5,32,rm(m,0x66a63cu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442669; continue; }
            pc = 0x442610; continue;
            }
            0x442610 => {
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x442613); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(5) { pc = 0x442669; continue; }
            pc = 0x442618; continue;
            }
            0x442618 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x442624); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            { let target = c.get(7,32); c.push(m, 0x44262c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x442656; continue; }
            pc = 0x442633; continue;
            }
            0x442633 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x442669; continue; }
            pc = 0x44263a; continue;
            }
            0x44263a => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x442656; continue; }
            pc = 0x44263e; continue;
            }
            0x44263e => {
            c.set(0,32,c.get(1,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1db),8));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1da); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            pc = 0x442656; continue;
            }
            0x442656 => {
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,0x1u32); }
            c.set(2,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x1d0); wm(m,a,32,c.get(2,32)); }
            pc = 0x442669; continue;
            }
            0x442669 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44267e; continue; }
            pc = 0x44266f; continue;
            }
            0x44266f => {
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x442672); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(5) { pc = 0x44267e; continue; }
            pc = 0x442677; continue;
            }
            0x442677 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x44267e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x44267e; continue;
            }
            0x44267e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[6].wrapping_add(0x1df); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x442687; continue;
            }
            0x442687 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00442690(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x442690;
    loop {
        match pc {
            0x442690 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4426b5; continue; }
            pc = 0x44269a; continue;
            }
            0x44269a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4426b5; continue; }
            pc = 0x4426a1; continue;
            }
            0x4426a1 => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a63cu32,32); c.push(m, 0x4426a8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x4426b5; continue; }
            pc = 0x4426ad; continue;
            }
            0x4426ad => {
            c.set(1,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x4425d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4426b5 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00442a10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x442a10;
    loop {
        match pc {
            0x442a10 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65e591u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442a53; continue; }
            pc = 0x442a32; continue;
            }
            0x442a32 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1e0),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x442a53; continue; }
            pc = 0x442a3c; continue;
            }
            0x442a3c => {
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.push(m, 0x442a49);
            { let args = []; c.r[0] = host.call(m, 0x442180, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x442a78; continue; }
            pc = 0x442a53; continue;
            }
            0x442a53 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x442a64);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x442a78 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,0x66a620u32,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,0x66a61cu32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442ac9; continue; }
            pc = 0x442a8d; continue;
            }
            0x442a8d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x442a9f; continue; }
            pc = 0x442a94; continue;
            }
            0x442a94 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x442a99); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            { let target = c.get(3,32); c.push(m, 0x442a9f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x442a9f; continue;
            }
            0x442a9f => {
            c.set(1,32,rm(m,c.r[6],32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1d8),8));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x1c6); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x1cc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x1d9); wm(m,a,8,c.get(0,8)); }
            pc = 0x442ac9; continue;
            }
            0x442ac9 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.push(m,0x3e800u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a5ccu32,32); c.push(m, 0x442add); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x442beb; continue; }
            pc = 0x442ae7; continue;
            }
            0x442ae7 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5a4u32,32); c.push(m, 0x442af3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5a8u32,32); c.push(m, 0x442b03); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x442b57; continue; }
            pc = 0x442b16; continue;
            }
            0x442b16 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x442b28; continue; }
            pc = 0x442b1d; continue;
            }
            0x442b1d => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x442b22); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            { let target = c.get(3,32); c.push(m, 0x442b28); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x442b28; continue;
            }
            0x442b28 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1da),8));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x1d4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x1db); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[6].wrapping_add(0x1dc); wm(m,a,8,0x1u32); }
            pc = 0x442b57; continue;
            }
            0x442b57 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5e4u32,32); c.push(m, 0x442b61); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x3c),8));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[6].wrapping_add(0x1d8); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x1c6); wm(m,a,8,0x1u32); }
            c.set(2,32,rm(m,0x6dccf0u32,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x1c8); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,0x66a5c0u32,32); c.push(m, 0x442b87); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a5c4u32,32); c.push(m, 0x442b91); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1c5),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x442beb; continue; }
            pc = 0x442b9b; continue;
            }
            0x442b9b => {
            c.set(2,32,rm(m,0x66d640u32,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,0x66a5b4u32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            pc = 0x442baf; continue;
            }
            0x442baf => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x6c),32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = c.get(7,32); c.push(m, 0x442bbf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x442baf; continue; }
            pc = 0x442bc3; continue;
            }
            0x442bc3 => {
            c.set(1,32,rm(m,c.r[6],32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x6c),32));
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a5b8u32,32); c.push(m, 0x442bd5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5bcu32,32); c.push(m, 0x442beb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x442beb; continue;
            }
            0x442beb => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x442c00);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00442d10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x442d10;
    loop {
        match pc {
            0x442d10 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x68),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x442d82; continue; }
            pc = 0x442d1b; continue;
            }
            0x442d1b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,0x66a620u32,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x66a61cu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x442d3e; continue; }
            pc = 0x442d30; continue;
            }
            0x442d30 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x442d35); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x442d3b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            pc = 0x442d3e; continue;
            }
            0x442d3e => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x442d50; continue; }
            pc = 0x442d44; continue;
            }
            0x442d44 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x442d49); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            { let target = c.get(5,32); c.push(m, 0x442d4e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6]; wm(m,a,32,c.get(3,32)); }
            pc = 0x442d50; continue;
            }
            0x442d50 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x442d65; continue; }
            pc = 0x442d57; continue;
            }
            0x442d57 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x442d5c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            { let target = c.get(5,32); c.push(m, 0x442d62); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            pc = 0x442d65; continue;
            }
            0x442d65 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x442d7a; continue; }
            pc = 0x442d6c; continue;
            }
            0x442d6c => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x442d71); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x442d77); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            pc = 0x442d7a; continue;
            }
            0x442d7a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,c.get(3,8)); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x442d82; continue;
            }
            0x442d82 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00443000(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x443000;
    loop {
        match pc {
            0x443000 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x68),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4430c6; continue; }
            pc = 0x44300e; continue;
            }
            0x44300e => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(7,32,0x80000000u32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(0x18));
            pc = 0x44301c; continue;
            }
            0x44301c => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[2],32),32); }
            if c.cond(4) { pc = 0x44302b; continue; }
            pc = 0x443020; continue;
            }
            0x443020 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x44301c; continue; }
            pc = 0x443029; continue;
            }
            0x443029 => {
            pc = 0x44302f; continue;
            }
            0x44302b => {
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x7c),32));
            pc = 0x44302f; continue;
            }
            0x44302f => {
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(4) { pc = 0x4430c6; continue; }
            pc = 0x443038; continue;
            }
            0x443038 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x134),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x443056; continue; }
            pc = 0x443044; continue;
            }
            0x443044 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x124),32));
            pc = 0x44304a; continue;
            }
            0x44304a => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[2],32),32); }
            if c.cond(4) { pc = 0x443056; continue; }
            pc = 0x44304e; continue;
            }
            0x44304e => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x44304a; continue; }
            pc = 0x443056; continue;
            }
            0x443056 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x78),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x1bc),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m,c.get(3,32));
            c.set(6,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.set(0,32,0x81020409u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(6,32); let r = c.imul(a,c.get(2,32),32); c.set(6,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            { let target = rm(m,0x66a598u32,32); c.push(m, 0x44309a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4430c6; continue; }
            pc = 0x44309e; continue;
            }
            0x44309e => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(0,32,c.get(6,32));
            if c.cond(13) { pc = 0x4430b4; continue; }
            pc = 0x4430a4; continue;
            }
            0x4430a4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a59cu32,32); c.push(m, 0x4430ae); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4430b4 => {
            { let a = c.get(6,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x4430be; continue; }
            pc = 0x4430b9; continue;
            }
            0x4430b9 => {
            c.set(0,32,0x7fu32);
            pc = 0x4430be; continue;
            }
            0x4430be => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a59cu32,32); c.push(m, 0x4430c6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4430c6; continue;
            }
            0x4430c6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004430d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4430d0;
    loop {
        match pc {
            0x4430d0 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x68),32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x443102; continue; }
            pc = 0x4430d8; continue;
            }
            0x4430d8 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(0x18));
            pc = 0x4430e1; continue;
            }
            0x4430e1 => {
            { let a = rm(m,c.r[2],32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4430f2; continue; }
            pc = 0x4430e5; continue;
            }
            0x4430e5 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x4430e1; continue; }
            pc = 0x4430ee; continue;
            }
            0x4430ee => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4430f2 => {
            { let a = c.r[0].wrapping_add(c.r[1]).wrapping_add(0x18c); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1a0); wm(m,a,8,0x0u32); }
            pc = 0x443102; continue;
            }
            0x443102 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00443110(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x443110;
    loop {
        match pc {
            0x443110 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x44311a; continue; }
            pc = 0x443117; continue;
            }
            0x443117 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x44311a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(0x7c));
            pc = 0x44311f; continue;
            }
            0x44311f => {
            { let a = rm(m,c.r[2],32); c.sub(a,0x1bbcu32,32); }
            if c.cond(4) { pc = 0x443130; continue; }
            pc = 0x443127; continue;
            }
            0x443127 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x44311f; continue; }
            pc = 0x443130; continue;
            }
            0x443130 => {
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(5) { pc = 0x443138; continue; }
            pc = 0x443135; continue;
            }
            0x443135 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x443138 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5c8u32,32); c.push(m, 0x443143); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004438d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4438d0;
    loop {
        match pc {
            0x4438d0 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x68),32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x44392c; continue; }
            pc = 0x4438d9; continue;
            }
            0x4438d9 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x44392c; continue; }
            pc = 0x4438e1; continue;
            }
            0x4438e1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(0x18));
            pc = 0x4438e6; continue;
            }
            0x4438e6 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[2],32),32); }
            if c.cond(4) { pc = 0x4438f3; continue; }
            pc = 0x4438ea; continue;
            }
            0x4438ea => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x4438e6; continue; }
            pc = 0x4438f3; continue;
            }
            0x4438f3 => {
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(4) { pc = 0x44392c; continue; }
            pc = 0x4438f8; continue;
            }
            0x4438f8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x1dd),8));
            c.set(0,32,c.get(2,32));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(4) { pc = 0x443911; continue; }
            pc = 0x44390a; continue;
            }
            0x44390a => {
            c.set(0,32,0x7fu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            pc = 0x443911; continue;
            }
            0x443911 => {
            c.set(6,32,c.get(0,32));
            c.push(m,c.get(7,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let target = rm(m,0x66a5d0u32,32); c.push(m, 0x443920); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x44392c; continue; }
            pc = 0x443924; continue;
            }
            0x443924 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a634u32,32); c.push(m, 0x44392c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x44392c; continue;
            }
            0x44392c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00446a00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x446a00;
    loop {
        match pc {
            0x446a00 => {
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6cbdd8u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,sx(rm(m,c.r[0].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(3,32,sx(rm(m,c.r[0].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(2,8); c.and(a,0x1u32,8); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.get(6,32));
            if c.cond(5) { pc = 0x446a56; continue; }
            pc = 0x446a46; continue;
            }
            0x446a46 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,c.get(1,8));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(40); c.fpu_exit(); return c.r;
            }
            0x446a52 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x446a56; continue;
            }
            0x446a56 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(6,32,sx(rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16),16)&0xffffffffu32);
            c.set(7,32,sx(rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x446d42; continue; }
            pc = 0x446a7c; continue;
            }
            0x446a7c => {
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fild32(m,c.r[4].wrapping_add(0x44));
            c.fstp32(m,c.r[4].wrapping_add(0x44));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            c.fstp64(m,c.r[4]);
            c.push(m, 0x446ab0);
            c.crt_call(m, 0x63b1f5);
            c.r[4] = c.r[4].wrapping_add(4);
            c.fstp64(m,c.r[4].wrapping_add(0x24));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x44),4));
            c.fstp64(m,c.r[4]);
            c.push(m, 0x446ac0);
            c.crt_call(m, 0x63b1f5);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,c.r[4].wrapping_add(0x24),8); c.fcomp(v); }
            c.set(2,32,c.get(3,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(10) { pc = 0x446c06; continue; }
            pc = 0x446ad6; continue;
            }
            0x446ad6 => {
            c.set(1,32,c.get(5,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('/',v); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x3c),4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x66bc6cu32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x44u32,8); }
            if c.cond(11) { pc = 0x446b31; continue; }
            pc = 0x446b1b; continue;
            }
            0x446b1b => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x3c),4); c.fop('*',v); }
            c.fild32(m,c.r[4].wrapping_add(0x28));
            c.fstp32(m,c.r[4].wrapping_add(0x3c));
            { let v = c.ld(m,c.r[4].wrapping_add(0x3c),4); c.fop('+',v); }
            pc = 0x446b45; continue;
            }
            0x446b31 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x3c),4); c.fop('*',v); }
            c.fild32(m,c.r[4].wrapping_add(0x28));
            c.fstp32(m,c.r[4].wrapping_add(0x3c));
            { let v = c.ld(m,c.r[4].wrapping_add(0x3c),4); c.fop('+',v); }
            pc = 0x446b45; continue;
            }
            0x446b45 => {
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            c.push(m, 0x446b50);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[0].wrapping_add(0xfffffffe));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,8,u32::from(c.cond(13)));
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(14)));
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x446d2b; continue; }
            pc = 0x446b70; continue;
            }
            0x446b70 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,0x66bc6cu32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x446b8a; continue; }
            pc = 0x446b83; continue;
            }
            0x446b83 => {
            c.set(1,32,0x1u32);
            pc = 0x446b8c; continue;
            }
            0x446b8a => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x446b8c; continue;
            }
            0x446b8c => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,0x66bc80u32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x41u32,8); }
            if c.cond(10) { pc = 0x446ba4; continue; }
            pc = 0x446b9d; continue;
            }
            0x446b9d => {
            c.set(0,32,0x1u32);
            pc = 0x446ba6; continue;
            }
            0x446ba4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x446ba6; continue;
            }
            0x446ba6 => {
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x446d2b; continue; }
            pc = 0x446bae; continue;
            }
            0x446bae => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x446d2b; continue; }
            pc = 0x446bc4; continue;
            }
            0x446bc4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(5,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(7,32));
            { let a = c.get(6,32); let r = c.imul(a,c.get(7,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(6,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.imul(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x446d2f; continue; }
            pc = 0x446bfc; continue;
            }
            0x446bfc => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x446d2f; continue;
            }
            0x446c06 => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.get(5,32));
            c.fild32(m,c.r[4].wrapping_add(0x28));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x3c),4); c.fop('/',v); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fild32(m,c.r[4].wrapping_add(0x28));
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x3c),4));
            { let v = c.ld(m,0x66bc6cu32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x44u32,8); }
            if c.cond(11) { pc = 0x446c61; continue; }
            pc = 0x446c4b; continue;
            }
            0x446c4b => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x3c));
            { let v = c.ld(m,c.r[4].wrapping_add(0x3c),4); c.fop('+',v); }
            pc = 0x446c75; continue;
            }
            0x446c61 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x3c));
            { let v = c.ld(m,c.r[4].wrapping_add(0x3c),4); c.fop('+',v); }
            pc = 0x446c75; continue;
            }
            0x446c75 => {
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            c.push(m, 0x446c80);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[0].wrapping_add(0xfffffffe));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            c.set(2,8,u32::from(c.cond(13)));
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(14)));
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x446d2f; continue; }
            pc = 0x446c9c; continue;
            }
            0x446c9c => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,0x66bc6cu32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x446cb6; continue; }
            pc = 0x446caf; continue;
            }
            0x446caf => {
            c.set(1,32,0x1u32);
            pc = 0x446cb8; continue;
            }
            0x446cb6 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x446cb8; continue;
            }
            0x446cb8 => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,0x66bc80u32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x41u32,8); }
            if c.cond(10) { pc = 0x446cd0; continue; }
            pc = 0x446cc9; continue;
            }
            0x446cc9 => {
            c.set(0,32,0x1u32);
            pc = 0x446cd2; continue;
            }
            0x446cd0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x446cd2; continue;
            }
            0x446cd2 => {
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x446d2f; continue; }
            pc = 0x446cd6; continue;
            }
            0x446cd6 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(5,32),32); c.set(6,32,r); }
            c.set(2,32,c.get(7,32));
            c.set(7,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.imul(a,c.get(6,32),32); c.set(7,32,r); }
            c.set(6,32,c.get(2,32));
            { let a = c.get(6,32); let r = c.imul(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x446d2f; continue; }
            pc = 0x446d24; continue;
            }
            0x446d24 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x446d2f; continue;
            }
            0x446d2b => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x446d2f; continue;
            }
            0x446d2f => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x446a52; continue; }
            pc = 0x446d42; continue;
            }
            0x446d42 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(40); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0044d400(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x44d400;
    loop {
        match pc {
            0x44d400 => {
            { let a = c.get(4,32); let r = c.sub(a,0x2cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x44d418);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x44d41f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b77f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(2,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[7].wrapping_add(0xffffffff));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x44d492);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x44d49e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x453e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[3].wrapping_add(0xffffffff));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x44d4c9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x44d4d5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x453e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x44d4fd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x44d509);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x453e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x44d531);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x44d53d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x453e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x44d54f; continue; }
            pc = 0x44d548; continue;
            }
            0x44d548 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.get(0,32));
            pc = 0x44d54f; continue;
            }
            0x44d54f => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x44d560; continue; }
            pc = 0x44d55a; continue;
            }
            0x44d55a => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            c.set(7,32,c.get(3,32));
            pc = 0x44d560; continue;
            }
            0x44d560 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(1,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44d5be; continue; }
            pc = 0x44d57e; continue;
            }
            0x44d57e => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x29e4),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x29e6),16));
            { let a = c.get(0,16); let r = c.add(a,c.get(2,16),16); c.set(0,16,r); }
            { let a = c.get(1,16); let r = c.add(a,c.get(7,16),16); c.set(1,16,r); }
            { let a = c.r[6].wrapping_add(0x28f4); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x28f6); wm(m,a,16,c.get(1,16)); }
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.set(1,32,sx(c.get(1,16),16)&0xffffffffu32);
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x98); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x9c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,c.get(1,32)); }
            pc = 0x44d5be; continue;
            }
            0x44d5be => {
            c.set(5,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x28f4),16),16)&0xffffffffu32);
            c.set(0,32,c.r[2].wrapping_add(c.r[5]));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x44d5e1; continue; }
            pc = 0x44d5d3; continue;
            }
            0x44d5d3 => {
            { let a = c.r[6].wrapping_add(0x9c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            pc = 0x44d5ed; continue;
            }
            0x44d5e1 => {
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x9c); wm(m,a,32,c.get(0,32)); }
            pc = 0x44d5ed; continue;
            }
            0x44d5ed => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x28f6),16),16)&0xffffffffu32);
            c.set(0,32,c.r[1].wrapping_add(c.r[7]));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x44d610; continue; }
            pc = 0x44d602; continue;
            }
            0x44d602 => {
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x98); wm(m,a,32,c.get(0,32)); }
            pc = 0x44d61c; continue;
            }
            0x44d610 => {
            { let a = c.r[6].wrapping_add(0x98); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,c.get(0,32)); }
            pc = 0x44d61c; continue;
            }
            0x44d61c => {
            { let a = rm(m,c.r[6].wrapping_add(0x98),32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x44d62a; continue; }
            pc = 0x44d624; continue;
            }
            0x44d624 => {
            { let a = c.r[6].wrapping_add(0x98); wm(m,a,32,c.get(3,32)); }
            pc = 0x44d62a; continue;
            }
            0x44d62a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            c.set(0,32,c.r[3].wrapping_add(0x707));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x44d640; continue; }
            pc = 0x44d63a; continue;
            }
            0x44d63a => {
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,c.get(0,32)); }
            pc = 0x44d640; continue;
            }
            0x44d640 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x44d654; continue; }
            pc = 0x44d64a; continue;
            }
            0x44d64a => {
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,0x0u32); }
            pc = 0x44d654; continue;
            }
            0x44d654 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            c.set(0,32,0x95fu32);
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x44d669; continue; }
            pc = 0x44d663; continue;
            }
            0x44d663 => {
            { let a = c.r[6].wrapping_add(0x9c); wm(m,a,32,c.get(0,32)); }
            pc = 0x44d669; continue;
            }
            0x44d669 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(5,32),32); c.set(3,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[7].wrapping_add(0xffffffff));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[5].wrapping_add(0x1));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x44d6be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x44d6ca);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x453e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[3].wrapping_add(0xffffffff));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x44d6f5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x44d701);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x453e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x44d729);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x44d735);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x453e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x44d75d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x44d769);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x453e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
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

pub fn f_00451eb0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x451eb0;
    loop {
        match pc {
            0x451eb0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.and(a,0xfffffff8u32,32); c.set(4,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0x70u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x451ed3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m, 0x451edc);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x48));
            c.set(0,16,rm(m,c.r[1],16));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            c.push(m, 0x451f02);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x451f09);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b77f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,16,rm(m,c.r[4].wrapping_add(0x4c),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x29e4),16));
            { let a = c.get(3,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x18),16),16); c.set(3,16,r); }
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x48),16),16); c.set(0,16,r); }
            c.set(2,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(3,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x29e6),16),16); c.set(3,16,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,16,c.get(0,16)); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x46); wm(m,a,16,c.get(3,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x46),32));
            { let a = c.get(0,32); let r = c.add(a,0xffffffbcu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,c.r[1].wrapping_add(0xffffffbc));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x451f57);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4474d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2],16));
            c.set(5,16,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x451fd4; continue; }
            pc = 0x451f87; continue;
            }
            0x451f87 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(7,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16));
            c.set(7,16,rm(m,0x6dd894u32,16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x451fd4; continue; }
            pc = 0x451faa; continue;
            }
            0x451faa => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0xe),8));
            { let a = c.get(2,16); c.sub(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x451fc7; continue; }
            pc = 0x451fc1; continue;
            }
            0x451fc1 => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(5,32,c.get(2,32));
            pc = 0x451fc7; continue;
            }
            0x451fc7 => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x451faa; continue; }
            pc = 0x451fd0; continue;
            }
            0x451fd0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x451fd4; continue;
            }
            0x451fd4 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b23),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x452491; continue; }
            pc = 0x451fe2; continue;
            }
            0x451fe2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m, 0x451ff5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x45c0f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x452007);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x45bcf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4525f1; continue; }
            pc = 0x45200f; continue;
            }
            0x45200f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x452020);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x45bdc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4525f1; continue; }
            pc = 0x452028; continue;
            }
            0x452028 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b23),8));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x452491; continue; }
            pc = 0x452040; continue;
            }
            0x452040 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[2],16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x28f8),16); c.sub(a,c.get(7,16),16); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x45216a; continue; }
            pc = 0x452061; continue;
            }
            0x452061 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x40),16));
            { let a = c.r[6].wrapping_add(0x28f8); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.set(0,32,c.r[2].wrapping_add(0xffffffbc));
            { let a = c.r[6].wrapping_add(0x28fa); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x42),32));
            { let a = c.get(0,32); let r = c.add(a,0xffffffbcu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x45209c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4474d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x45216a; continue; }
            pc = 0x4520a8; continue;
            }
            0x4520a8 => {
            c.set(0,32,0x1u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            pc = 0x4520b5; continue;
            }
            0x4520b5 => {
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            c.set(5,32,c.get(7,32));
            if c.cond(15) { pc = 0x452131; continue; }
            pc = 0x4520bb; continue;
            }
            0x4520bb => {
            pc = 0x4520c1; continue;
            }
            0x4520bd => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x4520c1; continue;
            }
            0x4520c1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,c.r[2].wrapping_add(0xffffffbc));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(0,16)); }
            pc = 0x4520cf; continue;
            }
            0x4520cf => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x42),32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(0,32); let r = c.add(a,0xffffffbcu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x4520ed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4474d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4520fc; continue; }
            pc = 0x4520f1; continue;
            }
            0x4520f1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4520cf; continue; }
            pc = 0x4520fa; continue;
            }
            0x4520fa => {
            pc = 0x45211c; continue;
            }
            0x4520fc => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x42),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[5]));
            c.set(2,32,c.r[7].wrapping_add(c.r[1]));
            { let a = c.r[6].wrapping_add(0x28f8); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[6].wrapping_add(0x28fa); wm(m,a,16,c.get(2,16)); }
            pc = 0x45211c; continue;
            }
            0x45211c => {
            c.set(1,32,c.r[0].wrapping_add(0x1));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x452128; continue; }
            pc = 0x452123; continue;
            }
            0x452123 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4520bd; continue; }
            pc = 0x452128; continue;
            }
            0x452128 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(3,16,rm(m,c.r[4].wrapping_add(0x42),16));
            pc = 0x452131; continue;
            }
            0x452131 => {
            c.set(1,32,c.r[0].wrapping_add(0x1));
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x45214c; continue; }
            pc = 0x452138; continue;
            }
            0x452138 => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(7,32); c.sub(a,0xfffffff6u32,32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            if c.cond(15) { pc = 0x4520b5; continue; }
            pc = 0x45214c; continue;
            }
            0x45214c => {
            { let a = c.get(0,32); c.sub(a,0xau32,32); }
            if c.cond(5) { pc = 0x45216a; continue; }
            pc = 0x452151; continue;
            }
            0x452151 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x28fa); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x28f8); wm(m,a,16,c.get(0,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            0x45216a => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x28f8),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[4].wrapping_add(0x40),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x45217e);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x28fa),16),16)&0xffffffffu32);
            c.set(7,32,c.get(0,32));
            c.set(0,32,sx(c.get(3,16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x452192);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x1f),8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x9c4u32,32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x452360; continue; }
            pc = 0x4521c2; continue;
            }
            0x4521c2 => {
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp64(m,c.r[4].wrapping_add(0x50));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4521d9);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4].wrapping_add(0x50));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x32u32); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(7,32)); }
            pc = 0x452214; continue;
            }
            0x45220f => {
            c.set(3,16,rm(m,c.r[4].wrapping_add(0x42),16));
            pc = 0x452214; continue;
            }
            0x452214 => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x28f8),16));
            { let a = c.get(7,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x40),16),16); }
            c.fild32(m,c.r[4].wrapping_add(0x20));
            if c.cond(13) { pc = 0x452246; continue; }
            pc = 0x452226; continue;
            }
            0x452226 => {
            c.fstp64(m,c.r[4].wrapping_add(0x58));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x58),8));
            { let v = c.ld(m,c.r[4].wrapping_add(0x50),8); c.fop('/',v); }
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            c.push(m, 0x45223d);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,16,c.get(0,16)); }
            pc = 0x452264; continue;
            }
            0x452246 => {
            c.fstp64(m,c.r[4].wrapping_add(0x58));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x58),8));
            { let v = c.ld(m,c.r[4].wrapping_add(0x50),8); c.fop('/',v); }
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            c.push(m, 0x45225d);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,16,c.get(7,16)); }
            pc = 0x452264; continue;
            }
            0x452264 => {
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x28fa),16));
            c.fild32(m,c.r[4].wrapping_add(0x24));
            { let a = c.get(5,16); c.sub(a,c.get(3,16),16); }
            if c.cond(13) { pc = 0x452292; continue; }
            pc = 0x452274; continue;
            }
            0x452274 => {
            c.fstp64(m,c.r[4].wrapping_add(0x58));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x58),8));
            { let v = c.ld(m,c.r[4].wrapping_add(0x50),8); c.fop('/',v); }
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            c.push(m, 0x45228b);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,16,c.get(0,16));
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            pc = 0x4522ae; continue;
            }
            0x452292 => {
            c.fstp64(m,c.r[4].wrapping_add(0x58));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x58),8));
            { let v = c.ld(m,c.r[4].wrapping_add(0x50),8); c.fop('/',v); }
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            c.push(m, 0x4522a9);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(7,16,c.get(5,16));
            pc = 0x4522ae; continue;
            }
            0x4522ae => {
            { let a = c.r[4].wrapping_add(0x42); wm(m,a,16,c.get(7,16)); }
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x42),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(1,32); let r = c.add(a,0xffffffbcu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xffffffbcu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1a); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x4522db);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4474d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            c.set(5,32,sx(rm(m,c.r[4].wrapping_add(0x40),16),16)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(7,32,sx(c.get(7,16),16)&0xffffffffu32);
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45230c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x45bcf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4525f1; continue; }
            pc = 0x452314; continue;
            }
            0x452314 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x45231d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x45bdc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4525f1; continue; }
            pc = 0x452325; continue;
            }
            0x452325 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x17),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x45235b; continue; }
            pc = 0x45232d; continue;
            }
            0x45232d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            if c.cond(15) { pc = 0x45220f; continue; }
            pc = 0x45235b; continue;
            }
            0x45235b => {
            c.set(3,16,rm(m,c.r[4].wrapping_add(0x42),16));
            pc = 0x452360; continue;
            }
            0x452360 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x17),8));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = 0x6cbdd2u32; wm(m,a,16,c.get(7,16)); }
            { let a = 0x6cbdd0u32; wm(m,a,16,c.get(7,16)); }
            if c.cond(4) { pc = 0x4525f1; continue; }
            pc = 0x45237c; continue;
            }
            0x45237c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            c.set(5,32,c.r[6].wrapping_add(0xa8));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4523a3; continue; }
            pc = 0x45238c; continue;
            }
            0x45238c => {
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x452394);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            c.push(m, 0x45239d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(7,32)); }
            pc = 0x4523a3; continue;
            }
            0x4523a3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,0xcu32);
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x3u32); }
            c.set(7,32,rm(m,c.r[2].wrapping_add(0x24),32));
            c.push(m, 0x4523b9);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x452480; continue; }
            pc = 0x4523c8; continue;
            }
            0x4523c8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(5,32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x1e),16));
            { let a = c.r[0]; wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x22),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[1].wrapping_add(0x2); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x44),16));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[1].wrapping_add(0x6); wm(m,a,16,c.get(3,16)); }
            { let a = 0x6cbdd0u32; wm(m,a,16,c.get(0,16)); }
            { let a = 0x6cbdd2u32; wm(m,a,16,c.get(3,16)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xfffffffc),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,16,c.get(1,16)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[7],32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xfffffffe),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[0].wrapping_add(0xa); wm(m,a,16,c.get(2,16)); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.push(m, 0x452436);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x445010, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x58));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x452465);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x48),16));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x4c),16));
            { let a = 0x6cbde4u32; wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = 0x6cbde6u32; wm(m,a,16,c.get(1,16)); }
            pc = 0x452480; continue;
            }
            0x452480 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x452486);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            0x452491 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2],16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x30),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,0x6cbdd0u32,16); c.sub(a,c.get(3,16),16); }
            c.set(3,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.and(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,0x6cbdd2u32,16); c.sub(a,c.get(3,16),16); }
            c.set(3,8,u32::from(c.cond(5)));
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4525b8; continue; }
            pc = 0x4524cd; continue;
            }
            0x4524cd => {
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x24),32));
            c.set(3,32,0xbu32);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(3,32)); }
            c.set(1,16,rm(m,c.r[2],16));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x45250c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,sx(rm(m,0x6cbdd0u32,16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(3,32)); }
            c.set(2,16,rm(m,c.r[0],16));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.set(2,32,sx(rm(m,0x6cbdd2u32,16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x45254d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(3,32)); }
            c.set(1,16,rm(m,c.r[2],16));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,0x2u32); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[7],32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xfffffffc),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[7],32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xfffffffe),16),16)&0xffffffffu32);
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x452594);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.set(1,16,rm(m,c.r[2],16));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4525b8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4525b8; continue;
            }
            0x4525b8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4525e3; continue; }
            pc = 0x4525c2; continue;
            }
            0x4525c2 => {
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4525ca);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4525d6);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xa8); wm(m,a,32,0x0u32); }
            pc = 0x4525e3; continue;
            }
            0x4525e3 => {
            { let a = c.r[6].wrapping_add(0x28f8); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[6].wrapping_add(0x28fa); wm(m,a,16,c.get(5,16)); }
            pc = 0x4525f1; continue;
            }
            0x4525f1 => {
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

pub fn f_004538e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4538e0;
    loop {
        match pc {
            0x4538e0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x30u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b24),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2b22),8));
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x2b2b),8));
            { let a = c.get(0,32); let r = c.or(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.or(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.or(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(6,32)); }
            if c.cond(4) { pc = 0x45392d; continue; }
            pc = 0x45391c; continue;
            }
            0x45391c => {
            { let a = c.r[6].wrapping_add(0x2b2b); wm(m,a,8,0x1u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x45392d => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x453de5; continue; }
            pc = 0x45394b; continue;
            }
            0x45394b => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(5,32,c.get(1,32));
            if c.cond(14) { pc = 0x45397f; continue; }
            pc = 0x453975; continue;
            }
            0x453975 => {
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x45398b; continue;
            }
            0x45397f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x45398b; continue;
            }
            0x45398b => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x4539a5; continue; }
            pc = 0x453993; continue;
            }
            0x453993 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4539b1; continue;
            }
            0x4539a5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(7,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            pc = 0x4539b1; continue;
            }
            0x4539b1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x453de5; continue; }
            pc = 0x4539bd; continue;
            }
            0x4539bd => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x453de5; continue; }
            pc = 0x4539c9; continue;
            }
            0x4539c9 => {
            c.push(m,0xfffffffdu32);
            c.push(m, 0x4539d0);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xfffffffdu32);
            c.push(m, 0x4539d7);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4539f8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36))]; c.r[0] = host.call(m, 0x4bb350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x453de5; continue; }
            pc = 0x453a03; continue;
            }
            0x453a03 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x453a32);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b95e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m, 0x453a42);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b77f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x453a5c);
            { let args = []; c.r[0] = host.call(m, 0x4b9540, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.set(2,32,rm(m,c.r[5],32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x453a66); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x453a78); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x453aa3; continue; }
            pc = 0x453a89; continue;
            }
            0x453a89 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x453aa3; continue; }
            pc = 0x453a94; continue;
            }
            0x453a94 => {
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x453aa3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x453aa3; continue;
            }
            0x453aa3 => {
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x453ab2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x453aca); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[5],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x453adc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x453b21; continue; }
            pc = 0x453b09; continue;
            }
            0x453b09 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x453b21; continue; }
            pc = 0x453b12; continue;
            }
            0x453b12 => {
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x453b21);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x453b21; continue;
            }
            0x453b21 => {
            c.set(1,32,rm(m,0x6ee120u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x453b9f; continue; }
            pc = 0x453b2b; continue;
            }
            0x453b2b => {
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x453b30); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6ee120u32,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x453b3f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,rm(m,0x6ee120u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x453b54); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6ee120u32,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x453b63); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x453b7c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x453b9f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x453b9f; continue;
            }
            0x453b9f => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x488),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x453c92; continue; }
            pc = 0x453bb5; continue;
            }
            0x453bb5 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x453bb7; continue;
            }
            0x453bb7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x168),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x168));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x453bc8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x168),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x168));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x453be3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6cbb40u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x168),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x168));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x453bfd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1));
            c.set(0,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x168),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x168));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x453c1f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1));
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x453c3a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x453c71);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xa0u32,32); c.set(6,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x488),8));
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x453bb7; continue; }
            pc = 0x453c8e; continue;
            }
            0x453c8e => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x453c92; continue;
            }
            0x453c92 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x7b4),8));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x453d7f; continue; }
            pc = 0x453ca2; continue;
            }
            0x453ca2 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x453ca4; continue;
            }
            0x453ca4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x494),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x494));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x453cb5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x494),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x494));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x453cd0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6cbb40u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x494),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x494));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x453cea); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1));
            c.set(0,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x494),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x494));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x453d0c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1));
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x453d27);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x453d5e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xa0u32,32); c.set(6,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x7b4),8));
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x453ca4; continue; }
            pc = 0x453d7b; continue;
            }
            0x453d7b => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x453d7f; continue;
            }
            0x453d7f => {
            c.push(m, 0x453d84);
            { let args = []; c.r[0] = host.call(m, 0x4b8510, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x453de5; continue; }
            pc = 0x453d88; continue;
            }
            0x453d88 => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(0,32));
            c.push(m, 0x453d98);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b85c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x453dae; continue; }
            pc = 0x453da3; continue;
            }
            0x453da3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x453de5; continue; }
            pc = 0x453dae; continue;
            }
            0x453dae => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x453de5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x453de5; continue;
            }
            0x453de5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00459790(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x459790;
    loop {
        match pc {
            0x459790 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x45979e);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.fild32(m,c.r[4].wrapping_add(0x1c));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,0x66cb34u32,4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            if c.cond(7) { pc = 0x45a44d; continue; }
            pc = 0x4597c6; continue;
            }
            0x4597c6 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x45a458)); continue;
            }
            0x4597cd => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2d14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x45a44d; continue; }
            pc = 0x4597db; continue;
            }
            0x4597db => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x4597e6);
            { let args = []; c.r[0] = host.call(m, 0x443940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b44),16));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2b58); wm(m,a,16,c.get(7,16)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b44),16));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x2c),16));
            c.push(m,0x3e8u32);
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2b5a); wm(m,a,16,c.get(3,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b44),16));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2b5e); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x23),8));
            { let a = c.r[6].wrapping_add(0x2b38); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x459851);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.push(m, 0x45986a);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b38),16));
            c.push(m,0x0u32);
            c.push(m,0x40u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.add(a,0x3e8u32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x45988f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x443150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b44),16));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2b54); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b44),16));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2b54),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x45a44d; continue; }
            pc = 0x4598bf; continue;
            }
            0x4598bf => {
            c.set(2,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(0,32); let r = c.add(a,0x2b6u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x29e6),16),16); c.set(0,16,r); }
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x18),16),16); c.set(0,16,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[3]));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x29e4),16),16); c.set(0,16,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b44),16));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2b5c); wm(m,a,16,c.get(2,16)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); c.fpu_exit(); return c.r;
            }
            0x459921 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2d14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x459994; continue; }
            pc = 0x45992b; continue;
            }
            0x45992b => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b50),16));
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x45a44d; continue; }
            pc = 0x45993c; continue;
            }
            0x45993c => {
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x2c),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x30),16));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2cde); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b50),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2cd8); wm(m,a,16,c.get(1,16)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b50),16));
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x30),16));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2cda); wm(m,a,16,c.get(0,16)); }
            { let a = rm(m,c.r[6].wrapping_add(0x2b50),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x2b50); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); c.fpu_exit(); return c.r;
            }
            0x459994 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b46),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4599d1; continue; }
            pc = 0x4599a3; continue;
            }
            0x4599a3 => {
            c.set(1,32,c.r[6].wrapping_add(0x2b9e));
            pc = 0x4599a9; continue;
            }
            0x4599a9 => {
            c.set(2,32,rm(m,0x6dccf0u32,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x2),32));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            c.set(5,16,rm(m,c.r[1],16));
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x2c),16),16); }
            c.set(3,8,u32::from(c.cond(4)));
            { let a = c.get(3,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4599d1; continue; }
            pc = 0x4599c9; continue;
            }
            0x4599c9 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4599a9; continue; }
            pc = 0x4599d1; continue;
            }
            0x4599d1 => {
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45a44d; continue; }
            pc = 0x4599d9; continue;
            }
            0x4599d9 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x0u32); }
            if c.cond(14) { pc = 0x459a89; continue; }
            pc = 0x4599e9; continue;
            }
            0x4599e9 => {
            c.set(0,32,c.r[6].wrapping_add(0x2ba0));
            c.set(3,32,0x1u32);
            c.set(7,32,c.r[6].wrapping_add(0x2b94));
            c.set(5,32,c.r[6].wrapping_add(0x2ba4));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            pc = 0x459a04; continue;
            }
            0x459a04 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = rm(m,c.r[2],32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x459a5d; continue; }
            pc = 0x459a12; continue;
            }
            0x459a12 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x459a22);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4430d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b46),16));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x459a38);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x2b46),16); let r = c.dec(a,16); { let a = c.r[6].wrapping_add(0x2b46); wm(m,a,16,r); } }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x10u32,32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,0x10u32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(7,32); let r = c.sub(a,0x10u32,32); c.set(7,32,r); }
            pc = 0x459a5d; continue;
            }
            0x459a5d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b46),16));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x10u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x459a04; continue; }
            pc = 0x459a89; continue;
            }
            0x459a89 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b46),16));
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(4) { pc = 0x45a44d; continue; }
            pc = 0x459a9a; continue;
            }
            0x459a9a => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x2c),16));
            c.push(m,0x7d0u32);
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2b98); wm(m,a,16,c.get(7,16)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b46),16));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2b9a); wm(m,a,16,c.get(3,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b46),16));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2b9e); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x23),8));
            { let a = c.r[6].wrapping_add(0x2b3a); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x459b01);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.push(m, 0x459b1a);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x40u32);
            c.push(m,0xffffffffu32);
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.add(a,0x7d0u32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x459b37);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x443150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b46),16));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2b94); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b46),16));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2b94),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2b94));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x45a44d; continue; }
            pc = 0x459b6c; continue;
            }
            0x459b6c => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a628u32,32); c.push(m, 0x459b86); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,0x6dccf0u32,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x2710));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b46),16));
            { let a = c.get(1,32); let r = c.add(a,0x2bau32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x29e6),16),16); c.set(0,16,r); }
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x18),16),16); c.set(0,16,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[3]));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x29e4),16),16); c.set(0,16,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b46),16));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2b9c); wm(m,a,16,c.get(2,16)); }
            { let a = rm(m,c.r[6].wrapping_add(0x2b46),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x2b46); wm(m,a,16,r); } }
            { let a = c.r[6].wrapping_add(0x2d14); wm(m,a,8,0x1u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); c.fpu_exit(); return c.r;
            }
            0x459c0b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2d14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x45a44d; continue; }
            pc = 0x459c19; continue;
            }
            0x459c19 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b48),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x459c56; continue; }
            pc = 0x459c28; continue;
            }
            0x459c28 => {
            c.set(1,32,c.r[6].wrapping_add(0x2bde));
            pc = 0x459c2e; continue;
            }
            0x459c2e => {
            c.set(2,32,rm(m,0x6dccf0u32,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x2),32));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            c.set(5,16,rm(m,c.r[1],16));
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x2c),16),16); }
            c.set(3,8,u32::from(c.cond(4)));
            { let a = c.get(3,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x459c56; continue; }
            pc = 0x459c4e; continue;
            }
            0x459c4e => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x459c2e; continue; }
            pc = 0x459c56; continue;
            }
            0x459c56 => {
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45a44d; continue; }
            pc = 0x459c5e; continue;
            }
            0x459c5e => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            if c.cond(14) { pc = 0x459d0e; continue; }
            pc = 0x459c6e; continue;
            }
            0x459c6e => {
            c.set(0,32,c.r[6].wrapping_add(0x2be0));
            c.set(3,32,0x1u32);
            c.set(7,32,c.r[6].wrapping_add(0x2bd4));
            c.set(5,32,c.r[6].wrapping_add(0x2be4));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            pc = 0x459c89; continue;
            }
            0x459c89 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = rm(m,c.r[2],32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x459ce2; continue; }
            pc = 0x459c97; continue;
            }
            0x459c97 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x459ca7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4430d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b48),16));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x459cbd);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x2b48),16); let r = c.dec(a,16); { let a = c.r[6].wrapping_add(0x2b48); wm(m,a,16,r); } }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x10u32,32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,0x10u32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(7,32); let r = c.sub(a,0x10u32,32); c.set(7,32,r); }
            pc = 0x459ce2; continue;
            }
            0x459ce2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b48),16));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x10u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x459c89; continue; }
            pc = 0x459d0e; continue;
            }
            0x459d0e => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b48),16));
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(4) { pc = 0x45a44d; continue; }
            pc = 0x459d1f; continue;
            }
            0x459d1f => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x2c),16));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2bd8); wm(m,a,16,c.get(7,16)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b48),16));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2bda); wm(m,a,16,c.get(3,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b48),16));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2bde); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x23),8));
            c.set(1,8,c.get(0,8));
            { let a = c.r[6].wrapping_add(0x2b3c); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x2bd4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x40u32); }
            c.push(m, 0x459d99);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x45a5f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x1eu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x459dba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.push(m, 0x459dd3);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x45a44d; continue; }
            pc = 0x459ddf; continue;
            }
            0x459ddf => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(2,32));
            c.push(m,0x1u32);
            { let a = c.get(0,32); let r = c.add(a,0xbb8u32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x459e0e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x443150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b48),16));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2bd4); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b48),16));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2bd4),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2bd4));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x45a44d; continue; }
            pc = 0x459e43; continue;
            }
            0x459e43 => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a628u32,32); c.push(m, 0x459e5d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,0x6dccf0u32,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x1770));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b48),16));
            { let a = c.get(1,32); let r = c.add(a,0x2beu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x29e6),16),16); c.set(0,16,r); }
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x18),16),16); c.set(0,16,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[3]));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x29e4),16),16); c.set(0,16,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b48),16));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2bdc); wm(m,a,16,c.get(2,16)); }
            { let a = rm(m,c.r[6].wrapping_add(0x2b48),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x2b48); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); c.fpu_exit(); return c.r;
            }
            0x459edb => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2d14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x45a44d; continue; }
            pc = 0x459ee9; continue;
            }
            0x459ee9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b4a),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x459f26; continue; }
            pc = 0x459ef8; continue;
            }
            0x459ef8 => {
            c.set(1,32,c.r[6].wrapping_add(0x2c1e));
            pc = 0x459efe; continue;
            }
            0x459efe => {
            c.set(2,32,rm(m,0x6dccf0u32,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x2),32));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            c.set(5,16,rm(m,c.r[1],16));
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x2c),16),16); }
            c.set(3,8,u32::from(c.cond(4)));
            { let a = c.get(3,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x459f26; continue; }
            pc = 0x459f1e; continue;
            }
            0x459f1e => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x459efe; continue; }
            pc = 0x459f26; continue;
            }
            0x459f26 => {
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45a44d; continue; }
            pc = 0x459f2e; continue;
            }
            0x459f2e => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x0u32); }
            if c.cond(14) { pc = 0x459fde; continue; }
            pc = 0x459f3e; continue;
            }
            0x459f3e => {
            c.set(0,32,c.r[6].wrapping_add(0x2c20));
            c.set(3,32,0x1u32);
            c.set(7,32,c.r[6].wrapping_add(0x2c14));
            c.set(5,32,c.r[6].wrapping_add(0x2c24));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            pc = 0x459f59; continue;
            }
            0x459f59 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = rm(m,c.r[2],32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x459fb2; continue; }
            pc = 0x459f67; continue;
            }
            0x459f67 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x459f77);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4430d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b4a),16));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x459f8d);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x2b4a),16); let r = c.dec(a,16); { let a = c.r[6].wrapping_add(0x2b4a); wm(m,a,16,r); } }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x10u32,32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,0x10u32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(7,32); let r = c.sub(a,0x10u32,32); c.set(7,32,r); }
            pc = 0x459fb2; continue;
            }
            0x459fb2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b4a),16));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x10u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x459f59; continue; }
            pc = 0x459fde; continue;
            }
            0x459fde => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b4a),16));
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(4) { pc = 0x45a44d; continue; }
            pc = 0x459fef; continue;
            }
            0x459fef => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x2c),16));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2c18); wm(m,a,16,c.get(7,16)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b4a),16));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2c1a); wm(m,a,16,c.get(3,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b4a),16));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2c1e); wm(m,a,16,c.get(1,16)); }
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x23),8));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp64(m,c.r[4].wrapping_add(0x1c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),8));
            { let v = c.ld(m,0x66dba8u32,8); c.fop('*',v); }
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            c.push(m, 0x45a05f);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,c.get(0,8));
            { let a = c.r[6].wrapping_add(0x2b3e); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x2c14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x40u32); }
            c.push(m, 0x45a089);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x45a5f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0xfa0u32);
            c.push(m, 0x45a099);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.push(m, 0x45a0b2);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x45a44d; continue; }
            pc = 0x45a0be; continue;
            }
            0x45a0be => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x0u32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.add(a,0xfa0u32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x45a0ed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x443150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b4a),16));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2c14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b4a),16));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2c14),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2c14));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x45a44d; continue; }
            pc = 0x45a122; continue;
            }
            0x45a122 => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a628u32,32); c.push(m, 0x45a13c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x66a918u32,4); c.fop('*',v); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x30));
            { let v = c.ld(m,c.r[4].wrapping_add(0x30),4); c.fop('+',v); }
            { let v = c.ld(m,0x66dba0u32,4); c.fop('+',v); }
            c.push(m, 0x45a15d);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b4a),16));
            { let a = c.get(1,32); let r = c.add(a,0x2c2u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x29e6),16),16); c.set(0,16,r); }
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x18),16),16); c.set(0,16,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[3]));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x29e4),16),16); c.set(0,16,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b4a),16));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2c1c); wm(m,a,16,c.get(2,16)); }
            { let a = rm(m,c.r[6].wrapping_add(0x2b4a),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x2b4a); wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); c.fpu_exit(); return c.r;
            }
            0x45a1d7 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2d14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x45a44d; continue; }
            pc = 0x45a1e5; continue;
            }
            0x45a1e5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b4c),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x45a222; continue; }
            pc = 0x45a1f4; continue;
            }
            0x45a1f4 => {
            c.set(1,32,c.r[6].wrapping_add(0x2c5e));
            pc = 0x45a1fa; continue;
            }
            0x45a1fa => {
            c.set(2,32,rm(m,0x6dccf0u32,32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x2),32));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            c.set(5,16,rm(m,c.r[1],16));
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x2c),16),16); }
            c.set(3,8,u32::from(c.cond(4)));
            { let a = c.get(3,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x45a222; continue; }
            pc = 0x45a21a; continue;
            }
            0x45a21a => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x45a1fa; continue; }
            pc = 0x45a222; continue;
            }
            0x45a222 => {
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x45a44d; continue; }
            pc = 0x45a22a; continue;
            }
            0x45a22a => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x45a289; continue; }
            pc = 0x45a230; continue;
            }
            0x45a230 => {
            c.set(7,32,c.r[6].wrapping_add(0x2c54));
            pc = 0x45a236; continue;
            }
            0x45a236 => {
            c.set(0,32,rm(m,0x6dccf0u32,32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(5,32,c.r[7].wrapping_add(0x10));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(7) { pc = 0x45a278; continue; }
            pc = 0x45a245; continue;
            }
            0x45a245 => {
            c.set(1,32,rm(m,c.r[7],32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x45a255);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4430d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[3].wrapping_add(0x1));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b4c),16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x45a26e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x2b4c),16); let r = c.dec(a,16); { let a = c.r[6].wrapping_add(0x2b4c); wm(m,a,16,r); } }
            pc = 0x45a278; continue;
            }
            0x45a278 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b4c),16));
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x45a236; continue; }
            pc = 0x45a289; continue;
            }
            0x45a289 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b4c),16));
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(4) { pc = 0x45a44d; continue; }
            pc = 0x45a29a; continue;
            }
            0x45a29a => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x2c),16));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2c58); wm(m,a,16,c.get(7,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b4c),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2c5a); wm(m,a,16,c.get(3,16)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b4c),16));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2c5e); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x23),8));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp64(m,c.r[4].wrapping_add(0x1c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),8));
            { let v = c.ld(m,0x66dba8u32,8); c.fop('*',v); }
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            c.push(m, 0x45a30b);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x2b40); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0x1388u32);
            c.push(m, 0x45a322);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x443690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            c.push(m, 0x45a33b);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b40),16));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x40u32);
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.add(a,0x1388u32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x45a360);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x443150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b4c),16));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2c54); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2b4c),16));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2c54),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x45a44d; continue; }
            pc = 0x45a394; continue;
            }
            0x45a394 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x2b4c); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2c54),32));
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a628u32,32); c.push(m, 0x45a3c3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x66bc80u32,4); c.fop('+',v); }
            { let v = c.ld(m,0x66a918u32,4); c.fop('*',v); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x30));
            { let v = c.ld(m,c.r[4].wrapping_add(0x30),4); c.fop('+',v); }
            c.push(m, 0x45a3e4);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b4c),16));
            { let a = c.get(1,32); let r = c.add(a,0x2c6u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x29e6),16),16); c.set(0,16,r); }
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x18),16),16); c.set(0,16,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[3]));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x29e4),16),16); c.set(0,16,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b4c),16));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x2c5c); wm(m,a,16,c.get(2,16)); }
            pc = 0x45a44d; continue;
            }
            0x45a44d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0045a5f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45a5f0;
    loop {
        match pc {
            0x45a5f0 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x2c),32));
            c.push(m,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(0,32));
            c.set(7,32,sx(rm(m,c.r[1].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.neg(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x30),32));
            c.set(3,32,sx(rm(m,c.r[1].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x6),16));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x2b34),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.imul(a,c.get(7,32),32); c.set(3,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(3,32); c.div32(b); }
            c.set(3,32,c.get(0,32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[1].wrapping_add(0x2b30),32); c.idiv32(b); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,0x40u32,32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,0x7fu32,32); }
            if c.cond(12) { pc = 0x45a675; continue; }
            pc = 0x45a670; continue;
            }
            0x45a670 => {
            { let a = c.r[1]; wm(m,a,8,0x0u32); }
            pc = 0x45a67b; continue;
            }
            0x45a675 => {
            c.set(2,8,0x7fu32);
            { let a = c.get(2,8); let r = c.sub(a,c.get(3,8),8); c.set(2,8,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            pc = 0x45a67b; continue;
            }
            0x45a67b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(13) { pc = 0x45a68c; continue; }
            pc = 0x45a682; continue;
            }
            0x45a682 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[2]; wm(m,a,8,0x0u32); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x45a68c => {
            { let a = c.get(0,32); c.sub(a,0x7fu32,32); }
            if c.cond(14) { pc = 0x45a69b; continue; }
            pc = 0x45a691; continue;
            }
            0x45a691 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[0]; wm(m,a,8,0x7fu32); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x45a69b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00460540(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x460540;
    loop {
        match pc {
            0x460540 => {
            c.set(0,32,rm(m,0x6df31cu32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x460557; continue; }
            pc = 0x460553; continue;
            }
            0x460553 => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4605d1; continue; }
            pc = 0x460557; continue;
            }
            0x460557 => {
            c.set(0,32,rm(m,0x6dfc18u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4605d1; continue; }
            pc = 0x460560; continue;
            }
            0x460560 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[6].wrapping_add(0x17c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6df31cu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4605d1; continue; }
            pc = 0x46057c; continue;
            }
            0x46057c => {
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x460585);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0xc8u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x66de7c),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m, 0x4605d1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4605d1; continue;
            }
            0x4605d1 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0046eb50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x46eb50;
    loop {
        match pc {
            0x46eb50 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.fstp64(m,c.r[4].wrapping_add(0x8));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x8),8));
            { let v = c.ld(m,0x66ea90u32,8); c.fop('/',v); }
            c.push(m, 0x46eb6e);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.fild32(m,c.r[4].wrapping_add(0x24));
            c.set(7,32,c.get(0,32));
            c.fstp64(m,c.r[4].wrapping_add(0x8));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x8),8));
            { let v = c.ld(m,0x66ea98u32,8); c.fop('/',v); }
            c.push(m, 0x46eb87);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,sx(c.get(7,16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,sx(rm(m,c.r[4].wrapping_add(0x1c),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2]); wm(m,a,16,c.get(7,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            { let a = c.r[2].wrapping_add(c.r[1]).wrapping_add(0x2); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,32,c.get(2,32)); }
            { let a = rm(m,c.r[6].wrapping_add(0xa6),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0xa6); wm(m,a,16,r); } }
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x46ebe7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0046f4c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x46f4c0;
    loop {
        match pc {
            0x46f4c0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc8),16));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(0,16); c.sub(a,0x1u32,16); }
            if c.cond(6) { pc = 0x46f534; continue; }
            pc = 0x46f4d8; continue;
            }
            0x46f4d8 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(4).wrapping_add(0xfffffff4));
            c.push(m,c.get(1,32));
            c.push(m, 0x46f4ed);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xcu32,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc8),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_mul(4).wrapping_add(0xfffffff4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x46f513);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0xc8),16); let r = c.dec(a,16); { let a = c.r[6].wrapping_add(0xc8); wm(m,a,16,r); } }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x46f550; continue; }
            pc = 0x46f527; continue;
            }
            0x46f527 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x46f52d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x46f534 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,16,c.get(7,16)); }
            if c.cond(4) { pc = 0x46f54a; continue; }
            pc = 0x46f541; continue;
            }
            0x46f541 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x46f547);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x46f54a; continue;
            }
            0x46f54a => {
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,c.get(7,32)); }
            pc = 0x46f550; continue;
            }
            0x46f550 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004c4130(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c4130;
    loop {
        match pc {
            0x4c4130 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661fd3u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6bf428u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4c415b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b2bd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xau32);
            c.set(1,32,c.r[6].wrapping_add(0x64));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            c.push(m, 0x4c416d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64e603, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0x4));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(5,32));
            { let a = c.r[6]; wm(m,a,32,0x671360u32); }
            c.set(7,32,c.r[6].wrapping_add(0x14));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[2]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[2].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x14u32);
            c.stos(m,32,true);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c419d);
            { let args = []; c.r[0] = host.call(m, 0x4c4460, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(5,32));
            c.set(3,32,0x4u32);
            pc = 0x4c41a4; continue;
            }
            0x4c41a4 => {
            { let a = c.r[7]; wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,0x6dccd0u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c41c3; continue; }
            pc = 0x4c41b3; continue;
            }
            0x4c41b3 => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a604u32,32); c.push(m, 0x4c41ba); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x66a5dcu32,32); c.push(m, 0x4c41c3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c41c3; continue;
            }
            0x4c41c3 => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x4c41a4; continue; }
            pc = 0x4c41c9; continue;
            }
            0x4c41c9 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4c41cb; continue;
            }
            0x4c41cb => {
            { let a = c.get(7,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x4c41de; continue; }
            pc = 0x4c41d0; continue;
            }
            0x4c41d0 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c41d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c4620, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0xau32,32); }
            if c.cond(12) { pc = 0x4c41cb; continue; }
            pc = 0x4c41de; continue;
            }
            0x4c41de => {
            c.set(0,32,rm(m,0x6dccd0u32,32));
            c.set(3,32,rm(m,0x670f70u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c422a; continue; }
            pc = 0x4c41ed; continue;
            }
            0x4c41ed => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x4u32); }
            pc = 0x4c41f5; continue;
            }
            0x4c41f5 => {
            c.set(7,32,rm(m,c.r[5],32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4c421c; continue; }
            pc = 0x4c41fc; continue;
            }
            0x4c41fc => {
            c.set(1,32,c.get(3,32));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a59cu32,32); c.push(m, 0x4c421c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4c421c; continue;
            }
            0x4c421c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4c41f5; continue; }
            pc = 0x4c422a; continue;
            }
            0x4c422a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
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

pub fn f_004c4540(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c4540;
    loop {
        match pc {
            0x4c4540 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6dccd0u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c4597; continue; }
            pc = 0x4c454a; continue;
            }
            0x4c454a => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,0x66a59cu32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,32,c.r[1].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x4u32); }
            pc = 0x4c4563; continue;
            }
            0x4c4563 => {
            c.set(6,32,rm(m,c.r[3],32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4c4585; continue; }
            pc = 0x4c4569; continue;
            }
            0x4c4569 => {
            c.set(1,32,c.get(7,32));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            { let target = c.get(5,32); c.push(m, 0x4c4585); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4c4585; continue;
            }
            0x4c4585 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4c4563; continue; }
            pc = 0x4c4593; continue;
            }
            0x4c4593 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x4c4597; continue;
            }
            0x4c4597 => {
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004c45a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c45a0;
    loop {
        match pc {
            0x4c45a0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,0x66a5c8u32,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(6,32,c.r[3].wrapping_add(0x4));
            pc = 0x4c45b1; continue;
            }
            0x4c45b1 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c45bf; continue; }
            pc = 0x4c45b7; continue;
            }
            0x4c45b7 => {
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x4c45ba); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(4) { pc = 0x4c45cf; continue; }
            pc = 0x4c45bf; continue;
            }
            0x4c45bf => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4c45b1; continue; }
            pc = 0x4c45c8; continue;
            }
            0x4c45c8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4c45cf => {
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x4c45c8; continue; }
            pc = 0x4c45d4; continue;
            }
            0x4c45d4 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4c45e8; continue; }
            pc = 0x4c45e0; continue;
            }
            0x4c45e0 => {
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4c45e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c4620, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c45e8; continue;
            }
            0x4c45e8 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c45c8; continue; }
            pc = 0x4c45f0; continue;
            }
            0x4c45f0 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a630u32,32); c.push(m, 0x4c45fe); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c45c8; continue; }
            pc = 0x4c4602; continue;
            }
            0x4c4602 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let target = rm(m,0x66a5b0u32,32); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004cdb20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4cdb20;
    loop {
        match pc {
            0x4cdb20 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x312),8));
            { let a = c.r[1].wrapping_add(0x312); wm(m,a,8,0x0u32); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00550550(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x550550;
    loop {
        match pc {
            0x550550 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[0],16),16); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            if c.cond(5) { pc = 0x550571; continue; }
            pc = 0x550564; continue;
            }
            0x550564 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[1],16),16); }
            if c.cond(4) { pc = 0x5506be; continue; }
            pc = 0x550571; continue;
            }
            0x550571 => {
            c.set(1,16,rm(m,c.r[1],16));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,16,c.get(1,16)); }
            c.set(1,16,rm(m,c.r[0],16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,16,c.get(1,16)); }
            if c.cond(4) { pc = 0x550660; continue; }
            pc = 0x55058d; continue;
            }
            0x55058d => {
            { let a = rm(m,c.r[6].wrapping_add(0xe),8); c.sub(a,0xffu32,8); }
            if c.cond(13) { pc = 0x550626; continue; }
            pc = 0x550597; continue;
            }
            0x550597 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x2a),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.get(1,16); c.sub(a,0x32u32,16); }
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,c.get(0,32));
            if c.cond(2) { pc = 0x5505e7; continue; }
            pc = 0x5505b8; continue;
            }
            0x5505b8 => {
            c.push(m, 0x5505bd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5506d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4u32);
            c.set(1,32,c.r[6].wrapping_add(0xc));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x5505ca);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,16); c.sub(a,0x32u32,16); }
            if c.cond(6) { pc = 0x5506a5; continue; }
            pc = 0x5505db; continue;
            }
            0x5505db => {
            { let a = c.get(0,32); let r = c.add(a,0xffffffceu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,16,c.get(0,16)); }
            pc = 0x5506a5; continue;
            }
            0x5505e7 => {
            c.push(m, 0x5505ec);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5506d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,c.get(5,32));
            pc = 0x5505ef; continue;
            }
            0x5505ef => {
            { let a = c.get(7,32); c.sub(a,0x2u32,32); }
            c.set(5,32,0x6e11e8u32);
            if c.cond(12) { pc = 0x5505fe; continue; }
            pc = 0x5505f9; continue;
            }
            0x5505f9 => {
            c.set(5,32,0x6e1200u32);
            pc = 0x5505fe; continue;
            }
            0x5505fe => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m, 0x550609);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x14),16));
            c.set(1,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(2,8); let r = c.add(a,c.get(3,8),8); c.set(2,8,r); }
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x4u32,32); }
            if c.cond(2) { pc = 0x5505ef; continue; }
            pc = 0x550623; continue;
            }
            0x550623 => {
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x5506a5; continue;
            }
            0x550626 => {
            { let a = rm(m,c.r[0],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x5506a5; continue; }
            pc = 0x55062b; continue;
            }
            0x55062b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x2a),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x55064b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5506d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x4u32);
            { let a = c.get(6,32); let r = c.add(a,0xcu32,32); c.set(6,32,r); }
            c.push(m,0xffu32);
            c.push(m,c.get(6,32));
            c.push(m, 0x55065b);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5506a5; continue;
            }
            0x550660 => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(3,8,rm(m,c.r[6].wrapping_add(0xe),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,16); c.sub(a,c.get(7,16),16); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,8); c.sub(a,0xffu32,8); }
            c.set(0,8,u32::from(c.cond(13)));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x5506a5; continue; }
            pc = 0x55067b; continue;
            }
            0x55067b => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(7,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x55069f; continue; }
            pc = 0x550697; continue;
            }
            0x550697 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,16,c.get(1,16)); }
            pc = 0x5506a5; continue;
            }
            0x55069f => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,16,0x0u32); }
            pc = 0x5506a5; continue;
            }
            0x5506a5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[0],32));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x20),32); c.push(m, 0x5506bc); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x5506be; continue;
            }
            0x5506be => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(32); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005506d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5506d0;
    loop {
        match pc {
            0x5506d0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x18),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x5506f8; continue;
            }
            0x5506f8 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2),16));
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(5) { pc = 0x550706; continue; }
            pc = 0x550701; continue;
            }
            0x550701 => {
            { let a = rm(m,c.r[6],16); c.sub(a,c.get(2,16),16); }
            if c.cond(4) { pc = 0x550755; continue; }
            pc = 0x550706; continue;
            }
            0x550706 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            c.set(1,32,0x6e11e8u32);
            if c.cond(12) { pc = 0x550715; continue; }
            pc = 0x550710; continue;
            }
            0x550710 => {
            c.set(1,32,0x6e1200u32);
            pc = 0x550715; continue;
            }
            0x550715 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(5,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.push(m,0x0u32);
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(3,16); let n = 0x1u32; let r = c.shr(a,n,16); c.set(3,16,r); }
            { let a = c.get(5,16); let r = c.sub(a,c.get(3,16),16); c.set(5,16,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.r[1].wrapping_add(0x10); wm(m,a,16,c.get(5,16)); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x12),16));
            { let a = c.get(2,16); let r = c.sub(a,rm(m,c.r[0].wrapping_add(0x6),16),16); c.set(2,16,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,16); let r = c.add(a,rm(m,c.r[6],16),16); c.set(2,16,r); }
            { let a = c.r[1].wrapping_add(0x12); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x550751);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5854d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x550755; continue;
            }
            0x550755 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x5506f8; continue; }
            pc = 0x550762; continue;
            }
            0x550762 => {
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

pub fn f_005708f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5708f0;
    loop {
        match pc {
            0x5708f0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            pc = 0x5708f3; continue;
            }
            0x5708f3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x64),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x5708ff; continue; }
            pc = 0x5708fb; continue;
            }
            0x5708fb => {
            { let a = c.get(0,8); let r = c.or(a,0xffu32,8); c.set(0,8,r); }
            pc = 0x57091b; continue;
            }
            0x5708ff => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x57090a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3a),8));
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x16),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x57091b; continue; }
            pc = 0x570914; continue;
            }
            0x570914 => {
            { let a = c.get(1,8); let r = c.or(a,0xffu32,8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.sub(a,c.get(0,8),8); c.set(1,8,r); }
            c.set(0,8,c.get(1,8));
            pc = 0x57091b; continue;
            }
            0x57091b => {
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,0x6dccf8u32,32));
            { let a = c.r[6].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x5708f3; continue; }
            pc = 0x57092e; continue;
            }
            0x57092e => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00570ba0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x570ba0;
    loop {
        match pc {
            0x570ba0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x60),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x570bc5; continue; }
            pc = 0x570bb5; continue;
            }
            0x570bb5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x570bc1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x570ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(3,32,c.get(0,32));
            pc = 0x570bca; continue;
            }
            0x570bc5 => {
            c.set(3,32,0xffffu32);
            pc = 0x570bca; continue;
            }
            0x570bca => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x3c),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x570bdf; continue; }
            pc = 0x570bd5; continue;
            }
            0x570bd5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,16,c.get(3,16));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x570bdf => {
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x570bf1);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m, 0x570c05);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(7,16); c.sub(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x570c2b; continue; }
            pc = 0x570c24; continue;
            }
            0x570c24 => {
            { let a = c.get(7,16); c.sub(a,c.get(3,16),16); }
            c.set(0,32,c.get(7,32));
            pc = 0x570c30; continue;
            }
            0x570c2b => {
            { let a = c.get(2,16); c.sub(a,c.get(3,16),16); }
            c.set(0,32,c.get(2,32));
            pc = 0x570c30; continue;
            }
            0x570c30 => {
            if c.cond(2) { pc = 0x570c34; continue; }
            pc = 0x570c32; continue;
            }
            0x570c32 => {
            c.set(0,32,c.get(3,32));
            pc = 0x570c34; continue;
            }
            0x570c34 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0057af40(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57af40;
    loop {
        match pc {
            0x57af40 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[1].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x57af56; continue; }
            pc = 0x57af52; continue;
            }
            0x57af52 => {
            { let a = c.r[1].wrapping_add(0x30); wm(m,a,8,0x1u32); }
            pc = 0x57af56; continue;
            }
            0x57af56 => {
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00580b00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x580b00;
    loop {
        match pc {
            0x580b00 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x29),8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x580bc6; continue; }
            pc = 0x580b11; continue;
            }
            0x580b11 => {
            c.set(1,32,rm(m,0x6dccf8u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x24),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x580bc6; continue; }
            pc = 0x580b22; continue;
            }
            0x580b22 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x580b44; continue; }
            pc = 0x580b26; continue;
            }
            0x580b26 => {
            c.set(2,8,rm(m,0x6758dbu32,8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x28),8));
            { let a = c.get(0,8); let r = c.add(a,c.get(2,8),8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,0x6758dcu32,8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(2) { pc = 0x580b61; continue; }
            pc = 0x580b3e; continue;
            }
            0x580b3e => {
            { let a = c.r[6].wrapping_add(0x29); wm(m,a,8,0x1u32); }
            pc = 0x580b61; continue;
            }
            0x580b44 => {
            c.set(0,8,rm(m,0x6758dbu32,8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x28),8));
            { let a = c.get(2,8); let r = c.sub(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,0x6758dbu32,8));
            c.set(0,8,c.get(2,8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(7) { pc = 0x580b61; continue; }
            pc = 0x580b5d; continue;
            }
            0x580b5d => {
            { let a = c.r[6].wrapping_add(0x29); wm(m,a,8,0xffu32); }
            pc = 0x580b61; continue;
            }
            0x580b61 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0xc650),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc654),32));
            c.set(1,32,rm(m,0x6e1924u32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x6),16));
            c.set(1,32,0x6da5f0u32);
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x580bb3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x6758ddu32,8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            pc = 0x580bc6; continue;
            }
            0x580bc6 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00580bd0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x580bd0;
    loop {
        match pc {
            0x580bd0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,c.r[6].wrapping_add(0x4));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x580bdf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x580ca0; continue; }
            pc = 0x580be7; continue;
            }
            0x580be7 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x29); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,8,0xffu32); }
            c.set(1,8,rm(m,0x6758ddu32,8));
            c.set(2,32,rm(m,0x6dccf8u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6e1924u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x34),8));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x6e2fb2),8));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x6e2f98),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0xfffffffe),16));
            c.set(0,32,rm(m,0x6e1924u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0xc650),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc654),32));
            c.set(1,32,rm(m,0x6e1924u32,32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x6),16));
            c.set(1,32,0x6da5f0u32);
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x580c9f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x580ca0; continue;
            }
            0x580ca0 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00580d20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x580d20;
    loop {
        match pc {
            0x580d20 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = rm(m,c.r[6].wrapping_add(0x29),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x580d91; continue; }
            pc = 0x580d29; continue;
            }
            0x580d29 => {
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(0xc650),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xc654),32));
            c.set(0,32,rm(m,0x6e192cu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x580d47);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x28),8));
            { let a = c.get(1,32); let n = 0x18u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xffffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x580d5b);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e1924u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6),16));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2),16));
            c.set(1,16,rm(m,c.r[0],16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x580d8c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x580d91; continue;
            }
            0x580d91 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0058f3c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x58f3c0;
    loop {
        match pc {
            0x58f3c0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x4u32); }
            pc = 0x58f3f0; continue;
            }
            0x58f3ec => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x58f3f0; continue;
            }
            0x58f3f0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[7]),8));
            c.set(2,8,rm(m,c.r[7],8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x58f42a; continue; }
            pc = 0x58f3ff; continue;
            }
            0x58f3ff => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(0,32,c.get(6,32));
            if c.cond(15) { pc = 0x58f411; continue; }
            pc = 0x58f40f; continue;
            }
            0x58f40f => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x58f411; continue;
            }
            0x58f411 => {
            c.push(m,c.get(1,32));
            { let a = c.r[3].wrapping_add(c.r[7]); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x58f41a);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x58f42a; continue; }
            pc = 0x58f428; continue;
            }
            0x58f428 => {
            c.set(5,32,c.get(0,32));
            pc = 0x58f42a; continue;
            }
            0x58f42a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x58f3ec; continue; }
            pc = 0x58f436; continue;
            }
            0x58f436 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.get(5,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0058f440(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x58f440;
    loop {
        match pc {
            0x58f440 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x58f4d0; continue; }
            pc = 0x58f453; continue;
            }
            0x58f453 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(6,32,c.get(0,32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x4u32); }
            pc = 0x58f46e; continue;
            }
            0x58f46e => {
            c.set(1,32,c.r[6].wrapping_add(c.r[5]));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[7]),8));
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x58f49c; continue; }
            pc = 0x58f480; continue;
            }
            0x58f480 => {
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffff));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x58f495; continue; }
            pc = 0x58f490; continue;
            }
            0x58f490 => {
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            pc = 0x58f4bf; continue;
            }
            0x58f495 => {
            c.set(0,32,0x1u32);
            pc = 0x58f4bf; continue;
            }
            0x58f49c => {
            if c.cond(13) { pc = 0x58f4c1; continue; }
            pc = 0x58f49e; continue;
            }
            0x58f49e => {
            c.set(1,32,c.get(3,32));
            c.set(2,32,0x1u32);
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x58f4bc; continue; }
            pc = 0x58f4b7; continue;
            }
            0x58f4b7 => {
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            pc = 0x58f4bf; continue;
            }
            0x58f4bc => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x58f4bf; continue;
            }
            0x58f4bf => {
            { let a = c.r[6]; wm(m,a,8,c.get(0,8)); }
            pc = 0x58f4c1; continue;
            }
            0x58f4c1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x58f46e; continue; }
            pc = 0x58f4cd; continue;
            }
            0x58f4cd => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x58f4d0; continue;
            }
            0x58f4d0 => {
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00595b00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x595b00;
    loop {
        match pc {
            0x595b00 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x595b0b);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x595b16; continue; }
            pc = 0x595b0f; continue;
            }
            0x595b0f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x595b16 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,16); c.sub(a,0xea9cu32,16); }
            if c.cond(2) { pc = 0x595b3d; continue; }
            pc = 0x595b21; continue;
            }
            0x595b21 => {
            { let a = c.get(7,16); c.sub(a,0xee48u32,16); }
            if c.cond(3) { pc = 0x595b3d; continue; }
            pc = 0x595b28; continue;
            }
            0x595b28 => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x595b36);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x594aa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x595b3d => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x595b58);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x594b50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x595c06; continue; }
            pc = 0x595b64; continue;
            }
            0x595b64 => {
            c.set(0,32,0x4u32);
            c.set(2,32,c.r[6].wrapping_add(0xca0e));
            pc = 0x595b6f; continue;
            }
            0x595b6f => {
            { let a = rm(m,c.r[2],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x595b86; continue; }
            pc = 0x595b74; continue;
            }
            0x595b74 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x18u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0xeu32,32); }
            if c.cond(12) { pc = 0x595b6f; continue; }
            pc = 0x595b7d; continue;
            }
            0x595b7d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x595b86 => {
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x595b94; continue; }
            pc = 0x595b8b; continue;
            }
            0x595b8b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x595b94 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m,0x1u32);
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.set(6,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0xc998));
            c.push(m, 0x595bcb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x443150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x595c06; continue; }
            pc = 0x595bd2; continue;
            }
            0x595bd2 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[0],32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a628u32,32); c.push(m, 0x595bea); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(7,16)); }
            pc = 0x595c06; continue;
            }
            0x595c06 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0059bc90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x59bc90;
    loop {
        match pc {
            0x59bc90 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x59bcac);
            { let args = []; c.r[0] = host.call(m, 0x57ff20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x59bece; continue; }
            pc = 0x59bcb7; continue;
            }
            0x59bcb7 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc30c),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x59bcf1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x58dac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x510),32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x59bd1d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x4d0),32),32); c.set(1,32,r); }
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x59bd35); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x4d4),32),32); c.set(1,32,r); }
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x59bd4d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(5,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x4d0),32),32); c.set(5,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(5,32); let r = c.add(a,c.get(3,32),32); c.set(5,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x59bd5f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x4d4),32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x59bd79); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(3,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x4d0),32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x59bd8f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.get(0,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x4d4),32),32); c.set(2,32,r); }
            c.set(0,32,c.get(1,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fild32(m,c.r[4].wrapping_add(0x18));
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('/',v); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x22u32);
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fild32(m,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fild32(m,c.r[4].wrapping_add(0x1c));
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x1c),4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x20));
            c.fild32(m,c.r[4].wrapping_add(0x28));
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x20),4); c.fop('r',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fild32(m,c.r[4].wrapping_add(0x34));
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fild32(m,c.r[4].wrapping_add(0x30));
            c.fstp32(m,c.r[4].wrapping_add(0x20));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x20),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('*',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x28),4); c.fop('r',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x1c),4); c.fop('-',v); }
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('-',v); }
            c.fopp('/');
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('*',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x1c),4); c.fop('+',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.push(m, 0x59be6d);
            { let args = []; c.r[0] = host.call(m, 0x557d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x3f8u32,32); c.set(6,32,r); }
            c.set(1,32,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m, 0x59be81);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x568a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x59be8c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x568b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x24),4));
            c.set(6,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.or(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x0u32);
            c.push(m, 0x59beb3);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.fld(c.ld(m,c.r[4].wrapping_add(0x20),4));
            { let a = c.get(0,32); let r = c.sub(a,0xau32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x59bec0);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[6].wrapping_add(0x40),32); c.push(m, 0x59bec6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x40),8));
            { let a = c.r[7].wrapping_add(0x29); wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x59bece; continue;
            }
            0x59bece => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0059c410(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x59c410;
    loop {
        match pc {
            0x59c410 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x59c42f; continue; }
            pc = 0x59c420; continue;
            }
            0x59c420 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x59c426);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(3,32)); }
            pc = 0x59c42f; continue;
            }
            0x59c42f => {
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xb8); wm(m,a,32,0xffffffffu32); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x98); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x9c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x59c45e);
            { let args = []; c.r[0] = host.call(m, 0x4b1650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            { let a = c.r[6].wrapping_add(0xbc); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x1c8); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1c9); wm(m,a,8,0x1eu32); }
            { let a = c.r[6].wrapping_add(0x1c4); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x59c490; continue; }
            pc = 0x59c481; continue;
            }
            0x59c481 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x59c487);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,c.get(3,32)); }
            pc = 0x59c490; continue;
            }
            0x59c490 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005a7350(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a7350;
    loop {
        match pc {
            0x5a7350 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6dd736u32,16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.push(m,c.get(6,32));
            if c.cond(7) { pc = 0x5a7376; continue; }
            pc = 0x5a7367; continue;
            }
            0x5a7367 => {
            c.set(1,32,rm(m,0x6dd730u32,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            pc = 0x5a7378; continue;
            }
            0x5a7376 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x5a7378; continue;
            }
            0x5a7378 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x5a738c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x6),16); c.sub(a,0x1bu32,16); }
            if c.cond(4) { pc = 0x5a7399; continue; }
            pc = 0x5a7393; continue;
            }
            0x5a7393 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x5a7399 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xc),8));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5a73c4; continue; }
            pc = 0x5a73b1; continue;
            }
            0x5a73b1 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5a73c4; continue; }
            pc = 0x5a73be; continue;
            }
            0x5a73be => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x5a73c4 => {
            c.set(0,32,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00441ac0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x441ac0;
    loop {
        match pc {
            0x441ac0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x124),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x441ada; continue; }
            pc = 0x441ad1; continue;
            }
            0x441ad1 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x441ad7);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x441ada; continue;
            }
            0x441ada => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x74),32));
            { let a = c.r[6].wrapping_add(0x124); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x441b04; continue; }
            pc = 0x441ae7; continue;
            }
            0x441ae7 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x441afb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x441b01);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x441b04; continue;
            }
            0x441b04 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x128),32));
            { let a = c.r[6].wrapping_add(0x74); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x441b1a; continue; }
            pc = 0x441b11; continue;
            }
            0x441b11 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x441b17);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x441b1a; continue;
            }
            0x441b1a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x78),32));
            { let a = c.r[6].wrapping_add(0x128); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x441b30; continue; }
            pc = 0x441b27; continue;
            }
            0x441b27 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x441b2d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x441b30; continue;
            }
            0x441b30 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x78); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1b4); wm(m,a,16,c.get(3,16)); }
            if c.cond(4) { pc = 0x441c3d; continue; }
            pc = 0x441b4c; continue;
            }
            0x441b4c => {
            c.set(7,32,c.r[6].wrapping_add(0x18));
            c.set(5,32,0x14u32);
            pc = 0x441b54; continue;
            }
            0x441b54 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a610u32,32); c.push(m, 0x441b5d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a614u32,32); c.push(m, 0x441b66); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[7]; wm(m,a,32,c.get(3,32)); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x441b54; continue; }
            pc = 0x441b6e; continue;
            }
            0x441b6e => {
            c.set(7,32,c.r[6].wrapping_add(0xd0));
            c.set(5,32,0x14u32);
            pc = 0x441b79; continue;
            }
            0x441b79 => {
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a618u32,32); c.push(m, 0x441b82); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x441b79; continue; }
            pc = 0x441b88; continue;
            }
            0x441b88 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(7,32,rm(m,0x66a61cu32,32));
            c.set(5,32,rm(m,0x66a620u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x441ba9; continue; }
            pc = 0x441b9b; continue;
            }
            0x441b9b => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x441ba0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,c.get(2,32));
            { let target = c.get(5,32); c.push(m, 0x441ba6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            pc = 0x441ba9; continue;
            }
            0x441ba9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x441bbe; continue; }
            pc = 0x441bb0; continue;
            }
            0x441bb0 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x441bb5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x441bbb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            pc = 0x441bbe; continue;
            }
            0x441bbe => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x441bd3; continue; }
            pc = 0x441bc5; continue;
            }
            0x441bc5 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x441bca); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            { let target = c.get(5,32); c.push(m, 0x441bd0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x441bd3; continue;
            }
            0x441bd3 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x441be5; continue; }
            pc = 0x441bd9; continue;
            }
            0x441bd9 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x441bde); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            { let target = c.get(5,32); c.push(m, 0x441be3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6]; wm(m,a,32,c.get(3,32)); }
            pc = 0x441be5; continue;
            }
            0x441be5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x441bfa; continue; }
            pc = 0x441bec; continue;
            }
            0x441bec => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x441bf1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            { let target = c.get(5,32); c.push(m, 0x441bf7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            pc = 0x441bfa; continue;
            }
            0x441bfa => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x441c0f; continue; }
            pc = 0x441c01; continue;
            }
            0x441c01 => {
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x441c06); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            { let target = c.get(5,32); c.push(m, 0x441c0c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            pc = 0x441c0f; continue;
            }
            0x441c0f => {
            c.push(m,0x3e8u32);
            { let target = rm(m,0x66a2bcu32,32); c.push(m, 0x441c1a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6dccdcu32,32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x441c2a; continue; }
            pc = 0x441c24; continue;
            }
            0x441c24 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x441c2a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x441c2a; continue;
            }
            0x441c2a => {
            { let a = 0x6dccd0u32; wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a624u32,32); c.push(m, 0x441c3a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x68); wm(m,a,32,c.get(3,32)); }
            pc = 0x441c3d; continue;
            }
            0x441c3d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x138),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x130); wm(m,a,32,c.get(3,32)); }
            if c.cond(14) { pc = 0x441c6c; continue; }
            pc = 0x441c4f; continue;
            }
            0x441c4f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x441c5e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x138),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x441c4f; continue; }
            pc = 0x441c6c; continue;
            }
            0x441c6c => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x138),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x12c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x441c85);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x1c7); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x1c6); wm(m,a,8,c.get(3,8)); }
            { let v = c.pop(m); c.set(5,32,v); }
            if c.cond(4) { pc = 0x441cbc; continue; }
            pc = 0x441c9f; continue;
            }
            0x441c9f => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x441cb3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x441cb9);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x441cbc; continue;
            }
            0x441cbc => {
            { let a = c.r[6].wrapping_add(0xcc); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004c4460(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c4460;
    loop {
        match pc {
            0x4c4460 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,0x66a610u32,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,0x66a614u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x66a5c8u32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(6,32,c.r[1].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x4u32); }
            pc = 0x4c4488; continue;
            }
            0x4c4488 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c44b0; continue; }
            pc = 0x4c448e; continue;
            }
            0x4c448e => {
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x4c4491); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x4c44a0; continue; }
            pc = 0x4c4496; continue;
            }
            0x4c4496 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x4c449b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x4c44b0; continue; }
            pc = 0x4c44a0; continue;
            }
            0x4c44a0 => {
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            { let target = c.get(3,32); c.push(m, 0x4c44a5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            { let target = c.get(5,32); c.push(m, 0x4c44aa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            pc = 0x4c44b0; continue;
            }
            0x4c44b0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4c4488; continue; }
            pc = 0x4c44be; continue;
            }
            0x4c44be => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,0x66a618u32,32));
            c.set(7,32,0xau32);
            c.set(6,32,c.r[0].wrapping_add(0x14));
            pc = 0x4c44d0; continue;
            }
            0x4c44d0 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c44d9; continue; }
            pc = 0x4c44d6; continue;
            }
            0x4c44d6 => {
            c.push(m,c.get(0,32));
            { let target = c.get(3,32); c.push(m, 0x4c44d9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c44d9; continue;
            }
            0x4c44d9 => {
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x4c44d0; continue; }
            pc = 0x4c44e5; continue;
            }
            0x4c44e5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(1,32); let r = c.add(a,0x64u32,32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let args = []; c.r[0] = host.call(m, 0x64e642, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004c4620(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c4620;
    loop {
        match pc {
            0x4c4620 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            c.set(7,32,c.get(1,32));
            if c.cond(12) { pc = 0x4c46fa; continue; }
            pc = 0x4c4634; continue;
            }
            0x4c4634 => {
            { let a = c.get(6,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x4c46fa; continue; }
            pc = 0x4c463d; continue;
            }
            0x4c463d => {
            { let a = rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x14),32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4c46fa; continue; }
            pc = 0x4c4647; continue;
            }
            0x4c4647 => {
            { let a = rm(m,c.r[7].wrapping_add(0x70),32); c.sub(a,0xau32,32); }
            if c.cond(12) { pc = 0x4c4673; continue; }
            pc = 0x4c464d; continue;
            }
            0x4c464d => {
            c.set(1,32,c.r[7].wrapping_add(0x64));
            c.push(m, 0x4c4655);
            { let args = []; c.r[0] = host.call(m, 0x64e725, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4c4673; continue; }
            pc = 0x4c465b; continue;
            }
            0x4c465b => {
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x4c4673; continue; }
            pc = 0x4c465f; continue;
            }
            0x4c465f => {
            { let a = c.get(3,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x4c4673; continue; }
            pc = 0x4c4664; continue;
            }
            0x4c4664 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a618u32,32); c.push(m, 0x4c466f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[7].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            pc = 0x4c4673; continue;
            }
            0x4c4673 => {
            c.set(1,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6bf118),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4c4680);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c46fa; continue; }
            pc = 0x4c4687; continue;
            }
            0x4c4687 => {
            { let a = c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4c4690);
            { let args = []; c.r[0] = host.call(m, 0x4bc970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6bf118),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c469e);
            { let args = []; c.r[0] = host.call(m, 0x6389a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4c46cc; continue; }
            pc = 0x4c46a7; continue;
            }
            0x4c46a7 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x4c46ad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6389b8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(5,32));
            { let target = rm(m,0x66a5e0u32,32); c.push(m, 0x4c46b9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x4c46cc; continue; }
            pc = 0x4c46c1; continue;
            }
            0x4c46c1 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4c46c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x6389ac, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4c46cc; continue;
            }
            0x4c46cc => {
            c.push(m,c.get(3,32));
            c.push(m, 0x4c46d2);
            { let args = []; c.r[0] = host.call(m, 0x6389a6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4c46f1; continue; }
            pc = 0x4c46dd; continue;
            }
            0x4c46dd => {
            c.set(0,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6bf118),32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5d4u32,32); c.push(m, 0x4c46ed); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c46f1; continue;
            }
            0x4c46f1 => {
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[7].wrapping_add(0x64));
            c.push(m, 0x4c46fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64e6fc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c46fa; continue;
            }
            0x4c46fa => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0057ff20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57ff20;
    loop {
        match pc {
            0x57ff20 => {
            c.set(0,32,rm(m,0x6e2f14u32,32));
            c.set(1,32,rm(m,0x6e2f18u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x57ff4d; continue; }
            pc = 0x57ff32; continue;
            }
            0x57ff32 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x57ff3a; continue;
            }
            0x57ff3a => {
            c.set(6,32,rm(m,c.r[0],32));
            { let a = rm(m,c.r[6].wrapping_add(0x20),32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x57ff46; continue; }
            pc = 0x57ff41; continue;
            }
            0x57ff41 => {
            { let a = rm(m,c.r[6].wrapping_add(0x24),32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x57ff53; continue; }
            pc = 0x57ff46; continue;
            }
            0x57ff46 => {
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x57ff3a; continue; }
            pc = 0x57ff4d; continue;
            }
            0x57ff4d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x57ff53 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x18),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x29),8));
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x57ff68; continue; }
            pc = 0x57ff5e; continue;
            }
            0x57ff5e => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x1c),32); c.push(m, 0x57ff65); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x29); wm(m,a,8,c.get(3,8)); }
            pc = 0x57ff68; continue;
            }
            0x57ff68 => {
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
