//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn church_side_menu_on_show(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a5550;
    loop {
        match pc {
            0x5a5550 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x5a5560);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6e51acu32,32));
            { let a = c.r[4].wrapping_add(0x7); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc324),32));
            { let a = c.r[6].wrapping_add(0x4d5); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5a55a8; continue; }
            pc = 0x5a5589; continue;
            }
            0x5a5589 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x76c),32));
            { let a = c.get(4,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x5a55a8; continue; }
            pc = 0x5a55a3; continue;
            }
            0x5a55a3 => {
            { let a = c.r[4].wrapping_add(0x7); wm(m,a,8,0x0u32); }
            pc = 0x5a55a8; continue;
            }
            0x5a55a8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[6].wrapping_add(0xac));
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x4d2); wm(m,a,8,0xffu32); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5a55c2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x64u32); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4d0),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x5a55fd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4d3),8));
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5a5681; continue; }
            pc = 0x5a560f; continue;
            }
            0x5a560f => {
            c.push(m,c.get(5,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x5a5612; continue;
            }
            0x5a5612 => {
            c.set(2,32,c.r[5].wrapping_mul(8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(7,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0xac));
            c.push(m,c.get(7,32));
            c.push(m, 0x5a5630);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a563c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5a5645); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5a5651); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x450));
            c.push(m, 0x5a5662);
            { let args = []; c.r[0] = host.call(m, 0x4b0540, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x5a566f; continue; }
            pc = 0x5a5667; continue;
            }
            0x5a5667 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5a5673; continue; }
            pc = 0x5a566f; continue;
            }
            0x5a566f => {
            { let a = rm(m,c.r[4].wrapping_add(0x1c),32); let r = c.add(a,c.get(3,32),32); { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,r); } }
            pc = 0x5a5673; continue;
            }
            0x5a5673 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x5a5612; continue; }
            pc = 0x5a5679; continue;
            }
            0x5a5679 => {
            { let a = c.r[6].wrapping_add(0x4d3); wm(m,a,8,0x1u32); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x5a5681; continue;
            }
            0x5a5681 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a5688);
            { let args = []; c.r[0] = host.call(m, 0x440e50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m, 0x5a568f);
            { let args = []; c.r[0] = host.call(m, 0x462280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m, 0x5a569d);
            { let args = []; c.r[0] = host.call(m, 0x466320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x3u32);
            c.push(m, 0x5a56aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x466430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4d5),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x794));
            c.push(m, 0x5a56cb);
            { let args = []; c.r[0] = host.call(m, 0x4fe360, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5a56e0; continue; }
            pc = 0x5a56d1; continue;
            }
            0x5a56d1 => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5a56e0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a56e0; continue;
            }
            0x5a56e0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4d5),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x794));
            c.push(m, 0x5a5702);
            { let args = []; c.r[0] = host.call(m, 0x4fe360, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x5a5716; continue; }
            pc = 0x5a5707; continue;
            }
            0x5a5707 => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m, 0x5a5716);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a5716; continue;
            }
            0x5a5716 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4d5),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x794));
            c.push(m, 0x5a5738);
            { let args = []; c.r[0] = host.call(m, 0x4fe360, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(14) { pc = 0x5a574c; continue; }
            pc = 0x5a573d; continue;
            }
            0x5a573d => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x2u32);
            c.push(m, 0x5a574c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a574c; continue;
            }
            0x5a574c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4d5),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x794));
            c.push(m, 0x5a576e);
            { let args = []; c.r[0] = host.call(m, 0x4fe360, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(14) { pc = 0x5a5782; continue; }
            pc = 0x5a5773; continue;
            }
            0x5a5773 => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x3u32);
            c.push(m, 0x5a5782);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a5782; continue;
            }
            0x5a5782 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4d5),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x794));
            c.push(m, 0x5a57a4);
            { let args = []; c.r[0] = host.call(m, 0x4fe360, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(14) { pc = 0x5a57b8; continue; }
            pc = 0x5a57a9; continue;
            }
            0x5a57a9 => {
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x4u32);
            c.push(m, 0x5a57b8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a57b8; continue;
            }
            0x5a57b8 => {
            { let a = c.r[6].wrapping_add(0x4d4); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1c5),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5a57d9; continue; }
            pc = 0x5a57d0; continue;
            }
            0x5a57d0 => {
            c.push(m,0x3u32);
            c.push(m,0x1u32);
            c.push(m, 0x5a57d9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4436e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a57d9; continue;
            }
            0x5a57d9 => {
            c.set(1,32,rm(m,0x6e556cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x5a57e7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
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

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "church_side_menu_on_show" => church_side_menu_on_show(m, host, regs),
        "game_screen_create_side_menus_and_dialogs" => game_screen_create_side_menus_and_dialogs(m, host, regs),
        _ => return None,
    })
}
