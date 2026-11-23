//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn tutorial_window_place_and_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x637e20;
    loop {
        match pc {
            0x637e20 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x637e34); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x290),32));
            c.set(7,32,c.r[6].wrapping_add(0xb0));
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(3,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x637e55); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(4));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x284),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x94));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x637e81);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x462520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x637e88); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x280),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x637ea2; continue; }
            pc = 0x637e9d; continue;
            }
            0x637e9d => {
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            pc = 0x637ea5; continue;
            }
            0x637ea2 => {
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x637ea5; continue;
            }
            0x637ea5 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,0x6dcce4u32,32));
            c.set(0,32,0x92492493u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(5,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(3,32,rm(m,c.r[7],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[3].wrapping_add(0x48),32); c.push(m, 0x637ee0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x637ef8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7],32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x637f09); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x637f28); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x198),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(7,32,c.r[6].wrapping_add(0x198));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[3].wrapping_add(0x44),32); c.push(m, 0x637f4e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x637f5f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x637f7a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x13c),32); c.push(m, 0x637f84); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x637f8f);
            { let args = []; c.r[0] = host.call(m, 0x4438a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x637fc4; continue; }
            pc = 0x637f97; continue;
            }
            0x637f97 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x28c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[6].wrapping_add(0x2a0); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,0x6dccf8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0]).wrapping_add(0x1));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x298),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x29c); wm(m,a,32,c.get(0,32)); }
            pc = 0x637fc4; continue;
            }
            0x637fc4 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x637fcf);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x637fe4; continue; }
            pc = 0x637fd7; continue;
            }
            0x637fd7 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.push(m, 0x637fe4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x593910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x637fe4; continue;
            }
            0x637fe4 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x637fef);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x637ffe; continue; }
            pc = 0x637ff3; continue;
            }
            0x637ff3 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x637ffe);
            { let args = []; c.r[0] = host.call(m, 0x453ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x637ffe; continue;
            }
            0x637ffe => {
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "tutorial_window_place_and_layout" => tutorial_window_place_and_layout(m, host, regs),
        _ => return None,
    })
}
