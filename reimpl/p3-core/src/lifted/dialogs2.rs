//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn route_stop_goods_dialog_reset(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x403480;
    loop {
        match pc {
            0x403480 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x40348e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x403495);
            { let args = []; c.r[0] = host.call(m, 0x4b1650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0xb4));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x4adc); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4034b7);
            { let args = []; c.r[0] = host.call(m, 0x40a500, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf0),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4034c7; continue; }
            pc = 0x4034c1; continue;
            }
            0x4034c1 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x4034c7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4034c7; continue;
            }
            0x4034c7 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x100),32));
            { let a = c.r[6].wrapping_add(0xf0); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4034dd; continue; }
            pc = 0x4034d7; continue;
            }
            0x4034d7 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x4034dd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4034dd; continue;
            }
            0x4034dd => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf4),32));
            { let a = c.r[6].wrapping_add(0x100); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4034f3; continue; }
            pc = 0x4034ed; continue;
            }
            0x4034ed => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x4034f3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4034f3; continue;
            }
            0x4034f3 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.r[6].wrapping_add(0xf4); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x403509; continue; }
            pc = 0x403503; continue;
            }
            0x403503 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x403509); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x403509; continue;
            }
            0x403509 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xfc),32));
            { let a = c.r[6].wrapping_add(0xf8); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x40351f; continue; }
            pc = 0x403519; continue;
            }
            0x403519 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x40351f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40351f; continue;
            }
            0x40351f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x104),32));
            { let a = c.r[6].wrapping_add(0xfc); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x403535; continue; }
            pc = 0x40352f; continue;
            }
            0x40352f => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x403535); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x403535; continue;
            }
            0x403535 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x108),32));
            { let a = c.r[6].wrapping_add(0x104); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x40354b; continue; }
            pc = 0x403545; continue;
            }
            0x403545 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x40354b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40354b; continue;
            }
            0x40354b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10c),32));
            { let a = c.r[6].wrapping_add(0x108); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x403561; continue; }
            pc = 0x40355b; continue;
            }
            0x40355b => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x403561); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x403561; continue;
            }
            0x403561 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x110),32));
            { let a = c.r[6].wrapping_add(0x10c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x403577; continue; }
            pc = 0x403571; continue;
            }
            0x403571 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x403577); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x403577; continue;
            }
            0x403577 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x114),32));
            { let a = c.r[6].wrapping_add(0x110); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x40358d; continue; }
            pc = 0x403587; continue;
            }
            0x403587 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x40358d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40358d; continue;
            }
            0x40358d => {
            c.push(m,0x50u32);
            c.set(1,32,c.r[6].wrapping_add(0x668));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x114); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4035a2);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x50u32);
            c.set(2,32,c.r[6].wrapping_add(0x6b8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4035b1);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x50u32);
            c.set(0,32,c.r[6].wrapping_add(0x708));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4035c0);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xac); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xb0); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x4ad8); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4035dc);
            { let args = []; c.r[0] = host.call(m, 0x41c3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x118),32));
            c.set(1,32,c.r[6].wrapping_add(0x118));
            { let a = c.r[6].wrapping_add(0x5448); wm(m,a,8,c.get(3,8)); }
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4035f4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x200),32));
            c.set(1,32,c.r[6].wrapping_add(0x200));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x403606); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2e8),32));
            c.set(1,32,c.r[6].wrapping_add(0x2e8));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x403618); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x40361f);
            { let args = []; c.r[0] = host.call(m, 0x40a8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4036ff; continue; }
            pc = 0x403627; continue;
            }
            0x403627 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x40362b; continue;
            }
            0x40362b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[5]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[5]));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x40363d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x100),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x40364f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf4),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x403661); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x403673); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xfc),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x403685); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x104),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x403697); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10c),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4036a9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x108),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4036bb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x114),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4036cd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x110),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x4036df); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xb4));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0xf8u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            c.push(m, 0x4036f7);
            { let args = []; c.r[0] = host.call(m, 0x40a8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x40362b; continue; }
            pc = 0x4036ff; continue;
            }
            0x4036ff => {
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

pub fn route_stop_goods_dialog_open(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x405a20;
    loop {
        match pc {
            0x405a20 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65b2b5u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x130u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[5].wrapping_add(0xa4),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x405a53; continue; }
            pc = 0x405a4a; continue;
            }
            0x405a4a => {
            c.set(0,32,rm(m,c.r[5],32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x405a53); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x405a53; continue;
            }
            0x405a53 => {
            { let a = c.r[5].wrapping_add(0x48); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x405a67); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x58));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x405a77); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(7,32,rm(m,0x6cbab8u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,0x6cbabcu32,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6cbab8u32,32));
            c.set(2,32,rm(m,0x6cbaccu32,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6cbabcu32,32));
            c.set(1,32,rm(m,0x6cbae0u32,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.set(2,32,c.get(0,32));
            { let a = c.r[5].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.and(a,0x80000003u32,32); c.set(2,32,r); }
            if c.cond(9) { pc = 0x405adc; continue; }
            pc = 0x405ad7; continue;
            }
            0x405ad7 => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.or(a,0xfffffffcu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x405adc; continue;
            }
            0x405adc => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x158),8));
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x150),32));
            { let a = c.r[5].wrapping_add(0xa4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x154),32));
            { let a = c.r[5].wrapping_add(0xa8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0x547c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x405b24);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x544c),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.r[5].wrapping_add(0x5480); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x5450),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(6,32,c.r[5].wrapping_add(0xb4));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x405b60);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x405b6a);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x405b74);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x405b8f);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x405ba4);
            { let args = []; c.r[0] = host.call(m, 0x462390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xa4),32));
            c.set(7,32,c.r[4].wrapping_add(0x64));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,0x37u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd72cu32,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.rep_movsd(m);
            c.set(1,32,c.r[5].wrapping_add(0xb4));
            c.push(m, 0x405bd9);
            { let args = []; c.r[0] = host.call(m, 0x40a8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x406157; continue; }
            pc = 0x405be1; continue;
            }
            0x405be1 => {
            c.set(1,32,c.r[5].wrapping_add(0x6b8));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            pc = 0x405beb; continue;
            }
            0x405beb => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(7,32,c.r[5].wrapping_add(0xb4));
            c.set(2,8,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x68),8));
            c.set(1,32,c.get(7,32));
            c.set(6,32,c.get(2,32));
            c.push(m, 0x405c04);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x405c0f);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(7,32,c.r[5].wrapping_add(0xb4));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x20),32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x405c33);
            { let args = []; c.r[0] = host.call(m, 0x40a980, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x406134; continue; }
            pc = 0x405c3b; continue;
            }
            0x405c3b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x405c51);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x40aa40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); c.sub(a,0x9u32,32); }
            if c.cond(7) { pc = 0x406114; continue; }
            pc = 0x405c63; continue;
            }
            0x405c63 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4066c0)); continue;
            }
            0x405c6a => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xf0),32));
            c.set(7,32,c.get(6,32));
            { let a = c.get(7,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(6,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x7c),32); c.push(m, 0x405c86); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xf0),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x405c96);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xf0),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x50); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xf0),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x405cbb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4cc0));
            { let a = c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4cc0); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            pc = 0x406111; continue;
            }
            0x405ce0 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x100),32));
            c.set(1,32,c.r[6].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[7]));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x405d03); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xf4),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x405d17); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xf8),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x405d29); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfc),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x405d3b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x104),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x405d4d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4db0));
            { let a = c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4db0); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0xe0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x405da0; continue; }
            pc = 0x405d7b; continue;
            }
            0x405d7b => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x104),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x405d8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4ae0); wm(m,a,32,0x4u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x0u32); }
            pc = 0x405eaa; continue;
            }
            0x405da0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x80),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x405def; continue; }
            pc = 0x405dab; continue;
            }
            0x405dab => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x100),32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x405dbb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4ae0); wm(m,a,32,0x3u32); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x110),32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x405dd7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x114),32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x405de8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            pc = 0x405e90; continue;
            }
            0x405def => {
            if c.cond(14) { pc = 0x405e30; continue; }
            pc = 0x405df1; continue;
            }
            0x405df1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xf8),32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x405e01);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4ae0); wm(m,a,32,0x1u32); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x110),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x405e1b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x114),32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x405e2c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            pc = 0x405e90; continue;
            }
            0x405e30 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x405e51; continue; }
            pc = 0x405e34; continue;
            }
            0x405e34 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfc),32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x405e44);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4ae0); wm(m,a,32,0x2u32); }
            pc = 0x405e6e; continue;
            }
            0x405e51 => {
            if c.cond(13) { pc = 0x405eaa; continue; }
            pc = 0x405e53; continue;
            }
            0x405e53 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xf4),32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x405e63);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4ae0); wm(m,a,32,0x0u32); }
            pc = 0x405e6e; continue;
            }
            0x405e6e => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x110),32));
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x405e7d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x114),32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x405e8e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            pc = 0x405e90; continue;
            }
            0x405e90 => {
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x2698),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x2698));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x405eaa); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x405eaa; continue;
            }
            0x405eaa => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4ae0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x405ec6; continue; }
            pc = 0x405eb5; continue;
            }
            0x405eb5 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(4) { pc = 0x405ec6; continue; }
            pc = 0x405eba; continue;
            }
            0x405eba => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x405ec6; continue; }
            pc = 0x405ec2; continue;
            }
            0x405ec2 => {
            c.set(0,8,0x1u32);
            pc = 0x405ec8; continue;
            }
            0x405ec6 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x405ec8; continue;
            }
            0x405ec8 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x110),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x405ed9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4ae0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x405ef5; continue; }
            pc = 0x405ee4; continue;
            }
            0x405ee4 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(4) { pc = 0x405ef5; continue; }
            pc = 0x405ee9; continue;
            }
            0x405ee9 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x405ef5; continue; }
            pc = 0x405ef1; continue;
            }
            0x405ef1 => {
            c.set(0,8,0x1u32);
            pc = 0x405ef7; continue;
            }
            0x405ef5 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x405ef7; continue;
            }
            0x405ef7 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x114),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x405f08); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x108),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x405f1f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x10c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x405f34); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(7,32,c.r[1].wrapping_add(c.r[5]));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x758),32));
            c.set(1,32,c.r[7].wrapping_add(0x758));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x405f57); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4ae0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x405f73; continue; }
            pc = 0x405f62; continue;
            }
            0x405f62 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(4) { pc = 0x405f73; continue; }
            pc = 0x405f67; continue;
            }
            0x405f67 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x405f73; continue; }
            pc = 0x405f6f; continue;
            }
            0x405f6f => {
            c.set(0,8,0x1u32);
            pc = 0x405f75; continue;
            }
            0x405f73 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x405f75; continue;
            }
            0x405f75 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x2698),32));
            { let a = c.get(7,32); let r = c.add(a,0x2698u32,32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x405f8a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4ae0),32));
            { let a = c.r[5].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4c20); wm(m,a,32,c.get(0,32)); }
            pc = 0x406114; continue;
            }
            0x405f9d => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x10c),32));
            c.set(1,32,c.r[6].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.get(3,32));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x405fbc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10c),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x405fd0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x5080));
            pc = 0x406100; continue;
            }
            0x405fdf => {
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(7,32,c.r[1].wrapping_add(c.r[5]).wrapping_add(0x758));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x405ff7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x406003); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4f90));
            pc = 0x406100; continue;
            }
            0x406012 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x108),32));
            c.set(1,32,c.r[6].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.get(3,32));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x406031);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x108),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x406045); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x5170));
            pc = 0x406100; continue;
            }
            0x406054 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x114),32));
            c.set(1,32,c.r[6].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.get(3,32));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x406073);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x114),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x406087); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x5260));
            pc = 0x406100; continue;
            }
            0x406093 => {
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(7,32,c.r[1].wrapping_add(c.r[5]).wrapping_add(0x2698));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4060ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4060b7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4ea0));
            pc = 0x406100; continue;
            }
            0x4060c3 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x110),32));
            c.set(1,32,c.r[6].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.get(3,32));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4060e2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x110),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4060f6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x5350));
            pc = 0x406100; continue;
            }
            0x406100 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x406111; continue;
            }
            0x406111 => {
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x406114; continue;
            }
            0x406114 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(7,32,c.r[5].wrapping_add(0xb4));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x40612a);
            { let args = []; c.r[0] = host.call(m, 0x40a980, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x24),32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x405c3b; continue; }
            pc = 0x406134; continue;
            }
            0x406134 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x40614f);
            { let args = []; c.r[0] = host.call(m, 0x40a8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x405beb; continue; }
            pc = 0x406157; continue;
            }
            0x406157 => {
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xa8),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x148); wm(m,a,32,0x0u32); }
            c.push(m,0xc8u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(7,16,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x8),16));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x547c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4061bd; continue; }
            pc = 0x406196; continue;
            }
            0x406196 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4061a1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x50a2b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(6,32,c.get(0,32));
            c.push(m,0x7d0u32);
            c.push(m,c.get(7,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4061b7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x50a2b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            pc = 0x4061ea; continue;
            }
            0x4061bd => {
            c.push(m, 0x4061c2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x518460, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0x10624dd3u32);
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.get(2,32));
            pc = 0x4061ea; continue;
            }
            0x4061ea => {
            c.set(2,32,c.r[5].wrapping_add(0x4bd0));
            c.set(7,32,c.r[5].wrapping_add(0x8d8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            pc = 0x406214; continue;
            }
            0x406208 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x406214; continue;
            }
            0x406214 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x40622e; continue; }
            pc = 0x406222; continue;
            }
            0x406222 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.r[7]; wm(m,a,32,c.get(6,32)); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x406243; continue; }
            pc = 0x40622b; continue;
            }
            0x40622b => {
            c.push(m,c.get(6,32));
            pc = 0x406238; continue;
            }
            0x40622e => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x406243; continue; }
            pc = 0x406237; continue;
            }
            0x406237 => {
            c.push(m,c.get(0,32));
            pc = 0x406238; continue;
            }
            0x406238 => {
            c.set(1,32,c.r[7].wrapping_add(0xfffffe80));
            c.push(m, 0x406243);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x406243; continue;
            }
            0x406243 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x1f48),32));
            c.set(0,32,0x2710u32);
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[7].wrapping_add(0x1f40); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x406264; continue; }
            pc = 0x406258; continue;
            }
            0x406258 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[7].wrapping_add(0x1dc0));
            c.push(m, 0x406264);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x406264; continue;
            }
            0x406264 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x40627d; continue; }
            pc = 0x406271; continue;
            }
            0x406271 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[7].wrapping_add(0xfffffe80));
            c.push(m, 0x40627d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40627d; continue;
            }
            0x40627d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(c.r[1]).wrapping_add(0x68),8));
            c.set(6,32,rm(m,c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xe0),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[2].wrapping_add(0xa0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x80));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4062ad);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3b9aca00u32,32); }
            if c.cond(4) { pc = 0x4062eb; continue; }
            pc = 0x4062b7; continue;
            }
            0x4062b7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4062d8; continue; }
            pc = 0x4062c5; continue;
            }
            0x4062c5 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            pc = 0x4062e9; continue;
            }
            0x4062d8 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            pc = 0x4062e9; continue;
            }
            0x4062e9 => {
            c.set(6,32,c.get(2,32));
            pc = 0x4062eb; continue;
            }
            0x4062eb => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(6,32));
            { let a = c.r[1]; wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4062f7);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3b9aca00u32,32); }
            if c.cond(4) { pc = 0x40631d; continue; }
            pc = 0x406301; continue;
            }
            0x406301 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(1,32,c.r[7].wrapping_add(0xfffffe80));
            if c.cond(14) { pc = 0x406313; continue; }
            pc = 0x40630b; continue;
            }
            0x40630b => {
            c.push(m,c.get(6,32));
            c.push(m, 0x406311);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x406361; continue;
            }
            0x406313 => {
            { let a = c.get(6,32); let r = c.neg(a,32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m, 0x40631b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x406361; continue;
            }
            0x40631d => {
            c.set(6,32,c.r[7].wrapping_add(0xfffffe80));
            c.push(m,0xffffffffu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x40632c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x69a368u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            c.push(m, 0x40633c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14c); wm(m,a,8,0x1u32); }
            c.push(m, 0x406350);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x148); wm(m,a,8,0x0u32); }
            c.push(m, 0x406361);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x406361; continue;
            }
            0x406361 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(6,32,rm(m,c.r[1],32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[2].wrapping_add(0xffffffb0); wm(m,a,32,c.get(6,32)); }
            if c.cond(14) { pc = 0x406393; continue; }
            pc = 0x406372; continue;
            }
            0x406372 => {
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.push(m,0x692c7cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x406382);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4b30); wm(m,a,32,c.get(6,32)); }
            c.push(m,c.get(6,32));
            pc = 0x4063c8; continue;
            }
            0x406393 => {
            if c.cond(13) { pc = 0x4063b8; continue; }
            pc = 0x406395; continue;
            }
            0x406395 => {
            { let a = c.get(6,32); let r = c.neg(a,32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.push(m,0x692c80u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4063a7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4b30); wm(m,a,32,c.get(6,32)); }
            c.push(m,c.get(6,32));
            pc = 0x4063c8; continue;
            }
            0x4063b8 => {
            c.push(m,0x692c84u32);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x4063c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            pc = 0x4063c8; continue;
            }
            0x4063c8 => {
            { let a = c.get(7,32); let r = c.add(a,0x1dc0u32,32); c.set(7,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4063d5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4063e1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4b30),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x40641f; continue; }
            pc = 0x4063f4; continue;
            }
            0x4063f4 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x672c14),8));
            c.fld(c.ld(m,c.r[6].wrapping_mul(4).wrapping_add(0x673a18),4));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x40640d; continue; }
            pc = 0x406405; continue;
            }
            0x406405 => {
            { let v = c.ld(m,0x66a91cu32,4); c.fop('*',v); }
            pc = 0x406413; continue;
            }
            0x40640d => {
            { let v = c.ld(m,0x66a918u32,4); c.fop('*',v); }
            pc = 0x406413; continue;
            }
            0x406413 => {
            c.push(m, 0x406418);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x4b30); wm(m,a,32,c.get(0,32)); }
            pc = 0x40641f; continue;
            }
            0x40641f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x190u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,0x14u32,32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x406208; continue; }
            pc = 0x406446; continue;
            }
            0x406446 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xa4),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(6,32,c.r[5].wrapping_add(0x200));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[5].wrapping_add(0x5448); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            if c.cond(5) { pc = 0x406474; continue; }
            pc = 0x40646b; continue;
            }
            0x40646b => {
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x406470); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            pc = 0x40647b; continue;
            }
            0x406474 => {
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x406479); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            pc = 0x40647b; continue;
            }
            0x40647b => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x2e8),32));
            c.set(7,32,c.r[5].wrapping_add(0x2e8));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x40648c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x545c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(4,32));
            { let a = c.get(0,32); let r = c.add(a,0x16eu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[5].wrapping_add(0x118));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4064c6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4b4e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,c.get(4,32));
            { let a = c.get(0,32); let r = c.add(a,0x1cu32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4064ed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4b4e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,c.get(4,32));
            { let a = c.get(0,32); let r = c.add(a,0x47u32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(3,32));
            c.push(m, 0x406514);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4b4e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x59f4),32));
            c.set(6,32,c.r[5].wrapping_add(0x59f4));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(2,32); let r = c.add(a,0x76u32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x406534); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x590c),32));
            c.set(1,32,c.r[5].wrapping_add(0x590c));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x406548); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x573c),32));
            c.set(1,32,c.r[5].wrapping_add(0x573c));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x40655c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x5824),32));
            c.set(1,32,c.r[5].wrapping_add(0x5824));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x406570); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x5654),32));
            c.set(1,32,c.r[5].wrapping_add(0x5654));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x406584); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x40658c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,0xbcu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0x5484));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4065bb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4b4e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,c.get(4,32));
            { let a = c.get(0,32); let r = c.add(a,0xe7u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,c.r[5].wrapping_add(0x556c));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4065ea);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4b4e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x132u32,32); c.set(0,32,r); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0x3d0));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x406619);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4b4e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,c.r[5].wrapping_add(0x590));
            { let a = c.get(2,32); let r = c.add(a,0xe7u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x1eu32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x40663f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x40664b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x5470),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x5474),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(6,32,c.r[5].wrapping_add(0x4b8));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffe2));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x406679);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x406685); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[5].wrapping_add(0x5adc); wm(m,a,32,0x4u32); }
            { let a = c.r[4].wrapping_add(0x148); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4066a3);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x140),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x13cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn auto_trade_goods_dialog_process_row_buttons(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x406d50;
    loop {
        match pc {
            0x406d50 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65b2efu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x4cu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x48));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x406d86);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[6].wrapping_add(0x4ae0));
            c.set(3,32,c.r[6].wrapping_add(0x2820));
            c.set(5,32,c.get(0,32));
            pc = 0x406d94; continue;
            }
            0x406d94 => {
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(1,32); let r = c.and(a,0x80000001u32,32); c.set(1,32,r); }
            if c.cond(9) { pc = 0x406da4; continue; }
            pc = 0x406d9f; continue;
            }
            0x406d9f => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.or(a,0xfffffffeu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x406da4; continue;
            }
            0x406da4 => {
            if c.cond(4) { pc = 0x406deb; continue; }
            pc = 0x406da6; continue;
            }
            0x406da6 => {
            { let a = rm(m,c.r[3],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x406dbe; continue; }
            pc = 0x406dab; continue;
            }
            0x406dab => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x114),32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[7]));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x406dbc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x406dcd; continue;
            }
            0x406dbe => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x114),32));
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x406dcd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x406dcd; continue;
            }
            0x406dcd => {
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x110),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            if c.cond(5) { pc = 0x406de6; continue; }
            pc = 0x406de2; continue;
            }
            0x406de2 => {
            c.push(m,0x0u32);
            pc = 0x406de8; continue;
            }
            0x406de6 => {
            c.push(m,0x1u32);
            pc = 0x406de8; continue;
            }
            0x406de8 => {
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x406deb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x406deb; continue;
            }
            0x406deb => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xffffe0c0),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10c),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            if c.cond(5) { pc = 0x406e06; continue; }
            pc = 0x406e02; continue;
            }
            0x406e02 => {
            c.push(m,0x0u32);
            pc = 0x406e08; continue;
            }
            0x406e06 => {
            c.push(m,0x1u32);
            pc = 0x406e08; continue;
            }
            0x406e08 => {
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x406e0b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xffffe0c0),32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xffffe0b8),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x108),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            if c.cond(5) { pc = 0x406e2b; continue; }
            pc = 0x406e27; continue;
            }
            0x406e27 => {
            c.push(m,0x0u32);
            pc = 0x406e2d; continue;
            }
            0x406e2b => {
            c.push(m,0x1u32);
            pc = 0x406e2d; continue;
            }
            0x406e2d => {
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x406e30); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x190u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x1220u32,32); }
            if c.cond(12) { pc = 0x406d94; continue; }
            pc = 0x406e4b; continue;
            }
            0x406e4b => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x406e4f; continue;
            }
            0x406e4f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x110),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m, 0x406e5c);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x406eb3; continue; }
            pc = 0x406e60; continue;
            }
            0x406e60 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x114),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m, 0x406e6d);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x406f05; continue; }
            pc = 0x406e75; continue;
            }
            0x406e75 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x108),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m, 0x406e82);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x406f55; continue; }
            pc = 0x406e8a; continue;
            }
            0x406e8a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m, 0x406e97);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x406fce; continue; }
            pc = 0x406e9f; continue;
            }
            0x406e9f => {
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,0x1220u32,32); }
            if c.cond(12) { pc = 0x406e4f; continue; }
            pc = 0x406eae; continue;
            }
            0x406eae => {
            pc = 0x407235; continue;
            }
            0x406eb3 => {
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,32,c.r[0].wrapping_add(c.r[6]));
            c.set(1,32,c.r[3].wrapping_add(0x2698));
            c.push(m, 0x406ecb);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x406ed3);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[3].wrapping_add(0x8e0),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x406eeb; continue; }
            pc = 0x406ee6; continue;
            }
            0x406ee6 => {
            c.set(7,32,0x3b9aca00u32);
            pc = 0x406eeb; continue;
            }
            0x406eeb => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,0x2710u32);
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x407084; continue; }
            pc = 0x406efc; continue;
            }
            0x406efc => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x407084; continue;
            }
            0x406f05 => {
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(3,32,c.r[1].wrapping_add(c.r[6]));
            c.set(1,32,c.r[3].wrapping_add(0x2698));
            c.push(m, 0x406f1d);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x406f25);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[3].wrapping_add(0x8e0),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x406f3d; continue; }
            pc = 0x406f38; continue;
            }
            0x406f38 => {
            c.set(7,32,0x3b9aca00u32);
            pc = 0x406f3d; continue;
            }
            0x406f3d => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); c.sub(a,0x1u32,32); }
            if c.cond(13) { pc = 0x407084; continue; }
            pc = 0x406f48; continue;
            }
            0x406f48 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x1u32); }
            pc = 0x407084; continue;
            }
            0x406f55 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4ae0),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x406f86; continue; }
            pc = 0x406f5f; continue;
            }
            0x406f5f => {
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x2698));
            c.push(m, 0x406f75);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x406f7d);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x406f8e; continue;
            }
            0x406f86 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            pc = 0x406f8e; continue;
            }
            0x406f8e => {
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,32,c.r[0].wrapping_add(c.r[6]));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x8e0),32));
            c.set(1,32,c.r[3].wrapping_add(0x758));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x406fb6; continue; }
            pc = 0x406fad; continue;
            }
            0x406fad => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,0x1u32);
            pc = 0x40707f; continue;
            }
            0x406fb6 => {
            c.push(m, 0x406fbb);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x406fc3);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x407084; continue;
            }
            0x406fce => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4ae0),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x406fff; continue; }
            pc = 0x406fd8; continue;
            }
            0x406fd8 => {
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2698));
            c.push(m, 0x406fee);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x406ff6);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x407007; continue;
            }
            0x406fff => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            pc = 0x407007; continue;
            }
            0x407007 => {
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(3,32,c.r[1].wrapping_add(c.r[6]));
            c.set(1,32,c.r[3].wrapping_add(0x758));
            c.push(m, 0x40701f);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x407027);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x407084; continue; }
            pc = 0x407032; continue;
            }
            0x407032 => {
            c.push(m,0xffffffffu32);
            c.set(1,32,c.r[3].wrapping_add(0x758));
            c.set(7,32,0x3b9aca00u32);
            c.push(m, 0x407044);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x69a368u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            c.push(m, 0x407054);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.r[3].wrapping_add(0x758));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,8,0x1u32); }
            c.push(m, 0x407069);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            c.push(m, 0x407077);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[3].wrapping_add(0x758));
            c.push(m,0x0u32);
            pc = 0x40707f; continue;
            }
            0x40707f => {
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x407084); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x407084; continue;
            }
            0x407084 => {
            { let a = c.get(5,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x407235; continue; }
            pc = 0x40708d; continue;
            }
            0x40708d => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa4),16));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x8d8),32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0x69u32); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,0x3b9aca00u32,32); }
            c.set(2,16,rm(m,c.r[2].wrapping_add(c.r[1]),8));
            { let a = c.r[4].wrapping_add(0x4e); wm(m,a,16,c.get(2,16)); }
            if c.cond(4) { pc = 0x407134; continue; }
            pc = 0x4070cb; continue;
            }
            0x4070cb => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            if c.cond(4) { pc = 0x407106; continue; }
            pc = 0x4070de; continue;
            }
            0x4070de => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x40713d; continue; }
            pc = 0x4070ef; continue;
            }
            0x4070ef => {
            c.set(0,32,0x51eb851fu32);
            c.set(1,32,c.get(2,32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(7,32,c.get(2,32));
            pc = 0x407139; continue;
            }
            0x407106 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x40713d; continue; }
            pc = 0x40711d; continue;
            }
            0x40711d => {
            c.set(0,32,0x10624dd3u32);
            c.set(1,32,c.get(2,32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(7,32,c.get(2,32));
            pc = 0x407139; continue;
            }
            0x407134 => {
            c.set(1,32,0x3b9aca00u32);
            pc = 0x407139; continue;
            }
            0x407139 => {
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            pc = 0x40713d; continue;
            }
            0x40713d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4ae0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x40714e; continue; }
            pc = 0x407148; continue;
            }
            0x407148 => {
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            pc = 0x40714e; continue;
            }
            0x40714e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x407161; continue; }
            pc = 0x40715b; continue;
            }
            0x40715b => {
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            pc = 0x407161; continue;
            }
            0x407161 => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,16,c.get(5,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x6b8));
            pc = 0x40716e; continue;
            }
            0x40716e => {
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x40717f; continue; }
            pc = 0x407172; continue;
            }
            0x407172 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x40716e; continue; }
            pc = 0x40717b; continue;
            }
            0x40717b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            pc = 0x40717f; continue;
            }
            0x40717f => {
            c.set(1,32,c.r[4].wrapping_add(0x48));
            { let a = c.r[4].wrapping_add(0x52); wm(m,a,16,c.get(0,16)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x407193);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); c.sub(a,0x3b9aca00u32,32); }
            if c.cond(4) { pc = 0x4071a9; continue; }
            pc = 0x40719b; continue;
            }
            0x40719b => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[3].wrapping_add(0x758));
            c.push(m, 0x4071a7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4071e7; continue;
            }
            0x4071a9 => {
            c.set(7,32,c.r[3].wrapping_add(0x758));
            c.push(m,0xffffffffu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4071b8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x69a368u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            c.push(m, 0x4071c8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,8,0x2u32); }
            c.push(m, 0x4071d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            c.push(m, 0x4071e7);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4071e7; continue;
            }
            0x4071e7 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4ae0),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x407235; continue; }
            pc = 0x4071f1; continue;
            }
            0x4071f1 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x6b8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4b30); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4ae0),32));
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(5) { pc = 0x407225; continue; }
            pc = 0x40720f; continue;
            }
            0x40720f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x6b8),32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4b30); wm(m,a,32,c.get(0,32)); }
            pc = 0x407229; continue;
            }
            0x407225 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x407229; continue;
            }
            0x407229 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[3].wrapping_add(0x2698));
            c.push(m, 0x407235);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x407235; continue;
            }
            0x407235 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x2824));
            pc = 0x40723d; continue;
            }
            0x40723d => {
            { let a = rm(m,c.r[7].wrapping_add(0xffffe0bc),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x407260; continue; }
            pc = 0x407246; continue;
            }
            0x407246 => {
            c.set(1,32,c.r[7].wrapping_add(0xffffdf34));
            c.push(m, 0x407251);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x407259);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(3,32,c.get(0,32));
            pc = 0x407265; continue;
            }
            0x407260 => {
            c.set(3,32,0x3b9aca00u32);
            pc = 0x407265; continue;
            }
            0x407265 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xffffe0c0),8));
            { let a = c.r[7].wrapping_add(0xffffe0c0); wm(m,a,8,0x0u32); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x407312; continue; }
            pc = 0x40727a; continue;
            }
            0x40727a => {
            c.set(3,32,c.r[7].wrapping_add(0xffffdf34));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x407287);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x407291);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4072a5; continue; }
            pc = 0x40729c; continue;
            }
            0x40729c => {
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4072a5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4072a5; continue;
            }
            0x4072a5 => {
            c.set(3,32,c.r[7].wrapping_add(0xfffffe74));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4072b2);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4072ba);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[7],8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[7]; wm(m,a,8,0x0u32); }
            if c.cond(5) { pc = 0x407461; continue; }
            pc = 0x4072ca; continue;
            }
            0x4072ca => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4072d1);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x4072db);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4072fd; continue; }
            pc = 0x4072e6; continue;
            }
            0x4072e6 => {
            c.push(m,0x692c94u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x4072f4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4072fd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4072fd; continue;
            }
            0x4072fd => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x190u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x40723d; continue; }
            pc = 0x40730d; continue;
            }
            0x40730d => {
            pc = 0x4075a2; continue;
            }
            0x407312 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa4),16));
            { let a = c.get(3,32); c.sub(a,0x3b9aca00u32,32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,16,c.get(1,16)); }
            if c.cond(4) { pc = 0x40736c; continue; }
            pc = 0x407326; continue;
            }
            0x407326 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            if c.cond(4) { pc = 0x40734e; continue; }
            pc = 0x407333; continue;
            }
            0x407333 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,0x1u32);
            c.set(7,32,c.r[1].wrapping_add(c.r[6]));
            pc = 0x407383; continue;
            }
            0x40734e => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,0x1u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(7,32,c.r[1].wrapping_add(c.r[6]));
            pc = 0x407383; continue;
            }
            0x40736c => {
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0x3b9aca00u32); }
            c.push(m,0x0u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[6]));
            pc = 0x407383; continue;
            }
            0x407383 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x758),32));
            c.set(1,32,c.r[7].wrapping_add(0x758));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x407392); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4ae0),32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4073c2; continue; }
            pc = 0x4073a6; continue;
            }
            0x4073a6 => {
            c.set(1,32,c.r[7].wrapping_add(0x2698));
            c.push(m, 0x4073b1);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4073b9);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            pc = 0x4073f3; continue;
            }
            0x4073c2 => {
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(5) { pc = 0x4073e5; continue; }
            pc = 0x4073c7; continue;
            }
            0x4073c7 => {
            c.set(1,32,c.r[7].wrapping_add(0x2698));
            c.push(m, 0x4073d2);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4073da);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            pc = 0x4073f3; continue;
            }
            0x4073e5 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4073f3; continue; }
            pc = 0x4073e9; continue;
            }
            0x4073e9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            pc = 0x4073f3; continue;
            }
            0x4073f3 => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,16,c.get(5,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x6b8));
            pc = 0x407400; continue;
            }
            0x407400 => {
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x407411; continue; }
            pc = 0x407404; continue;
            }
            0x407404 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x407400; continue; }
            pc = 0x40740d; continue;
            }
            0x40740d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            pc = 0x407411; continue;
            }
            0x407411 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.r[4].wrapping_add(0x52); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2]),8));
            c.set(1,32,c.r[4].wrapping_add(0x48));
            { let a = c.r[4].wrapping_add(0x4e); wm(m,a,16,c.get(0,16)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x69u32); }
            c.push(m, 0x407449);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x692c88u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x407459);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4075a2; continue;
            }
            0x407461 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa4),16));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4ae0),32));
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x40749a; continue; }
            pc = 0x407479; continue;
            }
            0x407479 => {
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x4074d0; continue; }
            pc = 0x407481; continue;
            }
            0x407481 => {
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x692c8cu32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x407495);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4074d0; continue;
            }
            0x40749a => {
            { let a = c.get(1,32); c.sub(a,0x3u32,32); }
            if c.cond(5) { pc = 0x4074c8; continue; }
            pc = 0x40749f; continue;
            }
            0x40749f => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x4074d0; continue; }
            pc = 0x4074aa; continue;
            }
            0x4074aa => {
            c.push(m,0x1u32);
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x692c90u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4074c3);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4074d0; continue;
            }
            0x4074c8 => {
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0x0u32); }
            pc = 0x4074d0; continue;
            }
            0x4074d0 => {
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[6]));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x8e0),32));
            { let a = c.get(1,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x40751c; continue; }
            pc = 0x4074e9; continue;
            }
            0x4074e9 => {
            c.set(1,32,c.r[0].wrapping_add(0x758));
            c.push(m, 0x4074f4);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4074fc);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            if c.cond(4) { pc = 0x407514; continue; }
            pc = 0x40750f; continue;
            }
            0x40750f => {
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            pc = 0x407521; continue;
            }
            0x407514 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            pc = 0x407521; continue;
            }
            0x40751c => {
            c.set(0,32,0x3b9aca00u32);
            pc = 0x407521; continue;
            }
            0x407521 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4ae0),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x407536; continue; }
            pc = 0x407530; continue;
            }
            0x407530 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            pc = 0x407536; continue;
            }
            0x407536 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x40754c; continue; }
            pc = 0x40753e; continue;
            }
            0x40753e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x6b8),32));
            { let a = c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4b30); wm(m,a,32,c.get(0,32)); }
            pc = 0x40754c; continue;
            }
            0x40754c => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,16,c.get(5,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x6b8));
            pc = 0x407559; continue;
            }
            0x407559 => {
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x40756a; continue; }
            pc = 0x40755d; continue;
            }
            0x40755d => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x407559; continue; }
            pc = 0x407566; continue;
            }
            0x407566 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            pc = 0x40756a; continue;
            }
            0x40756a => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.r[4].wrapping_add(0x52); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1]),8));
            c.set(0,32,c.r[4].wrapping_add(0x48));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x52); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x69u32); }
            c.push(m, 0x4075a2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4075a2; continue;
            }
            0x4075a2 => {
            c.set(1,32,c.r[6].wrapping_add(0x200));
            c.push(m, 0x4075ad);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4075e8; continue; }
            pc = 0x4075b1; continue;
            }
            0x4075b1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa4),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6dd72cu32,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),16));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x547c),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4075e8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x405a20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x4075e8; continue;
            }
            0x4075e8 => {
            c.set(1,32,c.r[6].wrapping_add(0x2e8));
            c.push(m, 0x4075f3);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x40763f; continue; }
            pc = 0x4075f7; continue;
            }
            0x4075f7 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa4),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,0x6dd72cu32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),16));
            pc = 0x407612; continue;
            }
            0x407612 => {
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),16));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x407612; continue; }
            pc = 0x407629; continue;
            }
            0x407629 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x547c),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x40763f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x405a20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x40763f; continue;
            }
            0x40763f => {
            c.set(1,32,c.r[6].wrapping_add(0x5484));
            c.push(m, 0x40764a);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4076b0; continue; }
            pc = 0x40764e; continue;
            }
            0x40764e => {
            c.set(7,32,c.r[6].wrapping_add(0x758));
            c.set(3,32,c.r[6].wrapping_add(0x4ae0));
            c.set(5,32,0x14u32);
            pc = 0x40765f; continue;
            }
            0x40765f => {
            { let a = rm(m,c.r[3],32); c.sub(a,0x4u32,32); }
            if c.cond(4) { pc = 0x4076a4; continue; }
            pc = 0x407664; continue;
            }
            0x407664 => {
            { let a = rm(m,c.r[7].wrapping_add(0x188),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4076a4; continue; }
            pc = 0x40766d; continue;
            }
            0x40766d => {
            c.push(m,0xffffffffu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x407676);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x69a368u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m, 0x407685);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x3u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x407696);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            c.push(m, 0x4076a4);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4076a4; continue;
            }
            0x4076a4 => {
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x190u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x40765f; continue; }
            pc = 0x4076b0; continue;
            }
            0x4076b0 => {
            c.set(1,32,c.r[6].wrapping_add(0x556c));
            c.push(m, 0x4076bb);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4076fd; continue; }
            pc = 0x4076bf; continue;
            }
            0x4076bf => {
            c.set(7,32,c.r[6].wrapping_add(0x758));
            c.set(3,32,c.r[6].wrapping_add(0x4ae0));
            c.set(5,32,0x14u32);
            pc = 0x4076d0; continue;
            }
            0x4076d0 => {
            { let a = rm(m,c.r[3],32); c.sub(a,0x4u32,32); }
            if c.cond(4) { pc = 0x4076f1; continue; }
            pc = 0x4076d5; continue;
            }
            0x4076d5 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x188),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4076f1; continue; }
            pc = 0x4076df; continue;
            }
            0x4076df => {
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4076e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4076f1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4076f1; continue;
            }
            0x4076f1 => {
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x190u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x4076d0; continue; }
            pc = 0x4076fd; continue;
            }
            0x4076fd => {
            c.set(1,32,c.r[6].wrapping_add(0x3d0));
            c.push(m, 0x407708);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x407c00; continue; }
            pc = 0x407710; continue;
            }
            0x407710 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x4u32); }
            c.push(m, 0x407728);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x708));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x14u32); }
            pc = 0x40773e; continue;
            }
            0x40773e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x1u32); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x4568),32));
            c.set(0,32,c.get(5,32));
            { let a = c.r[1].wrapping_add(0xffffffb0); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf0),32));
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            c.set(3,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[3].wrapping_add(0x4cc0));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x40777f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[6]));
            c.set(0,32,c.r[3].wrapping_add(0x4f90));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x758),32));
            c.set(1,32,c.r[7].wrapping_add(0x758));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4077a6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x2698),32));
            c.set(1,32,c.r[7].wrapping_add(0x2698));
            c.set(0,32,c.r[3].wrapping_add(0x4ea0));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4077c0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x110),32));
            c.set(1,32,c.r[5].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[2]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[2]));
            c.set(2,32,c.r[3].wrapping_add(0x5350));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x4077e6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x114),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            c.set(0,32,c.r[3].wrapping_add(0x5260));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4077fc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x108),32));
            c.set(0,32,c.r[3].wrapping_add(0x5170));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x407810); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10c),32));
            c.set(0,32,c.r[3].wrapping_add(0x5080));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x407824); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x100),32));
            { let a = c.get(3,32); let r = c.add(a,0x4db0u32,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x407838); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x407848); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf4),32));
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x407858); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xfc),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x407867); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x104),32));
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x407877); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4ae0),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xf4),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x407893);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4c20),32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4ae0); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xf4),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4078b2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4ae0),32); c.sub(a,0x4u32,32); }
            if c.cond(5) { pc = 0x4078c1; continue; }
            pc = 0x4078bc; continue;
            }
            0x4078bc => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            pc = 0x4078c1; continue;
            }
            0x4078c1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x108),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4078d8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10c),32));
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4078eb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4078f8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x4ae0),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x40790f; continue; }
            pc = 0x407903; continue;
            }
            0x407903 => {
            { let a = c.get(5,32); c.sub(a,0x2u32,32); }
            if c.cond(4) { pc = 0x40790f; continue; }
            pc = 0x407908; continue;
            }
            0x407908 => {
            c.set(0,32,0x1u32);
            pc = 0x407911; continue;
            }
            0x40790f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x407911; continue;
            }
            0x407911 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = c.get(1,8); let r = c.and(a,c.get(0,8),8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x110),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x407930); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x114),32));
            c.push(m,c.get(3,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(1,32,c.get(7,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x407943); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x407950); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x40773e; continue; }
            pc = 0x40796a; continue;
            }
            0x40796a => {
            c.set(1,32,c.r[6].wrapping_add(0x758));
            c.set(2,32,c.r[6].wrapping_add(0x4bd0));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            pc = 0x407986; continue;
            }
            0x407986 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[5],32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xa0),32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x40799d);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3b9aca00u32,32); }
            if c.cond(4) { pc = 0x4079d1; continue; }
            pc = 0x4079a7; continue;
            }
            0x4079a7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,32,rm(m,c.r[5],32));
            if c.cond(4) { pc = 0x4079c3; continue; }
            pc = 0x4079b8; continue;
            }
            0x4079b8 => {
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            pc = 0x4079d6; continue;
            }
            0x4079c3 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            pc = 0x4079d6; continue;
            }
            0x4079d1 => {
            c.set(0,32,rm(m,c.r[5],32));
            c.set(3,32,c.get(0,32));
            pc = 0x4079d6; continue;
            }
            0x4079d6 => {
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4079e0);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3b9aca00u32,32); }
            c.set(1,32,c.get(7,32));
            if c.cond(4) { pc = 0x407a0d; continue; }
            pc = 0x4079f0; continue;
            }
            0x4079f0 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4079f7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            c.push(m, 0x407a00);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x407a0b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x407a4b; continue;
            }
            0x407a0d => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x407a14); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xffffffffu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x407a1d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x69a368u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m, 0x407a2c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x5u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x407a3d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x4u32); }
            c.push(m, 0x407a4b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x407a4b; continue;
            }
            0x407a4b => {
            c.set(5,32,rm(m,c.r[5].wrapping_add(0xffffffb0),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x407a7d; continue; }
            pc = 0x407a52; continue;
            }
            0x407a52 => {
            c.push(m,c.get(5,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x692c98u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x407a62);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4b30); wm(m,a,32,c.get(5,32)); }
            c.set(7,32,c.r[1].wrapping_add(0x1f40));
            c.push(m,c.get(5,32));
            pc = 0x407ad3; continue;
            }
            0x407a7d => {
            if c.cond(13) { pc = 0x407ab9; continue; }
            pc = 0x407a7f; continue;
            }
            0x407a7f => {
            c.set(3,32,c.get(5,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,0x692c9cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x407a93);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(7,32,c.r[1].wrapping_add(0x1f40));
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4b30); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x407ab3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x407ada; continue;
            }
            0x407ab9 => {
            c.push(m,0x692ca0u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x407ac7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x0u32);
            c.set(7,32,c.r[2].wrapping_add(0x1f40));
            pc = 0x407ad3; continue;
            }
            0x407ad3 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x407ada);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x407ada; continue;
            }
            0x407ada => {
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x407ae6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa4),16));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0x69u32); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x6b8));
            c.set(3,32,0x4u32);
            pc = 0x407b1d; continue;
            }
            0x407b1d => {
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x407b2b; continue; }
            pc = 0x407b21; continue;
            }
            0x407b21 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x407b1d; continue; }
            pc = 0x407b29; continue;
            }
            0x407b29 => {
            pc = 0x407b2f; continue;
            }
            0x407b2b => {
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            pc = 0x407b2f; continue;
            }
            0x407b2f => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x34),16));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.r[4].wrapping_add(0x52); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2]),8));
            c.set(1,32,c.r[4].wrapping_add(0x48));
            { let a = c.r[4].wrapping_add(0x4e); wm(m,a,16,c.get(0,16)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x407b64);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x190u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,0x14u32,32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x407986; continue; }
            pc = 0x407b8a; continue;
            }
            0x407b8a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5adc),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x5654));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x407baf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[6].wrapping_add(0x59f4));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x407bbd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x5adc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x407bf2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            c.push(m, 0x407c00);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x407c00; continue;
            }
            0x407c00 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x407c11);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x58u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn dialog_parchment_open(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x41d9b0;
    loop {
        match pc {
            0x41d9b0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65c444u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xa4u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xa0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x41d9e4; continue; }
            pc = 0x41d9db; continue;
            }
            0x41d9db => {
            c.set(0,32,rm(m,c.r[5],32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x41d9e4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x41d9e4; continue;
            }
            0x41d9e4 => {
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x41d9f1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc4),32));
            c.set(7,32,c.r[5].wrapping_add(0x5b8));
            c.set(1,32,0x8u32);
            c.set(6,32,c.get(3,32));
            c.rep_movsd(m);
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[5].wrapping_add(0x5b4));
            c.push(m,c.get(0,32));
            c.push(m, 0x41da15);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x41da25);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x4),8));
            { let a = c.r[5].wrapping_add(0x94); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0xd),8));
            { let a = c.r[5].wrapping_add(0x4a8); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xe),8));
            { let a = c.r[5].wrapping_add(0x4a9); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xc),8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x41da73; continue; }
            pc = 0x41da47; continue;
            }
            0x41da47 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x41da57);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.r[5].wrapping_add(0x4a8); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.r[5].wrapping_add(0x4a9); wm(m,a,8,c.get(1,8)); }
            pc = 0x41da73; continue;
            }
            0x41da73 => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x4a8),8));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xf),8));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x494),32));
            { let a = c.r[5].wrapping_add(0x4aa); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x41da94; continue; }
            pc = 0x41da90; continue;
            }
            0x41da90 => {
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x41dab8; continue; }
            pc = 0x41da94; continue;
            }
            0x41da94 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x4a4),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x48c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x41dab8; continue; }
            pc = 0x41daa6; continue;
            }
            0x41daa6 => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x94),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x41dab8; continue; }
            pc = 0x41dab4; continue;
            }
            0x41dab4 => {
            { let a = rm(m,c.r[4].wrapping_add(0x24),32); let r = c.add(a,c.get(1,32),32); { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,r); } }
            pc = 0x41dab8; continue;
            }
            0x41dab8 => {
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x14),8));
            c.set(6,32,c.r[5].wrapping_add(0x4b0));
            { let a = c.r[5].wrapping_add(0x59c); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x18),32));
            { let a = c.r[5].wrapping_add(0x5ac); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0x5b0); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x41dae0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x41daee); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(6,16,rm(m,c.r[3].wrapping_add(0x6),16));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[5].wrapping_add(0x5a0); wm(m,a,32,c.get(6,32)); }
            if c.cond(4) { pc = 0x41dc63; continue; }
            pc = 0x41db08; continue;
            }
            0x41db08 => {
            c.set(1,32,c.r[6].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,c.r[2].wrapping_mul(8).wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m, 0x41db21);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x41db5c; continue; }
            pc = 0x41db37; continue;
            }
            0x41db37 => {
            c.push(m,0x4c6a30u32);
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            c.push(m,0x4c6910u32);
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,0xe8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x41db56);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x41db5e; continue;
            }
            0x41db5c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x41db5e; continue;
            }
            0x41db5e => {
            { let a = c.r[5].wrapping_add(0x598); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x5a0),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            if c.cond(14) { pc = 0x41dc67; continue; }
            pc = 0x41db83; continue;
            }
            0x41db83 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x41db85; continue;
            }
            0x41db85 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.set(1,32,0x6dcdd0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x41db96);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x695c9cu32);
            c.set(1,32,0x6dcdd0u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x41dba9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x10u32,32); }
            if c.cond(15) { pc = 0x41dbbb; continue; }
            pc = 0x41dbb4; continue;
            }
            0x41dbb4 => {
            c.set(0,8,rm(m,0x66de58u32,8));
            pc = 0x41dbe2; continue;
            }
            0x41dbbb => {
            { let a = c.get(1,32); c.sub(a,0x20u32,32); }
            if c.cond(15) { pc = 0x41dbc7; continue; }
            pc = 0x41dbc0; continue;
            }
            0x41dbc0 => {
            c.set(0,8,rm(m,0x66de57u32,8));
            pc = 0x41dbe2; continue;
            }
            0x41dbc7 => {
            { let a = c.get(1,32); c.sub(a,0x30u32,32); }
            if c.cond(15) { pc = 0x41dbd3; continue; }
            pc = 0x41dbcc; continue;
            }
            0x41dbcc => {
            c.set(0,8,rm(m,0x66de56u32,8));
            pc = 0x41dbe2; continue;
            }
            0x41dbd3 => {
            c.set(0,8,rm(m,0x66de55u32,8));
            { let a = c.get(1,32); c.sub(a,0x60u32,32); }
            if c.cond(14) { pc = 0x41dbe2; continue; }
            pc = 0x41dbdd; continue;
            }
            0x41dbdd => {
            c.set(0,8,rm(m,0x66de54u32,8));
            pc = 0x41dbe2; continue;
            }
            0x41dbe2 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x598),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_mul(8));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x41dc08);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x598),32));
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m, 0x41dc1c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x598),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[0]));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x130),32); c.push(m, 0x41dc35); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x598),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x41dc42); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x5a0),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x41db85; continue; }
            pc = 0x41dc61; continue;
            }
            0x41dc61 => {
            pc = 0x41dc67; continue;
            }
            0x41dc63 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            pc = 0x41dc67; continue;
            }
            0x41dc67 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x4a8),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x41dc7a; continue; }
            pc = 0x41dc71; continue;
            }
            0x41dc71 => {
            { let a = rm(m,c.r[5].wrapping_add(0x4aa),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x41dc92; continue; }
            pc = 0x41dc7a; continue;
            }
            0x41dc7a => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4a0),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x488),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x41dc92; continue;
            }
            0x41dc92 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x490),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x41dca4; continue; }
            pc = 0x41dca0; continue;
            }
            0x41dca0 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x41dca4; continue;
            }
            0x41dca4 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x41dcb4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41dcc4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,0x6cbb00u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x41dcdf; continue; }
            pc = 0x41dcd6; continue;
            }
            0x41dcd6 => {
            c.set(0,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x41dcdf; continue;
            }
            0x41dcdf => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x94),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x41dcf2; continue; }
            pc = 0x41dce9; continue;
            }
            0x41dce9 => {
            c.set(2,32,c.r[1].wrapping_add(c.r[1]));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            pc = 0x41dcfa; continue;
            }
            0x41dcf2 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            pc = 0x41dcfa; continue;
            }
            0x41dcfa => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x16),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x41df6d; continue; }
            pc = 0x41dd07; continue;
            }
            0x41dd07 => {
            c.fild32(m,c.r[4].wrapping_add(0x38));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fild32(m,c.r[4].wrapping_add(0x34));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('/',v); }
            { let v = c.ld(m,c.r[3].wrapping_add(0x10),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x4100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x41dd66; continue; }
            pc = 0x41dd2b; continue;
            }
            0x41dd2b => {
            c.set(0,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,c.r[3].wrapping_add(0x10),4); c.fop('*',v); }
            c.fild32(m,0x6cbb00u32);
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('/',v); }
            c.push(m, 0x41dd5b);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.imul(a,rm(m,0x6cbb00u32,32),32); c.set(6,32,r); }
            pc = 0x41dda1; continue;
            }
            0x41dd66 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.imul(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x14));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,c.r[3].wrapping_add(0x10),4); c.fop('/',v); }
            c.fild32(m,0x6cbb00u32);
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('/',v); }
            c.push(m, 0x41dd98);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.imul(a,rm(m,0x6cbb00u32,32),32); c.set(7,32,r); }
            pc = 0x41dda1; continue;
            }
            0x41dda1 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x498),32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            { let a = c.r[5].wrapping_add(0x49c); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[5].wrapping_add(0x458));
            c.push(m,c.get(0,32));
            c.push(m, 0x41ddc2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x462520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41ddd5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41dde8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41ddfb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x464),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffff));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x58),32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x70),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x41def4; continue; }
            pc = 0x41de27; continue;
            }
            0x41de27 => {
            c.set(2,32,rm(m,0x6cbb00u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x41e050; continue; }
            pc = 0x41de3d; continue;
            }
            0x41de3d => {
            c.set(7,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,c.r[3].wrapping_add(0x10),4); c.fop('*',v); }
            c.fild32(m,0x6cbb00u32);
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('/',v); }
            c.push(m, 0x41de65);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x498),32));
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.imul(a,rm(m,0x6cbb00u32,32),32); c.set(6,32,r); }
            c.set(0,32,c.get(7,32));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[5].wrapping_add(0x458));
            { let a = c.r[5].wrapping_add(0x49c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x41de8f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x462520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41dea2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41deb5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41dec8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x464),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffff));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x58),32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x70),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x41de27; continue; }
            pc = 0x41def4; continue;
            }
            0x41def4 => {
            c.set(2,32,rm(m,0x6cbb00u32,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,c.r[3].wrapping_add(0x10),4); c.fop('*',v); }
            c.fild32(m,0x6cbb00u32);
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),4); c.fop('/',v); }
            c.push(m, 0x41df22);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.imul(a,rm(m,0x6cbb00u32,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x41e050; continue; }
            pc = 0x41df39; continue;
            }
            0x41df39 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x38),32),32); }
            if c.cond(13) { pc = 0x41e050; continue; }
            pc = 0x41df43; continue;
            }
            0x41df43 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x498),32));
            c.set(6,32,c.get(0,32));
            c.set(0,32,c.get(1,32));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(7,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0x49c); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[5].wrapping_add(0x458));
            c.push(m,c.get(0,32));
            c.push(m, 0x41df68);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x462520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x41e050; continue;
            }
            0x41df6d => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x1c));
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fild32(m,0x6cbb00u32);
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m, 0x41df9b);
            { let args = []; c.r[0] = host.call(m, 0x63a5ce, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fild32(m,0x6cbb00u32);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),8); c.fop('*',v); }
            c.push(m, 0x41dfb1);
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x498),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            { let a = c.r[5].wrapping_add(0x49c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[3],32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[5].wrapping_add(0x458));
            c.push(m, 0x41dfd2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x462520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,0x6dcd28u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x41dfe4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41dff7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x44));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41e00a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x464),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffff));
            c.set(1,32,rm(m,0x6cbb00u32,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x58),32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x48),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xffffffff));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(6,32); let r = c.imul(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x41e050; continue; }
            pc = 0x41e048; continue;
            }
            0x41e048 => {
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.imul(a,c.get(1,32),32); c.set(6,32,r); }
            pc = 0x41e050; continue;
            }
            0x41e050 => {
            c.set(0,32,rm(m,0x6cbaf4u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x41e083; continue; }
            pc = 0x41e05c; continue;
            }
            0x41e05c => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x41e083; continue; }
            pc = 0x41e063; continue;
            }
            0x41e063 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(2,32)); }
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            pc = 0x41e087; continue;
            }
            0x41e083 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            pc = 0x41e087; continue;
            }
            0x41e087 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,8); c.and(a,0x3u32,8); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x41e0cb; continue; }
            pc = 0x41e0b6; continue;
            }
            0x41e0b6 => {
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.and(a,0x80000003u32,32); c.set(2,32,r); }
            if c.cond(9) { pc = 0x41e0c5; continue; }
            pc = 0x41e0c0; continue;
            }
            0x41e0c0 => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.or(a,0xfffffffcu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x41e0c5; continue;
            }
            0x41e0c5 => {
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            pc = 0x41e0cb; continue;
            }
            0x41e0cb => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x94),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x41e0db; continue; }
            pc = 0x41e0d5; continue;
            }
            0x41e0d5 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            pc = 0x41e0db; continue;
            }
            0x41e0db => {
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x41e0e8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x94),8));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(6,32)); }
            if c.cond(4) { pc = 0x41e101; continue; }
            pc = 0x41e0fa; continue;
            }
            0x41e0fa => {
            c.set(2,32,c.r[7].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            pc = 0x41e101; continue;
            }
            0x41e101 => {
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x54),32); c.push(m, 0x41e10e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x94),8));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x41e11e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x41c5a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x41e125);
            { let args = []; c.r[0] = host.call(m, 0x462390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x94),8));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x4a0),32));
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,c.r[5].wrapping_add(0xac));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x41e14f);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x55555556u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x94),8));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4a4),32));
            { let a = c.get(1,32); let r = c.and(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6dcd28u32,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x41e1a7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41e1ba); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x80),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x70),32); c.idiv32(b); }
            c.set(2,32,rm(m,0x6dcd28u32,32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x58),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,0x6dcd28u32);
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41e1f5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41e208); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x464),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffff));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x58),32); c.idiv32(b); }
            c.set(2,32,c.r[4].wrapping_add(0x7c));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0xa8));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x41e232); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x8c));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41e248); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcd28u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x98));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x41e25e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x9c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x90),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0xac));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x41e287);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x43c),8));
            c.set(1,32,c.get(3,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x41e29f; continue; }
            pc = 0x41e293; continue;
            }
            0x41e293 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x41e29d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x41e2a9; continue;
            }
            0x41e29f => {
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x41e2a9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x41e2a9; continue;
            }
            0x41e2a9 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[5].wrapping_add(0xa4));
            c.push(m, 0x41e2b9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x4b0),32));
            c.set(1,32,c.r[5].wrapping_add(0x4b0));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x41e2c8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(3,32,rm(m,0x6cbb00u32,32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x94),8));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x5a0),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x41e41f; continue; }
            pc = 0x41e30d; continue;
            }
            0x41e30d => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x4a8),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x41e326; continue; }
            pc = 0x41e317; continue;
            }
            0x41e317 => {
            { let a = rm(m,c.r[5].wrapping_add(0x4aa),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x41e326; continue; }
            pc = 0x41e320; continue;
            }
            0x41e320 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x41e338; continue;
            }
            0x41e326 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x488),32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x4a0),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x41e338; continue;
            }
            0x41e338 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(3,32,c.get(7,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x78),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x41e386; continue; }
            pc = 0x41e34e; continue;
            }
            0x41e34e => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            pc = 0x41e352; continue;
            }
            0x41e352 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x598),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x41e363); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x5a0),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0xe8u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(12) { pc = 0x41e352; continue; }
            pc = 0x41e386; continue;
            }
            0x41e386 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x5a0),32));
            c.set(0,32,c.r[1].wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x94),8));
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            if c.cond(14) { pc = 0x41e41f; continue; }
            pc = 0x41e3ca; continue;
            }
            0x41e3ca => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x598),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x60));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x41e3ef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x598),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x41e3fc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x5a0),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xe8u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x41e3ca; continue; }
            pc = 0x41e41f; continue;
            }
            0x41e41f => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4b0),32));
            c.set(3,32,c.r[5].wrapping_add(0x4b0));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x41e430); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x78),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x60));
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x41e45c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x4a8),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x41e46f; continue; }
            pc = 0x41e466; continue;
            }
            0x41e466 => {
            { let a = rm(m,c.r[5].wrapping_add(0x4aa),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x41e4d8; continue; }
            pc = 0x41e46f; continue;
            }
            0x41e46f => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x94),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x4a0),32));
            c.set(2,8,c.get(1,8));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[5].wrapping_add(0x478); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x48c),32));
            c.set(0,32,c.get(7,32));
            c.set(3,32,rm(m,0x6cbb00u32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,8,c.get(1,8));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[5].wrapping_add(0x47c); wm(m,a,32,c.get(6,32)); }
            if c.cond(5) { pc = 0x41e4d8; continue; }
            pc = 0x41e4cc; continue;
            }
            0x41e4cc => {
            { let a = c.get(6,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0x4a4),32),32); c.set(6,32,r); }
            { let a = c.r[5].wrapping_add(0x47c); wm(m,a,32,c.get(6,32)); }
            pc = 0x41e4d8; continue;
            }
            0x41e4d8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xb4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0xb0u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

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
            { let target = c.get(6,32); c.push(m, 0x41f559); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
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
            { let target = c.get(3,32); c.push(m, 0x41f8bb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
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
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ip_address_prompt_on_open(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x422900;
    loop {
        match pc {
            0x422900 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65c83au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            c.push(m, 0x422940);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x422947);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b5380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x31u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x1u32); }
            c.push(m, 0x42296c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x164),32));
            c.set(6,32,c.r[3].wrapping_add(0x164));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42297d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[7].wrapping_add(0x48),32); c.push(m, 0x422984); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x258u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42299a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x320u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[7].wrapping_add(0x78),32); c.push(m, 0x4229ae); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4229c1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x23c),32));
            c.set(7,32,c.r[3].wrapping_add(0x23c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x4229d2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4229dd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x66e10cu32,32));
            { let a = c.r[3].wrapping_add(0x2fc); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,0x66e110u32,32));
            { let a = c.r[3].wrapping_add(0x300); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(3,32); let r = c.add(a,0x330u32,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x422a03);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464f30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x69b28cu32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x422a10);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x69b2a8u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x422a1e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x422a29); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(5,32,0xffffffeeu32);
            c.set(1,32,c.get(6,32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x8c),32); c.push(m, 0x422a3a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x422a43); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x422a4d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x422a5f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,0x190u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x422a77); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(3,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x422a80); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(5,32,0xffffffeeu32);
            c.set(1,32,c.get(6,32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x8c),32); c.push(m, 0x422a91); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x422a9a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x422aa4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x422ab6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x190u32,32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[5].wrapping_add(0x330));
            { let target = rm(m,c.r[3].wrapping_add(0x78),32); c.push(m, 0x422ad1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x21u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(2,32));
            c.push(m, 0x422ae4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x424),32));
            c.set(1,32,c.r[5].wrapping_add(0x424));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x422af3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x422b06);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x500),32));
            c.set(3,32,c.r[5].wrapping_add(0x500));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x422b17); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x66e10cu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x422b25);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2d80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[5].wrapping_add(0x680));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x422b33); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[5].wrapping_add(0x424));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x422b41);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b1c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x422b4a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2e20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[5].wrapping_add(0x66c); wm(m,a,32,0xfu32); }
            c.set(5,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8c),32); c.push(m, 0x422b60); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,0x46u32,32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x422b6b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,0x190u32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x422b7d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x422b89);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x422b91);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x330));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x422b9f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x422ba7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x422bb5);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x422bc6);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_screen_create_side_menus_and_dialogs(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x425eee;
    loop {
        match pc {
            0x425eee => {
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xeu32); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x16c),32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x66de94u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x425f1b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x599e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x425f20);
            { let args = []; c.r[0] = host.call(m, 0x441240, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x425f27);
            { let args = []; c.r[0] = host.call(m, 0x429a40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,c.get(2,32)); }
            c.set(3,8,0x14u32);
            c.push(m,0x6967ecu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffa0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x425f42);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffa0));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x425f53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cba64u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x425f5e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cba64u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x425f6e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696804u32);
            c.push(m, 0x425f80);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x425f8b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb68),32));
            c.set(7,32,c.r[6].wrapping_add(0xb68));
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x425fa3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x425faa); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0xffffffd4),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,r); } }
            c.push(m, 0x425fb2);
            { let args = []; c.r[0] = host.call(m, 0x4416f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            c.set(1,32,c.r[5].wrapping_add(0xffffff50));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc40),32));
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,c.r[6].wrapping_add(0xc40));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x425fd5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x425fdc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff54),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb68),32));
            { let a = c.r[5].wrapping_add(0xffffff54); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0xb68));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x426000); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x426010); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x115cu32);
            c.push(m, 0x42601a);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x15u32); }
            if c.cond(4) { pc = 0x426031; continue; }
            pc = 0x426028; continue;
            }
            0x426028 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42602f);
            { let args = []; c.r[0] = host.call(m, 0x4786f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426033; continue;
            }
            0x426031 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426033; continue;
            }
            0x426033 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69681cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cc95cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42604d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc95cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426058); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6cc95cu32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x426065);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc95cu32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x426074); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc95cu32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x426083); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x700u32);
            c.push(m, 0x42608d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x16u32); }
            if c.cond(4) { pc = 0x4260a4; continue; }
            pc = 0x42609b; continue;
            }
            0x42609b => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4260a2);
            { let args = []; c.r[0] = host.call(m, 0x5acfa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4260a6; continue;
            }
            0x4260a4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4260a6; continue;
            }
            0x4260a6 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696834u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5524u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4260c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5524u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4260cb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e5524u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4260d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5524u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4260e7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5524u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x4260f6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x4c0u32);
            c.push(m, 0x426100);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x17u32); }
            if c.cond(4) { pc = 0x426117; continue; }
            pc = 0x42610e; continue;
            }
            0x42610e => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426115);
            { let args = []; c.r[0] = host.call(m, 0x5ae390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426119; continue;
            }
            0x426117 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426119; continue;
            }
            0x426119 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69684cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5528u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426133);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5528u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x42613e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e5528u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x42614b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5528u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x42615a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5528u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x426169); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x810u32);
            c.push(m, 0x426173);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x18u32); }
            if c.cond(4) { pc = 0x42618a; continue; }
            pc = 0x426181; continue;
            }
            0x426181 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426188);
            { let args = []; c.r[0] = host.call(m, 0x5a4110, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42618c; continue;
            }
            0x42618a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42618c; continue;
            }
            0x42618c => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696864u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e54e8u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4261a6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54e8u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4261b1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e54e8u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4261be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54e8u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4261cd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54e8u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x4261dc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x650u32);
            c.push(m, 0x4261e6);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x19u32); }
            if c.cond(4) { pc = 0x4261fd; continue; }
            pc = 0x4261f4; continue;
            }
            0x4261f4 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4261fb);
            { let args = []; c.r[0] = host.call(m, 0x5a0ab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4261ff; continue;
            }
            0x4261fd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4261ff; continue;
            }
            0x4261ff => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69687cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e54ccu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426219);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54ccu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426224); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e54ccu32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x426231);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54ccu32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x426240); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54ccu32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x42624f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3d0u32);
            c.push(m, 0x426259);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1au32); }
            if c.cond(4) { pc = 0x426270; continue; }
            pc = 0x426267; continue;
            }
            0x426267 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42626e);
            { let args = []; c.r[0] = host.call(m, 0x5ac410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426272; continue;
            }
            0x426270 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426272; continue;
            }
            0x426272 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696894u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e551cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42628c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e551cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426297); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e551cu32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4262a4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e551cu32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4262b3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e551cu32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x4262c2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xb20u32);
            c.push(m, 0x4262cc);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1bu32); }
            if c.cond(4) { pc = 0x4262e3; continue; }
            pc = 0x4262da; continue;
            }
            0x4262da => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4262e1);
            { let args = []; c.r[0] = host.call(m, 0x5a5d60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4262e5; continue;
            }
            0x4262e3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4262e5; continue;
            }
            0x4262e5 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6968acu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e54f8u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4262ff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54f8u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x42630a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e54f8u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x426317);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54f8u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x426326); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54f8u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x426335); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x7f8u32);
            c.push(m, 0x42633f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1cu32); }
            if c.cond(4) { pc = 0x426356; continue; }
            pc = 0x42634d; continue;
            }
            0x42634d => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426354);
            { let args = []; c.r[0] = host.call(m, 0x5a31f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426358; continue;
            }
            0x426356 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426358; continue;
            }
            0x426358 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6968c4u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e54e0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426372);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54e0u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x42637d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e54e0u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x42638a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54e0u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x426399); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54e0u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x4263a8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6e8u32);
            c.push(m, 0x4263b2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1du32); }
            if c.cond(4) { pc = 0x4263c9; continue; }
            pc = 0x4263c0; continue;
            }
            0x4263c0 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4263c7);
            { let args = []; c.r[0] = host.call(m, 0x5a2600, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4263cb; continue;
            }
            0x4263c9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4263cb; continue;
            }
            0x4263cb => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6968dcu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e54d8u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4263e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54d8u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4263f0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e54d8u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4263fd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54d8u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x42640c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54d8u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x42641b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x8f8u32);
            c.push(m, 0x426425);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1eu32); }
            if c.cond(4) { pc = 0x42643c; continue; }
            pc = 0x426433; continue;
            }
            0x426433 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42643a);
            { let args = []; c.r[0] = host.call(m, 0x5a8af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42643e; continue;
            }
            0x42643c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42643e; continue;
            }
            0x42643e => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6968f4u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5500u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426458);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5500u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426463); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e5500u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x426470);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5500u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x42647f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5500u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x42648e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6a8u32);
            c.push(m, 0x426498);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1fu32); }
            if c.cond(4) { pc = 0x4264af; continue; }
            pc = 0x4264a6; continue;
            }
            0x4264a6 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4264ad);
            { let args = []; c.r[0] = host.call(m, 0x59fc40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4264b1; continue;
            }
            0x4264af => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4264b1; continue;
            }
            0x4264b1 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69690cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e54c8u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4264cb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54c8u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4264d6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e54c8u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4264e3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54c8u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4264f2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54c8u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x426501); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2c8u32);
            c.push(m, 0x42650b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x20u32); }
            if c.cond(4) { pc = 0x426522; continue; }
            pc = 0x426519; continue;
            }
            0x426519 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426520);
            { let args = []; c.r[0] = host.call(m, 0x5aa6b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426524; continue;
            }
            0x426522 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426524; continue;
            }
            0x426524 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696924u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e550cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42653e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e550cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426549); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e550cu32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x426556);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e550cu32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x426565); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e550cu32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x426574); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x4e0u32);
            c.push(m, 0x42657e);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x21u32); }
            if c.cond(4) { pc = 0x426595; continue; }
            pc = 0x42658c; continue;
            }
            0x42658c => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426593);
            { let args = []; c.r[0] = host.call(m, 0x5a4f50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426597; continue;
            }
            0x426595 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426597; continue;
            }
            0x426597 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69693cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e54f0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4265b1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54f0u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4265bc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e54f0u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4265c9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54f0u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4265d8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54f0u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x4265e7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x920u32);
            c.push(m, 0x4265f1);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x22u32); }
            if c.cond(4) { pc = 0x426608; continue; }
            pc = 0x4265ff; continue;
            }
            0x4265ff => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426606);
            { let args = []; c.r[0] = host.call(m, 0x5ab1e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42660a; continue;
            }
            0x426608 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42660a; continue;
            }
            0x42660a => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696954u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5514u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426624);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5514u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x42662f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e5514u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x42663c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5514u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x42664b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5514u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x42665a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1e0u32);
            c.push(m, 0x426664);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x23u32); }
            if c.cond(4) { pc = 0x42667b; continue; }
            pc = 0x426672; continue;
            }
            0x426672 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426679);
            { let args = []; c.r[0] = host.call(m, 0x5a9a80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42667d; continue;
            }
            0x42667b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42667d; continue;
            }
            0x42667d => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69696cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5508u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426697);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5508u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4266a2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e5508u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4266af);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5508u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4266be); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5508u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x4266cd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x83c8u32);
            c.push(m, 0x4266d7);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x24u32); }
            if c.cond(4) { pc = 0x4266ee; continue; }
            pc = 0x4266e5; continue;
            }
            0x4266e5 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4266ec);
            { let args = []; c.r[0] = host.call(m, 0x486d20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4266f0; continue;
            }
            0x4266ee => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4266f0; continue;
            }
            0x4266f0 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696984u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6ce6d0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42670a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426715); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6ce6d0u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x426722);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x426731); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x426740); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m, 0x42674b);
            { let args = []; c.r[0] = host.call(m, 0x48ac30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffd0),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffd8),32));
            { let a = 0x6cc974u32; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffb8),32));
            { let a = 0x6cc978u32; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffb4),32));
            { let a = 0x6cc97cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6cc960u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6cc960u32);
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x42677c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69699cu32);
            c.push(m, 0x42678e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6cc960u32);
            c.push(m, 0x426798);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x47c9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6cc960u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4267a4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffd0),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffd8),32));
            { let a = 0x6ce6ecu32; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffb8),32));
            { let a = 0x6ce6f0u32; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffb4),32));
            { let a = 0x6ce6f4u32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6ce6d8u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ce6d8u32);
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x4267d5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6969b4u32);
            c.push(m, 0x4267e7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6ce6d8u32);
            c.push(m, 0x4267f1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x495c10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6ce6d8u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4267fd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1ccu32);
            c.push(m, 0x426807);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x25u32); }
            if c.cond(4) { pc = 0x42681e; continue; }
            pc = 0x426815; continue;
            }
            0x426815 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x42681c);
            { let args = []; c.r[0] = host.call(m, 0x5a7d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426820; continue;
            }
            0x42681e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426820; continue;
            }
            0x426820 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6969ccu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e54fcu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42683a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54fcu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426845); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e54fcu32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x426852);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54fcu32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x426861); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54fcu32,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffac));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x426870); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x42687c);
            { let args = []; c.r[0] = host.call(m, 0x441220, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6969e4u32);
            c.push(m, 0x426891);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cba74u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x42689c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6969fcu32);
            c.push(m, 0x4268ae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d9584u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x4268b9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696a14u32);
            c.push(m, 0x4268cb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cba98u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4268d6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696a2cu32);
            c.push(m, 0x4268e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbaacu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x4268f3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696a44u32);
            c.push(m, 0x426905);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cba90u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426910); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[6].wrapping_add(0xb0));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x426924);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42692f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.push(m, 0x42693a);
            { let args = []; c.r[0] = host.call(m, 0x46d010, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x42694a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696a5cu32);
            c.push(m, 0x42695c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc950u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x426967); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x426971);
            { let args = []; c.r[0] = host.call(m, 0x5465b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1238),32));
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.add(a,0x94u32,32); c.set(1,32,r); }
            c.push(m, 0x426984);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c5850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x100u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0x1258); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x125c); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x1254); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x4269b0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dcce0u32,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x4269c6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xb68));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x4269d6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51c0u32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(0xffffffd0));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x54),32); c.push(m, 0x4269ea); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[6].wrapping_add(0xb0));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4269fe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426a09); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.push(m, 0x426a14);
            { let args = []; c.r[0] = host.call(m, 0x59c9f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x100u32);
            c.push(m, 0x426a1e);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x26u32); }
            if c.cond(4) { pc = 0x426a35; continue; }
            pc = 0x426a2c; continue;
            }
            0x426a2c => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426a33);
            { let args = []; c.r[0] = host.call(m, 0x465010, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426a37; continue;
            }
            0x426a35 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426a37; continue;
            }
            0x426a37 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696a74u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cc7e0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426a51);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x426a5c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x219u32);
            c.push(m,0x1c3u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x426a73); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,0x6cc7e0u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x426a80);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x426a90); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x480u32);
            c.push(m, 0x426a9a);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x27u32); }
            if c.cond(4) { pc = 0x426ab1; continue; }
            pc = 0x426aa8; continue;
            }
            0x426aa8 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426aaf);
            { let args = []; c.r[0] = host.call(m, 0x466d70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426ab3; continue;
            }
            0x426ab1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426ab3; continue;
            }
            0x426ab3 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696a8cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cc7e8u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426acd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x426ad8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            c.push(m,0x1b0u32);
            c.push(m,0x1adu32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x426aef); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,0x6cc7e8u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x426afc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x426b0c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3cu32);
            c.push(m, 0x426b13);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x28u32); }
            if c.cond(4) { pc = 0x426b2a; continue; }
            pc = 0x426b21; continue;
            }
            0x426b21 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426b28);
            { let args = []; c.r[0] = host.call(m, 0x46afb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426b2c; continue;
            }
            0x426b2a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426b2c; continue;
            }
            0x426b2c => {
            c.push(m,0xd50u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cc7f4u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426b3e);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x29u32); }
            if c.cond(4) { pc = 0x426b55; continue; }
            pc = 0x426b4c; continue;
            }
            0x426b4c => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426b53);
            { let args = []; c.r[0] = host.call(m, 0x5f37d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426b57; continue;
            }
            0x426b55 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426b57; continue;
            }
            0x426b57 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696aa4u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e55c0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426b71);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e55c0u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x426b7c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e55c0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x1feu32);
            c.push(m,0x1a9u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x426b93); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e55c0u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x426ba3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1d60u32);
            c.push(m, 0x426bad);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2au32); }
            if c.cond(4) { pc = 0x426bc4; continue; }
            pc = 0x426bbb; continue;
            }
            0x426bbb => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426bc2);
            { let args = []; c.r[0] = host.call(m, 0x5c88e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426bc6; continue;
            }
            0x426bc4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426bc6; continue;
            }
            0x426bc6 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696ac4u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e556cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426be0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e556cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426beb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e556cu32,32));
            c.push(m,0x0u32);
            c.push(m,0x1feu32);
            c.push(m,0x1a9u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x426c02); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e556cu32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x426c12); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2270u32);
            c.push(m, 0x426c1c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2bu32); }
            if c.cond(4) { pc = 0x426c33; continue; }
            pc = 0x426c2a; continue;
            }
            0x426c2a => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426c31);
            { let args = []; c.r[0] = host.call(m, 0x5cb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426c35; continue;
            }
            0x426c33 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426c35; continue;
            }
            0x426c35 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696ae4u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5574u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426c4f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5574u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x426c5a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e5574u32,32));
            c.push(m,0x0u32);
            c.push(m,0x1feu32);
            c.push(m,0x1a9u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x426c71); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e5574u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x426c81); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1f18u32);
            c.push(m, 0x426c8b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2cu32); }
            if c.cond(4) { pc = 0x426ca2; continue; }
            pc = 0x426c99; continue;
            }
            0x426c99 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426ca0);
            { let args = []; c.r[0] = host.call(m, 0x5bb750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426ca4; continue;
            }
            0x426ca2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426ca4; continue;
            }
            0x426ca4 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696b04u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5550u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426cbe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5550u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426cc9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e5550u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x426cd9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x294u32);
            c.push(m, 0x426ce3);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2du32); }
            if c.cond(4) { pc = 0x426cfa; continue; }
            pc = 0x426cf1; continue;
            }
            0x426cf1 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426cf8);
            { let args = []; c.r[0] = host.call(m, 0x5dd9e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426cfc; continue;
            }
            0x426cfa => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426cfc; continue;
            }
            0x426cfc => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696b24u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5584u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426d16);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5584u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426d21); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e5584u32,32));
            c.push(m,0x0u32);
            c.push(m,0x1feu32);
            c.push(m,0x1a9u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x426d38); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e5584u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x426d48); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xeda0u32);
            c.push(m, 0x426d52);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2eu32); }
            if c.cond(4) { pc = 0x426d69; continue; }
            pc = 0x426d60; continue;
            }
            0x426d60 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426d67);
            { let args = []; c.r[0] = host.call(m, 0x5d8300, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426d6b; continue;
            }
            0x426d69 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426d6b; continue;
            }
            0x426d6b => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696b44u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e557cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426d85);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e557cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x426d90); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e557cu32,32));
            c.push(m,0x0u32);
            c.push(m,0x1feu32);
            c.push(m,0x1a9u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x426da7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e557cu32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x426db7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xff0u32);
            c.push(m, 0x426dc1);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2fu32); }
            if c.cond(4) { pc = 0x426dd8; continue; }
            pc = 0x426dcf; continue;
            }
            0x426dcf => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426dd6);
            { let args = []; c.r[0] = host.call(m, 0x5b2800, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426dda; continue;
            }
            0x426dd8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426dda; continue;
            }
            0x426dda => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696b64u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5534u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426df4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5534u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426dff); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e5534u32,32));
            c.push(m,0x0u32);
            c.push(m,0x1feu32);
            c.push(m,0x1a9u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x426e16); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e5534u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x426e26); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1440u32);
            c.push(m, 0x426e30);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x30u32); }
            if c.cond(4) { pc = 0x426e47; continue; }
            pc = 0x426e3e; continue;
            }
            0x426e3e => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426e45);
            { let args = []; c.r[0] = host.call(m, 0x5b8180, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426e49; continue;
            }
            0x426e47 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426e49; continue;
            }
            0x426e49 => {
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696b84u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5544u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426e63);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5544u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x426e6e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x15d0u32);
            c.push(m, 0x426e78);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x31u32); }
            if c.cond(4) { pc = 0x426e8f; continue; }
            pc = 0x426e86; continue;
            }
            0x426e86 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426e8d);
            { let args = []; c.r[0] = host.call(m, 0x5be7b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426e91; continue;
            }
            0x426e8f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426e91; continue;
            }
            0x426e91 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696b9cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5558u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426eab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5558u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426eb6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e5558u32,32));
            c.push(m,0x0u32);
            c.push(m,0x1feu32);
            c.push(m,0x1a9u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x426ecd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x1988u32);
            c.push(m, 0x426ed7);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x32u32); }
            if c.cond(4) { pc = 0x426eee; continue; }
            pc = 0x426ee5; continue;
            }
            0x426ee5 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426eec);
            { let args = []; c.r[0] = host.call(m, 0x5df530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426ef0; continue;
            }
            0x426eee => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426ef0; continue;
            }
            0x426ef0 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696bbcu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e558cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426f0a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e558cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426f15); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e558cu32,32));
            c.push(m,0x0u32);
            c.push(m,0x1feu32);
            c.push(m,0x1a9u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x426f2c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x638u32);
            c.push(m, 0x426f36);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x33u32); }
            if c.cond(4) { pc = 0x426f4d; continue; }
            pc = 0x426f44; continue;
            }
            0x426f44 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426f4b);
            { let args = []; c.r[0] = host.call(m, 0x5b1410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426f4f; continue;
            }
            0x426f4d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426f4f; continue;
            }
            0x426f4f => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696bdcu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5530u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426f69);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e5530u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426f74); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e5530u32,32));
            c.push(m,0x0u32);
            c.push(m,0x1feu32);
            c.push(m,0x1a9u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x426f8b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x1398u32);
            c.push(m, 0x426f95);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x34u32); }
            if c.cond(4) { pc = 0x426fac; continue; }
            pc = 0x426fa3; continue;
            }
            0x426fa3 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x426faa);
            { let args = []; c.r[0] = host.call(m, 0x5eb730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x426fae; continue;
            }
            0x426fac => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x426fae; continue;
            }
            0x426fae => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696bfcu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e55a8u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x426fc8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e55a8u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x426fd3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e55a8u32,32));
            c.push(m,0x0u32);
            c.push(m,0x1feu32);
            c.push(m,0x1a9u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x426fea); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x38cu32);
            c.push(m, 0x426ff4);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x35u32); }
            if c.cond(4) { pc = 0x42700b; continue; }
            pc = 0x427002; continue;
            }
            0x427002 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x427009);
            { let args = []; c.r[0] = host.call(m, 0x5b03d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42700d; continue;
            }
            0x42700b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42700d; continue;
            }
            0x42700d => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696c1cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e552cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x427027);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e552cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x427032); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1acu32);
            c.push(m, 0x42703c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x36u32); }
            if c.cond(4) { pc = 0x427053; continue; }
            pc = 0x42704a; continue;
            }
            0x42704a => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x427051);
            { let args = []; c.r[0] = host.call(m, 0x5b7240, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x427055; continue;
            }
            0x427053 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x427055; continue;
            }
            0x427055 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696c3cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e553cu32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42706f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e553cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42707a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1630u32);
            c.push(m, 0x427084);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x37u32); }
            if c.cond(4) { pc = 0x42709b; continue; }
            pc = 0x427092; continue;
            }
            0x427092 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x427099);
            { let args = []; c.r[0] = host.call(m, 0x5eec00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42709d; continue;
            }
            0x42709b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42709d; continue;
            }
            0x42709d => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696c5cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e55b8u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4270b7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e55b8u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4270c2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e55b8u32,32));
            c.push(m,0x0u32);
            c.push(m,0x1feu32);
            c.push(m,0x1a9u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x4270d9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x2e30u32);
            c.push(m, 0x4270e3);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x38u32); }
            if c.cond(4) { pc = 0x4270fa; continue; }
            pc = 0x4270f1; continue;
            }
            0x4270f1 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4270f8);
            { let args = []; c.r[0] = host.call(m, 0x5e9520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4270fc; continue;
            }
            0x4270fa => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4270fc; continue;
            }
            0x4270fc => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696c7cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e55a0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x427116);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e55a0u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x427121); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e55a0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x1feu32);
            c.push(m,0x1a9u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x4c),32); c.push(m, 0x427138); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x2a4u32);
            c.push(m, 0x427142);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x39u32); }
            if c.cond(4) { pc = 0x427159; continue; }
            pc = 0x427150; continue;
            }
            0x427150 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x427157);
            { let args = []; c.r[0] = host.call(m, 0x5bac20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42715b; continue;
            }
            0x427159 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x42715b; continue;
            }
            0x42715b => {
            c.push(m,0x1b4u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5548u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42716d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3au32); }
            if c.cond(4) { pc = 0x427184; continue; }
            pc = 0x42717b; continue;
            }
            0x42717b => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x427182);
            { let args = []; c.r[0] = host.call(m, 0x5ee000, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x427186; continue;
            }
            0x427184 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x427186; continue;
            }
            0x427186 => {
            c.push(m,0x1acu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e55b4u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x427198);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3bu32); }
            if c.cond(4) { pc = 0x4271af; continue; }
            pc = 0x4271a6; continue;
            }
            0x4271a6 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4271ad);
            { let args = []; c.r[0] = host.call(m, 0x5b6ac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4271b1; continue;
            }
            0x4271af => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4271b1; continue;
            }
            0x4271b1 => {
            c.push(m,0x440u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5538u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4271c3);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3cu32); }
            if c.cond(4) { pc = 0x427239; continue; }
            pc = 0x4271d3; continue;
            }
            0x4271d3 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4271da);
            { let args = []; c.r[0] = host.call(m, 0x41c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[7].wrapping_add(0xac); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[7].wrapping_add(0xb0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3eu32); }
            c.push(m, 0x4271f5);
            { let args = []; c.r[0] = host.call(m, 0x4c6910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x198));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3fu32); }
            c.push(m, 0x427204);
            { let args = []; c.r[0] = host.call(m, 0x4c6910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x280));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x40u32); }
            c.push(m, 0x427213);
            { let args = []; c.r[0] = host.call(m, 0x4c9790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x400));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x41u32); }
            c.push(m, 0x427222);
            { let args = []; c.r[0] = host.call(m, 0x4b03e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x420));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x42u32); }
            c.push(m, 0x427231);
            { let args = []; c.r[0] = host.call(m, 0x4209e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[7]; wm(m,a,32,0x66ca08u32); }
            pc = 0x42723b; continue;
            }
            0x427239 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x42723b; continue;
            }
            0x42723b => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696c9cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6cc93cu32; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x427256);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc93cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x427261); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1b4u32);
            c.push(m, 0x42726b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x43u32); }
            if c.cond(4) { pc = 0x427282; continue; }
            pc = 0x427279; continue;
            }
            0x427279 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x427280);
            { let args = []; c.r[0] = host.call(m, 0x5ed830, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x427284; continue;
            }
            0x427282 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x427284; continue;
            }
            0x427284 => {
            c.push(m,0x1b4u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e55b0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x427296);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x44u32); }
            if c.cond(4) { pc = 0x4272ad; continue; }
            pc = 0x4272a4; continue;
            }
            0x4272a4 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4272ab);
            { let args = []; c.r[0] = host.call(m, 0x5c7c50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4272af; continue;
            }
            0x4272ad => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4272af; continue;
            }
            0x4272af => {
            c.push(m,0x1b4u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5564u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4272c1);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x45u32); }
            if c.cond(4) { pc = 0x4272d8; continue; }
            pc = 0x4272cf; continue;
            }
            0x4272cf => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4272d6);
            { let args = []; c.r[0] = host.call(m, 0x5ded90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4272da; continue;
            }
            0x4272d8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4272da; continue;
            }
            0x4272da => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696cbcu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            { let a = 0x6e5588u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4272f4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x4272f9);
            { let args = []; c.r[0] = host.call(m, 0x41c7b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x42730c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x42731d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6d9594u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x42732e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbb6cu32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x42733e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x42734f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2640),32));
            c.set(1,32,c.r[6].wrapping_add(0x2640));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42735e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x5u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffff98));
            { let a = c.r[6].wrapping_add(0x488); wm(m,a,8,0x0u32); }
            c.push(m, 0x42736f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffff98));
            c.push(m, 0x427377);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4u32);
            c.set(1,32,c.r[6].wrapping_add(0x7d0));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x7cc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42738d);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffcc); wm(m,a,32,c.get(3,32)); }
            c.set(7,32,c.r[6].wrapping_add(0x168));
            pc = 0x42739b; continue;
            }
            0x42739b => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4273ac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x4273b3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[7].wrapping_add(0x98); wm(m,a,32,0x0u32); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x4273c9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x4273de); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x4273ee); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4273fa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xffffffcc),32));
            { let a = c.get(7,32); let r = c.add(a,0xa0u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x5u32,32); }
            { let a = c.r[5].wrapping_add(0xffffffcc); wm(m,a,32,c.get(3,32)); }
            if c.cond(12) { pc = 0x42739b; continue; }
            pc = 0x42740c; continue;
            }
            0x42740c => {
            c.push(m,0x4u32);
            c.set(0,32,c.r[6].wrapping_add(0x7e8));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x7b4); wm(m,a,8,0x0u32); }
            c.push(m, 0x427423);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[5].wrapping_add(0xffffff98));
            c.push(m,0x2u32);
            c.push(m, 0x427430);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffff98));
            c.push(m, 0x427438);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0x7fc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffcc); wm(m,a,32,c.get(3,32)); }
            c.set(7,32,c.r[6].wrapping_add(0x494));
            pc = 0x427449; continue;
            }
            0x427449 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x42745a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x427461); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[7].wrapping_add(0x98); wm(m,a,32,0x0u32); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x427477); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x42748c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x42749c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4274a8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xffffffcc),32));
            { let a = c.get(7,32); let r = c.add(a,0xa0u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x5u32,32); }
            { let a = c.r[5].wrapping_add(0xffffffcc); wm(m,a,32,c.get(3,32)); }
            if c.cond(12) { pc = 0x427449; continue; }
            pc = 0x4274ba; continue;
            }
            0x4274ba => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x25),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,8,u32::from(c.cond(5)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4274cf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b8520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4274d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b8380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x696cd4u32);
            c.push(m,0x0u32);
            c.push(m,0x696ce8u32);
            c.push(m,0x696cf0u32);
            c.set(1,32,0x6dd524u32);
            { let a = c.r[6].wrapping_add(0x2637); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x2639); wm(m,a,8,0x0u32); }
            c.push(m, 0x427501);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,0x6c7cccu32,32));
            c.set(3,32,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x46u32); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x427512; continue;
            }
            0x427512 => {
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x696cf8u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x427521);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x696d04u32);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,0x696d18u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x42753e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0x2889); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x7u32,32); }
            if c.cond(12) { pc = 0x427512; continue; }
            pc = 0x42754b; continue;
            }
            0x42754b => {
            c.set(2,32,c.r[3].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(2,32));
            c.push(m, 0x427558);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x47u32); }
            if c.cond(4) { pc = 0x427580; continue; }
            pc = 0x427566; continue;
            }
            0x427566 => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(7,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(3,32));
            c.push(m,0x4u32);
            c.push(m,c.get(7,32));
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x42757e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x427582; continue;
            }
            0x427580 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x427582; continue;
            }
            0x427582 => {
            c.push(m,c.get(3,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x46u32); }
            { let a = c.r[6].wrapping_add(0x287c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x427592);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x2880); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x427630; continue; }
            pc = 0x4275a5; continue;
            }
            0x4275a5 => {
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x696d20u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4275b4);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x696d28u32);
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6cbb44u32);
            c.push(m,c.get(1,32));
            c.push(m,0x696d3cu32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4275de);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x287c),32));
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)));
            c.push(m, 0x4275f1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x696d44u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x427600);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x696d50u32);
            c.push(m,0x64u32);
            c.push(m,c.get(1,32));
            c.push(m,0x696d64u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x42761d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[7].wrapping_add(c.r[2]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            if c.cond(12) { pc = 0x4275a5; continue; }
            pc = 0x427630; continue;
            }
            0x427630 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x2890));
            pc = 0x427638; continue;
            }
            0x427638 => {
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x696d6cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x427647);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x696d74u32);
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6cbb45u32);
            c.push(m,c.get(1,32));
            c.push(m,0x696d88u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x427671);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x42767d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x696d94u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x42768c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dd524u32);
            c.push(m,0x696da0u32);
            c.push(m,0x64u32);
            c.push(m,c.get(0,32));
            c.push(m,0x696db4u32);
            c.push(m, 0x4276a9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0x289c); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x427638; continue; }
            pc = 0x4276bd; continue;
            }
            0x4276bd => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(2,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m,c.get(1,32));
            c.push(m,0x696dc0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4276cf);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x696dccu32);
            c.push(m, 0x4276df);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.push(m,0x696dd4u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0x696de8u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4276f9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0x28c8); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m, 0x427713);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x48u32); }
            if c.cond(4) { pc = 0x42773b; continue; }
            pc = 0x427721; continue;
            }
            0x427721 => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(3,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x427739);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x42773d; continue;
            }
            0x42773b => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x42773d; continue;
            }
            0x42773d => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x46u32); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x28c8),8));
            { let a = c.r[6].wrapping_add(0x28a0); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x427755);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x28a4); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x28c8),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x427801; continue; }
            pc = 0x42776e; continue;
            }
            0x42776e => {
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x696df0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x42777d);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x696df8u32);
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6cbb46u32);
            c.push(m,c.get(1,32));
            c.push(m,0x696e0cu32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4277a7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28a0),32));
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)));
            c.push(m, 0x4277ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x696e14u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4277c9);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x696e20u32);
            c.push(m,0x64u32);
            c.push(m,c.get(1,32));
            c.push(m,0x696e34u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4277e6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28a4),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[7].wrapping_add(c.r[2]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x28c8),8));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x42776e; continue; }
            pc = 0x427801; continue;
            }
            0x427801 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x28a8));
            pc = 0x427809; continue;
            }
            0x427809 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x696e3cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x427818);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dd524u32);
            c.push(m,0x696e44u32);
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6cbb47u32);
            c.push(m,c.get(2,32));
            c.push(m,0x696e58u32);
            c.push(m, 0x427842);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x42784e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x696e64u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x42785d);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0x696e70u32);
            c.push(m,0x64u32);
            c.push(m,c.get(1,32));
            c.push(m,0x696e84u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x42787a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0x28b0); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x2u32,32); }
            if c.cond(12) { pc = 0x427809; continue; }
            pc = 0x42788e; continue;
            }
            0x42788e => {
            c.push(m,0x696e90u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x42789b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.push(m,0x696e98u32);
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6cbb48u32);
            c.push(m,c.get(2,32));
            c.push(m,0x696eacu32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4278c2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[6].wrapping_add(0x28bc));
            c.push(m, 0x4278d2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x696ebcu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x4278df);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.push(m,0x696ec8u32);
            c.push(m,0x64u32);
            c.push(m,c.get(0,32));
            c.push(m,0x696edcu32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4278f9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[6].wrapping_add(0x28c0); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x14u32); }
            c.push(m, 0x42790b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffa0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xeu32); }
            c.push(m, 0x427917);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x427923);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffff98));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            c.push(m, 0x42792f);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0x8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x42793e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn merchant_picker_dialog_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42dec0;
    loop {
        match pc {
            0x42dec0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d34cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,32,c.r[7].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x0u32); }
            c.push(m,c.get(0,32));
            c.push(m, 0x42def2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b1c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697014u32);
            c.push(m, 0x42df05);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x42df0c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b5380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x44));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.set(3,8,c.get(0,8));
            c.push(m, 0x42df21);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x610),32));
            c.set(6,32,c.r[7].wrapping_add(0x610));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42df32); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x66e10cu32,32));
            c.set(5,32,0x1u32);
            { let a = c.r[7].wrapping_add(0x6d0); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,0x66e110u32,32));
            { let a = c.r[7].wrapping_add(0x6d4); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[6].wrapping_add(0xe8); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xec); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x42df67); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x226u32);
            c.push(m,0x1c3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x42df78); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(5,32));
            c.push(m,0x2u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[7].wrapping_add(0x6d8); wm(m,a,32,0xff404040u32); }
            c.push(m, 0x42df8c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,c.r[7].wrapping_add(0x704));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x42df9a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464f30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x42dfa6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x42dfb2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,0xffffffceu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x32u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x42dfcf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let r = c.add(a,0xffffffceu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x32u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x42dfe1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x69b28cu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x42dfef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x69b2a8u32,32));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x42dffd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x42e005);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x42e00d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x42e016); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x42e020); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x69702cu32);
            c.push(m, 0x42e033);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x254),32));
            c.set(6,32,c.r[7].wrapping_add(0x254));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42e044); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x42e050); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x42e061);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn merchant_picker_dialog_refresh_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42e080;
    loop {
        match pc {
            0x42e080 => {
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6cbb94u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x5664),32));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x7f8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x608); wm(m,a,32,0x2u32); }
            c.set(0,32,rm(m,0x6dce78u32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dce78u32);
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x42e0b6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,0xfau32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(3,32,c.r[6].wrapping_add(0x24c));
            { let a = c.r[6].wrapping_add(0x604); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x60c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x42e0db);
            { let args = []; c.r[0] = host.call(m, 0x431060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x604),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xb4u32,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x23c); wm(m,a,32,0x91u32); }
            { let a = c.r[6].wrapping_add(0x240); wm(m,a,32,0xb4u32); }
            { let a = c.r[6].wrapping_add(0x248); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x244); wm(m,a,32,0x27bu32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x240),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x248),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x23c),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x244),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,0x1eu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x42e15f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x604),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x7f8),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(7,32,c.r[6].wrapping_add(0x254));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x42e183);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x7f8),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x42e191);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x7f8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x60c),32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x42e1ad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbb94u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[0].wrapping_add(0x210); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn parchment_info_dialog_init_controls(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x46cb10;
    loop {
        match pc {
            0x46cb10 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65f228u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,0x66de59u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x46cb4a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xb0),32));
            c.set(1,32,c.r[6].wrapping_add(0xb0));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x46cb59); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x46cb72);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x198),32));
            c.set(1,32,c.r[6].wrapping_add(0x198));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x46cb81); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x46cb94);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x280),32));
            c.set(1,32,c.r[6].wrapping_add(0x280));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x46cba3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x46cbb4);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn choice_list_dialog_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x470700;
    loop {
        match pc {
            0x470700 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65f410u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.push(m,0x6b9774u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x1u32); }
            c.push(m, 0x470745);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x6b9780u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x470762);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[6].wrapping_add(0xe6); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,0x6b9788u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x470783);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x470795);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            c.set(1,32,c.r[6].wrapping_add(0xf8));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x4707a4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66de59u32,8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x4707be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4a4),32));
            c.set(1,32,c.r[6].wrapping_add(0x4a4));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4707cd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x4707e6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x58c),32));
            c.set(1,32,c.r[6].wrapping_add(0x58c));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4707f5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x66e104u32,32));
            { let a = c.r[6].wrapping_add(0x650); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xe6),16));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x47082b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            { let a = c.r[6].wrapping_add(0xe8); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0xac); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xcc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xaa); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0xa8); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0xa6); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0xec); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xd0); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x470882);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x938),32));
            c.set(7,32,c.r[6].wrapping_add(0x938));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x470893); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[6].wrapping_add(0xab8));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x4708a1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xff000000u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4708ad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2d80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0xaa4); wm(m,a,32,0x10u32); }
            c.push(m, 0x4708c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2e20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6cbe00u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4708cc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4cab50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xaac); wm(m,a,32,0x1u32); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(0x170); wm(m,a,32,0x64u32); }
            { let a = c.r[7].wrapping_add(0x179); wm(m,a,8,0x1u32); }
            c.set(1,8,rm(m,0x66de56u32,8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x470901);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x680),32));
            c.set(7,32,c.r[6].wrapping_add(0x680));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x470912); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x6b04fcu32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47091f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66de56u32,8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x470939);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x768),32));
            c.set(7,32,c.r[6].wrapping_add(0x768));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x47094a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x6b0500u32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x470957);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66de56u32,8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x470971);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x850),32));
            c.set(7,32,c.r[6].wrapping_add(0x850));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x470982); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x69af30u32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47098f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6b9794u32);
            c.push(m, 0x4709a1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xae0),32));
            c.set(1,32,c.r[6].wrapping_add(0xae0));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4709b0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4709bd);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4709ce);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn choice_list_dialog_on_delete_answer_tail(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x472536;
    loop {
        match pc {
            0x472536 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x4u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x472547); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x472551); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x472560);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x472569); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x768),32));
            c.set(1,32,c.r[6].wrapping_add(0x768));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47257a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x680),32));
            c.set(1,32,c.r[6].wrapping_add(0x680));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47258b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4a4),32));
            c.set(1,32,c.r[6].wrapping_add(0x4a4));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47259c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x850),32));
            c.set(1,32,c.r[6].wrapping_add(0x850));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4725ad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x938),32));
            c.set(1,32,c.r[6].wrapping_add(0x938));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4725be); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn hometown_picker_dialog_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49bc00;
    loop {
        match pc {
            0x49bc00 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660e20u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x267c));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x49bc34);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x4d2u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x49bc4a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x49bc54);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b3ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[6].wrapping_add(0x25a4));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x49bc62);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b1c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x49bc78);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49bc7f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b5380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x49bc91);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x52a0),32));
            c.set(1,32,c.r[6].wrapping_add(0x52a0));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x49bca0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x49bcb3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x5388),32));
            c.set(1,32,c.r[6].wrapping_add(0x5388));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x49bcc2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x49bcd5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x5470),32));
            c.set(1,32,c.r[6].wrapping_add(0x5470));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x49bce4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,c.r[6].wrapping_add(0x164));
            c.set(5,32,0x28u32);
            pc = 0x49bcef; continue;
            }
            0x49bcef => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49bcfa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49bd02);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x49bcef; continue; }
            pc = 0x49bd0b; continue;
            }
            0x49bd0b => {
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x49bd1e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2680),32));
            c.set(7,32,c.r[6].wrapping_add(0x2680));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x49bd2f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6aee48u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x49bd3f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            c.push(m, 0x49bd50);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x49bd5d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6aee48u32,32));
            c.set(1,32,0x6dcdd0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x49bd6d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x58),32); c.push(m, 0x49bd77); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x14u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x5c),32); c.push(m, 0x49bd80); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,0x307u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,0x22bu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x49bd94); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49bd9c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x49bdae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2780),32));
            c.set(7,32,c.r[6].wrapping_add(0x2780));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x49bdbf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6bab5cu32);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x49bdcd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x2u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x49bdde);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x49bdeb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6bab70u32);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x49bdf9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x3u32); }
            c.push(m, 0x49be0a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x49be17);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49be22); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49be2a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x49be3d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2880),32));
            c.set(7,32,c.r[6].wrapping_add(0x2880));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x49be4e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6bab84u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x49be5c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6aee40u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x4u32); }
            c.push(m, 0x49be73);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x5u32); }
            c.push(m, 0x49be80);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x4u32); }
            c.push(m, 0x49be8e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x49be9b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49bea6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49beae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x49bec1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2980),32));
            c.set(7,32,c.r[6].wrapping_add(0x2980));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x49bed2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6bab86u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x49bee0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6aee44u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x6u32); }
            c.push(m, 0x49bef7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x7u32); }
            c.push(m, 0x49bf04);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x6u32); }
            c.push(m, 0x49bf12);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x49bf1f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49bf2a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49bf32);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x3e80));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x14u32); }
            pc = 0x49bf40; continue;
            }
            0x49bf40 => {
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x49bf53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xffffec00),32));
            c.set(5,32,c.r[7].wrapping_add(0xffffec00));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x49bf64); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x49bf70); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49bf78);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x49bf8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x49bf92); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49bf9d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49bfa5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(7,32); let r = c.add(a,0x100u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x49bf40; continue; }
            pc = 0x49bfb6; continue;
            }
            0x49bfb6 => {
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x49bfc7);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn hometown_picker_dialog_open(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x49c0d0;
    loop {
        match pc {
            0x49c0d0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660e3au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x18),8));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[3].wrapping_add(0x5558); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.set(6,32,c.r[3].wrapping_add(0x164));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(7,32,c.get(6,32));
            { let a = c.r[3].wrapping_add(0x555c); wm(m,a,32,c.get(2,32)); }
            c.set(5,32,0x28u32);
            pc = 0x49c117; continue;
            }
            0x49c117 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49c123); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x49c117; continue; }
            pc = 0x49c12c; continue;
            }
            0x49c12c => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x555c),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x49c212; continue; }
            pc = 0x49c13c; continue;
            }
            0x49c13c => {
            c.set(5,32,rm(m,0x6df2fcu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x16c),8));
            c.set(3,8,rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x144),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,0x2aaaaaabu32);
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x6ddb90),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x6ddb94),32));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xfffffff7));
            c.set(0,32,0x38e38e39u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xfffffff7));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x49c1a9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x49c1ba; continue; }
            pc = 0x49c1ae; continue;
            }
            0x49c1ae => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[0].wrapping_add(0x5388));
            pc = 0x49c1d4; continue;
            }
            0x49c1ba => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(9) { pc = 0x49c1ca; continue; }
            pc = 0x49c1be; continue;
            }
            0x49c1be => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,0x5470u32,32); c.set(1,32,r); }
            pc = 0x49c1d4; continue;
            }
            0x49c1ca => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[2].wrapping_add(0x52a0));
            pc = 0x49c1d4; continue;
            }
            0x49c1d4 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x49c1da);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x49c1ed); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49c1f9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x555c),32),32); }
            if c.cond(12) { pc = 0x49c13c; continue; }
            pc = 0x49c210; continue;
            }
            0x49c210 => {
            c.set(3,32,c.get(1,32));
            pc = 0x49c212; continue;
            }
            0x49c212 => {
            c.set(6,32,c.r[3].wrapping_add(0x3e80));
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            c.set(7,32,0x14u32);
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x49c222; continue;
            }
            0x49c222 => {
            c.push(m,0x6bab88u32);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m, 0x49c230);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.set(1,32,c.r[6].wrapping_add(0xffffec00));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x49c244);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x49c251);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6bab8au32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x49c25f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x1u32); }
            c.push(m, 0x49c273);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x49c280);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x100u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x49c222; continue; }
            pc = 0x49c289; continue;
            }
            0x49c289 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn message_box_screen_init_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ac950;
    loop {
        match pc {
            0x4ac950 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6615a6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            c.push(m, 0x4ac990);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4ac997);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b5380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x31u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,0x1u32); }
            c.push(m, 0x4ac9bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x174),32));
            c.set(6,32,c.r[3].wrapping_add(0x174));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x4ac9cd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[7].wrapping_add(0x48),32); c.push(m, 0x4ac9d4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x258u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4ac9ea); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x320u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[7].wrapping_add(0x78),32); c.push(m, 0x4ac9fe); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4aca11);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x24c),32));
            c.set(7,32,c.r[3].wrapping_add(0x24c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x4aca22); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4aca2d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x66e10cu32,32));
            { let a = c.r[3].wrapping_add(0x30c); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,0x66e110u32,32));
            { let a = c.r[3].wrapping_add(0x310); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(3,32); let r = c.add(a,0x340u32,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4aca53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464f30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x699948u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4aca60);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x699950u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4aca6e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x4aca79); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(5,32,0xffffffeeu32);
            c.set(1,32,c.get(6,32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x8c),32); c.push(m, 0x4aca8a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x4aca93); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4aca9d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x4acaaf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,0x190u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x4acac7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[3],32));
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x4acad4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,0xffffffeeu32);
            c.set(1,32,c.get(6,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x4acae5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x4acaee); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x4acaf8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7],32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4acb0a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.add(a,0x190u32,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,c.r[3].wrapping_add(0x340));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x78),32); c.push(m, 0x4acb2b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4acb33);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4acb3b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4acb43);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x4acb51);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4acb62);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn viper_message_box_setup_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ceca0;
    loop {
        match pc {
            0x4ceca0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66272bu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            c.push(m, 0x4ceccb);
            { let args = []; c.r[0] = host.call(m, 0x4c9340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x180));
            c.push(m, 0x4cecd6);
            { let args = []; c.r[0] = host.call(m, 0x4acea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.r[7].wrapping_add(0x258));
            c.set(3,32,c.r[7].wrapping_add(0xc58));
            c.set(5,32,0xau32);
            pc = 0x4cece7; continue;
            }
            0x4cece7 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4cecee);
            { let args = []; c.r[0] = host.call(m, 0x4d2100, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x4cecf6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x100u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x4cece7; continue; }
            pc = 0x4ced02; continue;
            }
            0x4ced02 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xb4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,0x6u32,32); }
            { let a = c.r[7].wrapping_add(0x10f8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x1100); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x1104); wm(m,a,32,0xffffffffu32); }
            if c.cond(12) { pc = 0x4ced28; continue; }
            pc = 0x4ced26; continue;
            }
            0x4ced26 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x4ced28; continue;
            }
            0x4ced28 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xbc),32));
            { let a = c.get(1,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4ced65; continue; }
            pc = 0x4ced34; continue;
            }
            0x4ced34 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xb8),32));
            c.set(6,32,rm(m,c.r[2],32));
            { let a = rm(m,c.r[6].wrapping_add(0xfffffff8),32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4ced65; continue; }
            pc = 0x4ced42; continue;
            }
            0x4ced42 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4ced50);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x180),32));
            c.set(1,32,c.r[7].wrapping_add(0x180));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4ced5f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4ced65; continue;
            }
            0x4ced65 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xac),32));
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4cee96; continue; }
            pc = 0x4ced85; continue;
            }
            0x4ced85 => {
            c.set(1,32,c.r[7].wrapping_add(0xc58));
            c.set(6,32,c.r[7].wrapping_add(0x258));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            pc = 0x4ced95; continue;
            }
            0x4ced95 => {
            { let a = rm(m,c.r[4].wrapping_add(0x1c),32); c.sub(a,0xau32,32); }
            if c.cond(13) { pc = 0x4cee96; continue; }
            pc = 0x4ceda0; continue;
            }
            0x4ceda0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xac),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,0x6bfd20u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4cedb7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64d3d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4cedca; continue; }
            pc = 0x4cedbe; continue;
            }
            0x4cedbe => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xac),32));
            c.set(1,32,rm(m,c.r[0],32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0xfffffff8),32));
            pc = 0x4cedca; continue;
            }
            0x4cedca => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.get(5,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xac),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m, 0x4cede5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64d0c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0xa4); wm(m,a,32,0x0u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m, 0x4cedfe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b0450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,c.r[3].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4cee1b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),32),32); }
            if c.cond(14) { pc = 0x4cee25; continue; }
            pc = 0x4cee21; continue;
            }
            0x4cee21 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x4cee25; continue;
            }
            0x4cee25 => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4cee2d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2e20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4cee38);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d2d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0xffffffu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4cee44);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2d70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x670f24u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4cee52);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2d80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0xa4); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4cee66);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xac),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.add(a,0x20u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x100u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4ced95; continue; }
            pc = 0x4cee96; continue;
            }
            0x4cee96 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[7].wrapping_add(0x10f8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4ceeb0; continue; }
            pc = 0x4ceea8; continue;
            }
            0x4ceea8 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xau32); }
            pc = 0x4ceeb0; continue;
            }
            0x4ceeb0 => {
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[3].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[0].wrapping_add(0x6dcd54));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x6dcd54),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x54));
            c.set(3,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x180),32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[7].wrapping_add(0x180));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4cef2a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.add(a,0xau32,32); c.set(3,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.set(6,32,c.r[1].wrapping_add(0xa));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x10f8),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x66bc6cu32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(5,32,c.r[6].wrapping_add(0x28));
            { let a = c.get(1,32); let r = c.add(a,0x28u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x4100u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x4cef8d; continue; }
            pc = 0x4cef72; continue;
            }
            0x4cef72 => {
            c.fild32(m,c.r[4].wrapping_add(0x54));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x66bc6cu32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x4100u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x4cefa1; continue; }
            pc = 0x4cef8d; continue;
            }
            0x4cef8d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xa));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(5,32,c.r[0].wrapping_add(c.r[5]).wrapping_add(0xa));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            pc = 0x4cefa1; continue;
            }
            0x4cefa1 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0x5u32); }
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(3,32)); }
            if c.cond(14) { pc = 0x4cefd3; continue; }
            pc = 0x4cefc6; continue;
            }
            0x4cefc6 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(0,32)); }
            pc = 0x4cefd3; continue;
            }
            0x4cefd3 => {
            c.fild32(m,c.r[4].wrapping_add(0x70));
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x66bc6cu32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(10) { pc = 0x4cf008; continue; }
            pc = 0x4cefec; continue;
            }
            0x4cefec => {
            c.push(m,0xcu32);
            c.set(1,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4ceff9);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4cf008; continue;
            }
            0x4cf008 => {
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x10f8),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4cf06e; continue; }
            pc = 0x4cf01f; continue;
            }
            0x4cf01f => {
            c.set(6,32,c.r[7].wrapping_add(0x258));
            pc = 0x4cf02b; continue;
            }
            0x4cf027 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            pc = 0x4cf02b; continue;
            }
            0x4cf02b => {
            c.set(2,32,c.get(0,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x60));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x64),32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x54),32); c.push(m, 0x4cf050); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x78));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4cf05d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x10f8),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x100u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4cf027; continue; }
            pc = 0x4cf06e; continue;
            }
            0x4cf06e => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4cf089; continue; }
            pc = 0x4cf076; continue;
            }
            0x4cf076 => {
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[7].wrapping_add(0x180));
            c.push(m,c.get(0,32));
            c.push(m, 0x4cf089);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4cf089; continue;
            }
            0x4cf089 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x54),32); c.push(m, 0x4cf095); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x4cf09f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dab4cu32,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dab44u32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x4cf0dc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xb0),32));
            c.set(5,32,c.r[7].wrapping_add(0xd98));
            { let a = c.r[7].wrapping_add(0x1100); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,c.get(5,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(3,32,c.r[7].wrapping_add(0x1098));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x3u32); }
            c.set(0,32,c.r[0].wrapping_add(0x6bfce4));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            pc = 0x4cf117; continue;
            }
            0x4cf117 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,c.get(3,32));
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4cf125);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b0450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x4cf12d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4cf136);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2d70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x670f24u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4cf143);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2d80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x670f28u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4cf151);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4cf15c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d2d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4cf168); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4cf171); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x100u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4cf117; continue; }
            pc = 0x4cf190; continue;
            }
            0x4cf190 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xb0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4cf2f7; continue; }
            pc = 0x4cf19f; continue;
            }
            0x4cf19f => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(15) { pc = 0x4cf2f7; continue; }
            pc = 0x4cf1a8; continue;
            }
            0x4cf1a8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[2],32));
            c.set(6,32,c.r[1].wrapping_add(0x6dcd28));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4cf1c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x6bfce8),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4cf1d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x48));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x4cf1ef); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x54),32); c.push(m, 0x4cf207); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4cf214); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4cf21e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x48));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4cf235); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xe98),32));
            c.set(3,32,c.r[7].wrapping_add(0xe98));
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x54),32); c.push(m, 0x4cf256); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4cf262); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4cf26b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[2]));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.sub(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x48));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4cf29b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x24));
            { let a = c.get(1,32); let r = c.sub(a,0x7u32,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4cf2b9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x8));
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x90));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x4cf2d8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x94),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x24));
            { let a = c.get(2,32); let r = c.sub(a,0x7u32,32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(3,32));
            pc = 0x4cf391; continue;
            }
            0x4cf2f7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[2],32));
            c.set(6,32,c.r[1].wrapping_add(0x6dcd28));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4cf30f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(3,32,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x48));
            c.set(1,32,c.r[3].wrapping_add(0x2));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4cf324); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x54),32); c.push(m, 0x4cf33c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4cf349); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4cf353); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x48));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x4cf37a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x24));
            { let a = c.get(1,32); let r = c.sub(a,0x7u32,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(5,32));
            pc = 0x4cf391; continue;
            }
            0x4cf391 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4cf398);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x11c),32); c.push(m, 0x4cf3a2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x9c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x98u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn bath_house_dialog_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x59fff0;
    loop {
        match pc {
            0x59fff0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(7,32,c.get(1,32));
            c.push(m, 0x5a0000);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x94),32));
            c.push(m,0x1u32);
            { let a = c.get(6,32); let r = c.neg(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[7].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[7].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x5a0045);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x5a0052);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5a0058);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a0076); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(6,32,c.r[7].wrapping_add(0x174));
            pc = 0x5a0084; continue;
            }
            0x5a0084 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6a1),8));
            c.set(2,32,0x1u32);
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0xac),8); c.and(a,c.get(2,8),8); }
            c.set(1,32,c.get(6,32));
            if c.cond(4) { pc = 0x5a00ee; continue; }
            pc = 0x5a00a2; continue;
            }
            0x5a00a2 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5a00a9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a00b5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a00d7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a00e0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a00ec); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a00f8; continue;
            }
            0x5a00ee => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a00f8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a00f8; continue;
            }
            0x5a00f8 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x5a0084; continue; }
            pc = 0x5a0110; continue;
            }
            0x5a0110 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(2,32));
            { let a = c.r[7].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0xa4); wm(m,a,8,c.get(1,8)); }
            c.push(m, 0x5a012e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn production_building_info_dialog_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a1790;
    loop {
        match pc {
            0x5a1790 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66506eu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,c.r[7].wrapping_add(0xac));
            c.set(0,8,rm(m,0x66de58u32,8));
            c.push(m,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5a17dc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[7].wrapping_add(0x194));
            c.set(0,8,rm(m,0x66de58u32,8));
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5a1802);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5a180b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5a1814);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c5140u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5a1820);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c5142u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5a182c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c5144u32);
            c.push(m, 0x5a183f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x450),32));
            c.set(6,32,c.r[7].wrapping_add(0x450));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5a1850); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[7].wrapping_add(0x5e0));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x5a185e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a1867);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2e20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x188),32));
            c.set(0,32,0x64u32);
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x180); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x5a1884; continue; }
            pc = 0x5a187c; continue;
            }
            0x5a187c => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a1884);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a1884; continue;
            }
            0x5a1884 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a1890); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a189d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a18a9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5a18b5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4417d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c5164u32);
            c.push(m, 0x5a18c8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x27c),32));
            c.set(6,32,c.r[7].wrapping_add(0x27c));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5a18d9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c5184u32);
            c.push(m, 0x5a18ec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x364),32));
            { let a = c.get(7,32); let r = c.add(a,0x364u32,32); c.set(7,32,r); }
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5a18fd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,0x9u32,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x5a1916); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbd9cu32,32));
            c.set(0,32,0x92492493u32);
            c.push(m,c.get(2,32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(6,32,rm(m,c.r[7],32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x9));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[6].wrapping_add(0x64),32); c.push(m, 0x5a1949); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5a195a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn production_building_info_dialog_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a1980;
    loop {
        match pc {
            0x5a1980 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5a1990);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0x1u32);
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x5a19d5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x5a19e2);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5a19e8);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a1a06); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x63f),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5a1b29; continue; }
            pc = 0x5a1a1c; continue;
            }
            0x5a1a1c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x640),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5a1b29; continue; }
            pc = 0x5a1a2a; continue;
            }
            0x5a1a2a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.set(7,32,c.r[6].wrapping_add(0xac));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a1a3d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a1a49); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,c.get(4,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x5a1a6a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a1a73); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a1a7f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x194),32));
            c.set(7,32,c.r[6].wrapping_add(0x194));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a1a92); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a1a9e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,c.get(4,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x5a1abf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a1ac8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a1ad4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x450),32));
            c.set(7,32,c.r[6].wrapping_add(0x450));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a1ae7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a1af3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,c.get(4,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x5a1b14); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a1b1d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a1b29); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a1b29; continue;
            }
            0x5a1b29 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5a1b47);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn building_info_popup_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a8450;
    loop {
        match pc {
            0x5a8450 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x665362u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5a847d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4417d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c5564u32);
            c.push(m, 0x5a8490);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            { let a = c.get(6,32); let r = c.add(a,0xacu32,32); c.set(6,32,r); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5a84a1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let r = c.add(a,0x9u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5a84ba); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5a84cb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn building_parchment_dialog_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5af090;
    loop {
        match pc {
            0x5af090 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x665786u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5af0bf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4417d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x3u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c5a4cu32);
            c.push(m, 0x5af0d2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.set(7,32,c.r[6].wrapping_add(0xac));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5af0e3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c5a6cu32);
            c.push(m, 0x5af0f6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x194),32));
            c.set(3,32,c.r[6].wrapping_add(0x194));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5af107); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,0x9u32,32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x5af120); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbd9cu32,32));
            c.set(0,32,0x92492493u32);
            c.push(m,c.get(2,32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(7,32,rm(m,c.r[3],32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x9));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[7].wrapping_add(0x64),32); c.push(m, 0x5af153); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de58u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5af16c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x27c),32));
            c.set(7,32,c.r[6].wrapping_add(0x27c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5af17d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5af186);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c5a8cu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5af192);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de58u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5af1ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x364),32));
            { let a = c.get(6,32); let r = c.add(a,0x364u32,32); c.set(6,32,r); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5af1bc); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5af1c5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c5a8eu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5af1d1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5af1e2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn building_parchment_dialog_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5af200;
    loop {
        match pc {
            0x5af200 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5af210);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(3,32,rm(m,c.r[6],32));
            c.set(5,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[3].wrapping_add(0x9c),32); c.push(m, 0x5af266); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5af280);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn militia_training_dialog_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5b1160;
    loop {
        match pc {
            0x5b1160 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66582au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0x7d00u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            c.push(m, 0x5b1199);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5b11a3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b3ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.push(m,0x6c5b18u32);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x1u32); }
            c.push(m, 0x5b11c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x66de59u32,8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5b11da);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa4),32));
            c.set(1,32,c.r[6].wrapping_add(0xa4));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5b11e9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5b11f5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.push(m, 0x5b1203);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5b1214);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn bath_house_bribe_dialog_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5b1bb0;
    loop {
        match pc {
            0x5b1bb0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66592au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x6c5bacu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            c.push(m, 0x5b1bf2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6c5bbcu32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x604));
            c.push(m, 0x5b1c0c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de59u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5b1c25);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.r[6].wrapping_add(0x94));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5b1c34); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5b1c4d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(3,32,c.r[6].wrapping_add(0x17c));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5b1c5e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5b1c77);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x264),32));
            c.set(7,32,c.r[6].wrapping_add(0x264));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5b1c88); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5b1ca1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x34c),32));
            c.set(5,32,c.r[6].wrapping_add(0x34c));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5b1cb2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5b1cbb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5b1cc4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c5bcau32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5b1cd0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c5bccu32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5b1cdc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6a2a6cu32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5b1ce9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x244); wm(m,a,32,0xff404040u32); }
            c.push(m, 0x5b1d06);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x438),32));
            c.set(7,32,c.r[6].wrapping_add(0x438));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5b1d17); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(6,32); let r = c.add(a,0x5c8u32,32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x5b1d25); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1b58u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c5bd0u32);
            c.push(m, 0x5b1d3b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x5b1d46);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4439b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x0u32); }
            c.push(m, 0x5b1d54);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5b1d65);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn money_lender_bank_dialog_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5b3160;
    loop {
        match pc {
            0x5b3160 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x665a78u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0x7d00u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            c.push(m, 0x5b3199);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5b31a3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b3ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.push(m,0x6c5c68u32);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x1u32); }
            c.push(m, 0x5b31c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m,0x6c5c78u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x434));
            c.push(m, 0x5b31da);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66de59u32,8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5b31f4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.r[6].wrapping_add(0x94));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5b3203); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5b321c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(1,32,c.r[6].wrapping_add(0x17c));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5b322b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c5c88u32);
            { let a = c.r[6].wrapping_add(0x244); wm(m,a,32,0xff404040u32); }
            c.push(m, 0x5b3248);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x448),32));
            c.set(1,32,c.r[6].wrapping_add(0x448));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5b3257); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c5ca0u32);
            c.push(m, 0x5b326a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x7fc),32));
            c.set(1,32,c.r[6].wrapping_add(0x7fc));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5b3279); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c5cb8u32);
            c.push(m, 0x5b328c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbb0),32));
            c.set(1,32,c.r[6].wrapping_add(0xbb0));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5b329b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.push(m, 0x5b32a9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x5b32ba);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_info_popup_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5b8080;
    loop {
        match pc {
            0x5b8080 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x665c32u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,0x66de59u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5b80ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa4),32));
            c.set(1,32,c.r[6].wrapping_add(0xa4));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5b80c9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5b80d5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5b80e6);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn expedition_dialog_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5b8410;
    loop {
        match pc {
            0x5b8410 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x665e04u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.push(m,0x6c5f3cu32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x1u32); }
            c.push(m, 0x5b8455);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0xcu32);
            c.push(m,0x6c5f48u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x5b8473);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[6].wrapping_add(0xb8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            c.push(m,0xeu32);
            c.push(m,0x6c5f50u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x5b8494);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,0xfu32);
            c.push(m,0x6c5f58u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[6].wrapping_add(0xbc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5b84b5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            c.push(m,0x190u32);
            c.push(m,0x6c5f60u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x5b84d9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[6].wrapping_add(0xb0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            c.push(m,0x5208u32);
            c.push(m,0x6c5f6cu32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[6].wrapping_add(0xb4); wm(m,a,32,0x12cu32); }
            c.push(m, 0x5b8507);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,0x791au32);
            c.push(m,0x6c5f74u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x5b8527);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x5208u32);
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5b853f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5b8549);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b3ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x3ee4u32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5b8558);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,0x6da820u32);
            { let a = c.r[6].wrapping_add(0x98); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5b8569);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[7].wrapping_add(0x1));
            { let a = c.r[6].wrapping_add(0x9c); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5b857d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,0x6da820u32);
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5b858e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5b85a2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xa8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de59u32,8));
            c.set(2,32,c.r[6].wrapping_add(0x2f0));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5b85ce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x1350));
            c.set(0,8,rm(m,0x66de58u32,8));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5b85f4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c5f7eu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5b8600);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x3d8));
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5b8626);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c2a44u32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5b8633);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0xc00));
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5b8659);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c5f80u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5b8665);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5b866e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0xce8));
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5b8694);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c5f82u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5b86a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5b86a9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,0x6b0e38u32);
            c.set(5,32,c.r[6].wrapping_add(0x4c0));
            pc = 0x5b86b4; continue;
            }
            0x5b86b4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            c.set(0,8,rm(m,0x66de56u32,8));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5b86d4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5b86de);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0xe8u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,0x6b0e58u32,32); }
            if c.cond(12) { pc = 0x5b86b4; continue; }
            pc = 0x5b86ef; continue;
            }
            0x5b86ef => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0xdd0));
            c.set(0,8,rm(m,0x66de55u32,8));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5b8715);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xeb8); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0xfa4); wm(m,a,32,0x1u32); }
            c.set(0,32,rm(m,0x6b0e5cu32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5b8736);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0xfa8));
            c.set(0,8,rm(m,0x66de58u32,8));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5b875c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1090));
            c.set(0,8,rm(m,0x66de58u32,8));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5b8782);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1du32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c5f84u32);
            c.push(m, 0x5b8795);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1180),32));
            c.set(1,32,c.r[6].wrapping_add(0x1180));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5b87a4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1du32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c5f9cu32);
            c.push(m, 0x5b87b7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1258),32));
            c.set(1,32,c.r[6].wrapping_add(0x1258));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5b87c6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5b87d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x140),32));
            c.set(7,32,c.r[6].wrapping_add(0x140));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5b87ea); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[6].wrapping_add(0x2d0));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x5b87f8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5b8804); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            c.push(m, 0x5b8812);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5b8823);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn trade_offer_dialog_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5bd1ce;
    loop {
        match pc {
            0x5bd1ce => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66609cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0x7d00u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            c.push(m, 0x5bd209);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5bd213);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b3ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.push(m,0x6c60d4u32);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x1u32); }
            c.push(m, 0x5bd230);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(2,32));
            c.push(m,0x6c60ecu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0xa60));
            c.push(m, 0x5bd24a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66de59u32,8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5bd264);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa4),32));
            c.set(1,32,c.r[6].wrapping_add(0xa4));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5bd273); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5bd28c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18c),32));
            c.set(1,32,c.r[6].wrapping_add(0x18c));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5bd29b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5bd2b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x274),32));
            c.set(1,32,c.r[6].wrapping_add(0x274));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5bd2c3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x254); wm(m,a,32,0xff404040u32); }
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5bd2e6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x35c),32));
            c.set(7,32,c.r[6].wrapping_add(0x35c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5bd2f7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5bd310);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x444),32));
            c.set(5,32,c.r[6].wrapping_add(0x444));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5bd321); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5bd32a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bd333);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c60fau32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5bd33f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c60fcu32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bd34b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5bd364);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x52c),32));
            c.set(7,32,c.r[6].wrapping_add(0x52c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5bd375); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5bd38e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x614),32));
            c.set(5,32,c.r[6].wrapping_add(0x614));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5bd39f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5bd3a8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bd3b1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c60feu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5bd3bd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c6100u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5bd3c9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5bd3dc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x700),32));
            c.set(7,32,c.r[6].wrapping_add(0x700));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5bd3ed); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.r[6].wrapping_add(0x890));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x5bd3fb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5bd40e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8b0),32));
            c.set(7,32,c.r[6].wrapping_add(0x8b0));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5bd41f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[6].wrapping_add(0xa40));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x5bd42d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6c6104u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x5bd448);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c610cu32);
            { let a = c.r[6].wrapping_add(0x1ee4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5bd461);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa74),32));
            c.set(1,32,c.r[6].wrapping_add(0xa74));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5bd470); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6c6138u32);
            c.push(m,0x0u32);
            c.push(m,0x6c6150u32);
            c.push(m,0x6c6124u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x5bd48b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6c615cu32);
            c.push(m, 0x5bd49d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xe20),32));
            c.set(1,32,c.r[6].wrapping_add(0xe20));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5bd4ac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x0u32); }
            c.push(m, 0x5bd4ba);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5bd4cb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_dialog_show(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5ef020;
    loop {
        match pc {
            0x5ef020 => {
            { let a = c.get(4,32); let r = c.sub(a,0x34u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x5ef033);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x5ef041);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x60),32); c.push(m, 0x5ef051); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x5ef061); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0x64u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[6],32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[7].wrapping_add(0x64),32); c.push(m, 0x5ef094); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ef09b);
            { let args = []; c.r[0] = host.call(m, 0x462390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.set(7,32,c.r[6].wrapping_add(0x98));
            c.push(m,c.get(7,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xc324),8));
            { let a = c.r[6].wrapping_add(0x15e8); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5ef0ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[7],32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[3].wrapping_add(0x48),32); c.push(m, 0x5ef0c3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xfffffffd));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5ef0d7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x3u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x5ef0ea); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ef0f6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xb78),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(7,32,c.r[6].wrapping_add(0xb78));
            c.push(m,0x64u32);
            { let a = c.get(0,32); let r = c.add(a,0x19au32,32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[3].wrapping_add(0x44),32); c.push(m, 0x5ef112); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x5ef12e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x188),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15e8),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x700),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[7].wrapping_add(0x180); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x5ef162; continue; }
            pc = 0x5ef15a; continue;
            }
            0x5ef15a => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5ef162);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5ef162; continue;
            }
            0x5ef162 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5ef16a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ef176); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5ef17f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x180));
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5ef1a5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5ef1ad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c725cu32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5ef1b9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x5ef1c5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x1c2u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5ef1f1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ef1fd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x268),32));
            c.set(3,32,c.r[6].wrapping_add(0x268));
            c.push(m,0x64u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x5ef212); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5ef21a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,0xfffffffeu32);
            c.set(1,32,c.get(7,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5ef228); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,c.r[6].wrapping_add(0x268));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x5ef236); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x350),32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x5ef245); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5ef24d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5ef258); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(7,32,c.r[6].wrapping_add(0x350));
            c.set(0,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1a));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x5ef26c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5ef278);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5ef280);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ef28c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5ef298); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xd28),32));
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(7,32),32); c.set(5,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0xd28));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5ef2b8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x438),32));
            c.set(3,32,c.r[6].wrapping_add(0x438));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5ef2c9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cc380u32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ef2e1);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x4u32); }
            c.set(2,32,c.r[1].wrapping_add(c.r[5]).wrapping_add(0xa));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x5ef300; continue;
            }
            0x5ef300 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x64u32);
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x160));
            c.set(2,32,c.get(5,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5ef321); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x64u32);
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x162));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5ef33e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5ef348);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5ef352);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5ef35e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ef36a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.add(a,0x100u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0xe8u32,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5ef300; continue; }
            pc = 0x5ef390; continue;
            }
            0x5ef390 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x7d8),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(7,32,c.r[6].wrapping_add(0x7d8));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5ef3a4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x4u32); }
            { let a = c.get(3,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(5,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x5ef3c0; continue;
            }
            0x5ef3c0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x64u32);
            c.set(1,32,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x14e));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x5ef3d5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5ef3df);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ef3eb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(5,32); let r = c.add(a,0x14u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5ef3c0; continue; }
            pc = 0x5ef3ff; continue;
            }
            0x5ef3ff => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(3,32,c.r[6].wrapping_add(0x11a8));
            c.set(5,32,0x4u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffc8));
            c.set(7,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            pc = 0x5ef41a; continue;
            }
            0x5ef41a => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5ef421); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x5ef41a; continue; }
            pc = 0x5ef434; continue;
            }
            0x5ef434 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(7,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.add(a,0x1c2u32,32); c.set(1,32,r); }
            c.set(5,32,0x4u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            pc = 0x5ef448; continue;
            }
            0x5ef448 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5ef452);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5ef45e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5ef465); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,0x0u32);
            c.set(2,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x2));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5ef47d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x5ef448; continue; }
            pc = 0x5ef486; continue;
            }
            0x5ef486 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1620),32));
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.r[6].wrapping_add(0x161c); wm(m,a,32,c.get(1,32)); }
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x5ef49e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1290),32));
            c.set(1,32,c.r[6].wrapping_add(0x1290));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x124),32); c.push(m, 0x5ef4b2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1378),32));
            c.set(1,32,c.r[6].wrapping_add(0x1378));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x5ef4c6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1460),32));
            c.set(1,32,c.r[6].wrapping_add(0x1460));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x124),32); c.push(m, 0x5ef4da); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x5ef4df);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x7d0u32);
            c.push(m,0x1u32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,rm(m,0x6dccf8u32,32));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x7d0));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[6].wrapping_add(0x1618); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ef505); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_dialog_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5efa00;
    loop {
        match pc {
            0x5efa00 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x667e44u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(5,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x6c7264u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x1u32); }
            c.push(m, 0x5efa42);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66de59u32,8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5efa5c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x98),32));
            c.set(1,32,c.r[5].wrapping_add(0x98));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5efa6b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5efa84);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x268),32));
            c.set(6,32,c.r[5].wrapping_add(0x268));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5efa95); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5efaae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x350),32));
            c.set(7,32,c.r[5].wrapping_add(0x350));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5efabf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5efac8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5efad1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c7276u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5efadd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c7278u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5efae9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[5].wrapping_add(0x1128));
            c.set(7,32,c.r[5].wrapping_add(0x7d8));
            c.set(6,32,c.r[5].wrapping_add(0xd28));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x4u32); }
            pc = 0x5efb07; continue;
            }
            0x5efb07 => {
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5efb1a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5efb21); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x5efb2d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5efb36);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2e20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5efb3f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2d70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66de56u32,8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5efb59);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xfffffc60),32));
            c.set(3,32,c.r[7].wrapping_add(0xfffffc60));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5efb6a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6c727cu32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5efb76);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5efb7f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de56u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5efb98);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5efb9f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6c7280u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5efbab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5efbb4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x100u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5efb07; continue; }
            pc = 0x5efbda; continue;
            }
            0x5efbda => {
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5efbed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xb78),32));
            c.set(6,32,c.r[5].wrapping_add(0xb78));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5efbfe); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[5].wrapping_add(0xd08));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x5efc0c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5efc26);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x11a8),32));
            c.set(6,32,c.r[5].wrapping_add(0x11a8));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x5efc37); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5efc51);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x1290),32));
            c.set(1,32,c.r[5].wrapping_add(0x1290));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5efc60); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5efc79);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x1378),32));
            c.set(1,32,c.r[5].wrapping_add(0x1378));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5efc88); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de56u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5efca1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x1460),32));
            c.set(1,32,c.r[5].wrapping_add(0x1460));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5efcb0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,0x69a35cu32);
            { let a = c.get(5,32); let r = c.add(a,0x1548u32,32); c.set(5,32,r); }
            c.set(3,32,0x4u32);
            pc = 0x5efcc0; continue;
            }
            0x5efcc0 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x5efcc8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5efcd2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x130),32); c.push(m, 0x5efcdf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5efce8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.add(a,0x20u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5efcc0; continue; }
            pc = 0x5efcf7; continue;
            }
            0x5efcf7 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x0u32); }
            c.push(m, 0x5efd05);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5efd16);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_dialog_draw_goods_wanted_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5f0430;
    loop {
        match pc {
            0x5f0430 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x667e56u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6a5da8u32,32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x0u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.get(3,32); let r = c.add(a,0x32u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x12cu32,32); c.set(7,32,r); }
            c.push(m, 0x5f0482);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0x15c8));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5f048f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x420c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15fa),8));
            c.set(2,32,rm(m,0x6a709cu32,32));
            c.push(m,0xff000000u32);
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,0x10u32);
            if c.cond(7) { pc = 0x5f04af; continue; }
            pc = 0x5f04a9; continue;
            }
            0x5f04a9 => {
            c.set(2,32,rm(m,0x6a70a0u32,32));
            pc = 0x5f04af; continue;
            }
            0x5f04af => {
            { let a = c.get(0,32); let r = c.sub(a,0x64u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.set(1,32,c.get(4,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5f04c2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5f04c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x420a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(7,32,c.get(0,32));
            c.push(m,0xff000000u32);
            { let a = c.get(7,32); let r = c.add(a,0x8u32,32); c.set(7,32,r); }
            c.push(m, 0x5f04d8);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6a0464u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5f04e5);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x5f04ec);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x699874u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xd28),32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(5,32,c.r[6].wrapping_add(0xd28));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5f0508); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x5f0516); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x5f0522);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6998a0u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x438),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(5,32,c.r[6].wrapping_add(0x438));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5f053d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5f0549); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x5f0551);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            c.push(m, 0x5f0558);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15fa),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            if c.cond(6) { pc = 0x5f064e; continue; }
            pc = 0x5f056e; continue;
            }
            0x5f056e => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x15ea),16); c.sub(a,0x18u32,16); }
            if c.cond(3) { pc = 0x5f0636; continue; }
            pc = 0x5f0587; continue;
            }
            0x5f0587 => {
            c.push(m,0xffffffffu32);
            c.push(m, 0x5f058e);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,rm(m,0x6cc380u32,32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x15ea),16));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5f05b1; continue; }
            pc = 0x5f05ab; continue;
            }
            0x5f05ab => {
            c.set(7,32,rm(m,0x6cc384u32,32));
            pc = 0x5f05b1; continue;
            }
            0x5f05b1 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5f05bf);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5f05c8);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(5,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0xd28),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0xd28));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x60),32); c.push(m, 0x5f05e8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5f05fe); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x24),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5f060c);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x5f0616);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x15ea),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5f0633);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            pc = 0x5f0636; continue;
            }
            0x5f0636 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x15fa),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x5f056e; continue; }
            pc = 0x5f064e; continue;
            }
            0x5f064e => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5f065f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn shipyard_parchment_dialog_show(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5f3ad0;
    loop {
        match pc {
            0x5f3ad0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x48u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x5f3ae3);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5f3aed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x60),32); c.push(m, 0x5f3afd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x5f3b0d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
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
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[6],32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[7].wrapping_add(0x64),32); c.push(m, 0x5f3b40); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5f3b47);
            { let args = []; c.r[0] = host.call(m, 0x462390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0xd0));
            c.set(0,8,rm(m,0x66de59u32,8));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5f3b6d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[6].wrapping_add(0x388));
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5f3b93);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x470));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de57u32,8));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5f3bb9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x558));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x5f3bdf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c736cu32);
            c.set(1,32,c.get(5,32));
            { let a = c.r[6].wrapping_add(0x620); wm(m,a,32,0xff404040u32); }
            c.push(m, 0x5f3bf5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c736eu32);
            c.set(1,32,c.r[6].wrapping_add(0x470));
            c.push(m, 0x5f3c05);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5f3c0d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x5f3c1a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xfffffffd));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5f3c2e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x3u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x5f3c45); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5f3c51); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,0xcu32,32); c.set(1,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x1b8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.add(a,0x9au32,32); c.set(2,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5f3c76);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x5f3c82); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5f3c8e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(7,32,c.r[6].wrapping_add(0x2a0));
            { let a = c.get(0,32); let r = c.add(a,0x176u32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5f3ca8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5f3cb4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5f3cc0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc54),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc58),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x558));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5f3cec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5f3cf8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5f3d04); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x6e8),32));
            c.set(7,32,c.r[6].wrapping_add(0x6e8));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5f3d15); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x55555556u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5f3d61);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5f3d6d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5f3d79); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,0x46u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x64u32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x94));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5f3dac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5f3dbc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let r = c.sub(a,0x1eu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5f3de1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x470));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1e));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x5f3e06); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5f3e0e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5f3e16);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5f3e24); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5f3e2f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[6].wrapping_add(0xc80); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.push(m,0x1u32);
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xc88); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc84); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5f3e59); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.set(7,32,c.r[6].wrapping_add(0x640));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc324),32));
            { let a = c.r[6].wrapping_add(0xcb8); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x5f3e8f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52b2c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x694),16));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x68c),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x698),16));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x67c),32));
            { let a = c.r[6].wrapping_add(0xcee); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0xcef); wm(m,a,8,c.get(3,8)); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x688),32));
            { let a = c.r[6].wrapping_add(0xcf8); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x684),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.r[6].wrapping_add(0xcec); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x680),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.r[6].wrapping_add(0xcf0); wm(m,a,8,0xau32); }
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.r[6].wrapping_add(0xcf4); wm(m,a,32,0x4u32); }
            { let a = c.r[6].wrapping_add(0xd3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xd4c); wm(m,a,32,c.get(1,32)); }
            c.set(3,32,c.r[0].wrapping_add(c.r[5]));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x690),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.r[6].wrapping_add(0xd0c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xcfc); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x688),32));
            { let a = c.r[6].wrapping_add(0xd48); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xd40); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x68c),32));
            { let a = c.r[6].wrapping_add(0xd44); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc88),32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x66deb0),8));
            { let a = c.r[6].wrapping_add(0xd08); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x66debc),8));
            { let a = c.r[6].wrapping_add(0xd10); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x66dec0),8));
            { let a = c.r[6].wrapping_add(0xd11); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x66deb8),8));
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.r[6].wrapping_add(0xd12); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x66deb4),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x644),32));
            { let a = c.r[6].wrapping_add(0xd13); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x648),32));
            { let a = c.r[6].wrapping_add(0xd28); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x64c),32));
            { let a = c.r[6].wrapping_add(0xd34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x668),32));
            { let a = c.r[6].wrapping_add(0xd38); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x650),32));
            { let a = c.r[6].wrapping_add(0xd2c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x66c),32));
            { let a = c.r[6].wrapping_add(0xd14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x674),32));
            { let a = c.r[6].wrapping_add(0xd30); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x670),32));
            { let a = c.r[6].wrapping_add(0xd24); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x678),32));
            { let a = c.r[6].wrapping_add(0xd18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x690),32));
            { let a = c.r[6].wrapping_add(0xd20); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xd1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0xd0c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcdd0u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,0x6dcdd0u32);
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x5f4004); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x50));
            c.set(2,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(8)));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,c.r[0].wrapping_add(0x32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xfffffffd));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x48));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x5f4042); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.set(5,32,c.r[6].wrapping_add(0x890));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5f4059);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let a = c.r[6].wrapping_add(0xcc4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xcc8); wm(m,a,32,c.get(7,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5f4075); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xcde); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,0x6dccf0u32,32));
            { let a = c.r[6].wrapping_add(0xce4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xce8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xc7c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xcc0); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xcbc); wm(m,a,32,c.get(7,32)); }
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
            { let args = []; c.r[0] = host.call(m, 0x639c28, c.r[1], &args); }
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
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "route_stop_goods_dialog_reset" => route_stop_goods_dialog_reset(m, host, regs),
        "route_stop_goods_dialog_open" => route_stop_goods_dialog_open(m, host, regs),
        "auto_trade_goods_dialog_process_row_buttons" => auto_trade_goods_dialog_process_row_buttons(m, host, regs),
        "dialog_parchment_open" => dialog_parchment_open(m, host, regs),
        "merchant_debug_dialog_show_selected_ship" => merchant_debug_dialog_show_selected_ship(m, host, regs),
        "ip_address_prompt_on_open" => ip_address_prompt_on_open(m, host, regs),
        "game_screen_create_side_menus_and_dialogs" => game_screen_create_side_menus_and_dialogs(m, host, regs),
        "merchant_picker_dialog_load_layout" => merchant_picker_dialog_load_layout(m, host, regs),
        "merchant_picker_dialog_refresh_list" => merchant_picker_dialog_refresh_list(m, host, regs),
        "parchment_info_dialog_init_controls" => parchment_info_dialog_init_controls(m, host, regs),
        "choice_list_dialog_load_layout" => choice_list_dialog_load_layout(m, host, regs),
        "choice_list_dialog_on_delete_answer_tail" => choice_list_dialog_on_delete_answer_tail(m, host, regs),
        "hometown_picker_dialog_load_layout" => hometown_picker_dialog_load_layout(m, host, regs),
        "hometown_picker_dialog_open" => hometown_picker_dialog_open(m, host, regs),
        "message_box_screen_init_layout" => message_box_screen_init_layout(m, host, regs),
        "viper_message_box_setup_layout" => viper_message_box_setup_layout(m, host, regs),
        "bath_house_dialog_prerender" => bath_house_dialog_prerender(m, host, regs),
        "production_building_info_dialog_load_layout" => production_building_info_dialog_load_layout(m, host, regs),
        "production_building_info_dialog_prerender" => production_building_info_dialog_prerender(m, host, regs),
        "building_info_popup_load_layout" => building_info_popup_load_layout(m, host, regs),
        "building_parchment_dialog_load_layout" => building_parchment_dialog_load_layout(m, host, regs),
        "building_parchment_dialog_prerender" => building_parchment_dialog_prerender(m, host, regs),
        "militia_training_dialog_load_layout" => militia_training_dialog_load_layout(m, host, regs),
        "bath_house_bribe_dialog_load_layout" => bath_house_bribe_dialog_load_layout(m, host, regs),
        "money_lender_bank_dialog_load_layout" => money_lender_bank_dialog_load_layout(m, host, regs),
        "town_info_popup_load_layout" => town_info_popup_load_layout(m, host, regs),
        "expedition_dialog_load_layout" => expedition_dialog_load_layout(m, host, regs),
        "trade_offer_dialog_load_layout" => trade_offer_dialog_load_layout(m, host, regs),
        "siege_dialog_show" => siege_dialog_show(m, host, regs),
        "siege_dialog_load_layout" => siege_dialog_load_layout(m, host, regs),
        "siege_dialog_draw_goods_wanted_page" => siege_dialog_draw_goods_wanted_page(m, host, regs),
        "shipyard_parchment_dialog_show" => shipyard_parchment_dialog_show(m, host, regs),
        "shipyard_parchment_dialog_update_rename_page" => shipyard_parchment_dialog_update_rename_page(m, host, regs),
        _ => return None,
    })
}
