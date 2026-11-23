//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn text_layout_base_add_segment(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x462c20;
    loop {
        match pc {
            0x462c20 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[7].wrapping_add(0x8),32),32); }
            if c.cond(12) { pc = 0x462c3b; continue; }
            pc = 0x462c32; continue;
            }
            0x462c32 => {
            c.push(m, 0x462c37);
            { let args = []; c.r[0] = host.call(m, 0x463000, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x462c51; continue;
            }
            0x462c3b => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x10),32));
            c.set(0,32,c.r[6].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.get(2,32));
            pc = 0x462c51; continue;
            }
            0x462c51 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[6].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(6,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(5,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x462c9e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x462ccd; continue; }
            pc = 0x462ca9; continue;
            }
            0x462ca9 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x462cbe); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            pc = 0x462ccd; continue;
            }
            0x462ccd => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            if c.cond(4) { pc = 0x462d4e; continue; }
            pc = 0x462cd9; continue;
            }
            0x462cd9 => {
            c.set(1,32,c.r[3].wrapping_add(0x1));
            c.push(m,c.get(1,32));
            c.push(m, 0x462ce2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x462d06; continue; }
            pc = 0x462ceb; continue;
            }
            0x462ceb => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            pc = 0x462cf5; continue;
            }
            0x462cf5 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]),8));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x462cf5; continue; }
            pc = 0x462d06; continue;
            }
            0x462d06 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[7].wrapping_add(c.r[3]); wm(m,a,8,0x0u32); }
            c.push(m, 0x462d12);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x462d2d);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x28),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x462d4e; continue;
            }
            0x462d4e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scrolling_scene_buffer_blit_wrapped(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x586900;
    loop {
        match pc {
            0x586900 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(7,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[6]));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x586941; continue; }
            pc = 0x58691c; continue;
            }
            0x58691c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(5,32),32); c.set(6,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.set(0,32,c.r[2].wrapping_add(c.r[5]));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x58693f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x586900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(6,32,c.get(5,32));
            pc = 0x586941; continue;
            }
            0x586941 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x10),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[3]));
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x586976; continue; }
            pc = 0x58694f; continue;
            }
            0x58694f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(5,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[5]));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x586970);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x586900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(3,32,c.get(5,32));
            pc = 0x586976; continue;
            }
            0x586976 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x586998; continue; }
            pc = 0x58697a; continue;
            }
            0x58697a => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x586998; continue; }
            pc = 0x58697e; continue;
            }
            0x58697e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x586995);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x586998; continue;
            }
            0x586998 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scrolling_scene_buffer_add_dirty_rect_wrapped(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5869a0;
    loop {
        match pc {
            0x5869a0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(c.r[5]));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x5869d8; continue; }
            pc = 0x5869bc; continue;
            }
            0x5869bc => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5869d4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5869a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x5869d8; continue;
            }
            0x5869d8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x10),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[3]));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x586a02; continue; }
            pc = 0x5869e6; continue;
            }
            0x5869e6 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x5869fa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5869a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x586a02; continue;
            }
            0x586a02 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x586a1c; continue; }
            pc = 0x586a06; continue;
            }
            0x586a06 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x586a1c; continue; }
            pc = 0x586a0a; continue;
            }
            0x586a0a => {
            c.set(0,32,c.r[1].wrapping_add(c.r[3]));
            c.set(2,32,c.r[6].wrapping_add(c.r[5]));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x586a19);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4bb130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x586a1c; continue;
            }
            0x586a1c => {
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

pub fn class22_reset(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x587530;
    loop {
        match pc {
            0x587530 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x58753a);
            { let args = []; c.r[0] = host.call(m, 0x58b490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x587541);
            { let args = []; c.r[0] = host.call(m, 0x4b1650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x94));
            c.push(m, 0x58754c);
            { let args = []; c.r[0] = host.call(m, 0x586c20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x3f8));
            c.push(m, 0x587557);
            { let args = []; c.r[0] = host.call(m, 0x5689f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x4b8));
            { let a = c.r[6].wrapping_add(0x41); wm(m,a,8,0x1u32); }
            c.push(m, 0x587566);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x4cc));
            { let a = c.r[6].wrapping_add(0x4d0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4d4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4d8); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x587585);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4fc),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x58759e; continue; }
            pc = 0x58758f; continue;
            }
            0x58758f => {
            c.push(m,c.get(0,32));
            c.push(m, 0x587595);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4fc); wm(m,a,32,c.get(3,32)); }
            pc = 0x58759e; continue;
            }
            0x58759e => {
            c.set(0,32,0xffffu32);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x588); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x58c); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x11u32);
            c.set(0,32,c.r[6].wrapping_add(0x574));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x504); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc640); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc644); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4e8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc5f4); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x50c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x570); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x598); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x599); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x56c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0xc6b2); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x4e4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc6b0); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0xc594); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0xc7b8); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0xc314); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xc318); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xc5f8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x594); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x590); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x58763f);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,0x66a4d4u32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0xc194));
            c.push(m,c.get(1,32));
            { let target = c.get(5,32); c.push(m, 0x587651); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[6].wrapping_add(0x534));
            c.set(0,32,0xffffffu32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0xc18c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc1a4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc190); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc174); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xc178); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xc1a8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xc180); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xc17c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xc184); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0xc1ac); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc1b0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc1b4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc188); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x59c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xc5fc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc7bc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc7c4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc7cc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc7c0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc7c8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc6b4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc6b8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x568); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc1c0); wm(m,a,32,c.get(3,32)); }
            { let target = c.get(5,32); c.push(m, 0x5876ef); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,0x1u32);
            c.set(1,32,c.r[6].wrapping_add(0x4ac));
            { let a = c.r[6].wrapping_add(0xc650); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc654); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4f8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4ec); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4f0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x500); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc648); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc64c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc6c4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc6c8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc31c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc628); wm(m,a,32,0xffu32); }
            { let a = c.r[6].wrapping_add(0xc320); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xcba4); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x587757);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x4b0));
            c.push(m, 0x587762);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x190u32);
            c.set(0,32,c.r[6].wrapping_add(0xc334));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x520); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4f4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4dc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc16c); wm(m,a,32,0x28u32); }
            { let a = c.r[6].wrapping_add(0xc170); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x558); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc1bc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc1b8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc994); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x564); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x528); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x52c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x560); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x5877cc);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x544));
            { let a = c.r[6].wrapping_add(0x525); wm(m,a,8,c.get(3,8)); }
            c.push(m,c.get(1,32));
            { let target = c.get(5,32); c.push(m, 0x5877de); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc6d4),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5877f1; continue; }
            pc = 0x5877e8; continue;
            }
            0x5877e8 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5877ee);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5877f1; continue;
            }
            0x5877f1 => {
            { let a = c.r[6].wrapping_add(0xc324); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,c.r[6].wrapping_add(0xc1c4));
            c.push(m,0x140u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0xc6d4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc6cc); wm(m,a,32,0xb4ffffffu32); }
            { let a = c.r[6].wrapping_add(0x508); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x58781f);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(7,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[0].wrapping_add(0x140));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x587839; continue; }
            pc = 0x58782e; continue;
            }
            0x58782e => {
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,8,0xffu32); }
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x58782e; continue; }
            pc = 0x587839; continue;
            }
            0x587839 => {
            { let a = c.r[6].wrapping_add(0xc304); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x554); wm(m,a,8,c.get(3,8)); }
            c.set(7,32,c.r[6].wrapping_add(0xc608));
            c.set(5,32,0x8u32);
            pc = 0x587851; continue;
            }
            0x587851 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x587871; continue; }
            pc = 0x587857; continue;
            }
            0x587857 => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x587863);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x58786f; continue; }
            pc = 0x587869; continue;
            }
            0x587869 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x58786f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58786f; continue;
            }
            0x58786f => {
            { let a = c.r[7]; wm(m,a,32,c.get(3,32)); }
            pc = 0x587871; continue;
            }
            0x587871 => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x587851; continue; }
            pc = 0x587877; continue;
            }
            0x587877 => {
            c.set(7,32,c.r[6].wrapping_add(0xc6f0));
            c.set(5,32,0x32u32);
            pc = 0x587882; continue;
            }
            0x587882 => {
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x58788e; continue; }
            pc = 0x587888; continue;
            }
            0x587888 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x58788e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58788e; continue;
            }
            0x58788e => {
            { let a = c.r[7]; wm(m,a,32,c.get(3,32)); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x587882; continue; }
            pc = 0x587896; continue;
            }
            0x587896 => {
            c.set(1,32,c.r[6].wrapping_add(0x4c4));
            { let a = c.r[6].wrapping_add(0x510); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x514); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x518); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x5878b3);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x4c8));
            c.push(m, 0x5878be);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[6].wrapping_add(0x418));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x4u32); }
            c.set(7,32,c.get(5,32));
            pc = 0x5878ce; continue;
            }
            0x5878ce => {
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5878da; continue; }
            pc = 0x5878d4; continue;
            }
            0x5878d4 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x5878da); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5878da; continue;
            }
            0x5878da => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.add(a,0x24u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5878ce; continue; }
            pc = 0x5878e8; continue;
            }
            0x5878e8 => {
            c.push(m,0x90u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5878f4);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4b4),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            if c.cond(4) { pc = 0x587912; continue; }
            pc = 0x587903; continue;
            }
            0x587903 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x587909);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4b4); wm(m,a,32,c.get(3,32)); }
            pc = 0x587912; continue;
            }
            0x587912 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x51c),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x587925; continue; }
            pc = 0x58791c; continue;
            }
            0x58791c => {
            c.push(m,c.get(0,32));
            c.push(m, 0x587922);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x587925; continue;
            }
            0x587925 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            { let a = c.r[6].wrapping_add(0x51c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x587944; continue; }
            pc = 0x587935; continue;
            }
            0x587935 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x58793b);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4e0); wm(m,a,32,c.get(3,32)); }
            pc = 0x587944; continue;
            }
            0x587944 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc6ec),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x58795d; continue; }
            pc = 0x58794e; continue;
            }
            0x58794e => {
            c.push(m,c.get(0,32));
            c.push(m, 0x587954);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bbb10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xc6ec); wm(m,a,32,c.get(3,32)); }
            pc = 0x58795d; continue;
            }
            0x58795d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc32c),32));
            { let a = c.r[6].wrapping_add(0xc6d0); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0xc328); wm(m,a,8,0x1u32); }
            if c.cond(4) { pc = 0x587983; continue; }
            pc = 0x587974; continue;
            }
            0x587974 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x58797a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xc32c); wm(m,a,32,c.get(3,32)); }
            pc = 0x587983; continue;
            }
            0x587983 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xc329); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0xc330); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x524); wm(m,a,8,0x1u32); }
            c.push(m, 0x58799e);
            { let args = []; c.r[0] = host.call(m, 0x595c10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0xcb90); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xcb94); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0xcb9c); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0xcb9d); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0xcb98); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc7dc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc7e0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc7e4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc7e8); wm(m,a,32,c.get(3,32)); }
            c.push(m,0x1a8u32);
            c.set(0,32,c.r[6].wrapping_add(0xc7ec));
            { let a = c.r[6].wrapping_add(0xc7d0); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(3,32));
            { let a = c.r[6].wrapping_add(0xc7d4); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0xc7d8); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x5879fb);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x198u32);
            c.set(1,32,c.r[6].wrapping_add(0xc998));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x587a0d);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x60u32);
            c.set(2,32,c.r[6].wrapping_add(0xcb30));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x587a1c);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x5a0));
            c.push(m, 0x587a2a);
            { let args = []; c.r[0] = host.call(m, 0x59dbe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x587a3a; continue; }
            pc = 0x587a34; continue;
            }
            0x587a34 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x587a3a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x587a3a; continue;
            }
            0x587a3a => {
            { let a = c.r[6].wrapping_add(0x530); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc634),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x587a53; continue; }
            pc = 0x587a4a; continue;
            }
            0x587a4a => {
            c.push(m,c.get(0,32));
            c.push(m, 0x587a50);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x587a53; continue;
            }
            0x587a53 => {
            { let a = c.r[6].wrapping_add(0xc634); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc638); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0xc63a); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0xc630); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0xc631); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0xc58c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc590); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc63c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0xc6bc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc6c0); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn class22_resize_view(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x589ed0;
    loop {
        match pc {
            0x589ed0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[5],32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x589ee9; continue; }
            pc = 0x589ee6; continue;
            }
            0x589ee6 => {
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            pc = 0x589ee9; continue;
            }
            0x589ee9 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = rm(m,c.r[7],32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x589ef6; continue; }
            pc = 0x589ef4; continue;
            }
            0x589ef4 => {
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            pc = 0x589ef6; continue;
            }
            0x589ef6 => {
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(3,32));
            c.fild64(m,c.r[4].wrapping_add(0x10));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fild32(m,c.r[6].wrapping_add(0x510));
            c.fstp32(m,c.r[4].wrapping_add(0x30));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x30),4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m, 0x589f2b);
            { let args = []; c.r[0] = host.call(m, 0x63a5ce, c.r[1], &args); }
            c.fret();
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m, 0x589f30);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7],32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x514),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.fild64(m,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(3,32,c.get(0,32));
            c.fstp32(m,c.r[4].wrapping_add(0x30));
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x30),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m, 0x589f6d);
            { let args = []; c.r[0] = host.call(m, 0x63a5ce, c.r[1], &args); }
            c.fret();
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x589f75);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc324),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(8) { pc = 0x589f8a; continue; }
            pc = 0x589f83; continue;
            }
            0x589f83 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x589f8a);
            { let args = []; c.r[0] = host.call(m, 0x589130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x589f8a; continue;
            }
            0x589f8a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x589f97);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x589de0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x589fa2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x590ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc31c),32));
            { let a = c.r[5]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc320),32));
            c.set(1,32,c.r[6].wrapping_add(0x5a0));
            { let a = c.r[7]; wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x589fbe);
            { let args = []; c.r[0] = host.call(m, 0x59dbe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x589fc9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc328),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x589fe4; continue; }
            pc = 0x589fd3; continue;
            }
            0x589fd3 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x589fda);
            { let args = []; c.r[0] = host.call(m, 0x58f1b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x50c); wm(m,a,32,0x0u32); }
            pc = 0x589fe4; continue;
            }
            0x589fe4 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc318),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            c.fild32(m,c.r[4].wrapping_add(0x24));
            { let a = c.get(7,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(7,32,r); }
            { let a = c.get(5,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(5,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            c.fild32(m,c.r[6].wrapping_add(0x510));
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x24),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x28),4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x24),4));
            c.push(m, 0x58a02a);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x28));
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x24),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x28),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x44u32,8); }
            if c.cond(11) { pc = 0x58a048; continue; }
            pc = 0x58a047; continue;
            }
            0x58a047 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x58a048; continue;
            }
            0x58a048 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x24));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x514),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x24));
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x24),4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x24),4));
            c.fop_st('+',0,0,false);
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x24),4));
            c.push(m, 0x58a09c);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x24));
            c.fstp32(m,c.r[4].wrapping_add(0x24));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x24),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x20),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x44u32,8); }
            c.set(0,32,c.r[1].wrapping_add(0x1));
            if c.cond(10) { pc = 0x58a0bf; continue; }
            pc = 0x58a0bd; continue;
            }
            0x58a0bd => {
            c.set(0,32,c.get(1,32));
            pc = 0x58a0bf; continue;
            }
            0x58a0bf => {
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x58a0cf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x590ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.push(m, 0x58a0da);
            { let args = []; c.r[0] = host.call(m, 0x59ceb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x58a0ea); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[6].wrapping_add(0xc648); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0xc64c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[6].wrapping_add(0xc5f4); wm(m,a,8,0x0u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58a138);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x58a143); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc324),16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6e6220u32);
            c.push(m, 0x58a157);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6141d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0xffffu32);
            c.push(m, 0x58a167);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42a910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcb90),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58a186; continue; }
            pc = 0x58a171; continue;
            }
            0x58a171 => {
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x58a17f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4430d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xcb94); wm(m,a,8,0x0u32); }
            pc = 0x58a186; continue;
            }
            0x58a186 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x514),32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x510),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(0x1));
            { let a = c.r[6].wrapping_add(0xc650); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x4d4),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc594),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0xc654); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x58a1d5; continue; }
            pc = 0x58a1ce; continue;
            }
            0x58a1ce => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a1d5);
            { let args = []; c.r[0] = host.call(m, 0x58a920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x58a1d5; continue;
            }
            0x58a1d5 => {
            c.set(7,32,c.r[6].wrapping_add(0xc1c4));
            c.push(m,0x140u32);
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x58a1e8);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(7,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[0].wrapping_add(0x140));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x58a204; continue; }
            pc = 0x58a1f7; continue;
            }
            0x58a1f7 => {
            { let a = c.get(2,8); let r = c.or(a,0xffu32,8); c.set(2,8,r); }
            pc = 0x58a1fa; continue;
            }
            0x58a1fa => {
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x58a1fa; continue; }
            pc = 0x58a204; continue;
            }
            0x58a204 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a20b);
            { let args = []; c.r[0] = host.call(m, 0x58a240, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a212);
            { let args = []; c.r[0] = host.call(m, 0x59b410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            { let a = c.r[6].wrapping_add(0xc648); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xc64c); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x58a22f);
            { let args = []; c.r[0] = host.call(m, 0x57ae10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn class22_update_scene_buffer_cells_and_animations(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x592000;
    loop {
        match pc {
            0x592000 => {
            { let a = c.get(4,32); let r = c.sub(a,0x40u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x38));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fild32(m,c.r[6].wrapping_add(0x510));
            c.fstp32(m,c.r[4].wrapping_add(0x34));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x34),4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            c.push(m, 0x59203d);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0xc));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x38),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x44u32,8); }
            if c.cond(11) { pc = 0x59205c; continue; }
            pc = 0x592058; continue;
            }
            0x592058 => {
            { let a = rm(m,c.r[4].wrapping_add(0xc),32); let r = c.inc(a,32); { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x59205c; continue;
            }
            0x59205c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x514),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.fild32(m,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x34));
            c.fild32(m,c.r[4].wrapping_add(0x38));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x34),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x38),4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            c.fop_st('+',0,0,false);
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            c.push(m, 0x5920a5);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x38));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x44u32,8); }
            if c.cond(11) { pc = 0x5920c3; continue; }
            pc = 0x5920c2; continue;
            }
            0x5920c2 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x5920c3; continue;
            }
            0x5920c3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0xc640),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x6e338cu32,8));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x1u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            if c.cond(15) { pc = 0x592776; continue; }
            pc = 0x5920ff; continue;
            }
            0x5920ff => {
            c.set(2,32,c.get(0,32));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc308));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            pc = 0x59210c; continue;
            }
            0x59210c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(3,32); let r = c.add(a,c.get(5,32),32); c.set(3,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0x560); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(7,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(1,32)); }
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x592195; continue; }
            pc = 0x592145; continue;
            }
            0x592145 => {
            { let a = rm(m,c.r[1].wrapping_add(0x2),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x592195; continue; }
            pc = 0x59214b; continue;
            }
            0x59214b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x2),8));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x59216d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x592183; continue; }
            pc = 0x59217b; continue;
            }
            0x59217b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            pc = 0x592185; continue;
            }
            0x592183 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x592185; continue;
            }
            0x592185 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x592195); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x592195; continue;
            }
            0x592195 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xfffffffc));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x59225a; continue; }
            pc = 0x5921ba; continue;
            }
            0x5921ba => {
            c.set(1,16,rm(m,c.r[0],16));
            { let a = c.get(1,16); c.and(a,c.get(1,16),16); }
            if c.cond(4) { pc = 0x592235; continue; }
            pc = 0x5921c2; continue;
            }
            0x5921c2 => {
            { let a = rm(m,c.r[0].wrapping_add(0x2),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x592235; continue; }
            pc = 0x5921c8; continue;
            }
            0x5921c8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x5921e2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x592235; continue; }
            pc = 0x5921eb; continue;
            }
            0x5921eb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7],16));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x2),8));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x59220d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x592223; continue; }
            pc = 0x59221b; continue;
            }
            0x59221b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            pc = 0x592225; continue;
            }
            0x592223 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x592225; continue;
            }
            0x592225 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x592235); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x592235; continue;
            }
            0x592235 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x510),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(7,32)); }
            if c.cond(2) { pc = 0x5921ba; continue; }
            pc = 0x59225a; continue;
            }
            0x59225a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.set(2,32,c.r[7].wrapping_add(0xfffffffd));
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x5923d0; continue; }
            pc = 0x59229e; continue;
            }
            0x59229e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x592326; continue; }
            pc = 0x5922b1; continue;
            }
            0x5922b1 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x592311; continue; }
            pc = 0x5922bf; continue;
            }
            0x5922bf => {
            { let a = rm(m,c.r[1].wrapping_add(0x2),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x592311; continue; }
            pc = 0x5922c5; continue;
            }
            0x5922c5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x2),8));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x5922e8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x5922fe; continue; }
            pc = 0x5922f6; continue;
            }
            0x5922f6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            pc = 0x592300; continue;
            }
            0x5922fe => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x592300; continue;
            }
            0x592300 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x592311); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x592311; continue;
            }
            0x592311 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x510),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(1,32)); }
            pc = 0x592326; continue;
            }
            0x592326 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(3) { pc = 0x592388; continue; }
            pc = 0x59233f; continue;
            }
            0x59233f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x59236d; continue; }
            pc = 0x59234d; continue;
            }
            0x59234d => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x2),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x59236d; continue; }
            pc = 0x592354; continue;
            }
            0x592354 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x59236d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x59236d; continue;
            }
            0x59236d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x510),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x59233f; continue; }
            pc = 0x592388; continue;
            }
            0x592388 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,0x1u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = rm(m,c.r[6].wrapping_add(0x55c),32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x59229e; continue; }
            pc = 0x5923c8; continue;
            }
            0x5923c8 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            pc = 0x5923d0; continue;
            }
            0x5923d0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x500),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc318),32));
            { let a = c.r[6].wrapping_add(0x4f8); wm(m,a,32,0x0u32); }
            c.set(3,32,c.r[2].wrapping_add(c.r[1]));
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x592401; continue; }
            pc = 0x5923f5; continue;
            }
            0x5923f5 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            pc = 0x592407; continue;
            }
            0x592401 => {
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            pc = 0x592407; continue;
            }
            0x592407 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x5926e9; continue; }
            pc = 0x592422; continue;
            }
            0x592422 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x59243f; continue; }
            pc = 0x592439; continue;
            }
            0x592439 => {
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            pc = 0x59243f; continue;
            }
            0x59243f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0xffu32); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x5926a3; continue; }
            pc = 0x59245d; continue;
            }
            0x59245d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x5924be; continue; }
            pc = 0x59246b; continue;
            }
            0x59246b => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x12),8));
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x2),8));
            { let a = c.get(3,8); c.sub(a,c.get(2,8),8); }
            if c.cond(3) { pc = 0x5924be; continue; }
            pc = 0x592476; continue;
            }
            0x592476 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[2].wrapping_add(0x2),8));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.get(7,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x592494); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x5924a8; continue; }
            pc = 0x5924a0; continue;
            }
            0x5924a0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            pc = 0x5924aa; continue;
            }
            0x5924a8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5924aa; continue;
            }
            0x5924aa => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x5924be); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x5924be; continue;
            }
            0x5924be => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            if c.cond(7) { pc = 0x5924e2; continue; }
            pc = 0x5924de; continue;
            }
            0x5924de => {
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(1,8)); }
            pc = 0x5924e2; continue;
            }
            0x5924e2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            if c.cond(14) { pc = 0x592676; continue; }
            pc = 0x5924f1; continue;
            }
            0x5924f1 => {
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            pc = 0x5924f5; continue;
            }
            0x5924f5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x514),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2),8));
            c.set(7,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(3,32,c.r[3].wrapping_add(c.r[2]).wrapping_add(0x1));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x592667; continue; }
            pc = 0x592520; continue;
            }
            0x592520 => {
            c.set(0,8,c.get(1,8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x592667; continue; }
            pc = 0x59252a; continue;
            }
            0x59252a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x592537);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x590a20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x592567; continue; }
            pc = 0x592544; continue;
            }
            0x592544 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8));
            pc = 0x592551; continue;
            }
            0x592551 => {
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x59255f; continue; }
            pc = 0x592555; continue;
            }
            0x592555 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x14u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x592551; continue; }
            pc = 0x59255d; continue;
            }
            0x59255d => {
            pc = 0x592567; continue;
            }
            0x59255f => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x592667; continue; }
            pc = 0x592567; continue;
            }
            0x592567 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4dc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5925c4; continue; }
            pc = 0x592577; continue;
            }
            0x592577 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4dc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x592593);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4dc),32));
            { let a = c.r[6].wrapping_add(0x4e0); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0xffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5925c1);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x5925c4; continue;
            }
            0x5925c4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x48),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc); wm(m,a,16,c.get(1,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x11); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xe); wm(m,a,16,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4f8); wm(m,a,32,c.get(0,32)); }
            pc = 0x592667; continue;
            }
            0x592667 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5924f5; continue; }
            pc = 0x592676; continue;
            }
            0x592676 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x510),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(0,32,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            if c.cond(2) { pc = 0x59245d; continue; }
            pc = 0x5926a3; continue;
            }
            0x5926a3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x55c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc314),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            if c.cond(2) { pc = 0x592422; continue; }
            pc = 0x5926e9; continue;
            }
            0x5926e9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4f8),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4e0),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x592759; continue; }
            pc = 0x592703; continue;
            }
            0x592703 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x55c); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(3,8,rm(m,c.r[7].wrapping_add(0x10),8));
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,c.get(5,32));
            c.set(0,32,rm(m,c.r[5],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x592726); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x59273a; continue; }
            pc = 0x592732; continue;
            }
            0x592732 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            pc = 0x59273c; continue;
            }
            0x59273a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x59273c; continue;
            }
            0x59273c => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x59274e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x592703; continue; }
            pc = 0x592759; continue;
            }
            0x592759 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x59210c; continue; }
            pc = 0x592776; continue;
            }
            0x592776 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x5927b3; continue; }
            pc = 0x592784; continue;
            }
            0x592784 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2a0),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5927a2; continue; }
            pc = 0x59279d; continue;
            }
            0x59279d => {
            { let a = c.get(1,32); let r = c.add(a,0xffffffc4u32,32); c.set(1,32,r); }
            pc = 0x5927a4; continue;
            }
            0x5927a2 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x5927a4; continue;
            }
            0x5927a4 => {
            { let a = c.r[1].wrapping_add(0x5d); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x592784; continue; }
            pc = 0x5927b3; continue;
            }
            0x5927b3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x59285c; continue; }
            pc = 0x5927c7; continue;
            }
            0x5927c7 => {
            c.set(0,32,rm(m,0x6e51b4u32,32));
            pc = 0x5927cc; continue;
            }
            0x5927cc => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2a0),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5927ef; continue; }
            pc = 0x5927ea; continue;
            }
            0x5927ea => {
            c.set(1,32,c.r[0].wrapping_add(0xffffffc4));
            pc = 0x5927f1; continue;
            }
            0x5927ef => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x5927f1; continue;
            }
            0x5927f1 => {
            { let a = rm(m,0x6e51b4u32,16); let r = c.inc(a,16); { let a = 0x6e51b4u32; wm(m,a,16,r); } }
            c.set(0,32,rm(m,0x6e51b4u32,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x59281b; continue; }
            pc = 0x59280f; continue;
            }
            0x59280f => {
            { let a = 0x6e51b4u32; wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,0x6e51b4u32,32));
            pc = 0x59281b; continue;
            }
            0x59281b => {
            c.set(7,16,rm(m,c.r[1].wrapping_add(0x68),16));
            { let a = c.get(7,16); c.sub(a,c.get(2,16),16); }
            if c.cond(4) { pc = 0x592841; continue; }
            pc = 0x592824; continue;
            }
            0x592824 => {
            c.push(m, 0x592829);
            { let args = []; c.r[0] = host.call(m, 0x583080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x59283a; continue; }
            pc = 0x59282d; continue;
            }
            0x59282d => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x11u32,32); }
            if c.cond(13) { pc = 0x59285a; continue; }
            pc = 0x59283a; continue;
            }
            0x59283a => {
            c.set(0,32,rm(m,0x6e51b4u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x592841; continue;
            }
            0x592841 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x5927cc; continue; }
            pc = 0x592858; continue;
            }
            0x592858 => {
            pc = 0x59285c; continue;
            }
            0x59285a => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x59285c; continue;
            }
            0x59285c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(14) { pc = 0x592899; continue; }
            pc = 0x59286a; continue;
            }
            0x59286a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2a0),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x592888; continue; }
            pc = 0x592883; continue;
            }
            0x592883 => {
            { let a = c.get(1,32); let r = c.add(a,0xffffffc4u32,32); c.set(1,32,r); }
            pc = 0x59288a; continue;
            }
            0x592888 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x59288a; continue;
            }
            0x59288a => {
            { let a = c.r[1].wrapping_add(0x68); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x59286a; continue; }
            pc = 0x592899; continue;
            }
            0x592899 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn class22_set_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x598700;
    loop {
        match pc {
            0x598700 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x524),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x59872a; continue; }
            pc = 0x598712; continue;
            }
            0x598712 => {
            c.set(0,32,rm(m,0x6dcce4u32,32));
            c.set(1,32,rm(m,0x6dcce0u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x5a0));
            c.push(m, 0x59872a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x59da40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x59872a; continue;
            }
            0x59872a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc608),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5987a8; continue; }
            pc = 0x598734; continue;
            }
            0x598734 => {
            c.set(2,32,rm(m,0x6dcce0u32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x1eu32); }
            { let a = c.get(2,32); let r = c.add(a,0xffffffecu32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x59875b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dcce0u32,32));
            c.set(7,32,c.r[6].wrapping_add(0xc60c));
            { let a = c.get(0,32); let r = c.add(a,0xffffffccu32,32); c.set(0,32,r); }
            c.set(3,32,0x6u32);
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x598772; continue;
            }
            0x598772 => {
            c.set(1,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x59877e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x27u32,32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x598772; continue; }
            pc = 0x59878f; continue;
            }
            0x59878f => {
            c.set(3,32,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            { let a = c.get(3,32); let r = c.add(a,0x27u32,32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc624),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5987a8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5987a8; continue;
            }
            0x5987a8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[1],32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[7].wrapping_add(0xc),32); c.push(m, 0x5987d2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn class22_apply_display_options(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x599e40;
    loop {
        match pc {
            0x599e40 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x664ec0u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x4f0); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.r[6].wrapping_add(0xc328); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(7,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc32c),32));
            { let a = c.r[6].wrapping_add(0x4ec); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x599efa; continue; }
            pc = 0x599e8d; continue;
            }
            0x599e8d => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x599ef1; continue; }
            pc = 0x599e93; continue;
            }
            0x599e93 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x599ea3; continue; }
            pc = 0x599e9d; continue;
            }
            0x599e9d => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x599ea3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x599ea3; continue;
            }
            0x599ea3 => {
            c.push(m,0xcu32);
            c.push(m, 0x599eaa);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x599ec2; continue; }
            pc = 0x599eb9; continue;
            }
            0x599eb9 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x599ec0);
            { let args = []; c.r[0] = host.call(m, 0x585f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x599ec4; continue;
            }
            0x599ec2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x599ec4; continue;
            }
            0x599ec4 => {
            { let a = c.r[6].wrapping_add(0x530); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc32c),32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x599ede);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xc32c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x508); wm(m,a,32,0x1u32); }
            pc = 0x599ef1; continue;
            }
            0x599ef1 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x599ef8);
            { let args = []; c.r[0] = host.call(m, 0x59b410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x599f59; continue;
            }
            0x599efa => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x599f59; continue; }
            pc = 0x599efe; continue;
            }
            0x599efe => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x599f0e; continue; }
            pc = 0x599f08; continue;
            }
            0x599f08 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x599f0e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x599f0e; continue;
            }
            0x599f0e => {
            c.push(m,0x48u32);
            c.push(m, 0x599f15);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,0x1u32);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x599f32; continue; }
            pc = 0x599f29; continue;
            }
            0x599f29 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x599f30);
            { let args = []; c.r[0] = host.call(m, 0x586030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x599f34; continue;
            }
            0x599f32 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x599f34; continue;
            }
            0x599f34 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[6].wrapping_add(0x530); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x599f49);
            { let args = []; c.r[0] = host.call(m, 0x58f1b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x50c); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x508); wm(m,a,32,c.get(7,32)); }
            pc = 0x599f59; continue;
            }
            0x599f59 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xc304); wm(m,a,8,0x1u32); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x599f77; continue; }
            pc = 0x599f6c; continue;
            }
            0x599f6c => {
            c.set(0,32,rm(m,0x6dcce0u32,32));
            c.set(1,32,rm(m,0x6dcce4u32,32));
            pc = 0x599f77; continue;
            }
            0x599f77 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x5a0));
            c.push(m, 0x599f84);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x59da40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc),32); c.push(m, 0x599f9f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            { let a = c.r[6].wrapping_add(0x524); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x599fc7; continue; }
            pc = 0x599fad; continue;
            }
            0x599fad => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc594),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x599fe5; continue; }
            pc = 0x599fb7; continue;
            }
            0x599fb7 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xc594); wm(m,a,8,0x0u32); }
            c.push(m, 0x599fc5);
            { let args = []; c.r[0] = host.call(m, 0x58b5f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x599fe5; continue;
            }
            0x599fc7 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xc594),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x599fe5; continue; }
            pc = 0x599fd1; continue;
            }
            0x599fd1 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xc594); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x599fde);
            { let args = []; c.r[0] = host.call(m, 0x58a920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x599fe5);
            { let args = []; c.r[0] = host.call(m, 0x58b1b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x599fe5; continue;
            }
            0x599fe5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x599fef);
            { let args = []; c.r[0] = host.call(m, 0x581aa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6e4778u32; wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x59a03b; continue; }
            pc = 0x59a008; continue;
            }
            0x59a008 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2a0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(2)),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x59a029; continue; }
            pc = 0x59a024; continue;
            }
            0x59a024 => {
            c.set(1,32,c.r[0].wrapping_add(0xffffffc4));
            pc = 0x59a02b; continue;
            }
            0x59a029 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x59a02b; continue;
            }
            0x59a02b => {
            c.push(m, 0x59a030);
            { let args = []; c.r[0] = host.call(m, 0x5834c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x59a008; continue; }
            pc = 0x59a03b; continue;
            }
            0x59a03b => {
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            c.set(7,32,0x2u32);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let a = rm(m,c.r[2].wrapping_add(0x190),32); c.sub(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(13)));
            { let a = c.r[6].wrapping_add(0xc329); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x59a05e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58f4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x16c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,8,u32::from(c.cond(13)));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x6c4010u32; wm(m,a,8,c.get(2,8)); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(32); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_gl_cfg_and_init_graphics(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bc010;
    loop {
        match pc {
            0x4bc010 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661d5du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,0x6bec68u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4bc047);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,rm(m,0x6bec70u32,32));
            c.push(m,0x6bedb6u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.push(m, 0x4bc061);
            { let args = []; c.r[0] = host.call(m, 0x63bece, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc105; continue; }
            pc = 0x4bc06e; continue;
            }
            0x4bc06e => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(5) { pc = 0x4bc0f4; continue; }
            pc = 0x4bc077; continue;
            }
            0x4bc077 => {
            c.push(m,c.get(6,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            { let a = 0x6dbb58u32; wm(m,a,8,0x0u32); }
            c.push(m, 0x4bc08e);
            { let args = []; c.r[0] = host.call(m, 0x63be3b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,0x6dbb58u32,8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bc0ea; continue; }
            pc = 0x4bc09a; continue;
            }
            0x4bc09a => {
            c.push(m,0x6beda0u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4bc0a9);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc0d1; continue; }
            pc = 0x4bc0b0; continue;
            }
            0x4bc0b0 => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4bc0b7);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc0d5; continue; }
            pc = 0x4bc0be; continue;
            }
            0x4bc0be => {
            c.push(m,0x6beda8u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc0ca);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc0b0; continue; }
            pc = 0x4bc0d1; continue;
            }
            0x4bc0d1 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4bc0ea; continue;
            }
            0x4bc0d5 => {
            c.push(m,0x6bedb0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc0e1);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bc0f2; continue; }
            pc = 0x4bc0ea; continue;
            }
            0x4bc0ea => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x4bc077; continue; }
            pc = 0x4bc0f0; continue;
            }
            0x4bc0f0 => {
            pc = 0x4bc0f4; continue;
            }
            0x4bc0f2 => {
            c.set(3,8,0x1u32);
            pc = 0x4bc0f4; continue;
            }
            0x4bc0f4 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4bc0fa);
            { let args = []; c.r[0] = host.call(m, 0x63bdbe, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bc105; continue; }
            pc = 0x4bc101; continue;
            }
            0x4bc101 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bc10e; continue; }
            pc = 0x4bc105; continue;
            }
            0x4bc105 => {
            c.set(1,32,rm(m,0x6beec4u32,32));
            c.push(m,c.get(1,32));
            pc = 0x4bc10f; continue;
            }
            0x4bc10e => {
            c.push(m,c.get(7,32));
            pc = 0x4bc10f; continue;
            }
            0x4bc10f => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4bc11a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6bef00u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bc130);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x2u32); }
            c.push(m, 0x4bc13f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x1u32); }
            c.push(m, 0x4bc14d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,0x6bedb6u32);
            { let a = c.r[3].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            c.set(5,32,rm(m,0x6bed10u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4bc167);
            { let args = []; c.r[0] = host.call(m, 0x63bece, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4bc268; continue; }
            pc = 0x4bc174; continue;
            }
            0x4bc174 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(5) { pc = 0x4bc204; continue; }
            pc = 0x4bc184; continue;
            }
            0x4bc184 => {
            c.push(m,c.get(6,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            { let a = 0x6dbb58u32; wm(m,a,8,0x0u32); }
            c.push(m, 0x4bc19b);
            { let args = []; c.r[0] = host.call(m, 0x63be3b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,0x6dbb58u32,8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bc1f7; continue; }
            pc = 0x4bc1a7; continue;
            }
            0x4bc1a7 => {
            c.push(m,0x6beda0u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4bc1b6);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc1de; continue; }
            pc = 0x4bc1bd; continue;
            }
            0x4bc1bd => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4bc1c4);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc1e2; continue; }
            pc = 0x4bc1cb; continue;
            }
            0x4bc1cb => {
            c.push(m,0x6beda8u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc1d7);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc1bd; continue; }
            pc = 0x4bc1de; continue;
            }
            0x4bc1de => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4bc1f7; continue;
            }
            0x4bc1e2 => {
            c.push(m,0x6bedb0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc1ee);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bc1ff; continue; }
            pc = 0x4bc1f7; continue;
            }
            0x4bc1f7 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x4bc184; continue; }
            pc = 0x4bc1fd; continue;
            }
            0x4bc1fd => {
            pc = 0x4bc204; continue;
            }
            0x4bc1ff => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x4bc204; continue;
            }
            0x4bc204 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4bc20a);
            { let args = []; c.r[0] = host.call(m, 0x63bdbe, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x17),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bc268; continue; }
            pc = 0x4bc215; continue;
            }
            0x4bc215 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc268; continue; }
            pc = 0x4bc219; continue;
            }
            0x4bc219 => {
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m,0x6bedb8u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4bc229);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,0x1u32);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4bc245; continue; }
            pc = 0x4bc235; continue;
            }
            0x4bc235 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.r[3].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            pc = 0x4bc268; continue;
            }
            0x4bc245 => {
            c.set(6,32,0x6becccu32);
            pc = 0x4bc24a; continue;
            }
            0x4bc24a => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4bc253);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc25d; continue; }
            pc = 0x4bc25a; continue;
            }
            0x4bc25a => {
            { let a = c.r[3].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            pc = 0x4bc25d; continue;
            }
            0x4bc25d => {
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x6becdcu32,32); }
            if c.cond(12) { pc = 0x4bc24a; continue; }
            pc = 0x4bc268; continue;
            }
            0x4bc268 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,0x6bedb6u32);
            { let a = c.r[3].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            c.set(5,32,rm(m,0x6bed20u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bc282);
            { let args = []; c.r[0] = host.call(m, 0x63bece, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4bc37d; continue; }
            pc = 0x4bc28f; continue;
            }
            0x4bc28f => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(5) { pc = 0x4bc315; continue; }
            pc = 0x4bc298; continue;
            }
            0x4bc298 => {
            c.push(m,c.get(6,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            { let a = 0x6dbb58u32; wm(m,a,8,0x0u32); }
            c.push(m, 0x4bc2af);
            { let args = []; c.r[0] = host.call(m, 0x63be3b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,0x6dbb58u32,8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bc30b; continue; }
            pc = 0x4bc2bb; continue;
            }
            0x4bc2bb => {
            c.push(m,0x6beda0u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4bc2ca);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc2f2; continue; }
            pc = 0x4bc2d1; continue;
            }
            0x4bc2d1 => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4bc2d8);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc2f6; continue; }
            pc = 0x4bc2df; continue;
            }
            0x4bc2df => {
            c.push(m,0x6beda8u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc2eb);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc2d1; continue; }
            pc = 0x4bc2f2; continue;
            }
            0x4bc2f2 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4bc30b; continue;
            }
            0x4bc2f6 => {
            c.push(m,0x6bedb0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc302);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bc313; continue; }
            pc = 0x4bc30b; continue;
            }
            0x4bc30b => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x4bc298; continue; }
            pc = 0x4bc311; continue;
            }
            0x4bc311 => {
            pc = 0x4bc315; continue;
            }
            0x4bc313 => {
            c.set(3,8,0x1u32);
            pc = 0x4bc315; continue;
            }
            0x4bc315 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4bc31b);
            { let args = []; c.r[0] = host.call(m, 0x63bdbe, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bc37d; continue; }
            pc = 0x4bc322; continue;
            }
            0x4bc322 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc37d; continue; }
            pc = 0x4bc326; continue;
            }
            0x4bc326 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,0x6bedbcu32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4bc336);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,0x1u32);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4bc356; continue; }
            pc = 0x4bc342; continue;
            }
            0x4bc342 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.r[0].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bc37d; continue;
            }
            0x4bc356 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(6,32,0x6becccu32);
            pc = 0x4bc35f; continue;
            }
            0x4bc35f => {
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bc368);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc372; continue; }
            pc = 0x4bc36f; continue;
            }
            0x4bc36f => {
            { let a = c.r[3].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            pc = 0x4bc372; continue;
            }
            0x4bc372 => {
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x6becdcu32,32); }
            if c.cond(12) { pc = 0x4bc35f; continue; }
            pc = 0x4bc37d; continue;
            }
            0x4bc37d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,0x6bedb6u32);
            { let a = c.r[2].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            c.set(5,32,rm(m,0x6bed2cu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bc39b);
            { let args = []; c.r[0] = host.call(m, 0x63bece, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc494; continue; }
            pc = 0x4bc3a8; continue;
            }
            0x4bc3a8 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(5) { pc = 0x4bc42e; continue; }
            pc = 0x4bc3b1; continue;
            }
            0x4bc3b1 => {
            c.push(m,c.get(6,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            { let a = 0x6dbb58u32; wm(m,a,8,0x0u32); }
            c.push(m, 0x4bc3c8);
            { let args = []; c.r[0] = host.call(m, 0x63be3b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,0x6dbb58u32,8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bc424; continue; }
            pc = 0x4bc3d4; continue;
            }
            0x4bc3d4 => {
            c.push(m,0x6beda0u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4bc3e3);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc40b; continue; }
            pc = 0x4bc3ea; continue;
            }
            0x4bc3ea => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4bc3f1);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bc40f; continue; }
            pc = 0x4bc3f8; continue;
            }
            0x4bc3f8 => {
            c.push(m,0x6beda8u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc404);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc3ea; continue; }
            pc = 0x4bc40b; continue;
            }
            0x4bc40b => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4bc424; continue;
            }
            0x4bc40f => {
            c.push(m,0x6bedb0u32);
            c.push(m,0x0u32);
            c.push(m, 0x4bc41b);
            { let args = []; c.r[0] = host.call(m, 0x63bd1a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bc42c; continue; }
            pc = 0x4bc424; continue;
            }
            0x4bc424 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x4bc3b1; continue; }
            pc = 0x4bc42a; continue;
            }
            0x4bc42a => {
            pc = 0x4bc42e; continue;
            }
            0x4bc42c => {
            c.set(3,8,0x1u32);
            pc = 0x4bc42e; continue;
            }
            0x4bc42e => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4bc434);
            { let args = []; c.r[0] = host.call(m, 0x63bdbe, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bc494; continue; }
            pc = 0x4bc43b; continue;
            }
            0x4bc43b => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bc494; continue; }
            pc = 0x4bc43f; continue;
            }
            0x4bc43f => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,0x6bedc0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4bc44f);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(5,32,0x1u32);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4bc46f; continue; }
            pc = 0x4bc45f; continue;
            }
            0x4bc45f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.r[3].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bc498; continue;
            }
            0x4bc46f => {
            c.set(6,32,0x6becccu32);
            pc = 0x4bc474; continue;
            }
            0x4bc474 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bc47d);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bc487; continue; }
            pc = 0x4bc484; continue;
            }
            0x4bc484 => {
            { let a = c.r[3].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            pc = 0x4bc487; continue;
            }
            0x4bc487 => {
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x6becdcu32,32); }
            if c.cond(12) { pc = 0x4bc474; continue; }
            pc = 0x4bc492; continue;
            }
            0x4bc492 => {
            pc = 0x4bc498; continue;
            }
            0x4bc494 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x40),32));
            pc = 0x4bc498; continue;
            }
            0x4bc498 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,0x670f7cu32,32));
            c.set(0,32,rm(m,0x6bed44u32,32));
            c.set(6,32,rm(m,0x66a2c8u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6bec5cu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = c.get(6,32); c.push(m, 0x4bc4b9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x2b11u32,32); }
            { let a = c.r[3].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x4bc4cc; continue; }
            pc = 0x4bc4c3; continue;
            }
            0x4bc4c3 => {
            { let a = c.r[3].wrapping_add(0x38); wm(m,a,32,0x2b11u32); }
            pc = 0x4bc4f1; continue;
            }
            0x4bc4cc => {
            { let a = c.get(0,32); c.sub(a,0xac44u32,32); }
            if c.cond(14) { pc = 0x4bc4dc; continue; }
            pc = 0x4bc4d3; continue;
            }
            0x4bc4d3 => {
            { let a = c.r[3].wrapping_add(0x38); wm(m,a,32,0xac44u32); }
            pc = 0x4bc4f1; continue;
            }
            0x4bc4dc => {
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x2b11u32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4bc4f1; continue; }
            pc = 0x4bc4e8; continue;
            }
            0x4bc4e8 => {
            c.set(2,32,rm(m,0x670f7cu32,32));
            { let a = c.r[3].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bc4f1; continue;
            }
            0x4bc4f1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,0x670f80u32,32));
            c.set(2,32,rm(m,0x6bed50u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6bec5cu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = c.get(6,32); c.push(m, 0x4bc50c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            { let a = c.r[3].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x4bc51d; continue; }
            pc = 0x4bc514; continue;
            }
            0x4bc514 => {
            { let a = c.r[3].wrapping_add(0x3c); wm(m,a,32,0x8u32); }
            pc = 0x4bc542; continue;
            }
            0x4bc51d => {
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            if c.cond(14) { pc = 0x4bc52b; continue; }
            pc = 0x4bc522; continue;
            }
            0x4bc522 => {
            { let a = c.r[3].wrapping_add(0x3c); wm(m,a,32,0x10u32); }
            pc = 0x4bc542; continue;
            }
            0x4bc52b => {
            { let a = c.get(0,32); let r = c.and(a,0x80000007u32,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x4bc537; continue; }
            pc = 0x4bc532; continue;
            }
            0x4bc532 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xfffffff8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x4bc537; continue;
            }
            0x4bc537 => {
            if c.cond(4) { pc = 0x4bc542; continue; }
            pc = 0x4bc539; continue;
            }
            0x4bc539 => {
            c.set(1,32,rm(m,0x670f80u32,32));
            { let a = c.r[3].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            pc = 0x4bc542; continue;
            }
            0x4bc542 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,0x670f84u32,32));
            c.set(1,32,rm(m,0x6bed60u32,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6bec5cu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = c.get(6,32); c.push(m, 0x4bc55d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            { let a = c.r[3].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x4bc56e; continue; }
            pc = 0x4bc565; continue;
            }
            0x4bc565 => {
            { let a = c.r[3].wrapping_add(0x40); wm(m,a,32,0x1u32); }
            pc = 0x4bc57a; continue;
            }
            0x4bc56e => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(14) { pc = 0x4bc57a; continue; }
            pc = 0x4bc573; continue;
            }
            0x4bc573 => {
            { let a = c.r[3].wrapping_add(0x40); wm(m,a,32,0x2u32); }
            pc = 0x4bc57a; continue;
            }
            0x4bc57a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,0x6beef0u32,32));
            c.set(2,32,rm(m,0x6beca4u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6bec5cu32,32));
            c.push(m,0x400u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a2c4u32,32); c.push(m, 0x4bc5a3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x6bedc4u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4bc5bc);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x4bc5d2; continue; }
            pc = 0x4bc5c4; continue;
            }
            0x4bc5c4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[3].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[3].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            pc = 0x4bc5d2; continue;
            }
            0x4bc5d2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,0x6becb4u32,32));
            c.set(1,32,rm(m,0x6bec5cu32,32));
            c.push(m,c.get(2,32));
            c.push(m,0x20u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = c.get(6,32); c.push(m, 0x4bc5e8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(6,32,rm(m,0x66a528u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x2u32);
            c.push(m,c.get(2,32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            c.push(m,0x0u32);
            c.push(m,0x10u32);
            { let a = c.r[3].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[3].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            { let target = c.get(6,32); c.push(m, 0x4bc604); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4bc610; continue; }
            pc = 0x4bc609; continue;
            }
            0x4bc609 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[3].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            pc = 0x4bc610; continue;
            }
            0x4bc610 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x54u32);
            { let a = c.r[3].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            { let target = c.get(6,32); c.push(m, 0x4bc620); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4bc62c; continue; }
            pc = 0x4bc625; continue;
            }
            0x4bc625 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[3].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bc62c; continue;
            }
            0x4bc62c => {
            c.push(m,c.get(3,32));
            c.push(m, 0x4bc632);
            { let args = []; c.r[0] = host.call(m, 0x4bc680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,8,c.get(0,8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bc64f; continue; }
            pc = 0x4bc63b; continue;
            }
            0x4bc63b => {
            c.push(m, 0x4bc640);
            { let args = []; c.r[0] = host.call(m, 0x4bb240, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m, 0x4bc645);
            { let args = []; c.r[0] = host.call(m, 0x4bae80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6dcb8cu32; wm(m,a,32,0x0u32); }
            pc = 0x4bc64f; continue;
            }
            0x4bc64f => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x0u32); }
            c.push(m, 0x4bc65d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4bc66a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_erase_subtree(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0050;
    loop {
        match pc {
            0x4c0050 => {
            c.set(0,32,rm(m,0x6dd564u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            c.set(6,32,c.get(7,32));
            if c.cond(4) { pc = 0x4c008d; continue; }
            pc = 0x4c0064; continue;
            }
            0x4c0064 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c006f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c0050, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[7].wrapping_add(0xc));
            c.push(m, 0x4c0079);
            { let args = []; c.r[0] = host.call(m, 0x4bf330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x4c007f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dd564u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            c.set(7,32,c.get(6,32));
            if c.cond(5) { pc = 0x4c0064; continue; }
            pc = 0x4c008d; continue;
            }
            0x4c008d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn convoy_choice_window_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40c5f0;
    loop {
        match pc {
            0x40c5f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x44u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x60),32); c.push(m, 0x40c609); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x40c619); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x1174));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x40c626);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1630),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x18));
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fild32(m,0x6cbb00u32);
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x1c),4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m, 0x40c65f);
            { let args = []; c.r[0] = host.call(m, 0x63a5ce, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fild32(m,0x6cbb00u32);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),8); c.fop('*',v); }
            c.push(m, 0x40c675);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,0x6dcdd0u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40c68b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1720),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x3c),32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x1c));
            c.fstp32(m,c.r[4].wrapping_add(0x1c));
            c.fild32(m,0x6cbb00u32);
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x1c),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m, 0x40c6cf);
            { let args = []; c.r[0] = host.call(m, 0x63a5ce, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fild32(m,0x6cbb00u32);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4].wrapping_add(0x14));
            { let v = c.ld(m,c.r[4].wrapping_add(0x14),8); c.fop('*',v); }
            c.push(m, 0x40c6e5);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x40c701; continue; }
            pc = 0x40c6f0; continue;
            }
            0x40c6f0 => {
            c.set(3,32,rm(m,0x6cbb00u32,32));
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x40c701; continue;
            }
            0x40c701 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(3,8); c.and(a,0x3u32,8); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x40c749; continue; }
            pc = 0x40c734; continue;
            }
            0x40c734 => {
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.and(a,0x80000003u32,32); c.set(1,32,r); }
            if c.cond(9) { pc = 0x40c743; continue; }
            pc = 0x40c73e; continue;
            }
            0x40c73e => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.or(a,0xfffffffcu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x40c743; continue;
            }
            0x40c743 => {
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            pc = 0x40c749; continue;
            }
            0x40c749 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x40c755); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,rm(m,0x6dcdd0u32,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x40c776); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40c789); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1524),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1720),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x3c),32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1520),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x40c7c6);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1630),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x2));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40c7ed); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x40c800); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x40c81f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            c.set(1,32,c.r[6].wrapping_add(0x116c));
            c.push(m,c.get(0,32));
            c.push(m, 0x40c831);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1530),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14f0),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x40c84d; continue; }
            pc = 0x40c841; continue;
            }
            0x40c841 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x40c84d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x40c84d; continue;
            }
            0x40c84d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x40c860; continue; }
            pc = 0x40c857; continue;
            }
            0x40c857 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x40c85d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x40c860; continue;
            }
            0x40c860 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,0x0u32); }
            c.push(m, 0x40c87b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x41c5a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1720),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1634),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x171c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(7,32,c.r[6].wrapping_add(0x1634));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x40c8b2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1540),32));
            c.set(3,32,c.r[6].wrapping_add(0x1540));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x40c8c9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x40c8d1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x40c8e8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn building_scene_window_draw(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x465ba0;
    loop {
        match pc {
            0x465ba0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x3cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(7,32,c.r[6].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x465bc2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x465be9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x46630d; continue; }
            pc = 0x465bf1; continue;
            }
            0x465bf1 => {
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(1,32));
            c.push(m, 0x465bfb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb7c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x46630d; continue; }
            pc = 0x465c06; continue;
            }
            0x465c06 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x465c10);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x465c1b);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xe4),8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x465cb9; continue; }
            pc = 0x465c30; continue;
            }
            0x465c30 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x465c4d);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x465c65);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.add(a,0xcu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(7,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[6].wrapping_add(0xa4));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x465cb9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, 0x4673f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(36);
            pc = 0x465cb9; continue;
            }
            0x465cb9 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = rm(m,c.r[1].wrapping_add(0x24),8); c.sub(a,0x1u32,8); }
            if c.cond(6) { pc = 0x465db4; continue; }
            pc = 0x465cc9; continue;
            }
            0x465cc9 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[2]),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x465db4; continue; }
            pc = 0x465ce9; continue;
            }
            0x465ce9 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x38),32));
            pc = 0x465cf5; continue;
            }
            0x465cf5 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xd0),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2]),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x465d95; continue; }
            pc = 0x465d08; continue;
            }
            0x465d08 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xbc),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[1]),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x98),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1]),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x465d4b; continue; }
            pc = 0x465d29; continue;
            }
            0x465d29 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x465d4b; continue; }
            pc = 0x465d30; continue;
            }
            0x465d30 => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0xc),32));
            pc = 0x465d4b; continue;
            }
            0x465d4b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x465d55);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xf0),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xc));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xc));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x465d8e);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x465d95; continue;
            }
            0x465d95 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[2]),32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x465cf5; continue; }
            pc = 0x465db2; continue;
            }
            0x465db2 => {
            pc = 0x465dc0; continue;
            }
            0x465db4 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x38),32));
            pc = 0x465dc0; continue;
            }
            0x465dc0 => {
            { let a = rm(m,c.r[6].wrapping_add(0xec),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x465e5d; continue; }
            pc = 0x465dce; continue;
            }
            0x465dce => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x465dda);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            pc = 0x465de5; continue;
            }
            0x465de5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x18u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.or(a,0xffffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x465dfe);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xec),16));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1]));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xffffff6c));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x465e4d; continue; }
            pc = 0x465e22; continue;
            }
            0x465e22 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xffffff60));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x465e4d; continue; }
            pc = 0x465e31; continue;
            }
            0x465e31 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,0x1u32);
            { let a = c.get(2,32); let r = c.sub(a,0x1au32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,0xcu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x465e4a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x465e4d; continue;
            }
            0x465e4d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xa0u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x465de5; continue; }
            pc = 0x465e5d; continue;
            }
            0x465e5d => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xfc),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x465eb0; continue; }
            pc = 0x465e67; continue;
            }
            0x465e67 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xf8),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x465e73);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xa0ffffffu32);
            c.push(m, 0x465e7d);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x1au32,32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xec),16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x1bu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xc));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,0xcu32,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x465ead);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            pc = 0x465eb0; continue;
            }
            0x465eb0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x465eda; continue; }
            pc = 0x465ebd; continue;
            }
            0x465ebd => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x465eda; continue; }
            pc = 0x465ec4; continue;
            }
            0x465ec4 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.set(3,32,rm(m,c.r[2],32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,rm(m,c.r[2].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[2].wrapping_add(0xc),32));
            pc = 0x465eda; continue;
            }
            0x465eda => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x465ee4);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m, 0x465eeb);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x465f00); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x465fa0; continue; }
            pc = 0x465f0a; continue;
            }
            0x465f0a => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x465f19); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x465f3a; continue; }
            pc = 0x465f23; continue;
            }
            0x465f23 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x465f2e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x465f3e; continue;
            }
            0x465f3a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x465f3e; continue;
            }
            0x465f3e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x465f5c);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[7].wrapping_add(c.r[7]));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x465f80);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x465f96); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x465f0a; continue; }
            pc = 0x465fa0; continue;
            }
            0x465fa0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x465fca; continue; }
            pc = 0x465fad; continue;
            }
            0x465fad => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x465fca; continue; }
            pc = 0x465fb4; continue;
            }
            0x465fb4 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.set(3,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0xc),32));
            pc = 0x465fca; continue;
            }
            0x465fca => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x465fd4);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x465fe9);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x14),32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x466018; continue; }
            pc = 0x465ff8; continue;
            }
            0x465ff8 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x466018; continue; }
            pc = 0x465fff; continue;
            }
            0x465fff => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0xc),32));
            pc = 0x466018; continue;
            }
            0x466018 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x466022);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x46603e);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x14),32); c.sub(a,0x2u32,32); }
            if c.cond(14) { pc = 0x46606d; continue; }
            pc = 0x46604d; continue;
            }
            0x46604d => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x46606d; continue; }
            pc = 0x466054; continue;
            }
            0x466054 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x20u32,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0xc),32));
            pc = 0x46606d; continue;
            }
            0x46606d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x466077);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x46609a);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x14),32); c.sub(a,0x3u32,32); }
            if c.cond(14) { pc = 0x4660c9; continue; }
            pc = 0x4660a9; continue;
            }
            0x4660a9 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4660c9; continue; }
            pc = 0x4660b0; continue;
            }
            0x4660b0 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x30u32,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0xc),32));
            pc = 0x4660c9; continue;
            }
            0x4660c9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4660d3);
            { let args = []; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4660ef);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x14),32); c.sub(a,0x4u32,32); }
            if c.cond(14) { pc = 0x46611e; continue; }
            pc = 0x4660fe; continue;
            }
            0x4660fe => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x46611e; continue; }
            pc = 0x466105; continue;
            }
            0x466105 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x40u32,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0xc),32));
            pc = 0x46611e; continue;
            }
            0x46611e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x466128);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x11u32); }
            pc = 0x46613b; continue;
            }
            0x46613b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xc));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x466158);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x46613b; continue; }
            pc = 0x466170; continue;
            }
            0x466170 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            { let a = rm(m,c.r[0].wrapping_add(0x14),32); c.sub(a,0x5u32,32); }
            if c.cond(14) { pc = 0x46619c; continue; }
            pc = 0x46617c; continue;
            }
            0x46617c => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x46619c; continue; }
            pc = 0x466183; continue;
            }
            0x466183 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.add(a,0x50u32,32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[2],32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,rm(m,c.r[2].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[2].wrapping_add(0xc),32));
            pc = 0x46619c; continue;
            }
            0x46619c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4661a6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x11u32); }
            pc = 0x4661b9; continue;
            }
            0x4661b9 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xc));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4661dd);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4661b9; continue; }
            pc = 0x4661f5; continue;
            }
            0x4661f5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            { let a = rm(m,c.r[0].wrapping_add(0x14),32); c.sub(a,0x7u32,32); }
            if c.cond(14) { pc = 0x466221; continue; }
            pc = 0x466201; continue;
            }
            0x466201 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x466221; continue; }
            pc = 0x466208; continue;
            }
            0x466208 => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0x70u32,32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[1],32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0xc),32));
            pc = 0x466221; continue;
            }
            0x466221 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x46622b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x22u32); }
            pc = 0x46623e; continue;
            }
            0x46623e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xc));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x46625b);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x46623e; continue; }
            pc = 0x466273; continue;
            }
            0x466273 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9c),32));
            { let a = rm(m,c.r[0].wrapping_add(0x14),32); c.sub(a,0x6u32,32); }
            if c.cond(14) { pc = 0x46629d; continue; }
            pc = 0x46627f; continue;
            }
            0x46627f => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x46629d; continue; }
            pc = 0x466286; continue;
            }
            0x466286 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,0x60u32,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[0],32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0xc),32));
            pc = 0x4662a1; continue;
            }
            0x46629d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            pc = 0x4662a1; continue;
            }
            0x4662a1 => {
            c.push(m,c.get(1,32));
            c.push(m, 0x4662a7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x22u32); }
            pc = 0x4662ba; continue;
            }
            0x4662ba => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xc));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4662de);
            { let args = []; c.r[0] = host.call(m, 0x4bb330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4662ba; continue; }
            pc = 0x4662f6; continue;
            }
            0x4662f6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x466300);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x46630a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x46630d; continue;
            }
            0x46630d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_view_ship_set_pose(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x56b140;
    loop {
        match pc {
            0x56b140 => {
            { let a = c.get(4,32); let r = c.sub(a,0x80u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x94),32));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x56b1e0; continue; }
            pc = 0x56b161; continue;
            }
            0x56b161 => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x16),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x56b1e0; continue; }
            pc = 0x56b169; continue;
            }
            0x56b169 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x98),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x18),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x56b1e0; continue; }
            pc = 0x56b177; continue;
            }
            0x56b177 => {
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            c.set(0,32,0x66666667u32);
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0x66666667u32);
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            c.set(2,8,0x5u32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let (lo,hi) = c.imul1(c.get(0,8),c.get(2,8),8); c.set(0,16,lo | (hi<<8)); }
            c.set(1,8,c.get(0,8));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xc),8));
            c.set(2,8,0xecu32);
            { let (lo,hi) = c.imul1(c.get(0,8),c.get(2,8),8); c.set(0,16,lo | (hi<<8)); }
            { let a = c.get(0,8); let r = c.sub(a,c.get(1,8),8); c.set(0,8,r); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x9c),8));
            { let a = c.get(0,8); let r = c.add(a,c.get(3,8),8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x56b787; continue; }
            pc = 0x56b1d9; continue;
            }
            0x56b1d9 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x94),32));
            pc = 0x56b1e0; continue;
            }
            0x56b1e0 => {
            c.set(1,32,rm(m,0x6e51c0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x6e1950u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x56b1fc);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x56b211; continue; }
            pc = 0x56b200; continue;
            }
            0x56b200 => {
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x56b211);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x59c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x56b211; continue;
            }
            0x56b211 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[0],8));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x56b242; continue; }
            pc = 0x56b236; continue;
            }
            0x56b236 => {
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x56b242);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x56b7a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x56b242; continue;
            }
            0x56b242 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3a),8));
            c.push(m,c.get(7,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x56b2fe; continue; }
            pc = 0x56b24e; continue;
            }
            0x56b24e => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc650),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc654),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6e1924u32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(7,32,c.r[0].wrapping_add(0x6));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x56b275); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x16),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6e1938),32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6e2d58u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7],16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6e1924u32,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            pc = 0x56b2fe; continue;
            }
            0x56b2fe => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4a),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x56b3ab; continue; }
            pc = 0x56b309; continue;
            }
            0x56b309 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc650),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc654),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6e190cu32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(7,32,c.r[0].wrapping_add(0x6));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x56b330); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x16),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6e1938),32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7],16));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6e190cu32,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(2,32)); }
            pc = 0x56b3ab; continue;
            }
            0x56b3ab => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x56b3b9; continue; }
            pc = 0x56b3b4; continue;
            }
            0x56b3b4 => {
            { let a = rm(m,c.r[6].wrapping_add(0x40),32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x56b3fa; continue; }
            pc = 0x56b3b9; continue;
            }
            0x56b3b9 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3c),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x40),16));
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.add(a,c.get(0,16),16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = rm(m,c.r[6].wrapping_add(0x16),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,r); } }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[6].wrapping_add(0xc));
            { let a = c.get(0,32); let r = c.add(a,0x6e1950u32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x56b3e1);
            { let args = []; c.r[0] = host.call(m, 0x559130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x3c),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x40),16));
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,r); } }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x16),16); let r = c.sub(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,r); } }
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,32,c.get(7,32)); }
            pc = 0x56b3fa; continue;
            }
            0x56b3fa => {
            { let a = rm(m,c.r[6].wrapping_add(0x4c),32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x56b40e; continue; }
            pc = 0x56b3ff; continue;
            }
            0x56b3ff => {
            c.set(0,32,c.r[4].wrapping_add(0x80));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x56b40e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x56baa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x56b40e; continue;
            }
            0x56b40e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[4].wrapping_add(0x60));
            c.set(7,32,c.r[6].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x56b421);
            { let args = []; c.r[0] = host.call(m, 0x5595f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xa8),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,c.get(2,32));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(3,8); let n = 0x2u32; let r = c.shl(a,n,8); c.set(3,8,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0x66666667u32);
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,8,0x5u32);
            { let a = c.get(0,8); let r = c.add(a,c.get(3,8),8); c.set(0,8,r); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0xac),8));
            { let (lo,hi) = c.imul1(c.get(0,8),c.get(2,8),8); c.set(0,16,lo | (hi<<8)); }
            { let a = c.get(0,8); let r = c.add(a,c.get(1,8),8); c.set(0,8,r); }
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x6e1950u32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(5,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x56b49e);
            { let args = []; c.r[0] = host.call(m, 0x559790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(2,8,rm(m,c.r[0],8));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x56b4e3; continue; }
            pc = 0x56b4b2; continue;
            }
            0x56b4b2 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x56b4be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x56b7a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x70));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a4ccu32,32); c.push(m, 0x56b4d3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m, 0x56b4e3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x56b4e3; continue;
            }
            0x56b4e3 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3a),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x56b5b3; continue; }
            pc = 0x56b4ee; continue;
            }
            0x56b4ee => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,0x6e1924u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0xc650),32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0xc654),32));
            c.set(7,32,c.r[0].wrapping_add(0x6));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x56b50d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x16),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6e1938),32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6e2d58u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7],16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x40),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6e1924u32,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x4),16));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a4ccu32,32); c.push(m, 0x56b5a3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m, 0x56b5b3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x56b5b3; continue;
            }
            0x56b5b3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x56b5ee; continue; }
            pc = 0x56b5ba; continue;
            }
            0x56b5ba => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x56b5c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x56baa0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.set(0,32,c.r[4].wrapping_add(0x80));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a4ccu32,32); c.push(m, 0x56b5de); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m, 0x56b5ee);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x56b5ee; continue;
            }
            0x56b5ee => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4a),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x56b6b0; continue; }
            pc = 0x56b5f9; continue;
            }
            0x56b5f9 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,0x6e190cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0xc650),32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0xc654),32));
            c.set(7,32,c.r[0].wrapping_add(0x6));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x56b618); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x16),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6e1938),32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7],16));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x40),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6e190cu32,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x4),16));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a4ccu32,32); c.push(m, 0x56b6a0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m, 0x56b6b0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x56b6b0; continue;
            }
            0x56b6b0 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x50),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x56b742; continue; }
            pc = 0x56b6bb; continue;
            }
            0x56b6bb => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,0x66666667u32);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            c.push(m,0x0u32);
            c.set(7,32,c.get(2,32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(7,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x18),8));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x6e1964u32,32); c.set(7,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(3,32,sx(rm(m,c.r[6].wrapping_add(0x16),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[7],32),32); c.set(0,32,r); }
            c.set(5,32,c.r[2].wrapping_mul(8).wrapping_add(0x6e1860));
            c.set(2,8,rm(m,c.r[7],8));
            { let a = c.get(0,32); let r = c.add(a,0x80u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,c.get(2,8),8); c.set(0,8,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(0,8)); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x40),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(7,32,sx(rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2),16),16)&0xffffffffu32);
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            c.set(0,32,sx(rm(m,c.r[0],16),16)&0xffffffffu32);
            { let a = c.get(7,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(7,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x56b742); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x56b742; continue;
            }
            0x56b742 => {
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.push(m, 0x56b74d);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(4) { pc = 0x56b787; continue; }
            pc = 0x56b752; continue;
            }
            0x56b752 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x56b763);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x59c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x50));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a4ccu32,32); c.push(m, 0x56b778); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x56b787);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x56b787; continue;
            }
            0x56b787 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x80u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn harbor_ship_sprite_get_name_label_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x56b7a0;
    loop {
        match pc {
            0x56b7a0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc654),32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0xc650),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x38),16));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x3c),16));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x1u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x1u32,8); }
            if c.cond(4) { pc = 0x56b815; continue; }
            pc = 0x56b7e0; continue;
            }
            0x56b7e0 => {
            { let a = c.get(0,32); let r = c.and(a,0x400u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x400u32,32); }
            if c.cond(4) { pc = 0x56b815; continue; }
            pc = 0x56b7ec; continue;
            }
            0x56b7ec => {
            c.set(1,32,0x6dcd28u32);
            c.push(m, 0x56b7f6);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x56b7fc);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x160u32,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x56b808);
            { let args = []; c.r[0] = host.call(m, 0x4bba90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(3,32,c.get(0,32));
            pc = 0x56b81b; continue;
            }
            0x56b815 => {
            c.set(3,32,rm(m,0x66e0fcu32,32));
            pc = 0x56b81b; continue;
            }
            0x56b81b => {
            c.set(0,32,rm(m,c.r[1],32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x56b824); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x16),16),16)&0xffffffffu32);
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6e1938),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xfffffffe));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6dcd28u32,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x56b879); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x4));
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(3,32));
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(0xfffffffe));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x4));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn map_object_table_rebuild_kind_index(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x586eb0;
    loop {
        match pc {
            0x586eb0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,0x1u32);
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(6) { pc = 0x586eeb; continue; }
            pc = 0x586ec9; continue;
            }
            0x586ec9 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x586ee3; continue; }
            pc = 0x586ed3; continue;
            }
            0x586ed3 => {
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x586ed8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14),32));
            c.set(0,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x586ee3; continue;
            }
            0x586ee3 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x586ec9; continue; }
            pc = 0x586eeb; continue;
            }
            0x586eeb => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.get(5,32); let r = c.add(a,0x1bcu32,32); c.set(5,32,r); }
            pc = 0x586ef9; continue;
            }
            0x586ef9 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xfffffe58),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x586f44; continue; }
            pc = 0x586f03; continue;
            }
            0x586f03 => {
            c.set(0,32,c.r[3].wrapping_add(c.r[3]));
            c.push(m,c.get(0,32));
            c.push(m, 0x586f0c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            c.set(6,32,0x1u32);
            if c.cond(6) { pc = 0x586f44; continue; }
            pc = 0x586f1d; continue;
            }
            0x586f1d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x586f3f; continue; }
            pc = 0x586f2b; continue;
            }
            0x586f2b => {
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x50),32); c.push(m, 0x586f30); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x10),32),32); }
            if c.cond(5) { pc = 0x586f3f; continue; }
            pc = 0x586f36; continue;
            }
            0x586f36 => {
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[1].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0xfffffffe); wm(m,a,16,c.get(6,16)); }
            pc = 0x586f3f; continue;
            }
            0x586f3f => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x586f1d; continue; }
            pc = 0x586f44; continue;
            }
            0x586f44 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x6au32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x586ef9; continue; }
            pc = 0x586f55; continue;
            }
            0x586f55 => {
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

pub fn town_view_enter_battle_location(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x58a310;
    loop {
        match pc {
            0x58a310 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[6].wrapping_add(0xc648); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0xc64c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[6].wrapping_add(0xc5f4); wm(m,a,8,0x0u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58a364);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x58a36f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc324),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(8) { pc = 0x58a380; continue; }
            pc = 0x58a379; continue;
            }
            0x58a379 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a380);
            { let args = []; c.r[0] = host.call(m, 0x589130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x58a380; continue;
            }
            0x58a380 => {
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x5a0));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,8); let r = c.or(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0xc324); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58a3a0);
            { let args = []; c.r[0] = host.call(m, 0x59dbe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x58a3ab); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a3b2);
            { let args = []; c.r[0] = host.call(m, 0x58eb30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc328),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58a3cd; continue; }
            pc = 0x58a3bc; continue;
            }
            0x58a3bc => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a3c3);
            { let args = []; c.r[0] = host.call(m, 0x58f1b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x50c); wm(m,a,32,0x0u32); }
            pc = 0x58a3cd; continue;
            }
            0x58a3cd => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.and(a,0x7fu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x58a421; continue; }
            pc = 0x58a3db; continue;
            }
            0x58a3db => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x2c1),8));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6ddbc0),8));
            { let a = c.r[6].wrapping_add(0x570); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2]));
            c.push(m, 0x58a41f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x521580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58a43d; continue;
            }
            0x58a421 => {
            c.set(1,32,c.r[7].wrapping_mul(4));
            c.set(2,32,0x6ddba4u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x570));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[2],32));
            c.push(m, 0x58a43d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x515b80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58a43d; continue;
            }
            0x58a43d => {
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.push(m, 0x58a448);
            { let args = []; c.r[0] = host.call(m, 0x59ceb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x58a458); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xc324),8));
            { let a = c.get(1,32); let r = c.and(a,0x7fu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x58a46d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42a910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xc324),16));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6e6220u32);
            c.push(m, 0x58a481);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6141d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,c.r[6].wrapping_add(0xc1c4));
            c.push(m,0x140u32);
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x58a494);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(7,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[0].wrapping_add(0x140));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x58a4b1; continue; }
            pc = 0x58a4a4; continue;
            }
            0x58a4a4 => {
            { let a = c.get(2,8); let r = c.or(a,0xffu32,8); c.set(2,8,r); }
            pc = 0x58a4a7; continue;
            }
            0x58a4a7 => {
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x58a4a7; continue; }
            pc = 0x58a4b1; continue;
            }
            0x58a4b1 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a4b8);
            { let args = []; c.r[0] = host.call(m, 0x58a240, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a4bf);
            { let args = []; c.r[0] = host.call(m, 0x59b410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            { let a = c.r[6].wrapping_add(0xc648); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xc64c); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x58a4dc);
            { let args = []; c.r[0] = host.call(m, 0x57ae10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_view_switch_to_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x58a4f0;
    loop {
        match pc {
            0x58a4f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc628),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x58a576; continue; }
            pc = 0x58a509; continue;
            }
            0x58a509 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a510);
            { let args = []; c.r[0] = host.call(m, 0x589130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a517);
            { let args = []; c.r[0] = host.call(m, 0x58b490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0xc628); wm(m,a,32,c.get(1,32)); }
            c.push(m,0xffu32);
            c.set(1,32,0x6e6220u32);
            c.push(m, 0x58a534);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6141d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x564),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58a576; continue; }
            pc = 0x58a53e; continue;
            }
            0x58a53e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xc5f4); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0xc648); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xc64c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58a564);
            { let args = []; c.r[0] = host.call(m, 0x58cf70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x58a56f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x564),8); let r = c.xor(a,0x1u32,8); { let a = c.r[6].wrapping_add(0x564); wm(m,a,8,r); } }
            pc = 0x58a576; continue;
            }
            0x58a576 => {
            c.set(1,32,c.r[6].wrapping_add(0x5a0));
            c.push(m, 0x58a581);
            { let args = []; c.r[0] = host.call(m, 0x59dbe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x58a58c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x58a5a5; continue; }
            pc = 0x58a594; continue;
            }
            0x58a594 => {
            c.push(m,0x0u32);
            c.push(m,0xffu32);
            c.set(1,32,0x6e6220u32);
            c.push(m, 0x58a5a5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6141d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x58a5a5; continue;
            }
            0x58a5a5 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(7,32,c.get(5,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x58a6d3; continue; }
            pc = 0x58a5bf; continue;
            }
            0x58a5bf => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x58a6d3; continue; }
            pc = 0x58a5c7; continue;
            }
            0x58a5c7 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x34),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x58a66f; continue; }
            pc = 0x58a5d3; continue;
            }
            0x58a5d3 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x800));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[0],8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            c.set(5,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(7,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x28));
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fild32(m,c.r[6].wrapping_add(0x510));
            c.fstp32(m,c.r[4].wrapping_add(0x34));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x34),4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x28));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            c.push(m, 0x58a62b);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x34));
            c.fstp32(m,c.r[4].wrapping_add(0x34));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x28),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x34),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x44u32,8); }
            if c.cond(11) { pc = 0x58a649; continue; }
            pc = 0x58a648; continue;
            }
            0x58a648 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            pc = 0x58a649; continue;
            }
            0x58a649 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a650);
            { let args = []; c.r[0] = host.call(m, 0x593b70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x58a66b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x590ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x58a66f; continue;
            }
            0x58a66f => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58a915; continue; }
            pc = 0x58a67b; continue;
            }
            0x58a67b => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x58a687);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42cd90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a690);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58f4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc650),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc654),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc7cc),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58a6c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x58ad30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); c.fpu_exit(); return c.r;
            }
            0x58a6d3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[6].wrapping_add(0xc648); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0xc64c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[6].wrapping_add(0xc5f4); wm(m,a,8,0x0u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58a721);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x58a72c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a733);
            { let args = []; c.r[0] = host.call(m, 0x589130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0xc324); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x58a745);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42aae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x58a758; continue; }
            pc = 0x58a74d; continue;
            }
            0x58a74d => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x58a758);
            { let args = []; c.r[0] = host.call(m, 0x42a6c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x58a758; continue;
            }
            0x58a758 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a75f);
            { let args = []; c.r[0] = host.call(m, 0x58eb30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); c.sub(a,rm(m,0x6de4b0u32,32),32); }
            if c.cond(3) { pc = 0x58a7b4; continue; }
            pc = 0x58a767; continue;
            }
            0x58a767 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x2c1),8));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6ddbc0),8));
            { let a = c.r[6].wrapping_add(0x570); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2]));
            c.push(m, 0x58a7ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x521580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a7b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58f4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58a7b4; continue;
            }
            0x58a7b4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc650),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc654),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc7cc),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58a7ed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x58ad30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc328),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58a808; continue; }
            pc = 0x58a7f7; continue;
            }
            0x58a7f7 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a7fe);
            { let args = []; c.r[0] = host.call(m, 0x58f1b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x50c); wm(m,a,32,0x0u32); }
            pc = 0x58a808; continue;
            }
            0x58a808 => {
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.push(m, 0x58a813);
            { let args = []; c.r[0] = host.call(m, 0x59ceb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x58a823); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xc324),16));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6e6220u32);
            { let a = c.r[6].wrapping_add(0x508); wm(m,a,32,0x0u32); }
            c.push(m, 0x58a841);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6141d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); c.sub(a,rm(m,0x6de4b0u32,32),32); }
            if c.cond(3) { pc = 0x58a854; continue; }
            pc = 0x58a849; continue;
            }
            0x58a849 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x58a854);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8f00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58a854; continue;
            }
            0x58a854 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x510),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x514),32));
            { let a = c.r[6].wrapping_add(0xc650); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(1,32,c.r[0].wrapping_add(0x1));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc594),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0xc654); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x58a8ac; continue; }
            pc = 0x58a89e; continue;
            }
            0x58a89e => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a8a5);
            { let args = []; c.r[0] = host.call(m, 0x58a920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a8ac);
            { let args = []; c.r[0] = host.call(m, 0x58b1b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x58a8ac; continue;
            }
            0x58a8ac => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a8b3);
            { let args = []; c.r[0] = host.call(m, 0x598280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58a8ba);
            { let args = []; c.r[0] = host.call(m, 0x595ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0xc1c4));
            c.push(m,0x140u32);
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x58a8cd);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(7,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[0].wrapping_add(0x140));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x58a8e9; continue; }
            pc = 0x58a8dc; continue;
            }
            0x58a8dc => {
            { let a = c.get(2,8); let r = c.or(a,0xffu32,8); c.set(2,8,r); }
            pc = 0x58a8df; continue;
            }
            0x58a8df => {
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x58a8df; continue; }
            pc = 0x58a8e9; continue;
            }
            0x58a8e9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            { let a = c.r[6].wrapping_add(0xc648); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0xc64c); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x58a915; continue; }
            pc = 0x58a909; continue;
            }
            0x58a909 => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x58a915);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42cd90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58a915; continue;
            }
            0x58a915 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_view_free_path_tables(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x58b3f0;
    loop {
        match pc {
            0x58b3f0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0x6u32); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(6,32,c.r[5].wrapping_add(0xc670));
            c.push(m,c.get(7,32));
            pc = 0x58b407; continue;
            }
            0x58b407 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xffffffe8),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x58b431; continue; }
            pc = 0x58b410; continue;
            }
            0x58b410 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x58b423; continue; }
            pc = 0x58b41a; continue;
            }
            0x58b41a => {
            c.push(m,c.get(0,32));
            c.push(m, 0x58b420);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x58b423; continue;
            }
            0x58b423 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[1].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xffffffe8),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x58b410; continue; }
            pc = 0x58b431; continue;
            }
            0x58b431 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x58b440; continue; }
            pc = 0x58b437; continue;
            }
            0x58b437 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x58b43d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x58b440; continue;
            }
            0x58b440 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6]; wm(m,a,32,c.get(3,32)); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x58b407; continue; }
            pc = 0x58b450; continue;
            }
            0x58b450 => {
            c.push(m,0x18u32);
            c.set(2,32,c.r[5].wrapping_add(0xc658));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x58b45f);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc59c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(4) { pc = 0x58b476; continue; }
            pc = 0x58b46d; continue;
            }
            0x58b46d => {
            c.push(m,c.get(0,32));
            c.push(m, 0x58b473);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x58b476; continue;
            }
            0x58b476 => {
            { let a = c.r[5].wrapping_add(0xc59c); wm(m,a,32,c.get(3,32)); }
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

pub fn town_view_unload_scene(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x58b490;
    loop {
        match pc {
            0x58b490 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            if c.cond(4) { pc = 0x58b4f7; continue; }
            pc = 0x58b4a1; continue;
            }
            0x58b4a1 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x58b4f7; continue; }
            pc = 0x58b4ae; continue;
            }
            0x58b4ae => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x58b4b0; continue;
            }
            0x58b4b0 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x58b4d5; continue; }
            pc = 0x58b4ca; continue;
            }
            0x58b4ca => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x58b4d3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x62f5a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58b4da; continue;
            }
            0x58b4d5 => {
            c.push(m, 0x58b4da);
            { let args = []; c.r[0] = host.call(m, 0x6358a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x58b4da; continue;
            }
            0x58b4da => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xc),8));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x58b4b0; continue; }
            pc = 0x58b4f7; continue;
            }
            0x58b4f7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc308),32));
            c.set(7,32,c.r[6].wrapping_add(0xc308));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x58b512; continue; }
            pc = 0x58b507; continue;
            }
            0x58b507 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x58b512 => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58b51a);
            { let args = []; c.r[0] = host.call(m, 0x59b410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58b524);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x58b090, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58b52b);
            { let args = []; c.r[0] = host.call(m, 0x58b5f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x94));
            c.push(m, 0x58b536);
            { let args = []; c.r[0] = host.call(m, 0x586d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            c.set(1,32,c.r[6].wrapping_add(0x3f8));
            c.push(m, 0x58b543);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x568ab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,0x3u32);
            pc = 0x58b548; continue;
            }
            0x58b548 => {
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.push(m, 0x58b550);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[7]; wm(m,a,32,c.get(3,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x58b548; continue; }
            pc = 0x58b55b; continue;
            }
            0x58b55b => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58b562);
            { let args = []; c.r[0] = host.call(m, 0x58b3f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            { let a = c.r[6].wrapping_add(0xc314); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc318); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc31c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc320); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x528); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x52c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xc324); wm(m,a,32,0xffffffffu32); }
            c.set(7,32,c.r[6].wrapping_add(0xc69c));
            c.set(5,32,0x5u32);
            pc = 0x58b5a5; continue;
            }
            0x58b5a5 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x58b5b4; continue; }
            pc = 0x58b5ab; continue;
            }
            0x58b5ab => {
            c.push(m,c.get(0,32));
            c.push(m, 0x58b5b1);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x58b5b4; continue;
            }
            0x58b5b4 => {
            { let a = c.r[7]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[7].wrapping_add(0xffffffec); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x58b5a5; continue; }
            pc = 0x58b5bf; continue;
            }
            0x58b5bf => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58b5c6);
            { let args = []; c.r[0] = host.call(m, 0x595c10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[6].wrapping_add(0xc58c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc590); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc63c); wm(m,a,8,c.get(3,8)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_view_update_frame(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x58b7f0;
    loop {
        match pc {
            0x58b7f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x58u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            { let a = c.get(0,32); let r = c.and(a,0x7fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x58b83d; continue; }
            pc = 0x58b809; continue;
            }
            0x58b809 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x76a),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x776),16),16); }
            c.set(2,8,u32::from(c.cond(2)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x56c); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x9bc),8));
            c.push(m,c.get(1,32));
            pc = 0x58b859; continue;
            }
            0x58b83d => {
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.r[6].wrapping_add(0x56c); wm(m,a,8,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x58b850; continue; }
            pc = 0x58b84d; continue;
            }
            0x58b84d => {
            c.push(m,c.get(0,32));
            pc = 0x58b859; continue;
            }
            0x58b850 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x9bc),8));
            c.push(m,c.get(2,32));
            pc = 0x58b859; continue;
            }
            0x58b859 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58b860);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x591ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58b86a);
            { let args = []; c.r[0] = host.call(m, 0x59bee0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[6].wrapping_add(0x599));
            c.set(1,32,c.r[6].wrapping_add(0x598));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6e55d0u32);
            c.push(m, 0x58b882);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x613070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x590),32));
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            c.set(3,32,0xffffu32);
            if c.cond(4) { pc = 0x58b945; continue; }
            pc = 0x58b898; continue;
            }
            0x58b898 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x588),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x58c),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0xe0u32);
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58b8c2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x58b8d9; continue; }
            pc = 0x58b8c6; continue;
            }
            0x58b8c6 => {
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,0x60u32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58b8d9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x58b8d9; continue;
            }
            0x58b8d9 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x594),32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x60u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x58b8fa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x58b912; continue; }
            pc = 0x58b8fe; continue;
            }
            0x58b8fe => {
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,0xe0u32);
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x58b912);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58b740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x58b912; continue;
            }
            0x58b912 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58b92d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x59b620, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0x594); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x590); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x588); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x58c); wm(m,a,32,c.get(3,32)); }
            pc = 0x58b945; continue;
            }
            0x58b945 => {
            c.set(2,32,c.r[4].wrapping_add(0x48));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a0d8u32,32); c.push(m, 0x58b950); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x52),16));
            { let a = c.get(0,16); c.sub(a,0x1eu32,16); }
            if c.cond(5) { pc = 0x58b9a5; continue; }
            pc = 0x58b95b; continue;
            }
            0x58b95b => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xcb9d),8),8)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(1,32); let r = c.and(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x58b9e3; continue; }
            pc = 0x58b96c; continue;
            }
            0x58b96c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x58b999; continue; }
            pc = 0x58b97b; continue;
            }
            0x58b97b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcb98),32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x40u32);
            c.push(m,c.get(5,32));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x58b999);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x443150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            pc = 0x58b999; continue;
            }
            0x58b999 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x50),8));
            { let a = c.r[6].wrapping_add(0xcb9d); wm(m,a,8,c.get(1,8)); }
            pc = 0x58b9e3; continue;
            }
            0x58b9a5 => {
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x58b9e3; continue; }
            pc = 0x58b9aa; continue;
            }
            0x58b9aa => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0xcb9c),8),8)&0xffffffffu32);
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x58b9e3; continue; }
            pc = 0x58b9bd; continue;
            }
            0x58b9bd => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            { let a = c.r[6].wrapping_add(0xcb9c); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(2,32); c.sub(a,rm(m,0x6de4b0u32,32),32); }
            if c.cond(3) { pc = 0x58b9e3; continue; }
            pc = 0x58b9d1; continue;
            }
            0x58b9d1 => {
            { let a = c.get(0,8); c.sub(a,0xcu32,8); }
            { let a = c.r[6].wrapping_add(0xcb94); wm(m,a,8,c.get(0,8)); }
            if c.cond(14) { pc = 0x58b9e3; continue; }
            pc = 0x58b9db; continue;
            }
            0x58b9db => {
            { let a = c.get(0,8); let r = c.sub(a,0xcu32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0xcb94); wm(m,a,8,c.get(0,8)); }
            pc = 0x58b9e3; continue;
            }
            0x58b9e3 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xcb94),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcb90),32));
            if c.cond(14) { pc = 0x58ba50; continue; }
            pc = 0x58b9f3; continue;
            }
            0x58b9f3 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58ba05; continue; }
            pc = 0x58b9f7; continue;
            }
            0x58b9f7 => {
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a5c8u32,32); c.push(m, 0x58ba00); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x58ba62; continue; }
            pc = 0x58ba05; continue;
            }
            0x58ba05 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcb90),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58ba1d; continue; }
            pc = 0x58ba0f; continue;
            }
            0x58ba0f => {
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x58ba1d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4430d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58ba1d; continue;
            }
            0x58ba1d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcb98),32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x40u32);
            c.push(m,c.get(5,32));
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x58ba3a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x443150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            { let a = c.r[6].wrapping_add(0xcb90); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xcb94),8));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0xcb94); wm(m,a,8,c.get(0,8)); }
            pc = 0x58ba62; continue;
            }
            0x58ba50 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58ba62; continue; }
            pc = 0x58ba54; continue;
            }
            0x58ba54 => {
            c.set(0,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x58ba62);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4430d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58ba62; continue;
            }
            0x58ba62 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x51c),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x58bb18; continue; }
            pc = 0x58ba70; continue;
            }
            0x58ba70 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x7a4),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x58bb18; continue; }
            pc = 0x58ba92; continue;
            }
            0x58ba92 => {
            c.push(m,c.get(1,32));
            c.push(m, 0x58ba98);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc95cu32,32));
            c.push(m, 0x58baad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x47b830, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x51c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x58babf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59d5e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[6].wrapping_add(0xc648); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[6].wrapping_add(0xc64c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0xc5f4); wm(m,a,8,0x0u32); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58bb0d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x530),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x58bb18); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x58bb18; continue;
            }
            0x58bb18 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x50c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58bb36; continue; }
            pc = 0x58bb22; continue;
            }
            0x58bb22 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc328),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58bb36; continue; }
            pc = 0x58bb2c; continue;
            }
            0x58bb2c => {
            { let a = c.r[6].wrapping_add(0x50c); wm(m,a,32,0x0u32); }
            pc = 0x58bb36; continue;
            }
            0x58bb36 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x508),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58bb55; continue; }
            pc = 0x58bb40; continue;
            }
            0x58bb40 => {
            c.set(1,32,rm(m,0x6e51c0u32,32));
            c.push(m, 0x58bb4b);
            { let args = []; c.r[0] = host.call(m, 0x59ceb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x508); wm(m,a,32,0x0u32); }
            pc = 0x58bb55; continue;
            }
            0x58bb55 => {
            c.set(3,8,rm(m,0x6dd5deu32,8));
            c.push(m, 0x58bb60);
            { let args = []; c.r[0] = host.call(m, 0x462310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.or(a,c.get(3,8),8); c.set(0,8,r); }
            if c.cond(5) { pc = 0x58be4d; continue; }
            pc = 0x58bb68; continue;
            }
            0x58bb68 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4e4),32));
            c.set(0,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(7) { pc = 0x58be4d; continue; }
            pc = 0x58bb7b; continue;
            }
            0x58bb7b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4e8),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,0x5u32,32); }
            { let a = c.r[6].wrapping_add(0x4e4); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x58bb9f; continue; }
            pc = 0x58bb9a; continue;
            }
            0x58bb9a => {
            c.set(5,32,0x5u32);
            pc = 0x58bb9f; continue;
            }
            0x58bb9f => {
            c.push(m, 0x58bba4);
            { let args = []; c.r[0] = host.call(m, 0x462310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58bbc7; continue; }
            pc = 0x58bba8; continue;
            }
            0x58bba8 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = 0x6dcbe6u32; wm(m,a,8,c.get(0,8)); }
            { let a = 0x6dcbe8u32; wm(m,a,8,c.get(1,8)); }
            { let a = 0x6dcbe7u32; wm(m,a,8,c.get(2,8)); }
            { let a = 0x6dcbe5u32; wm(m,a,8,c.get(2,8)); }
            pc = 0x58bbd8; continue;
            }
            0x58bbc7 => {
            c.set(0,8,rm(m,0x6dcbe6u32,8));
            c.set(1,8,rm(m,0x6dcbe8u32,8));
            c.set(2,8,rm(m,0x6dcbe7u32,8));
            pc = 0x58bbd8; continue;
            }
            0x58bbd8 => {
            c.set(7,32,rm(m,0x6e51c0u32,32));
            c.set(3,8,rm(m,c.r[7].wrapping_add(0xbc),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x58be4d; continue; }
            pc = 0x58bbec; continue;
            }
            0x58bbec => {
            c.set(3,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(0,8)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(3,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(3,8,r); }
            { let a = c.get(3,8); let r = c.and(a,0x7eu32,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(3,8); let r = c.or(a,c.get(0,8),8); c.set(3,8,r); }
            c.set(0,8,c.get(1,8));
            { let a = c.get(0,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x7eu32,8); c.set(0,8,r); }
            { let a = c.get(1,8); let r = c.and(a,0xfeu32,8); c.set(1,8,r); }
            { let a = 0x6dcbe6u32; wm(m,a,8,c.get(3,8)); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,8); let r = c.or(a,c.get(1,8),8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(2,8)); }
            { let a = 0x6dcbe8u32; wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,8,c.get(2,8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.and(a,0x7eu32,8); c.set(1,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0xfeu32,8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,0x6dcbe5u32,8));
            { let a = c.get(1,8); let r = c.or(a,c.get(2,8),8); c.set(1,8,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = 0x6dcbe7u32; wm(m,a,8,c.get(1,8)); }
            { let a = 0x6dcbe5u32; wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x58bc7f; continue; }
            pc = 0x58bc77; continue;
            }
            0x58bc77 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x58bc7f; continue; }
            pc = 0x58bc7b; continue;
            }
            0x58bc7b => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x58bc7f; continue;
            }
            0x58bc7f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58bc9d; continue; }
            pc = 0x58bc87; continue;
            }
            0x58bc87 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x58bc9d; continue; }
            pc = 0x58bc8f; continue;
            }
            0x58bc8f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x58bc9d; continue;
            }
            0x58bc9d => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x58bd11; continue; }
            pc = 0x58bca1; continue;
            }
            0x58bca1 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x58bd11; continue; }
            pc = 0x58bca5; continue;
            }
            0x58bca5 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x58bd11; continue; }
            pc = 0x58bca9; continue;
            }
            0x58bca9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x58bd11; continue; }
            pc = 0x58bcb1; continue;
            }
            0x58bcb1 => {
            c.set(0,32,rm(m,0x6dab48u32,32));
            c.set(1,32,rm(m,0x6dab50u32,32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(15) { pc = 0x58bcc6; continue; }
            pc = 0x58bcc1; continue;
            }
            0x58bcc1 => {
            c.set(7,32,0x1u32);
            pc = 0x58bcc6; continue;
            }
            0x58bcc6 => {
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            if c.cond(15) { pc = 0x58bcd3; continue; }
            pc = 0x58bccb; continue;
            }
            0x58bccb => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x1u32); }
            pc = 0x58bcd3; continue;
            }
            0x58bcd3 => {
            c.set(2,32,rm(m,0x6dcce4u32,32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x58bce3; continue; }
            pc = 0x58bcde; continue;
            }
            0x58bcde => {
            c.set(3,32,0x1u32);
            pc = 0x58bce3; continue;
            }
            0x58bce3 => {
            c.set(0,32,rm(m,0x6dcce0u32,32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x58bcf5; continue; }
            pc = 0x58bced; continue;
            }
            0x58bced => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            pc = 0x58bcf5; continue;
            }
            0x58bcf5 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x58bd11; continue; }
            pc = 0x58bcf9; continue;
            }
            0x58bcf9 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x58bd11; continue; }
            pc = 0x58bcfd; continue;
            }
            0x58bcfd => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x58bd11; continue; }
            pc = 0x58bd05; continue;
            }
            0x58bd05 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58be4d; continue; }
            pc = 0x58bd11; continue;
            }
            0x58bd11 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc648),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x58bd3d; continue; }
            pc = 0x58bd21; continue;
            }
            0x58bd21 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc64c),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x58bd3d; continue; }
            pc = 0x58bd31; continue;
            }
            0x58bd31 => {
            { let a = c.r[6].wrapping_add(0xc648); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0xc64c); wm(m,a,32,c.get(0,32)); }
            pc = 0x58bd3d; continue;
            }
            0x58bd3d => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x58bdce; continue; }
            pc = 0x58bd45; continue;
            }
            0x58bd45 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            pc = 0x58bd46; continue;
            }
            0x58bd46 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x58bd73; continue; }
            pc = 0x58bd4e; continue;
            }
            0x58bd4e => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x58bd5f; continue; }
            pc = 0x58bd52; continue;
            }
            0x58bd52 => {
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58bd5b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58e440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x40u32);
            pc = 0x58bdc0; continue;
            }
            0x58bd5f => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x58bd73; continue; }
            pc = 0x58bd63; continue;
            }
            0x58bd63 => {
            c.push(m,0x80u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58bd6f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58e440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x40u32);
            pc = 0x58bdc0; continue;
            }
            0x58bd73 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58bd9c; continue; }
            pc = 0x58bd7b; continue;
            }
            0x58bd7b => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x58bd8d; continue; }
            pc = 0x58bd7f; continue;
            }
            0x58bd7f => {
            c.push(m,0x80u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58bd8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58e440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58bdbb; continue;
            }
            0x58bd8d => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x58bda4; continue; }
            pc = 0x58bd91; continue;
            }
            0x58bd91 => {
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58bd9a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58e440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58bdbb; continue;
            }
            0x58bd9c => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x58bda4; continue; }
            pc = 0x58bda0; continue;
            }
            0x58bda0 => {
            c.push(m,0x0u32);
            pc = 0x58bdc0; continue;
            }
            0x58bda4 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x58bdac; continue; }
            pc = 0x58bda8; continue;
            }
            0x58bda8 => {
            c.push(m,0x40u32);
            pc = 0x58bdc0; continue;
            }
            0x58bdac => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x58bdb7; continue; }
            pc = 0x58bdb0; continue;
            }
            0x58bdb0 => {
            c.push(m,0x80u32);
            pc = 0x58bdc0; continue;
            }
            0x58bdb7 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58bdc7; continue; }
            pc = 0x58bdbb; continue;
            }
            0x58bdbb => {
            c.push(m,0xc0u32);
            pc = 0x58bdc0; continue;
            }
            0x58bdc0 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58bdc7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58e440, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58bdc7; continue;
            }
            0x58bdc7 => {
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x58bd46; continue; }
            pc = 0x58bdce; continue;
            }
            0x58bdce => {
            { let a = rm(m,c.r[6].wrapping_add(0xc6b2),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x58be22; continue; }
            pc = 0x58bdd8; continue;
            }
            0x58bdd8 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xc6b8),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xc640),8));
            c.set(3,8,rm(m,c.r[6].wrapping_add(0xc644),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            { let a = c.get(1,8); let r = c.add(a,c.get(2,8),8); c.set(1,8,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xc6b4),8));
            { let a = c.get(2,8); let r = c.add(a,c.get(3,8),8); c.set(2,8,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc95cu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x58be08);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x47b8c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58be18; continue; }
            pc = 0x58be0c; continue;
            }
            0x58be0c => {
            { let a = c.r[6].wrapping_add(0xc6cc); wm(m,a,32,0xb4ffffffu32); }
            pc = 0x58be22; continue;
            }
            0x58be18 => {
            { let a = c.r[6].wrapping_add(0xc6cc); wm(m,a,32,0xff808080u32); }
            pc = 0x58be22; continue;
            }
            0x58be22 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x554),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58be4d; continue; }
            pc = 0x58be2c; continue;
            }
            0x58be2c => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.push(m, 0x58be3c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b77f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58be4d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x598ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x58be4d; continue;
            }
            0x58be4d => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58be54);
            { let args = []; c.r[0] = host.call(m, 0x5948d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58be5b);
            { let args = []; c.r[0] = host.call(m, 0x592e60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc640),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc644),32));
            { let a = c.r[6].wrapping_add(0xc648); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x554),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0xc64c); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x58be9f; continue; }
            pc = 0x58be7d; continue;
            }
            0x58be7d => {
            c.set(2,32,c.r[6].wrapping_add(0x544));
            c.set(0,32,c.r[6].wrapping_add(0x534));
            c.set(1,32,rm(m,c.r[2],32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xc),32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x58be9f; continue;
            }
            0x58be9f => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x525),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58bf3a; continue; }
            pc = 0x58bead; continue;
            }
            0x58bead => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.push(m, 0x58bebd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b77f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x58bec2);
            { let args = []; c.r[0] = host.call(m, 0x462310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,0x6e59ccu32,8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(0,8)); }
            if c.cond(4) { pc = 0x58bf3a; continue; }
            pc = 0x58bed6; continue;
            }
            0x58bed6 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[6].wrapping_add(0x525); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x58beeb; continue; }
            pc = 0x58bee9; continue;
            }
            0x58bee9 => {
            c.set(2,32,c.get(0,32));
            pc = 0x58beeb; continue;
            }
            0x58beeb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x58bef9; continue; }
            pc = 0x58bef6; continue;
            }
            0x58bef6 => {
            c.set(7,32,c.r[0].wrapping_add(0xffffffff));
            pc = 0x58bef9; continue;
            }
            0x58bef9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc654),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x514),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x510),32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc650),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6e59ccu32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6e55d0),32));
            c.push(m, 0x58bf3a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x608550, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x58bf3a; continue;
            }
            0x58bf3a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x58c0ee; continue; }
            pc = 0x58bf4e; continue;
            }
            0x58bf4e => {
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc608),32));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0x800u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x800u32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x58bf7e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc60c),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0x8000u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8000u32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x58bfb4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc610),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.sub(a,0x8u32,8); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x58bfe5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc614),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0x10u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.sub(a,0x10u32,8); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x58c016); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc618),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0x1000u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x1000u32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x58c04c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc61c),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0x4000000u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4000000u32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x58c082); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc620),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0x400u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x400u32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x58c0b8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc624),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0x200u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x200u32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x58c0ee); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x58c0ee; continue;
            }
            0x58c0ee => {
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x190),32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(2,32); c.sub(a,0x3u32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            c.push(m,c.get(2,32));
            c.push(m, 0x58c108);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58f4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x524),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58c337; continue; }
            pc = 0x58c116; continue;
            }
            0x58c116 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x58c14b; continue; }
            pc = 0x58c126; continue;
            }
            0x58c126 => {
            c.set(0,32,0xaaaaaaabu32);
            { let b = rm(m,0x6dfbc4u32,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x14u32,32); }
            { let a = c.r[6].wrapping_add(0xc16c); wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x58c155; continue; }
            pc = 0x58c13f; continue;
            }
            0x58c13f => {
            { let a = c.r[6].wrapping_add(0xc16c); wm(m,a,32,0x14u32); }
            pc = 0x58c155; continue;
            }
            0x58c14b => {
            { let a = c.r[6].wrapping_add(0xc16c); wm(m,a,32,0x28u32); }
            pc = 0x58c155; continue;
            }
            0x58c155 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc170),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc16c),32));
            c.set(0,32,rm(m,0x6dccf8u32,32));
            c.set(2,32,c.r[7].wrapping_add(c.r[1]));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x58c337; continue; }
            pc = 0x58c171; continue;
            }
            0x58c171 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            { let a = c.r[6].wrapping_add(0xc170); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x58c196; continue; }
            pc = 0x58c191; continue;
            }
            0x58c191 => {
            c.set(0,32,0x5u32);
            pc = 0x58c196; continue;
            }
            0x58c196 => {
            c.set(7,32,rm(m,0x6e2d50u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x58c337; continue; }
            pc = 0x58c1a4; continue;
            }
            0x58c1a4 => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            pc = 0x58c1a8; continue;
            }
            0x58c1a8 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x58c2c8; continue; }
            pc = 0x58c1b0; continue;
            }
            0x58c1b0 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(1,32,c.r[7].wrapping_add(0x4));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x58c1be); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x58c1dc; continue; }
            pc = 0x58c1d4; continue;
            }
            0x58c1d4 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x58c2bd; continue; }
            pc = 0x58c1dc; continue;
            }
            0x58c1dc => {
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x18),32); c.idiv32(b); }
            c.set(3,32,c.get(0,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[4].wrapping_add(0x18),32); c.idiv32(b); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x58c1fc; continue; }
            pc = 0x58c1f4; continue;
            }
            0x58c1f4 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58c2bd; continue; }
            pc = 0x58c1fc; continue;
            }
            0x58c1fc => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x28),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[7].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x524),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58c25a; continue; }
            pc = 0x58c213; continue;
            }
            0x58c213 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc654),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x28),16));
            c.set(1,16,c.get(5,16));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xc650),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x58c25a; continue; }
            pc = 0x58c237; continue;
            }
            0x58c237 => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x2c),32),32); }
            if c.cond(15) { pc = 0x58c25a; continue; }
            pc = 0x58c23c; continue;
            }
            0x58c23c => {
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x58c25a; continue; }
            pc = 0x58c240; continue;
            }
            0x58c240 => {
            { let a = c.get(0,32); c.sub(a,rm(m,0x6dcce4u32,32),32); }
            if c.cond(13) { pc = 0x58c25a; continue; }
            pc = 0x58c248; continue;
            }
            0x58c248 => {
            c.push(m,0xffu32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x5a0));
            c.push(m, 0x58c25a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x59edf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x58c25a; continue;
            }
            0x58c25a => {
            { let a = rm(m,0x6dfbc4u32,32); c.sub(a,0xc8u32,32); }
            if c.cond(3) { pc = 0x58c2bd; continue; }
            pc = 0x58c266; continue;
            }
            0x58c266 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x524),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58c2bd; continue; }
            pc = 0x58c270; continue;
            }
            0x58c270 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x24),16));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc654),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xc650),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,16); let r = c.inc(a,16); c.set(0,16,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x28),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x58c2bd; continue; }
            pc = 0x58c29a; continue;
            }
            0x58c29a => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x2c),32),32); }
            if c.cond(15) { pc = 0x58c2bd; continue; }
            pc = 0x58c29f; continue;
            }
            0x58c29f => {
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x58c2bd; continue; }
            pc = 0x58c2a3; continue;
            }
            0x58c2a3 => {
            { let a = c.get(1,32); c.sub(a,rm(m,0x6dcce4u32,32),32); }
            if c.cond(13) { pc = 0x58c2bd; continue; }
            pc = 0x58c2ab; continue;
            }
            0x58c2ab => {
            c.push(m,0xffu32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x5a0));
            c.push(m, 0x58c2bd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x59edf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x58c2bd; continue;
            }
            0x58c2bd => {
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x30),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x58c1b0; continue; }
            pc = 0x58c2c8; continue;
            }
            0x58c2c8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc16c),32));
            c.set(3,32,c.r[6].wrapping_add(0x5a0));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x58c2dc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59e6e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcd04u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x58));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dcd20),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x58c31a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x59eb00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.sub(a,0x10000u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x58c1a8; continue; }
            pc = 0x58c337; continue;
            }
            0x58c337 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58c33e);
            { let args = []; c.r[0] = host.call(m, 0x5984e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58c345);
            { let args = []; c.r[0] = host.call(m, 0x596ee0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dccf8u32,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc62c),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x58c6b5; continue; }
            pc = 0x58c358; continue;
            }
            0x58c358 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xc594),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x58c6b5; continue; }
            pc = 0x58c366; continue;
            }
            0x58c366 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xc630),8));
            { let a = c.get(2,8); let r = c.add(a,0x5u32,8); c.set(2,8,r); }
            c.set(0,8,c.get(2,8));
            { let a = c.r[6].wrapping_add(0xc630); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,8); c.sub(a,0xdu32,8); }
            if c.cond(2) { pc = 0x58c39c; continue; }
            pc = 0x58c37b; continue;
            }
            0x58c37b => {
            { let a = c.get(0,8); let r = c.sub(a,0xdu32,8); c.set(0,8,r); }
            c.set(1,32,0xdu32);
            { let a = c.r[6].wrapping_add(0xc630); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc631),8));
            { let a = c.get(0,32); let r = c.add(a,0x3u32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0xc631); wm(m,a,8,c.get(2,8)); }
            pc = 0x58c39c; continue;
            }
            0x58c39c => {
            { let a = rm(m,c.r[6].wrapping_add(0xc324),32); c.sub(a,0xffu32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x58c63b; continue; }
            pc = 0x58c3d0; continue;
            }
            0x58c3d0 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.get(3,8); let r = c.dec(a,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x58c3df);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x9u32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc630),8));
            c.set(1,32,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,0x84210843u32);
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(1,8,c.get(3,8));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(3,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x4));
            c.push(m, 0x58c41d);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0x8000000fu32,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x58c429; continue; }
            pc = 0x58c424; continue;
            }
            0x58c424 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xfffffff0u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x58c429; continue;
            }
            0x58c429 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x1u32); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xc631),8));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x95a02569u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x14),8));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x6));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x58c47f);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x3u32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            c.set(0,8,c.get(2,8));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(2,8)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            c.set(2,32,sx(c.get(7,16),16)&0xffffffffu32);
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(15) { pc = 0x58c549; continue; }
            pc = 0x58c4ab; continue;
            }
            0x58c4ab => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x58c4b3; continue;
            }
            0x58c4b3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(c.r[5]));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x58c531; continue; }
            pc = 0x58c4c6; continue;
            }
            0x58c4c6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x58c4d4; continue;
            }
            0x58c4d4 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xc314),16));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc308),32));
            { let a = c.get(1,16); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x1c),16),16); c.set(1,16,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x58c50a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xeu32,32); }
            if c.cond(4) { pc = 0x58c514; continue; }
            pc = 0x58c50f; continue;
            }
            0x58c50f => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x58c514; continue;
            }
            0x58c514 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.set(0,32,c.r[5].wrapping_add(c.r[5]));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x58c4d4; continue; }
            pc = 0x58c531; continue;
            }
            0x58c531 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(2,32,sx(c.get(7,16),16)&0xffffffffu32);
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x58c4b3; continue; }
            pc = 0x58c53d; continue;
            }
            0x58c53d => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58c601; continue; }
            pc = 0x58c549; continue;
            }
            0x58c549 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x514),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            c.set(7,32,c.r[2].wrapping_add(0x1));
            if c.cond(4) { pc = 0x58c576; continue; }
            pc = 0x58c565; continue;
            }
            0x58c565 => {
            c.set(1,32,c.r[3].wrapping_add(0x1));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x510),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            pc = 0x58c594; continue;
            }
            0x58c576 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x510),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            pc = 0x58c594; continue;
            }
            0x58c594 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,0x12u32);
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x58c5ab);
            { let args = []; c.r[0] = host.call(m, 0x557d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x3f8));
            c.set(1,32,c.get(3,32));
            c.set(7,32,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m, 0x58c5bf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x568a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58c5cc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x568b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(5,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x50u32,32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,0x20u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x58c5eb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m, 0x58c5f0);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0x8000003fu32,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x58c5fc; continue; }
            pc = 0x58c5f7; continue;
            }
            0x58c5f7 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffc0u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x58c5fc; continue;
            }
            0x58c5fc => {
            { let a = c.get(0,8); let r = c.add(a,0x40u32,8); c.set(0,8,r); }
            { let a = c.r[7].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            pc = 0x58c601; continue;
            }
            0x58c601 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58c620; continue; }
            pc = 0x58c609; continue;
            }
            0x58c609 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x58c620; continue; }
            pc = 0x58c611; continue;
            }
            0x58c611 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x2u32); }
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            pc = 0x58c620; continue;
            }
            0x58c620 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x58c3d0; continue; }
            pc = 0x58c63b; continue;
            }
            0x58c63b => {
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.set(2,32,c.r[4].wrapping_add(0x13));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6e55d0u32);
            c.push(m, 0x58c64f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x613070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,0x6dccf8u32,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc62c),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x64u32,32); }
            if c.cond(7) { pc = 0x58c69c; continue; }
            pc = 0x58c664; continue;
            }
            0x58c664 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58c699; continue; }
            pc = 0x58c66c; continue;
            }
            0x58c66c => {
            c.set(3,8,rm(m,c.r[6].wrapping_add(0xc594),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x58c699; continue; }
            pc = 0x58c676; continue;
            }
            0x58c676 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x1356u32);
            c.set(7,32,c.r[2].wrapping_mul(4).wrapping_add(0x5f));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xc62c); wm(m,a,32,c.get(0,32)); }
            pc = 0x58c6a2; continue;
            }
            0x58c699 => {
            { let a = c.get(2,32); let r = c.add(a,0x32u32,32); c.set(2,32,r); }
            pc = 0x58c69c; continue;
            }
            0x58c69c => {
            { let a = c.r[6].wrapping_add(0xc62c); wm(m,a,32,c.get(2,32)); }
            pc = 0x58c6a2; continue;
            }
            0x58c6a2 => {
            c.set(0,32,rm(m,0x6dccf8u32,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc62c),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x58c358; continue; }
            pc = 0x58c6b5; continue;
            }
            0x58c6b5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc62c),32));
            { let a = c.get(1,32); let r = c.sub(a,0x32u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x58c6cb; continue; }
            pc = 0x58c6c2; continue;
            }
            0x58c6c2 => {
            { let a = c.get(0,32); let r = c.add(a,0x32u32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xc62c); wm(m,a,32,c.get(0,32)); }
            pc = 0x58c6cb; continue;
            }
            0x58c6cb => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc594),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58c865; continue; }
            pc = 0x58c6d9; continue;
            }
            0x58c6d9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x58c720; continue; }
            pc = 0x58c6e9; continue;
            }
            0x58c6e9 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x898),16));
            { let a = c.get(0,16); c.sub(a,0x3e8u32,16); }
            if c.cond(6) { pc = 0x58c70b; continue; }
            pc = 0x58c704; continue;
            }
            0x58c704 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0xfu32); }
            pc = 0x58c725; continue;
            }
            0x58c70b => {
            c.set(1,16,0x1f4u32);
            { let a = c.get(1,16); c.sub(a,c.get(0,16),16); }
            { let a = c.get(2,8); let r = c.sbb(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x5u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.add(a,0x5u32,8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            pc = 0x58c725; continue;
            }
            0x58c720 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x28u32); }
            pc = 0x58c725; continue;
            }
            0x58c725 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc590),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x58c865; continue; }
            pc = 0x58c73c; continue;
            }
            0x58c73c => {
            c.push(m, 0x58c741);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x75u32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(3,32,c.get(2,32));
            { let a = c.get(3,8); let r = c.add(a,0x4u32,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x58c757);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0xd0u32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc314),16));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(3,8));
            { let a = c.get(2,8); let r = c.add(a,0x6u32,8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(2,8)); }
            c.set(2,16,c.get(2,8));
            { let a = c.get(0,16); let r = c.imul(a,c.get(2,16),16); c.set(0,16,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc308),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x58c7a0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xeu32,32); }
            if c.cond(5) { pc = 0x58c865; continue; }
            pc = 0x58c7a9; continue;
            }
            0x58c7a9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x58c7da; continue; }
            pc = 0x58c7c0; continue;
            }
            0x58c7c0 => {
            c.set(7,32,c.r[1].wrapping_add(0x1));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x514),32));
            { let a = c.get(7,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x510),32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(0x1));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            pc = 0x58c801; continue;
            }
            0x58c7da => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x510),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x514),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(0x1));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            pc = 0x58c801; continue;
            }
            0x58c801 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            c.push(m, 0x58c80a);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.and(a,0xc0u32,8); c.set(0,8,r); }
            c.push(m,0x1bu32);
            { let a = c.get(0,8); let r = c.add(a,0x20u32,8); c.set(0,8,r); }
            { let a = c.get(3,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(7,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x58c827);
            { let args = []; c.r[0] = host.call(m, 0x557d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x3f8));
            c.set(1,32,c.get(7,32));
            c.set(3,32,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m, 0x58c83b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x568a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x58c846);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x568b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x58c85f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = rm(m,c.r[6].wrapping_add(0xc590),32); let r = c.inc(a,32); { let a = c.r[6].wrapping_add(0xc590); wm(m,a,32,r); } }
            pc = 0x58c865; continue;
            }
            0x58c865 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x58cb58; continue; }
            pc = 0x58c879; continue;
            }
            0x58c879 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2c8),32); c.and(a,0x4000000u32,32); }
            if c.cond(4) { pc = 0x58cb51; continue; }
            pc = 0x58c897; continue;
            }
            0x58c897 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc63c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x58cb58; continue; }
            pc = 0x58c8a5; continue;
            }
            0x58c8a5 => {
            { let a = c.r[6].wrapping_add(0xc63c); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xfu32); }
            pc = 0x58c8bc; continue;
            }
            0x58c8bc => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0xfu32); }
            pc = 0x58c8cc; continue;
            }
            0x58c8cc => {
            c.push(m, 0x58c8d1);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x7u32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,0xe070381du32);
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(3,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x3));
            c.push(m, 0x58c908);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0xdu32);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x1u32); }
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,0xe070381du32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x6));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc594),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x58c95a; continue; }
            pc = 0x58c949; continue;
            }
            0x58c949 => {
            c.push(m, 0x58c94e);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x3u32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            pc = 0x58c95c; continue;
            }
            0x58c95a => {
            c.set(2,8,0x1u32);
            pc = 0x58c95c; continue;
            }
            0x58c95c => {
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,c.get(2,8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,sx(c.get(1,16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(15) { pc = 0x58ca41; continue; }
            pc = 0x58c986; continue;
            }
            0x58c986 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            pc = 0x58c98e; continue;
            }
            0x58c98e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x58ca1d; continue; }
            pc = 0x58c9a3; continue;
            }
            0x58c9a3 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(5,32,c.r[1].wrapping_add(c.r[2]));
            pc = 0x58c9ae; continue;
            }
            0x58c9ae => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc314),16));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,16); let r = c.imul(a,c.get(5,16),16); c.set(0,16,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc308),32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[0]),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x50),32); c.push(m, 0x58c9ea); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xeu32,32); }
            if c.cond(5) { pc = 0x58ca01; continue; }
            pc = 0x58c9ef; continue;
            }
            0x58c9ef => {
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xc30c),32));
            { let a = rm(m,c.r[7].wrapping_add(c.r[1]),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x58ca06; continue; }
            pc = 0x58ca01; continue;
            }
            0x58ca01 => {
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            pc = 0x58ca06; continue;
            }
            0x58ca06 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x58c9ae; continue; }
            pc = 0x58ca1d; continue;
            }
            0x58ca1d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x58c98e; continue; }
            pc = 0x58ca35; continue;
            }
            0x58ca35 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58cb1b; continue; }
            pc = 0x58ca41; continue;
            }
            0x58ca41 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x58ca6e; continue; }
            pc = 0x58ca54; continue;
            }
            0x58ca54 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x514),32));
            c.set(5,32,c.r[3].wrapping_add(0x1));
            { let a = c.get(5,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x510),32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(0x1));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            pc = 0x58ca95; continue;
            }
            0x58ca6e => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x510),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4d0),32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x514),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(0x1));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4d4),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            pc = 0x58ca95; continue;
            }
            0x58ca95 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0x12u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(3,32,c.get(0,32));
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58cab0);
            { let args = []; c.r[0] = host.call(m, 0x557d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x3f8));
            c.set(7,32,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m, 0x58cac2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x568a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[6].wrapping_add(0x3f8));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x58cad3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x568b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let r = c.add(a,0xc7u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(0,32));
            c.push(m,0x20u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x58caf4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0x80u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x58cb02);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x57af40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,0xbb8u32);
            c.set(0,32,c.r[2].wrapping_add(c.r[5]));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(2,32); let r = c.add(a,rm(m,0x6dccf8u32,32),32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            pc = 0x58cb1b; continue;
            }
            0x58cb1b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.add(a,0xau32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x58c8cc; continue; }
            pc = 0x58cb35; continue;
            }
            0x58cb35 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.add(a,0x14u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x58c8bc; continue; }
            pc = 0x58cb4f; continue;
            }
            0x58cb4f => {
            pc = 0x58cb58; continue;
            }
            0x58cb51 => {
            { let a = c.r[6].wrapping_add(0xc63c); wm(m,a,8,0x0u32); }
            pc = 0x58cb58; continue;
            }
            0x58cb58 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58cb5f);
            { let args = []; c.r[0] = host.call(m, 0x592000, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc5fc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc5f8),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x58cb72);
            { let args = []; c.r[0] = host.call(m, 0x581890, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x58cb9c; continue; }
            pc = 0x58cb84; continue;
            }
            0x58cb84 => {
            c.set(7,32,rm(m,0x6e31e0u32,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x58cb9c; continue; }
            pc = 0x58cb8e; continue;
            }
            0x58cb8e => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x58cb95);
            { let args = []; c.r[0] = host.call(m, 0x580b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x1c),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x58cb8e; continue; }
            pc = 0x58cb9c; continue;
            }
            0x58cb9c => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58cba3);
            { let args = []; c.r[0] = host.call(m, 0x594770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x3f8));
            c.push(m, 0x58cbae);
            { let args = []; c.r[0] = host.call(m, 0x568c80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58cbb5);
            { let args = []; c.r[0] = host.call(m, 0x58aa30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58cbbc);
            { let args = []; c.r[0] = host.call(m, 0x59baf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58cbc3);
            { let args = []; c.r[0] = host.call(m, 0x599690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(3) { pc = 0x58cbe1; continue; }
            pc = 0x58cbd5; continue;
            }
            0x58cbd5 => {
            c.set(1,32,0x6e6220u32);
            c.push(m, 0x58cbdf);
            { let args = []; c.r[0] = host.call(m, 0x614080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x58cbfd; continue;
            }
            0x58cbe1 => {
            c.set(0,32,rm(m,0x6e59ccu32,32));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x58cbfd; continue; }
            pc = 0x58cbea; continue;
            }
            0x58cbea => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6e55d0),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x20),32); c.push(m, 0x58cbfd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x58cbfd; continue;
            }
            0x58cbfd => {
            c.push(m, 0x58cc02);
            { let args = []; c.r[0] = host.call(m, 0x56db10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m, 0x58cc07);
            { let args = []; c.r[0] = host.call(m, 0x56ce40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc304),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x58cc18; continue; }
            pc = 0x58cc11; continue;
            }
            0x58cc11 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58cc18);
            { let args = []; c.r[0] = host.call(m, 0x598d00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x58cc18; continue;
            }
            0x58cc18 => {
            c.set(0,8,rm(m,0x6dcbe0u32,8));
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = 0x6dcbe0u32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x58cc5b; continue; }
            pc = 0x58cc34; continue;
            }
            0x58cc34 => {
            c.push(m, 0x58cc39);
            { let args = []; c.r[0] = host.call(m, 0x462310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x58cc5b; continue; }
            pc = 0x58cc3d; continue;
            }
            0x58cc3d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x58cc5b; continue; }
            pc = 0x58cc4d; continue;
            }
            0x58cc4d => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x58cc54);
            { let args = []; c.r[0] = host.call(m, 0x59ae40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6dcbe0u32; wm(m,a,8,0x0u32); }
            pc = 0x58cc5b; continue;
            }
            0x58cc5b => {
            c.set(0,32,rm(m,0x6df270u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x58cc8b; continue; }
            pc = 0x58cc64; continue;
            }
            0x58cc64 => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x58cc8b; continue; }
            pc = 0x58cc74; continue;
            }
            0x58cc74 => {
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x58cc8b; continue; }
            pc = 0x58cc86; continue;
            }
            0x58cc86 => {
            c.push(m, 0x58cc8b);
            { let args = []; c.r[0] = host.call(m, 0x631a50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x58cc8b; continue;
            }
            0x58cc8b => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x58u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_streak_line_invalidate_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x575d10;
    loop {
        match pc {
            0x575d10 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,sx(rm(m,c.r[1].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,sx(rm(m,c.r[1].wrapping_add(0x23),8),8)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[1].wrapping_add(0x21),8),8)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x575d38); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x575d41);
            { let args = []; c.r[0] = host.call(m, 0x6592a3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0xc650),32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0xc654),32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x575d6a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.get(6,32); let r = c.neg(a,32); c.set(6,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a4c8u32,32); c.push(m, 0x575d7b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m, 0x575d8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_streak_line_is_on_screen(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x575e60;
    loop {
        match pc {
            0x575e60 => {
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.set(0,32,sx(rm(m,c.r[1].wrapping_add(0x12),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,sx(rm(m,c.r[1].wrapping_add(0x23),8),8)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[1].wrapping_add(0x21),8),8)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a4d8u32,32); c.push(m, 0x575e89); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x575e92);
            { let args = []; c.r[0] = host.call(m, 0x6592a3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0xc650),32));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0xc654),32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x575ebb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x575eca); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(6,32)); }
            { let target = rm(m,0x66a4e8u32,32); c.push(m, 0x575ef1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,u32::from(c.cond(5)));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "text_layout_base_add_segment" => text_layout_base_add_segment(m, host, regs),
        "scrolling_scene_buffer_blit_wrapped" => scrolling_scene_buffer_blit_wrapped(m, host, regs),
        "scrolling_scene_buffer_add_dirty_rect_wrapped" => scrolling_scene_buffer_add_dirty_rect_wrapped(m, host, regs),
        "class22_reset" => class22_reset(m, host, regs),
        "class22_resize_view" => class22_resize_view(m, host, regs),
        "class22_update_scene_buffer_cells_and_animations" => class22_update_scene_buffer_cells_and_animations(m, host, regs),
        "class22_set_size" => class22_set_size(m, host, regs),
        "class22_apply_display_options" => class22_apply_display_options(m, host, regs),
        "load_gl_cfg_and_init_graphics" => load_gl_cfg_and_init_graphics(m, host, regs),
        "std_map_erase_subtree" => std_map_erase_subtree(m, host, regs),
        "convoy_choice_window_layout" => convoy_choice_window_layout(m, host, regs),
        "building_scene_window_draw" => building_scene_window_draw(m, host, regs),
        "town_view_ship_set_pose" => town_view_ship_set_pose(m, host, regs),
        "harbor_ship_sprite_get_name_label_rect" => harbor_ship_sprite_get_name_label_rect(m, host, regs),
        "map_object_table_rebuild_kind_index" => map_object_table_rebuild_kind_index(m, host, regs),
        "town_view_enter_battle_location" => town_view_enter_battle_location(m, host, regs),
        "town_view_switch_to_town" => town_view_switch_to_town(m, host, regs),
        "town_view_free_path_tables" => town_view_free_path_tables(m, host, regs),
        "town_view_unload_scene" => town_view_unload_scene(m, host, regs),
        "town_view_update_frame" => town_view_update_frame(m, host, regs),
        "town_streak_line_invalidate_rect" => town_streak_line_invalidate_rect(m, host, regs),
        "town_streak_line_is_on_screen" => town_streak_line_is_on_screen(m, host, regs),
        _ => return None,
    })
}
