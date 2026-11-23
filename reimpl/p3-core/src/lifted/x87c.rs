//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

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
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x5a3d28); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
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
            { let target = rm(m,c.r[0].wrapping_add(0x124),32); c.push(m, 0x5a3dc0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
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

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "hanse_side_menu_update" => hanse_side_menu_update(m, host, regs),
        "shipyard_parchment_dialog_update_rename_page" => shipyard_parchment_dialog_update_rename_page(m, host, regs),
        _ => return None,
    })
}
