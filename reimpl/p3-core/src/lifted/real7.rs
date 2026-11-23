//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn new_game_screen_process_input(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x498760;
    loop {
        match pc {
            0x498760 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x660aa7u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x4cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,8,rm(m,0x6d1c78u32,8));
            c.set(0,8,0x1u32);
            c.push(m,c.get(7,32));
            { let a = c.get(0,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(6,32)); }
            if c.cond(5) { pc = 0x4987a5; continue; }
            pc = 0x498792; continue;
            }
            0x498792 => {
            { let a = c.get(1,8); let r = c.or(a,c.get(0,8),8); c.set(1,8,r); }
            { let a = 0x6d1c78u32; wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bb0),32));
            { let a = 0x6d1c74u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x4987a5; continue;
            }
            0x4987a5 => {
            c.set(7,32,c.r[6].wrapping_add(0xaec));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,0x0u32); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(1,32)); }
            pc = 0x4987b7; continue;
            }
            0x4987b7 => {
            c.push(m, 0x4987bc);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            if c.cond(5) { pc = 0x4987d8; continue; }
            pc = 0x4987c3; continue;
            }
            0x4987c3 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xe8u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x12u32,32); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x4987b7; continue; }
            pc = 0x4987d8; continue;
            }
            0x4987d8 => {
            { let a = c.get(0,32); c.sub(a,0x11u32,32); }
            if c.cond(7) { pc = 0x498e60; continue; }
            pc = 0x4987e1; continue;
            }
            0x4987e1 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x499960)); continue;
            }
            0x4987e8 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x46c); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4987fb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbd4),32));
            c.set(3,32,c.r[6].wrapping_add(0xbd4));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49880e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498e53; continue;
            }
            0x498813 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x46c); wm(m,a,32,0x1u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498826); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbd4),32));
            c.set(3,32,c.r[6].wrapping_add(0xbd4));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x498839); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498e53; continue;
            }
            0x49883e => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x468),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m, 0x498852);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.r[6].wrapping_add(0x468); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x49886d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x468),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x49888d; continue; }
            pc = 0x49887c; continue;
            }
            0x49887c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcbc),32));
            c.set(1,32,c.r[6].wrapping_add(0xcbc));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x49888d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49888d; continue;
            }
            0x49888d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xda4),32));
            c.set(7,32,c.r[6].wrapping_add(0xda4));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4988a0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xcbc));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            pc = 0x498e55; continue;
            }
            0x4988ad => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x468),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m, 0x4988c1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544e00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.r[6].wrapping_add(0x468); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4988dc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544e00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x468),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4988fc; continue; }
            pc = 0x4988eb; continue;
            }
            0x4988eb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xda4),32));
            c.set(1,32,c.r[6].wrapping_add(0xda4));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4988fc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4988fc; continue;
            }
            0x4988fc => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcbc),32));
            c.set(7,32,c.r[6].wrapping_add(0xcbc));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49890f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xda4));
            c.push(m,c.get(0,32));
            pc = 0x498e54; continue;
            }
            0x49891b => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ba0),32));
            { let a = c.get(2,32); let r = c.add(a,0xfffffffbu32,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x1ba0); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.sub(a,0x514u32,32); }
            if c.cond(15) { pc = 0x49894e; continue; }
            pc = 0x498933; continue;
            }
            0x498933 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xe8c),32));
            c.set(1,32,c.r[6].wrapping_add(0xe8c));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x1ba0); wm(m,a,32,0x514u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49894e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49894e; continue;
            }
            0x49894e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf74),32));
            c.set(7,32,c.r[6].wrapping_add(0xf74));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x498961); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xe8c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            pc = 0x498e55; continue;
            }
            0x49896e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ba0),32));
            { let a = c.get(2,32); let r = c.add(a,0x5u32,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x1ba0); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.sub(a,0x578u32,32); }
            if c.cond(12) { pc = 0x4989a1; continue; }
            pc = 0x498986; continue;
            }
            0x498986 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xf74),32));
            c.set(1,32,c.r[6].wrapping_add(0xf74));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x1ba0); wm(m,a,32,0x578u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4989a1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4989a1; continue;
            }
            0x4989a1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe8c),32));
            c.set(7,32,c.r[6].wrapping_add(0xe8c));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4989b4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xf74));
            c.push(m,c.get(1,32));
            pc = 0x498e54; continue;
            }
            0x4989c0 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1ba4),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x1ba4); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x4989ee; continue; }
            pc = 0x4989d3; continue;
            }
            0x4989d3 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x122c),32));
            c.set(1,32,c.r[6].wrapping_add(0x122c));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x1ba4); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4989ee); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4989ee; continue;
            }
            0x4989ee => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1ba4),32));
            c.push(m,0x20u32);
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x66f608u32,32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1b80));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x498a0b);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1314),32));
            c.set(7,32,c.r[6].wrapping_add(0x1314));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498a21); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x122c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            pc = 0x498a95; continue;
            }
            0x498a2b => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ba4),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x1ba4); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x498a5a; continue; }
            pc = 0x498a3f; continue;
            }
            0x498a3f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1314),32));
            c.set(1,32,c.r[6].wrapping_add(0x1314));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x1ba4); wm(m,a,32,0x4u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498a5a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498a5a; continue;
            }
            0x498a5a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1ba4),32));
            c.push(m,0x20u32);
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x66f608u32,32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1b80));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x498a77);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x122c),32));
            c.set(7,32,c.r[6].wrapping_add(0x122c));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498a8d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x1314));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            pc = 0x498a95; continue;
            }
            0x498a95 => {
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.push(m, 0x498aa0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x460680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x69a918u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x628));
            c.push(m, 0x498ab2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498e60; continue;
            }
            0x498ab7 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x1ba8),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            c.set(1,32,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x1ba8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x498ad6; continue; }
            pc = 0x498ace; continue;
            }
            0x498ace => {
            { let a = c.get(0,32); let r = c.sub(a,0x5u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x498b3e; continue; }
            pc = 0x498ad3; continue;
            }
            0x498ad3 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x498adc; continue; }
            pc = 0x498ad6; continue;
            }
            0x498ad6 => {
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,c.get(2,32)); }
            pc = 0x498adc; continue;
            }
            0x498adc => {
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x498ae7; continue; }
            pc = 0x498ae1; continue;
            }
            0x498ae1 => {
            { let a = c.r[6].wrapping_add(0x1ba8); wm(m,a,32,c.get(2,32)); }
            pc = 0x498ae7; continue;
            }
            0x498ae7 => {
            { let a = rm(m,c.r[6].wrapping_add(0x1ba8),32); c.sub(a,c.get(2,32),32); }
            if c.cond(15) { pc = 0x498b05; continue; }
            pc = 0x498aef; continue;
            }
            0x498aef => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x13fc),32));
            c.set(1,32,c.r[6].wrapping_add(0x13fc));
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x1ba8); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x498b05); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498b05; continue;
            }
            0x498b05 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14e4),32));
            c.set(7,32,c.r[6].wrapping_add(0x14e4));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498b18); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,c.r[6].wrapping_add(0x13fc));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x498b2b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x460680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[6].wrapping_add(0x16b4));
            c.set(2,32,c.r[6].wrapping_add(0x15cc));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x498bf6; continue;
            }
            0x498b3e => {
            c.set(0,32,rm(m,0x6d1c74u32,32));
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,c.get(0,32)); }
            pc = 0x498ae7; continue;
            }
            0x498b4b => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1ba8),32));
            c.set(1,32,0x6u32);
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x1ba8); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x498ba3; continue; }
            pc = 0x498b64; continue;
            }
            0x498b64 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4999a8)); continue;
            }
            0x498b6b => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1bb0),32));
            c.set(0,32,0xdu32);
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x498ba3; continue; }
            pc = 0x498b7a; continue;
            }
            0x498b7a => {
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,c.get(0,32)); }
            pc = 0x498ba3; continue;
            }
            0x498b82 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bb0),32));
            { let a = 0x6d1c74u32; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,0x0u32); }
            pc = 0x498ba3; continue;
            }
            0x498b99 => {
            { let a = c.r[6].wrapping_add(0x1ba8); wm(m,a,32,0x2u32); }
            pc = 0x498ba3; continue;
            }
            0x498ba3 => {
            { let a = rm(m,c.r[6].wrapping_add(0x1ba8),32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x498bc2; continue; }
            pc = 0x498bab; continue;
            }
            0x498bab => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14e4),32));
            { let a = c.r[6].wrapping_add(0x1ba8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x14e4));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498bc2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498bc2; continue;
            }
            0x498bc2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x13fc),32));
            c.set(7,32,c.r[6].wrapping_add(0x13fc));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x498bd5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x14e4));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x498be8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x460680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,c.r[6].wrapping_add(0x16b4));
            c.set(0,32,c.r[6].wrapping_add(0x15cc));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x498bf6; continue;
            }
            0x498bf6 => {
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.push(m, 0x498c01);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x460680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x498c08);
            { let args = []; c.r[0] = host.call(m, 0x499af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x498e60; continue;
            }
            0x498c0d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1bb0),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x498c3b; continue; }
            pc = 0x498c20; continue;
            }
            0x498c20 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x15cc),32));
            c.set(1,32,c.r[6].wrapping_add(0x15cc));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498c3b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498c3b; continue;
            }
            0x498c3b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x16b4),32));
            c.set(7,32,c.r[6].wrapping_add(0x16b4));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x498c4e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x15cc));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            pc = 0x498e55; continue;
            }
            0x498c5b => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1bb0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1ba8),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,0x7u32,32); }
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,c.get(7,32));
            if c.cond(7) { pc = 0x498d09; continue; }
            pc = 0x498c79; continue;
            }
            0x498c79 => {
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x4999c4)); continue;
            }
            0x498c80 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x498d09; continue; }
            pc = 0x498c88; continue;
            }
            0x498c88 => {
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,0x0u32); }
            pc = 0x498cf8; continue;
            }
            0x498c94 => {
            { let a = c.get(0,32); c.sub(a,0xeu32,32); }
            if c.cond(12) { pc = 0x498d09; continue; }
            pc = 0x498c99; continue;
            }
            0x498c99 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x16b4),32));
            c.set(1,32,c.r[6].wrapping_add(0x16b4));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,0xeu32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x498cb4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498d09; continue;
            }
            0x498cb6 => {
            { let a = c.get(0,32); c.sub(a,0xdu32,32); }
            if c.cond(12) { pc = 0x498d09; continue; }
            pc = 0x498cbb; continue;
            }
            0x498cbb => {
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,0xdu32); }
            pc = 0x498cf8; continue;
            }
            0x498cc7 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(12) { pc = 0x498d09; continue; }
            pc = 0x498ccc; continue;
            }
            0x498ccc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x16b4),32));
            c.set(1,32,c.r[6].wrapping_add(0x16b4));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,0x2u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x498ce7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498d09; continue;
            }
            0x498ce9 => {
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x498d09; continue; }
            pc = 0x498cee; continue;
            }
            0x498cee => {
            { let a = c.r[6].wrapping_add(0x1bb0); wm(m,a,32,0x4u32); }
            pc = 0x498cf8; continue;
            }
            0x498cf8 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x16b4),32));
            c.set(1,32,c.r[6].wrapping_add(0x16b4));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498d09); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498d09; continue;
            }
            0x498d09 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1ba8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x498d24; continue; }
            pc = 0x498d13; continue;
            }
            0x498d13 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x15cc),32));
            c.set(1,32,c.r[6].wrapping_add(0x15cc));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x498d24); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498d24; continue;
            }
            0x498d24 => {
            c.set(1,32,c.r[6].wrapping_add(0x16b4));
            c.set(2,32,c.r[6].wrapping_add(0x15cc));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x498e55; continue;
            }
            0x498d37 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bac),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x498d5e; continue; }
            pc = 0x498d41; continue;
            }
            0x498d41 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x179c),32));
            c.set(1,32,c.r[6].wrapping_add(0x179c));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x1bac); wm(m,a,32,0x5u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x498d5c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498d65; continue;
            }
            0x498d5e => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x1bac); wm(m,a,32,c.get(0,32)); }
            pc = 0x498d65; continue;
            }
            0x498d65 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1884),32));
            c.set(7,32,c.r[6].wrapping_add(0x1884));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498d78); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x179c));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            pc = 0x498e55; continue;
            }
            0x498d85 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bac),32));
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            if c.cond(5) { pc = 0x498d9c; continue; }
            pc = 0x498d90; continue;
            }
            0x498d90 => {
            { let a = c.r[6].wrapping_add(0x1bac); wm(m,a,32,0x0u32); }
            pc = 0x498dc3; continue;
            }
            0x498d9c => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            { let a = c.r[6].wrapping_add(0x1bac); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x498dc3; continue; }
            pc = 0x498da8; continue;
            }
            0x498da8 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1884),32));
            c.set(1,32,c.r[6].wrapping_add(0x1884));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x1bac); wm(m,a,32,0x4u32); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498dc3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498dc3; continue;
            }
            0x498dc3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x179c),32));
            c.set(7,32,c.r[6].wrapping_add(0x179c));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x498dd6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x1884));
            c.push(m,c.get(1,32));
            pc = 0x498e54; continue;
            }
            0x498ddf => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x474),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x498df1; continue; }
            pc = 0x498de9; continue;
            }
            0x498de9 => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x474); wm(m,a,8,c.get(0,8)); }
            pc = 0x498df1; continue;
            }
            0x498df1 => {
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x474),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x196c),32));
            c.set(7,32,c.r[6].wrapping_add(0x196c));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(0,8,u32::from(c.cond(7)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498e0e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            pc = 0x498e42; continue;
            }
            0x498e12 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x474),8));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(3) { pc = 0x498e24; continue; }
            pc = 0x498e1c; continue;
            }
            0x498e1c => {
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x474); wm(m,a,8,c.get(0,8)); }
            pc = 0x498e24; continue;
            }
            0x498e24 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x196c),32));
            c.set(7,32,c.r[6].wrapping_add(0x196c));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x498e37); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0x474),8); c.sub(a,0x2u32,8); }
            c.set(0,8,u32::from(c.cond(2)));
            c.push(m,c.get(0,32));
            pc = 0x498e42; continue;
            }
            0x498e42 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1a54),32));
            c.set(3,32,c.r[6].wrapping_add(0x1a54));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498e53); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498e53; continue;
            }
            0x498e53 => {
            c.push(m,c.get(3,32));
            pc = 0x498e54; continue;
            }
            0x498e54 => {
            c.push(m,c.get(7,32));
            pc = 0x498e55; continue;
            }
            0x498e55 => {
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.push(m, 0x498e60);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x460680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x498e60; continue;
            }
            0x498e60 => {
            c.set(7,32,c.r[6].wrapping_add(0x628));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x498e6d);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x499014; continue; }
            pc = 0x498e75; continue;
            }
            0x498e75 => {
            c.set(1,32,rm(m,0x66e0a0u32,32));
            c.push(m,c.get(1,32));
            c.push(m,0x6d1c90u32);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x498e8b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x498e96);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,0x6d1c90u32);
            c.push(m, 0x498ea2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x49b180, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6d1c90u32);
            c.push(m, 0x498eac);
            { let args = []; c.r[0] = host.call(m, 0x4b70c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            c.push(m,0x3u32);
            if c.cond(5) { pc = 0x498ec4; continue; }
            pc = 0x498eb3; continue;
            }
            0x498eb3 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9f8),32));
            c.set(1,32,c.r[6].wrapping_add(0x9f8));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x498ec2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498ed3; continue;
            }
            0x498ec4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x904),32));
            c.set(1,32,c.r[6].wrapping_add(0x904));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x498ed3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x498ed3; continue;
            }
            0x498ed3 => {
            c.set(1,32,rm(m,0x66e09cu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x498ee5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6d1c90u32);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x498ef4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x498efd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x1b80));
            c.set(1,32,0x8u32);
            pc = 0x498f0a; continue;
            }
            0x498f0a => {
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x498f0a; continue; }
            pc = 0x498f14; continue;
            }
            0x498f14 => {
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,0x24u32); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x498f1d; continue;
            }
            0x498f1d => {
            c.set(2,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x66f6b0),32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x498f2e);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x498f55; continue; }
            pc = 0x498f38; continue;
            }
            0x498f38 => {
            c.set(0,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x66f6b0),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x498f49);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1ba4); wm(m,a,32,c.get(3,32)); }
            pc = 0x498f55; continue;
            }
            0x498f55 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x5u32,32); }
            if c.cond(12) { pc = 0x498f1d; continue; }
            pc = 0x498f5b; continue;
            }
            0x498f5b => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x1ba4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1314),32));
            c.set(1,32,c.r[6].wrapping_add(0x1314));
            { let a = c.get(3,32); c.sub(a,0x4u32,32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498f77); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1ba4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x122c),32));
            c.set(1,32,c.r[6].wrapping_add(0x122c));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x498f92); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1ba4),32));
            c.push(m,0x20u32);
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x66f608u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x1b80));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x498fb0);
            { let args = []; c.r[0] = host.call(m, 0x63b3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x499002; continue; }
            pc = 0x498fb7; continue;
            }
            0x498fb7 => {
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x69a918u32,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            c.push(m, 0x498fd5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6baa14u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m, 0x498fe2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x628));
            c.push(m, 0x498ff1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x499000);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x499014; continue;
            }
            0x499002 => {
            c.set(2,32,rm(m,0x69a918u32,32));
            c.set(1,32,c.r[6].wrapping_add(0x628));
            c.push(m,c.get(2,32));
            c.push(m, 0x499014);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x499014; continue;
            }
            0x499014 => {
            c.set(7,32,c.r[6].wrapping_add(0x810));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x499021);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4990cd; continue; }
            pc = 0x499029; continue;
            }
            0x499029 => {
            c.push(m,0x0u32);
            c.push(m,0x6d2f60u32);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x49903a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x499045);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,0x6d2f60u32);
            c.push(m, 0x499051);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x49c0d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6d2f60u32);
            c.push(m, 0x49905b);
            { let args = []; c.r[0] = host.call(m, 0x4b70c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            c.push(m,0x3u32);
            if c.cond(5) { pc = 0x499073; continue; }
            pc = 0x499062; continue;
            }
            0x499062 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9f8),32));
            c.set(1,32,c.r[6].wrapping_add(0x9f8));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x499071); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x499082; continue;
            }
            0x499073 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x904),32));
            c.set(1,32,c.r[6].wrapping_add(0x904));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x499082); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x499082; continue;
            }
            0x499082 => {
            c.set(0,32,rm(m,0x66e09cu32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x499093);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6d2f60u32);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x4990a2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x4990ab); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9f8),32));
            c.set(1,32,c.r[6].wrapping_add(0x9f8));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x4990bc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x904),32));
            c.set(1,32,c.r[6].wrapping_add(0x904));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x4990cd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4990cd; continue;
            }
            0x4990cd => {
            c.set(1,32,c.r[6].wrapping_add(0x71c));
            c.push(m, 0x4990d8);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x49921b; continue; }
            pc = 0x4990e0; continue;
            }
            0x4990e0 => {
            c.set(7,32,c.r[6].wrapping_add(0x1bc0));
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x4990f3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x4990fe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x499105);
            { let args = []; c.r[0] = host.call(m, 0x435130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x49910f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            c.push(m,0x3u32);
            if c.cond(5) { pc = 0x499127; continue; }
            pc = 0x499116; continue;
            }
            0x499116 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9f8),32));
            c.set(1,32,c.r[6].wrapping_add(0x9f8));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x499125); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x499136; continue;
            }
            0x499127 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x904),32));
            c.set(1,32,c.r[6].wrapping_add(0x904));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x499136); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x499136; continue;
            }
            0x499136 => {
            c.set(1,32,rm(m,0x66e09cu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x499148);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x499153);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x470),32));
            c.set(7,32,c.r[6].wrapping_add(0x470));
            c.push(m,0x6d1c82u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x49916a);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.set(1,32,c.get(4,32));
            if c.cond(5) { pc = 0x49917e; continue; }
            pc = 0x499177; continue;
            }
            0x499177 => {
            c.push(m,0x6ba7e0u32);
            pc = 0x49919d; continue;
            }
            0x49917e => {
            c.push(m,c.get(7,32));
            c.push(m, 0x499184);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x49918e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x549940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4991ac; continue; }
            pc = 0x499192; continue;
            }
            0x499192 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6ba7ecu32);
            pc = 0x49919d; continue;
            }
            0x49919d => {
            c.push(m, 0x4991a2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4991ac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x549940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4991ac; continue;
            }
            0x4991ac => {
            c.set(0,32,rm(m,0x6df2fcu32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x468),32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[0],32),32); }
            if c.cond(2) { pc = 0x4991c5; continue; }
            pc = 0x4991bb; continue;
            }
            0x4991bb => {
            { let a = c.r[6].wrapping_add(0x468); wm(m,a,32,0x0u32); }
            pc = 0x4991c5; continue;
            }
            0x4991c5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x468),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m, 0x4991d7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544e00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x468); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4991ee);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9f8),32));
            c.set(1,32,c.r[6].wrapping_add(0x9f8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,0x1u32);
            { let a = c.r[6].wrapping_add(0x468); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x49920a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x904),32));
            c.set(1,32,c.r[6].wrapping_add(0x904));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x49921b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49921b; continue;
            }
            0x49921b => {
            c.set(7,32,c.r[6].wrapping_add(0x9f8));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x499228);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x499252; continue; }
            pc = 0x49922c; continue;
            }
            0x49922c => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x499235); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49923c);
            { let args = []; c.r[0] = host.call(m, 0x499be0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0xfffffffeu32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499252 => {
            c.set(7,32,c.r[6].wrapping_add(0x904));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x49925f);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x499948; continue; }
            pc = 0x499267; continue;
            }
            0x499267 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49926e);
            { let args = []; c.r[0] = host.call(m, 0x499be0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bb4),32));
            c.set(3,32,0x3u32);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x499288; continue; }
            pc = 0x49927d; continue;
            }
            0x49927d => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x499288);
            { let args = []; c.r[0] = host.call(m, 0x4429d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x499288; continue;
            }
            0x499288 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x2c),32); c.push(m, 0x499291); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1ba0),32));
            { let a = c.r[2]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1ba8),8));
            { let a = c.r[1].wrapping_add(0xf); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(0,8,rm(m,0x66e920u32,8));
            { let a = c.r[2].wrapping_add(0xb); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1b9c),32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x66f6a4),8));
            { let a = c.r[0].wrapping_add(0xa); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x474),8));
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.r[2].wrapping_add(0x19); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1ba4),8));
            { let a = c.r[1].wrapping_add(0xe); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[2].wrapping_add(0xc); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bac),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1b98),8));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.r[0].wrapping_add(0x9); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1bb4),8));
            { let a = c.r[2].wrapping_add(0xd); wm(m,a,8,c.get(1,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1b84),8));
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[1].wrapping_add(0x11); wm(m,a,8,c.get(0,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1b94),8));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.r[0].wrapping_add(0x16); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1b90),8));
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[2].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1b80),8));
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[1].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1b8c),8));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.r[0].wrapping_add(0x12); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1b88),8));
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[2].wrapping_add(0x17); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x184),8));
            { let a = c.r[2].wrapping_add(0x25); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x468),8));
            { let a = c.r[1].wrapping_add(0x1e); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[2].wrapping_add(0x27); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x468),32));
            c.set(1,32,rm(m,0x6df2fcu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4993cf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544de0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.add(a,0x27u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4993e1);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1ba8),32));
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            if c.cond(7) { pc = 0x499509; continue; }
            pc = 0x4993f3; continue;
            }
            0x4993f3 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4999e4)); continue;
            }
            0x4993fa => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,8,0xeu32); }
            pc = 0x499509; continue;
            }
            0x499409 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bb0),32));
            { let a = c.get(0,32); c.sub(a,0xeu32,32); }
            if c.cond(7) { pc = 0x499509; continue; }
            pc = 0x499418; continue;
            }
            0x499418 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x499a00)); continue;
            }
            0x49941f => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,8,0x0u32); }
            pc = 0x499509; continue;
            }
            0x49942d => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,8,0x1u32); }
            pc = 0x499509; continue;
            }
            0x49943c => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,8,0x2u32); }
            pc = 0x499509; continue;
            }
            0x49944b => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,8,0x3u32); }
            pc = 0x499509; continue;
            }
            0x499459 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,8,0x4u32); }
            pc = 0x499509; continue;
            }
            0x499468 => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,8,0x5u32); }
            pc = 0x499509; continue;
            }
            0x499477 => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,8,0x6u32); }
            pc = 0x499509; continue;
            }
            0x499485 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,8,0x7u32); }
            pc = 0x499509; continue;
            }
            0x499491 => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,8,0x8u32); }
            pc = 0x499509; continue;
            }
            0x49949d => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,8,0x9u32); }
            pc = 0x499509; continue;
            }
            0x4994a8 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,8,0xau32); }
            pc = 0x499509; continue;
            }
            0x4994b4 => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,8,0xbu32); }
            pc = 0x499509; continue;
            }
            0x4994c0 => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,8,0xcu32); }
            pc = 0x499509; continue;
            }
            0x4994cb => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,8,0xdu32); }
            pc = 0x499509; continue;
            }
            0x4994d7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bb0),32));
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4994ff; continue; }
            pc = 0x4994e2; continue;
            }
            0x4994e2 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4994f3; continue; }
            pc = 0x4994e5; continue;
            }
            0x4994e5 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x499509; continue; }
            pc = 0x4994e8; continue;
            }
            0x4994e8 => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,8,0x12u32); }
            pc = 0x499509; continue;
            }
            0x4994f3 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,8,0x11u32); }
            pc = 0x499509; continue;
            }
            0x4994ff => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,8,0x10u32); }
            pc = 0x499509; continue;
            }
            0x499509 => {
            { let a = rm(m,c.r[6].wrapping_add(0x1bb4),32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x499527; continue; }
            pc = 0x499511; continue;
            }
            0x499511 => {
            c.set(0,32,0xfffffffdu32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x499527 => {
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,0x0u32);
            c.push(m, 0x499534);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4349a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbdc8u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x49957b; continue; }
            pc = 0x49953d; continue;
            }
            0x49953d => {
            c.push(m,0x2cu32);
            c.push(m, 0x499544);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1u32); }
            if c.cond(4) { pc = 0x49955e; continue; }
            pc = 0x499555; continue;
            }
            0x499555 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x49955c);
            { let args = []; c.r[0] = host.call(m, 0x444ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x499560; continue;
            }
            0x49955e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x499560; continue;
            }
            0x499560 => {
            { let a = 0x6cbdc8u32; wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x28),8));
            { let a = c.get(1,8); c.sub(a,0x1u32,8); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            if c.cond(4) { pc = 0x49957b; continue; }
            pc = 0x499574; continue;
            }
            0x499574 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x49957b);
            { let args = []; c.r[0] = host.call(m, 0x444d30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x49957b; continue;
            }
            0x49957b => {
            c.set(0,32,rm(m,0x6ba7f8u32,32));
            c.set(1,16,rm(m,0x6ba7fcu32,16));
            c.set(2,8,rm(m,0x6ba7feu32,8));
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1b9c),32));
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,0x66f6b0u32);
            c.push(m,0x14u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,8,c.get(2,8)); }
            c.set(3,32,rm(m,c.r[1],32));
            c.push(m, 0x4995b0);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(7,32,c.get(0,32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffd8));
            c.push(m,c.get(1,32));
            c.push(m,0x6ba800u32);
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(7,32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x4995d1);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,0x6df2f0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m, 0x4995e5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x54b230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[5].wrapping_add(0xffffffa8));
            c.push(m, 0x4995ed);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x4995f3);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x499600);
            { let args = []; c.r[0] = host.call(m, 0x547f10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x468),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1b98),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1ba4),8));
            { let a = c.r[5].wrapping_add(0xffffffb1); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[5].wrapping_add(0xffffffb3); wm(m,a,8,c.get(3,8)); }
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x46c),8));
            c.set(2,8,0x1u32);
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.get(2,8); let r = c.sub(a,c.get(3,8),8); c.set(2,8,r); }
            { let a = c.r[5].wrapping_add(0xffffffa8); wm(m,a,32,0xa7u32); }
            { let a = c.r[5].wrapping_add(0xffffffb0); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[5].wrapping_add(0xffffffb2); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[5].wrapping_add(0xffffffb4); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x499639);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x16du32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[5].wrapping_add(0xffffffb5); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x499649);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1b84),8));
            { let a = c.r[5].wrapping_add(0xffffffb6); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,0x6df31cu32,32));
            { let a = c.get(2,8); let r = c.dec(a,8); c.set(2,8,r); }
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            { let a = c.r[5].wrapping_add(0xffffffb8); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[5].wrapping_add(0xffffffac); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x49967b; continue; }
            pc = 0x499665; continue;
            }
            0x499665 => {
            c.set(0,32,0xfffffffdu32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49967b => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffa8));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x499689);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6df2f0u32,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffbc));
            { let a = c.get(2,32); let r = c.or(a,0x10u32,32); c.set(2,32,r); }
            { let a = 0x6df2f0u32; wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x4996a0);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1bac),32));
            c.set(7,32,0x2u32);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,0xc8u32); }
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x66de7c),32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0xffffffc8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[5].wrapping_add(0xffffffcc); wm(m,a,32,0xffffffffu32); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[5].wrapping_add(0xffffffbc));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xffffffc0); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4996f1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4996fb);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x49970b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x168));
            c.push(m, 0x499716);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            c.push(m, 0x49971f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x2e8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x49972d);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x499736);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe8),32));
            { let a = c.r[5].wrapping_add(0xffffffc0); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[5].wrapping_add(0xffffffc2); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,0xaau32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3u32); }
            if c.cond(14) { pc = 0x4997a8; continue; }
            pc = 0x49975c; continue;
            }
            0x49975c => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffc2),32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[7]),8));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffc4));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0xbu32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x49979f; continue; }
            pc = 0x49978a; continue;
            }
            0x49978a => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffbc));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x499798);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[7].wrapping_add(0x1));
            { let a = c.r[5].wrapping_add(0xffffffc2); wm(m,a,16,c.get(2,16)); }
            pc = 0x49979f; continue;
            }
            0x49979f => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe8),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffff8),32),32); }
            if c.cond(12) { pc = 0x49975c; continue; }
            pc = 0x4997a8; continue;
            }
            0x4997a8 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffc2),32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffc4));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[5].wrapping_add(0xffffffbc));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,8,0x0u32); }
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4997c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            { let a = c.r[5].wrapping_add(0xffffffc2); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,0xabu32); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0xfffffff8),32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x499828; continue; }
            pc = 0x4997dc; continue;
            }
            0x4997dc => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffc2),32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[7]),8));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[5].wrapping_add(0xffffffc4));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,0xbu32);
            { let a = c.r[2].wrapping_add(c.r[7]); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.get(7,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sbb(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x49981f; continue; }
            pc = 0x49980a; continue;
            }
            0x49980a => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffbc));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x499818);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[7].wrapping_add(0x1));
            { let a = c.r[5].wrapping_add(0xffffffc2); wm(m,a,16,c.get(2,16)); }
            pc = 0x49981f; continue;
            }
            0x49981f => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffff8),32),32); }
            if c.cond(12) { pc = 0x4997dc; continue; }
            pc = 0x499828; continue;
            }
            0x499828 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffc2),32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffc4));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[5].wrapping_add(0xffffffbc));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,8,0x0u32); }
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x499847);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6ba81cu32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            c.push(m, 0x49985c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x499861);
            { let args = []; c.r[0] = host.call(m, 0x42da50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x499901; continue;
            }
            0x499869 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x499871);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x9u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x6u32); }
            c.push(m, 0x49987f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x499887);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let a = c.r[1].wrapping_add(0x7cc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6ba838u32);
            c.push(m, 0x4998a5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x4998b0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x424d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x4998ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x4998c2);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.push(m,0xfu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            { let a = c.r[2].wrapping_add(0x7e4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4998d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x4998e0);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x5u32); }
            { let a = c.r[1].wrapping_add(0x7fc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x4998f8);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x4998feu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4998fe => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            pc = 0x499901; continue;
            }
            0x499901 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x468),16));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x3u32); }
            c.push(m, 0x499923);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x58a4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[6].wrapping_add(0x1bb8); wm(m,a,32,0xfffffffdu32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            c.push(m, 0x499939);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x499948);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x499948; continue;
            }
            0x499948 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bb8),32));
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

pub fn choice_list_dialog_on_delete_answer(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x472470;
    loop {
        match pc {
            0x472470 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65f4a9u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1084),32));
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,0x66bdc0u32,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x472560; continue; }
            pc = 0x4724b0; continue;
            }
            0x4724b0 => {
            c.push(m,0x6b97ccu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x4724bd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.push(m,0x6b97d4u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4724cf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x938));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x4724e0);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4724eb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f436, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            c.push(m, 0x4724fa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f436, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x6u32); }
            c.push(m, 0x472506);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x5u32); }
            c.push(m, 0x472512);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            c.push(m, 0x47251e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x7u32); }
            c.push(m,c.get(2,32));
            c.push(m, 0x47252b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x653f49, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x472536; continue;
            }
            0x47252d => {
            c.set(0,32,0x472533u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x472533 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x472536; continue;
            }
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
            pc = 0x472560; continue;
            }
            0x472560 => {
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

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "new_game_screen_process_input" => new_game_screen_process_input(m, host, regs),
        "choice_list_dialog_on_delete_answer" => choice_list_dialog_on_delete_answer(m, host, regs),
        _ => return None,
    })
}
