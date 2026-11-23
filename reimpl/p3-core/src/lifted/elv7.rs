//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn operations_save_game(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5472f0;
    loop {
        match pc {
            0x5472f0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x66389du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x44u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x547332);
            { let args = []; c.r[0] = host.call(m, 0x54c3f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            c.push(m, 0x54733e);
            { let args = []; c.r[0] = host.call(m, 0x653b0b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffb8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3u32); }
            c.push(m, 0x54734a);
            { let args = []; c.r[0] = host.call(m, 0x653b0b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(9) { pc = 0x5474a8; continue; }
            pc = 0x54735f; continue;
            }
            0x54735f => {
            { let a = rm(m,c.r[7].wrapping_add(0x91c),32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x5474bd; continue; }
            pc = 0x54736b; continue;
            }
            0x54736b => {
            { let a = rm(m,0x6de4aau32,16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x54738d; continue; }
            pc = 0x547375; continue;
            }
            0x547375 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x547380);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d6780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x547375; continue; }
            pc = 0x54738d; continue;
            }
            0x54738d => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = rm(m,c.r[1].wrapping_add(0xd),8); c.sub(a,0x5u32,8); }
            if c.cond(5) { pc = 0x5473a6; continue; }
            pc = 0x547399; continue;
            }
            0x547399 => {
            c.push(m,0x6c3860u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m, 0x5473a6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5473a6; continue;
            }
            0x5473a6 => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = rm(m,c.r[2].wrapping_add(0xd),8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x5473bf; continue; }
            pc = 0x5473b2; continue;
            }
            0x5473b2 => {
            c.push(m,0x6c386cu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m, 0x5473bf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5473bf; continue;
            }
            0x5473bf => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = rm(m,c.r[0].wrapping_add(0xd),8); c.sub(a,0x3u32,8); }
            if c.cond(3) { pc = 0x5473d7; continue; }
            pc = 0x5473ca; continue;
            }
            0x5473ca => {
            c.push(m,0x6c3878u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m, 0x5473d7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5473d7; continue;
            }
            0x5473d7 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a2b0u32,32); c.push(m, 0x5473e3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x5cu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m, 0x5473ed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f60a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x5473ef; continue;
            }
            0x5473ef => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[3]).wrapping_add(0x8),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x547406; continue; }
            pc = 0x5473f7; continue;
            }
            0x5473f7 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m, 0x547400);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f60a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0xcu32,32); }
            if c.cond(12) { pc = 0x5473ef; continue; }
            pc = 0x547406; continue;
            }
            0x547406 => {
            c.set(2,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,0x6c3884u32);
            c.set(0,32,c.r[5].wrapping_add(0xffffffdc));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x547418);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x5u32); }
            c.push(m, 0x547425);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffdc));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            c.push(m, 0x547431);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe8),32));
            c.push(m,0x0u32);
            c.push(m,0x9001u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            c.push(m, 0x547444);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x653c1d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x54767d; continue; }
            pc = 0x54744c; continue;
            }
            0x54744c => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x547453);
            { let args = []; c.r[0] = host.call(m, 0x549c60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x54745b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[3].wrapping_add(0x4));
            c.set(6,32,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x547469);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x547668; continue; }
            pc = 0x547479; continue;
            }
            0x547479 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.push(m,c.get(6,32));
            if c.cond(4) { pc = 0x54766d; continue; }
            pc = 0x547482; continue;
            }
            0x547482 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffd8),32));
            c.push(m, 0x54748a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x549cc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            c.push(m, 0x547495);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x54c410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x6u32); }
            c.push(m, 0x5474a3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x653d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x547534; continue;
            }
            0x5474a8 => {
            { let a = rm(m,c.r[7].wrapping_add(0x920),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x54736b; continue; }
            pc = 0x5474b4; continue;
            }
            0x5474b4 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[7].wrapping_add(0x2c),32),32); }
            if c.cond(4) { pc = 0x54736b; continue; }
            pc = 0x5474bd; continue;
            }
            0x5474bd => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffb8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3u32); }
            c.push(m, 0x5474c9);
            { let args = []; c.r[0] = host.call(m, 0x653b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            c.push(m, 0x5474d5);
            { let args = []; c.r[0] = host.call(m, 0x653b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x5474e1);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            c.push(m, 0x5474ed);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5474fc);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,0x1u32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x547511 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffd8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,0xdu32,32); }
            if c.cond(4) { pc = 0x547521; continue; }
            pc = 0x54751c; continue;
            }
            0x54751c => {
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            if c.cond(5) { pc = 0x547528; continue; }
            pc = 0x547521; continue;
            }
            0x547521 => {
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,0x1u32); }
            pc = 0x547528; continue;
            }
            0x547528 => {
            c.set(0,32,0x54752eu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x54752e => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xffffffdc),32));
            pc = 0x547534; continue;
            }
            0x547534 => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x4u32); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x8u32); }
            c.push(m, 0x547547);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x547566; continue;
            }
            0x547549 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffd8),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x54755a; continue; }
            pc = 0x547553; continue;
            }
            0x547553 => {
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,0x1u32); }
            pc = 0x54755a; continue;
            }
            0x54755a => {
            c.set(0,32,0x547560u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x547560 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xffffffdc),32));
            pc = 0x547566; continue;
            }
            0x547566 => {
            c.push(m,c.get(6,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x4u32); }
            c.push(m, 0x547573);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x547579);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x54767d; continue; }
            pc = 0x547587; continue;
            }
            0x547587 => {
            c.set(2,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,0x6c388cu32);
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x547599);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xau32); }
            c.push(m, 0x5475a6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0x8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            c.push(m, 0x5475b2);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe8),32));
            c.push(m,0x0u32);
            c.push(m,0x9001u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            c.push(m, 0x5475c5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x653c1d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x54767d; continue; }
            pc = 0x5475cd; continue;
            }
            0x5475cd => {
            c.set(1,32,0x6dfc90u32);
            c.push(m, 0x5475d7);
            { let args = []; c.r[0] = host.call(m, 0x54d7f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5475df);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[3].wrapping_add(0x4));
            c.set(6,32,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x5475ed);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x547668; continue; }
            pc = 0x5475f9; continue;
            }
            0x5475f9 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.push(m,c.get(6,32));
            if c.cond(4) { pc = 0x54766d; continue; }
            pc = 0x5475fe; continue;
            }
            0x5475fe => {
            c.set(1,32,0x6dfc90u32);
            c.push(m, 0x547608);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54d870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            c.push(m, 0x547613);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x54c410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xbu32); }
            c.push(m, 0x547621);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x653d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x54762f; continue;
            }
            0x547623 => {
            c.set(0,32,0x547629u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x547629 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xffffffdc),32));
            pc = 0x54762f; continue;
            }
            0x54762f => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x4u32); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xdu32); }
            c.push(m, 0x547642);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x547650; continue;
            }
            0x547644 => {
            c.set(0,32,0x54764au32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x54764a => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xffffffdc),32));
            pc = 0x547650; continue;
            }
            0x547650 => {
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x4u32); }
            c.push(m, 0x54765d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x547663);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x54767d; continue;
            }
            0x547668 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x547675; continue; }
            pc = 0x54766c; continue;
            }
            0x54766c => {
            c.push(m,c.get(7,32));
            pc = 0x54766d; continue;
            }
            0x54766d => {
            c.push(m, 0x547672);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x547675; continue;
            }
            0x547675 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            c.push(m, 0x54767d);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x54767d; continue;
            }
            0x54767d => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffb8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3u32); }
            c.push(m, 0x547689);
            { let args = []; c.r[0] = host.call(m, 0x653b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            c.push(m, 0x547695);
            { let args = []; c.r[0] = host.call(m, 0x653b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x5476a1);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            c.push(m, 0x5476ad);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5476bc);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn operations_save_alderman_character_file(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5499f0;
    loop {
        match pc {
            0x5499f0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x66396eu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x30u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x924),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x549a24);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6de4acu32,32));
            c.set(6,32,rm(m,0x6dfcacu32,32));
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x549a47);
            { let args = []; c.r[0] = host.call(m, 0x653b0b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            c.push(m, 0x549a56);
            { let args = []; c.r[0] = host.call(m, 0x54c3f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            if c.cond(4) { pc = 0x549a77; continue; }
            pc = 0x549a5e; continue;
            }
            0x549a5e => {
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x924),32));
            pc = 0x549a64; continue;
            }
            0x549a64 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6],16));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x549a77; continue; }
            pc = 0x549a6d; continue;
            }
            0x549a6d => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x230c),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x549a64; continue; }
            pc = 0x549a77; continue;
            }
            0x549a77 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x549a7e);
            { let args = []; c.r[0] = host.call(m, 0x4f3fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x549a90; continue; }
            pc = 0x549a87; continue;
            }
            0x549a87 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x549a8e);
            { let args = []; c.r[0] = host.call(m, 0x54cb90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            pc = 0x549a90; continue;
            }
            0x549a90 => {
            c.push(m,0x0u32);
            c.push(m,0x9001u32);
            c.push(m,0x6c3918u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x549aa4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x653c1d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x549b71; continue; }
            pc = 0x549aac; continue;
            }
            0x549aac => {
            c.push(m,c.get(3,32));
            c.push(m, 0x549ab2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[3].wrapping_add(0x4));
            c.set(7,32,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x549ac0);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x549b5c; continue; }
            pc = 0x549ace; continue;
            }
            0x549ace => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x549b59; continue; }
            pc = 0x549ad6; continue;
            }
            0x549ad6 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x549ae2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[7].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffdc),32));
            c.push(m, 0x549af1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f4010, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x549af9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54cbd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m, 0x549b07);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x54c410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            c.push(m, 0x549b15);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x653d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x549b20; continue;
            }
            0x549b17 => {
            c.set(0,32,0x549b1du32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x549b1d => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xffffffe8),32));
            pc = 0x549b20; continue;
            }
            0x549b20 => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1u32); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            c.push(m, 0x549b33);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x549b3e; continue;
            }
            0x549b35 => {
            c.set(0,32,0x549b3bu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x549b3b => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xffffffe8),32));
            pc = 0x549b3e; continue;
            }
            0x549b3e => {
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1u32); }
            c.push(m, 0x549b4b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x549b54);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x549b71; continue;
            }
            0x549b59 => {
            c.push(m,c.get(7,32));
            pc = 0x549b61; continue;
            }
            0x549b5c => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x549b69; continue; }
            pc = 0x549b60; continue;
            }
            0x549b60 => {
            c.push(m,c.get(0,32));
            pc = 0x549b61; continue;
            }
            0x549b61 => {
            c.push(m, 0x549b66);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x549b69; continue;
            }
            0x549b69 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x549b71);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x549b71; continue;
            }
            0x549b71 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            c.push(m, 0x549b7d);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x549b8c);
            { let args = []; c.r[0] = host.call(m, 0x653b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
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
        "operations_save_game" => operations_save_game(m, host, regs),
        "operations_save_alderman_character_file" => operations_save_alderman_character_file(m, host, regs),
        _ => return None,
    })
}
