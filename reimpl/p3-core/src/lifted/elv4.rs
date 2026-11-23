//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn multiplayer_screen_start_game_tail(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43c596;
    loop {
        match pc {
            0x43c596 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x1c4),8));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43c5c0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x58a4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,8,rm(m,0x6dfc14u32,8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6e55d0u32);
            c.push(m, 0x43c5d1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x611e50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = c.r[6].wrapping_add(0x4cd0); wm(m,a,8,0x1u32); }
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

pub fn multiplayer_screen_update_hotseat_setup_tail_enter_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43e148;
    loop {
        match pc {
            0x43e148 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffb0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(0xffffffa0));
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,0xc8u32); }
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xffffffa4); wm(m,a,32,0x7du32); }
            { let a = c.r[5].wrapping_add(0xffffffa8); wm(m,a,32,0x19u32); }
            { let a = c.r[5].wrapping_add(0xffffffac); wm(m,a,32,0x0u32); }
            c.push(m, 0x43e17b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43e185);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x43e196);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,0xffu32);
            c.set(1,32,0x6e6220u32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x43e1a9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6141d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x19),8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x43e1c2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x58a4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4cd0); wm(m,a,8,0x1u32); }
            c.push(m, 0x43e1d0);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,0x1a0au32);
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

pub fn nav_map_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4499d0;
    loop {
        match pc {
            0x4499d0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x5cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4499de);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44a2a7; continue; }
            pc = 0x4499e6; continue;
            }
            0x4499e6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4499f7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m, 0x449a0b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            if c.cond(14) { pc = 0x449a4c; continue; }
            pc = 0x449a1c; continue;
            }
            0x449a1c => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            pc = 0x449a22; continue;
            }
            0x449a22 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(c.r[1].wrapping_mul(2)),16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[5]),8));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x449a47; continue; }
            pc = 0x449a40; continue;
            }
            0x449a40 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x449a22; continue; }
            pc = 0x449a45; continue;
            }
            0x449a45 => {
            pc = 0x449a4c; continue;
            }
            0x449a47 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x449a4c; continue;
            }
            0x449a4c => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x29e4),16));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[7],16),16); c.set(0,16,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,16,c.get(0,16)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x18),16),16); c.set(0,16,r); }
            c.set(7,32,rm(m,c.r[7],32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x29e6),16),16); c.set(0,16,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x3e); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(3,8,u32::from(c.cond(14)));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            c.set(0,8,u32::from(c.cond(14)));
            { let a = c.get(3,32); let r = c.and(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,8,u32::from(c.cond(13)));
            { let a = c.get(3,32); let r = c.and(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3e),32));
            c.set(1,8,u32::from(c.cond(13)));
            { let a = c.get(1,32); c.and(a,c.get(3,32),32); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,u32::from(c.cond(5))); }
            { let a = c.get(2,32); let r = c.add(a,0xffffffbcu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xffffffbcu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x26); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x449ae1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4474d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b21),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44a2a7; continue; }
            pc = 0x449aef; continue;
            }
            0x449aef => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.get(3,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x449b73; continue; }
            pc = 0x449af8; continue;
            }
            0x449af8 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x449b0d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x44fb40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x449b73; continue; }
            pc = 0x449b11; continue;
            }
            0x449b11 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x449b23; continue; }
            pc = 0x449b19; continue;
            }
            0x449b19 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x449b21);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x455b50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x449b73; continue;
            }
            0x449b23 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,c.r[6].wrapping_add(0x28fc));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x449b3a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x446d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.sub(a,0x28u32,32); }
            if c.cond(13) { pc = 0x449b73; continue; }
            pc = 0x449b41; continue;
            }
            0x449b41 => {
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x449b55);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x449b73; continue; }
            pc = 0x449b59; continue;
            }
            0x449b59 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(2,32,rm(m,0x66fe90u32,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6d9584u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x449b73);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4a9690, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x449b73; continue;
            }
            0x449b73 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,0x1u32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,0x2u32,32); }
            c.set(7,32,c.get(0,32));
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.or(a,c.get(7,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x449b99; continue; }
            pc = 0x449b89; continue;
            }
            0x449b89 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2b04),16));
            { let a = c.r[6].wrapping_add(0x2b06); wm(m,a,16,c.get(2,16)); }
            pc = 0x449ba2; continue;
            }
            0x449b99 => {
            { let a = c.r[6].wrapping_add(0x2b06); wm(m,a,16,0xffu32); }
            pc = 0x449ba2; continue;
            }
            0x449ba2 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2b04),16));
            c.set(2,32,rm(m,0x6de4b0u32,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x449bc4; continue; }
            pc = 0x449bba; continue;
            }
            0x449bba => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6de4b8),8));
            pc = 0x449be3; continue;
            }
            0x449bc4 => {
            { let a = c.get(1,16); c.sub(a,0x27u32,16); }
            if c.cond(5) { pc = 0x449c42; continue; }
            pc = 0x449bca; continue;
            }
            0x449bca => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x449bdb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe1),8));
            pc = 0x449be3; continue;
            }
            0x449be3 => {
            c.set(0,32,c.get(1,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x6ddb90),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x6ddb94),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x1au32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x1au32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x48));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x449c36);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x48));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x449c42);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x453e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x449c42; continue;
            }
            0x449c42 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2b1e),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(7,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x449f2c; continue; }
            pc = 0x449c57; continue;
            }
            0x449c57 => {
            { let a = rm(m,c.r[4].wrapping_add(0x1c),32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x449ebb; continue; }
            pc = 0x449c62; continue;
            }
            0x449c62 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0],16));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x30),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x449ebb; continue; }
            pc = 0x449c88; continue;
            }
            0x449c88 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,0xffffffffu32,32); }
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x449ebb; continue; }
            pc = 0x449ca2; continue;
            }
            0x449ca2 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x24),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(7) { pc = 0x449d10; continue; }
            pc = 0x449cbc; continue;
            }
            0x449cbc => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x44a2b4)); continue;
            }
            0x449cc3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            pc = 0x449ce1; continue;
            }
            0x449ccb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb0),32));
            pc = 0x449ce1; continue;
            }
            0x449cd3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xac),32));
            pc = 0x449ce1; continue;
            }
            0x449cdb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb8),32));
            pc = 0x449ce1; continue;
            }
            0x449ce1 => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),32),32); }
            if c.cond(13) { pc = 0x449d10; continue; }
            pc = 0x449ce6; continue;
            }
            0x449ce6 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x449d10; continue; }
            pc = 0x449ced; continue;
            }
            0x449ced => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            pc = 0x449d14; continue;
            }
            0x449d10 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            pc = 0x449d14; continue;
            }
            0x449d14 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            { let a = c.get(7,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,sx(rm(m,c.r[6].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x449dae; continue; }
            pc = 0x449d7d; continue;
            }
            0x449d7d => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x44a2c4)); continue;
            }
            0x449d84 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            pc = 0x449daa; continue;
            }
            0x449d8c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xb0),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x449dae; continue;
            }
            0x449d98 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xac),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x449dae; continue;
            }
            0x449da4 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xb8),32));
            pc = 0x449daa; continue;
            }
            0x449daa => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x449dae; continue;
            }
            0x449dae => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x449eb7; continue; }
            pc = 0x449dba; continue;
            }
            0x449dba => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.r[4].wrapping_add(0x48));
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,0x0u32); }
            c.push(m,c.get(1,32));
            c.push(m, 0x449dd3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bbb00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,0x38u32,32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,0x34u32,32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(1,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); let r = c.and(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x449e3b; continue; }
            pc = 0x449dfc; continue;
            }
            0x449dfc => {
            c.set(0,32,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0x80000003u32,32); c.set(3,32,r); }
            if c.cond(9) { pc = 0x449e24; continue; }
            pc = 0x449e1f; continue;
            }
            0x449e1f => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.or(a,0xfffffffcu32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            pc = 0x449e24; continue;
            }
            0x449e24 => {
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(3,8,0x1u32);
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x449e3f; continue; }
            pc = 0x449e3b; continue;
            }
            0x449e3b => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x13),8));
            pc = 0x449e3f; continue;
            }
            0x449e3f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x449e4c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bbb10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x449eb7; continue; }
            pc = 0x449e53; continue;
            }
            0x449e53 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x34),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x68),32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x80),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x84),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x449ea5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36))]; c.r[0] = host.call(m, 0x446a00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(40);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x449eb7; continue; }
            pc = 0x449ea9; continue;
            }
            0x449ea9 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x2b23); wm(m,a,8,0x1u32); }
            c.push(m, 0x449eb7);
            { let args = []; c.r[0] = host.call(m, 0x451eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x449eb7; continue;
            }
            0x449eb7 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x74),32));
            pc = 0x449ebb; continue;
            }
            0x449ebb => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b23),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x449f2c; continue; }
            pc = 0x449ec5; continue;
            }
            0x449ec5 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x449ecc);
            { let args = []; c.r[0] = host.call(m, 0x44d400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b1f),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x449eee; continue; }
            pc = 0x449ed6; continue;
            }
            0x449ed6 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x3c),16));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x3e),16));
            { let a = c.r[6].wrapping_add(0x2900); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x2902); wm(m,a,16,c.get(1,16)); }
            pc = 0x449eee; continue;
            }
            0x449eee => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x2902),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(0x3e),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x2900),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(0x3c),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x190u32,32); }
            if c.cond(14) { pc = 0x449f25; continue; }
            pc = 0x449f1e; continue;
            }
            0x449f1e => {
            { let a = c.r[6].wrapping_add(0x2b24); wm(m,a,8,0x1u32); }
            pc = 0x449f25; continue;
            }
            0x449f25 => {
            { let a = c.r[6].wrapping_add(0x2b1f); wm(m,a,8,0x1u32); }
            pc = 0x449f2c; continue;
            }
            0x449f2c => {
            { let a = c.get(3,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x44a2a7; continue; }
            pc = 0x449f35; continue;
            }
            0x449f35 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b23),8));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,0x0u32); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            if c.cond(4) { pc = 0x449f61; continue; }
            pc = 0x449f49; continue;
            }
            0x449f49 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x2b23); wm(m,a,8,0x0u32); }
            c.push(m, 0x449f57);
            { let args = []; c.r[0] = host.call(m, 0x451eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x5cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x449f61 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.set(3,32,c.r[6].wrapping_add(0x28fc));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x449f78);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x446d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x449f8e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = rm(m,c.r[7].wrapping_add(0xe1),8); c.sub(a,0x28u32,8); }
            if c.cond(3) { pc = 0x44a047; continue; }
            pc = 0x449f9d; continue;
            }
            0x449f9d => {
            c.push(m,0xff000000u32);
            c.push(m, 0x449fa7);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe1),8));
            c.set(5,32,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            c.set(7,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,sx(rm(m,c.r[3],16),16)&0xffffffffu32);
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x6ddb90),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x6ddb94),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.add(a,0x16u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[0].wrapping_add(0x16));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x28fe),16),16)&0xffffffffu32);
            c.set(3,8,u32::from(c.cond(15)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(2,8,u32::from(c.cond(15)));
            { let a = c.get(3,32); let r = c.and(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(3,32); let r = c.and(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x28fe),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(2,8,u32::from(c.cond(12)));
            { let a = c.get(3,32); let r = c.and(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x6ddbb0),32); c.sub(a,c.get(2,32),32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x6ddbb4),32); c.sub(a,c.get(0,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.and(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.or(a,c.get(1,32),32); c.set(3,32,r); }
            if c.cond(4) { pc = 0x44a043; continue; }
            pc = 0x44a03c; continue;
            }
            0x44a03c => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x1u32); }
            pc = 0x44a047; continue;
            }
            0x44a043 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x44a047; continue;
            }
            0x44a047 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x34),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44a07a; continue; }
            pc = 0x44a04f; continue;
            }
            0x44a04f => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x44a0ad; continue; }
            pc = 0x44a059; continue;
            }
            0x44a059 => {
            { let a = c.get(5,32); c.sub(a,0x28u32,32); }
            c.set(0,8,u32::from(c.cond(12)));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            c.push(m,0xfffffffeu32);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x44a074);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x44fb40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,c.get(0,8)); }
            pc = 0x44a0ad; continue;
            }
            0x44a07a => {
            { let a = c.get(5,32); c.sub(a,0x28u32,32); }
            if c.cond(5) { pc = 0x44a0ad; continue; }
            pc = 0x44a07f; continue;
            }
            0x44a07f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44a0ad; continue; }
            pc = 0x44a089; continue;
            }
            0x44a089 => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,0x66f351u32,8));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x44a0ad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x44a0ad; continue;
            }
            0x44a0ad => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b1e),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x44a2a7; continue; }
            pc = 0x44a0bb; continue;
            }
            0x44a0bb => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x48));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x44a0ea);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b24),8));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,0x28u32,32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.and(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x44a228; continue; }
            pc = 0x44a113; continue;
            }
            0x44a113 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x44a124);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x0u32); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xe),16));
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x44a173; continue; }
            pc = 0x44a141; continue;
            }
            0x44a141 => {
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            pc = 0x44a147; continue;
            }
            0x44a147 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x134),16); c.sub(a,0x11u32,16); }
            if c.cond(4) { pc = 0x44a162; continue; }
            pc = 0x44a159; continue;
            }
            0x44a159 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x39),8));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x44a16e; continue; }
            pc = 0x44a162; continue;
            }
            0x44a162 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x44a147; continue; }
            pc = 0x44a16c; continue;
            }
            0x44a16c => {
            pc = 0x44a173; continue;
            }
            0x44a16e => {
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x1u32); }
            pc = 0x44a173; continue;
            }
            0x44a173 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x44a217; continue; }
            pc = 0x44a17f; continue;
            }
            0x44a17f => {
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x44a194);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,c.get(2,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x44a217; continue; }
            pc = 0x44a1a8; continue;
            }
            0x44a1a8 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x44a1bc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x58a4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x2b04); wm(m,a,16,0xffu32); }
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x44a1d1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x44a1da); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x44a1ea); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x44a1f7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x98); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x9c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            pc = 0x44a299; continue;
            }
            0x44a217 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,0x6e553cu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x44a228);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5b7390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x44a228; continue;
            }
            0x44a228 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x70),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x44a246; continue; }
            pc = 0x44a234; continue;
            }
            0x44a234 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x44a242);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x451140, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,c.get(0,8)); }
            pc = 0x44a246; continue;
            }
            0x44a246 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2b24),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x44a257; continue; }
            pc = 0x44a250; continue;
            }
            0x44a250 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x44a257);
            { let args = []; c.r[0] = host.call(m, 0x44d780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x44a257; continue;
            }
            0x44a257 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2b1e),8));
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x98); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x9c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(7,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x44a299; continue; }
            pc = 0x44a283; continue;
            }
            0x44a283 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x44a299; continue; }
            pc = 0x44a28b; continue;
            }
            0x44a28b => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x44a299);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x44f980, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x44a299; continue;
            }
            0x44a299 => {
            { let a = c.r[6].wrapping_add(0x2b1f); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x2b24); wm(m,a,8,0x0u32); }
            pc = 0x44a2a7; continue;
            }
            0x44a2a7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x5cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_copy_alloc_tail(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4645aa;
    loop {
        match pc {
            0x4645aa => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x4645c9; continue; }
            pc = 0x4645b1; continue;
            }
            0x4645b1 => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x4645b7; continue; }
            pc = 0x4645b5; continue;
            }
            0x4645b5 => {
            c.set(0,32,c.get(7,32));
            pc = 0x4645b7; continue;
            }
            0x4645b7 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4645c6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4645c9; continue;
            }
            0x4645c9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4645ef; continue; }
            pc = 0x4645d3; continue;
            }
            0x4645d3 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4645e6; continue; }
            pc = 0x4645db; continue;
            }
            0x4645db => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4645e6; continue; }
            pc = 0x4645e0; continue;
            }
            0x4645e0 => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x4645ef; continue;
            }
            0x4645e6 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4645ec);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4645ef; continue;
            }
            0x4645ef => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            c.set(0,32,c.r[2].wrapping_add(0x1));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            if c.cond(7) { pc = 0x46460c; continue; }
            pc = 0x46460a; continue;
            }
            0x46460a => {
            c.set(7,32,c.get(3,32));
            pc = 0x46460c; continue;
            }
            0x46460c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
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

pub fn event_window_show_next_queued_event(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x469700;
    loop {
        match pc {
            0x469700 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65f179u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[6].wrapping_add(0x3d8),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x469ad5; continue; }
            pc = 0x46972c; continue;
            }
            0x46972c => {
            c.push(m, 0x469731);
            { let args = []; c.r[0] = host.call(m, 0x462310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x469ad5; continue; }
            pc = 0x469739; continue;
            }
            0x469739 => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x469742);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x469755);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3d0),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x46976c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3cc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3c8),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,0xffffffffu32); }
            c.set(1,16,rm(m,c.r[0],16));
            c.set(0,16,rm(m,c.r[2],16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3c4),32));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x469798);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x469a23; continue; }
            pc = 0x4697a7; continue;
            }
            0x4697a7 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3c8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3c4),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3d0),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3cc),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3d4),32));
            c.set(2,32,c.r[0].wrapping_mul(4).wrapping_add(0xfffffffc));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x4697e6);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0]).wrapping_add(0xfffffffe));
            c.push(m,c.get(1,32));
            c.push(m, 0x4697f8);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(3,32,c.get(0,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[2]).wrapping_add(0xfffffffe));
            c.push(m,c.get(0,32));
            c.push(m, 0x46980a);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            c.set(1,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.push(m, 0x469822);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x1u32); }
            if c.cond(4) { pc = 0x46985b; continue; }
            pc = 0x469835; continue;
            }
            0x469835 => {
            c.push(m,0x64f253u32);
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m,0x4103c0u32);
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x469851);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            pc = 0x469863; continue;
            }
            0x46985b => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            pc = 0x469863; continue;
            }
            0x469863 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            c.set(0,32,c.r[7].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m, 0x46987f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x2u32); }
            if c.cond(4) { pc = 0x4698b4; continue; }
            pc = 0x469892; continue;
            }
            0x469892 => {
            c.push(m,0x64f253u32);
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.push(m,0x4103c0u32);
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4698ae);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4698b6; continue;
            }
            0x4698b4 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4698b6; continue;
            }
            0x4698b6 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c4),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.set(2,32,c.r[1].wrapping_mul(4).wrapping_add(0xfffffffc));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4698dc);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c8),32));
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1]).wrapping_add(0xfffffffe));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4698f7);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3cc),32));
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1]).wrapping_add(0xfffffffe));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x469916);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3d0),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_mul(4).wrapping_add(0xfffffffc));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x469938);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3d4),32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_mul(4).wrapping_add(0xfffffffc));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x469956);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3d8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.r[6].wrapping_add(0x3cc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x3c4); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x3c8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3d0); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x3d4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x3d8); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x46999d; continue; }
            pc = 0x469994; continue;
            }
            0x469994 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x46999a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x46999d; continue;
            }
            0x46999d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4699ae; continue; }
            pc = 0x4699a5; continue;
            }
            0x4699a5 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4699ab);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4699ae; continue;
            }
            0x4699ae => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4699bf; continue; }
            pc = 0x4699b6; continue;
            }
            0x4699b6 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4699bc);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4699bf; continue;
            }
            0x4699bf => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4699e4; continue; }
            pc = 0x4699c7; continue;
            }
            0x4699c7 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4699db);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x4699e1);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4699e4; continue;
            }
            0x4699e4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x469a09; continue; }
            pc = 0x4699ec; continue;
            }
            0x4699ec => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x469a00);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x469a06);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x469a09; continue;
            }
            0x469a09 => {
            { let a = c.r[6].wrapping_add(0x3ea); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x469a23 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c4),32));
            { let a = c.r[6].wrapping_add(0x3d8); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x469a3c; continue; }
            pc = 0x469a33; continue;
            }
            0x469a33 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x469a39);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x469a3c; continue;
            }
            0x469a3c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3c8),32));
            { let a = c.r[6].wrapping_add(0x3c4); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x469a55; continue; }
            pc = 0x469a4c; continue;
            }
            0x469a4c => {
            c.push(m,c.get(0,32));
            c.push(m, 0x469a52);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x469a55; continue;
            }
            0x469a55 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3cc),32));
            { let a = c.r[6].wrapping_add(0x3c8); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x469a6e; continue; }
            pc = 0x469a65; continue;
            }
            0x469a65 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x469a6b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x469a6e; continue;
            }
            0x469a6e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d0),32));
            { let a = c.r[6].wrapping_add(0x3cc); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x469a9b; continue; }
            pc = 0x469a7e; continue;
            }
            0x469a7e => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(2,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x469a92);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x469a98);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x469a9b; continue;
            }
            0x469a9b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3d4),32));
            { let a = c.r[6].wrapping_add(0x3d0); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x469ac8; continue; }
            pc = 0x469aab; continue;
            }
            0x469aab => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x469abf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x469ac5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x469ac8; continue;
            }
            0x469ac8 => {
            { let a = c.r[6].wrapping_add(0x3d4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x3ea); wm(m,a,8,0x0u32); }
            pc = 0x469ad5; continue;
            }
            0x469ad5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn route_save_to_file_failure_tail(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d5e9c;
    loop {
        match pc {
            0x4d5e9c => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffc4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x4d5ea8);
            { let args = []; c.r[0] = host.call(m, 0x653b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            c.push(m, 0x4d5eb4);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4d5ec3);
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
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn st_check_game_end(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4e4a44;
    loop {
        match pc {
            0x4e4a44 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662bfau32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xa),16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.sub(a,0x12u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            if c.cond(7) { pc = 0x4e4b50; continue; }
            pc = 0x4e4a7c; continue;
            }
            0x4e4a7c => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4e55fc)); continue;
            }
            0x4e4a83 => {
            c.set(7,32,0x16d00u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4a91 => {
            c.set(7,32,0x2da00u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4a9f => {
            c.set(7,32,0x44700u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4aad => {
            c.set(7,32,0x5b400u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4abb => {
            c.set(7,32,0x72100u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4ac9 => {
            c.set(7,32,0x88e00u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4ad7 => {
            c.set(7,32,0x9fb00u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4ae2 => {
            c.set(7,32,0xb6800u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4aed => {
            c.set(7,32,0xcd500u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4af8 => {
            c.set(7,32,0xe4200u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4b03 => {
            c.set(7,32,0x156300u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4b0e => {
            c.set(7,32,0x1c8400u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4b19 => {
            c.set(7,32,0x2ac600u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4b24 => {
            c.set(7,32,0x474a00u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4b2f => {
            c.set(7,32,0x11170u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4b3a => {
            c.set(7,32,0x186a0u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4b45 => {
            c.set(7,32,0x30d40u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x4e4b59; continue;
            }
            0x4e4b50 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            pc = 0x4e4b59; continue;
            }
            0x4e4b59 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            if c.cond(7) { pc = 0x4e55e2; continue; }
            pc = 0x4e4b69; continue;
            }
            0x4e4b69 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4e5648)); continue;
            }
            0x4e4b70 => {
            c.set(0,32,rm(m,0x6de4acu32,32));
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4e4cd9; continue; }
            pc = 0x4e4b82; continue;
            }
            0x4e4b82 => {
            c.set(3,32,0x14u32);
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(3,16),16); }
            c.set(6,32,c.get(3,32));
            if c.cond(6) { pc = 0x4e4bd6; continue; }
            pc = 0x4e4b92; continue;
            }
            0x4e4b92 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4e4b9d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(0xd),8); c.sub(a,0x3u32,8); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,c.get(2,16),16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x4e4bc8; continue; }
            pc = 0x4e4bb9; continue;
            }
            0x4e4bb9 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x39c),8); c.sub(a,0x4u32,8); }
            if c.cond(4) { pc = 0x4e4c16; continue; }
            pc = 0x4e4bc8; continue;
            }
            0x4e4bc8 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4e4b92; continue; }
            pc = 0x4e4bd6; continue;
            }
            0x4e4bd6 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,0x6de4a1u32,8));
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(0,16,rm(m,c.r[2].wrapping_mul(2).wrapping_add(0x672d94),16));
            c.set(2,32,rm(m,0x6dd73cu32,32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6de4acu32,32));
            c.set(1,32,c.r[2].wrapping_add(c.r[7]));
            pc = 0x4e4c05; continue;
            }
            0x4e4c05 => {
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x4e4c0c; continue; }
            pc = 0x4e4c09; continue;
            }
            0x4e4c09 => {
            { let a = c.r[5]; wm(m,a,32,c.get(1,32)); }
            pc = 0x4e4c0c; continue;
            }
            0x4e4c0c => {
            c.set(0,32,0x1u32);
            pc = 0x4e55e4; continue;
            }
            0x4e4c16 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xcu32);
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x6de4b8),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6ddbb0),16));
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4e4c58);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            { let a = 0x6dfc04u32; wm(m,a,32,0x0u32); }
            c.push(m, 0x4e4c6f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.set(1,32,c.get(4,32));
            if c.cond(5) { pc = 0x4e4cac; continue; }
            pc = 0x4e4c7f; continue;
            }
            0x4e4c7f => {
            c.push(m,0x6dd75au32);
            c.push(m, 0x4e4c89);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x0u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd75bu32);
            c.push(m, 0x4e4ca2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.push(m,0xfu32);
            pc = 0x4e55cf; continue;
            }
            0x4e4cac => {
            c.push(m,0x6dd75cu32);
            c.push(m, 0x4e4cb6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x1u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd75du32);
            c.push(m, 0x4e4ccf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.push(m,c.get(1,32));
            pc = 0x4e55cc; continue;
            }
            0x4e4cd9 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4e4ce9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xcu32);
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x6de4b8),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6ddbb0),16));
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4e4d2b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            { let a = 0x6dfc04u32; wm(m,a,32,0x0u32); }
            c.push(m, 0x4e4d42);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd75eu32);
            c.push(m, 0x4e4d53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x2u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd75fu32);
            pc = 0x4e5230; continue;
            }
            0x4e4d6c => {
            c.set(0,32,rm(m,0x6de4acu32,32));
            c.set(3,32,rm(m,c.r[5],32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x1u32); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x4e4f7f; continue; }
            pc = 0x4e4d96; continue;
            }
            0x4e4d96 => {
            c.set(3,32,0x14u32);
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(3,16),16); }
            c.set(6,32,c.get(3,32));
            if c.cond(6) { pc = 0x4e4f15; continue; }
            pc = 0x4e4daa; continue;
            }
            0x4e4daa => {
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            pc = 0x4e4dad; continue;
            }
            0x4e4dad => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4e4db8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0xd),8); c.sub(a,0x3u32,8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,c.get(2,16),16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.or(a,c.get(2,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4e4ef8; continue; }
            pc = 0x4e4dda; continue;
            }
            0x4e4dda => {
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x4e4ef0; continue; }
            pc = 0x4e4dea; continue;
            }
            0x4e4dea => {
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(0x6f1));
            pc = 0x4e4df5; continue;
            }
            0x4e4df5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4e4e0d; continue; }
            pc = 0x4e4dfd; continue;
            }
            0x4e4dfd => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x9f8u32,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4e4df5; continue; }
            pc = 0x4e4e08; continue;
            }
            0x4e4e08 => {
            pc = 0x4e4ef0; continue;
            }
            0x4e4e0d => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x19),8));
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.push(m,c.get(1,32));
            c.push(m,0xcu32);
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x6de4b8),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6ddbb0),16));
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4e4e4f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            { let a = 0x6dfc04u32; wm(m,a,32,0x0u32); }
            c.push(m, 0x4e4e66);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,c.get(4,32));
            if c.cond(5) { pc = 0x4e4ea0; continue; }
            pc = 0x4e4e72; continue;
            }
            0x4e4e72 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd760u32);
            c.push(m, 0x4e4e80);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x3u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd761u32);
            c.push(m, 0x4e4e99);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.push(m,0xfu32);
            pc = 0x4e4ed3; continue;
            }
            0x4e4ea0 => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd762u32);
            c.push(m, 0x4e4eae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x4u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd763u32);
            c.push(m, 0x4e4ec7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m,0x10u32);
            pc = 0x4e4ed3; continue;
            }
            0x4e4ed3 => {
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4e4ee2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            pc = 0x4e4ef0; continue;
            }
            0x4e4ef0 => {
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x34),32));
            if c.cond(5) { pc = 0x4e4f09; continue; }
            pc = 0x4e4ef8; continue;
            }
            0x4e4ef8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4e4dad; continue; }
            pc = 0x4e4f09; continue;
            }
            0x4e4f09 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4e55e2; continue; }
            pc = 0x4e4f15; continue;
            }
            0x4e4f15 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4e4f58; continue; }
            pc = 0x4e4f29; continue;
            }
            0x4e4f29 => {
            c.set(6,32,rm(m,c.r[1],32));
            c.set(7,32,0x2u32);
            pc = 0x4e4f30; continue;
            }
            0x4e4f30 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x6),16); c.sub(a,c.get(7,16),16); }
            c.set(1,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)));
            if c.cond(4) { pc = 0x4e4f49; continue; }
            pc = 0x4e4f3d; continue;
            }
            0x4e4f3d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4e4f30; continue; }
            pc = 0x4e4f47; continue;
            }
            0x4e4f47 => {
            pc = 0x4e4f58; continue;
            }
            0x4e4f49 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd73cu32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(1,32)); }
            pc = 0x4e4f58; continue;
            }
            0x4e4f58 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4e4f75; continue; }
            pc = 0x4e4f60; continue;
            }
            0x4e4f60 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,0x6de4acu32,32));
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x4e4f75; continue; }
            pc = 0x4e4f72; continue;
            }
            0x4e4f72 => {
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4e4f75; continue;
            }
            0x4e4f75 => {
            c.set(0,32,0x1u32);
            pc = 0x4e55e4; continue;
            }
            0x4e4f7f => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4e4f90);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xcu32);
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x6de4b8),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6ddbb0),16));
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4e4fd2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            { let a = 0x6dfc04u32; wm(m,a,32,0x0u32); }
            c.push(m, 0x4e4fe9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd764u32);
            c.push(m, 0x4e4ffa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x5u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd765u32);
            pc = 0x4e5230; continue;
            }
            0x4e5013 => {
            c.set(0,32,rm(m,0x6de4acu32,32));
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,c.get(1,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4e51a1; continue; }
            pc = 0x4e5035; continue;
            }
            0x4e5035 => {
            c.set(3,32,0x14u32);
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(3,16),16); }
            c.set(6,32,c.get(3,32));
            if c.cond(6) { pc = 0x4e5085; continue; }
            pc = 0x4e5045; continue;
            }
            0x4e5045 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4e5050);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0xd),8); c.sub(a,0x3u32,8); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,c.get(1,16),16); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.or(a,c.get(1,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x4e5078; continue; }
            pc = 0x4e506c; continue;
            }
            0x4e506c => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x6de52du32,8));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4e50b5; continue; }
            pc = 0x4e5078; continue;
            }
            0x4e5078 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4e5045; continue; }
            pc = 0x4e5085; continue;
            }
            0x4e5085 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0xc),16));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4e5180; continue; }
            pc = 0x4e5099; continue;
            }
            0x4e5099 => {
            c.set(6,32,rm(m,c.r[2],32));
            c.set(2,32,c.r[6].wrapping_add(0x6));
            pc = 0x4e509e; continue;
            }
            0x4e509e => {
            { let a = rm(m,c.r[2],16); c.sub(a,0x20u32,16); }
            if c.cond(4) { pc = 0x4e5178; continue; }
            pc = 0x4e50a8; continue;
            }
            0x4e50a8 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x18u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4e509e; continue; }
            pc = 0x4e50b0; continue;
            }
            0x4e50b0 => {
            pc = 0x4e5180; continue;
            }
            0x4e50b5 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xcu32);
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x6de4b8),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6ddbb0),16));
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4e50f7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            { let a = 0x6dfc04u32; wm(m,a,32,0x0u32); }
            c.push(m, 0x4e510e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.set(1,32,c.get(4,32));
            if c.cond(5) { pc = 0x4e514b; continue; }
            pc = 0x4e511e; continue;
            }
            0x4e511e => {
            c.push(m,0x6dd766u32);
            c.push(m, 0x4e5128);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x6u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd767u32);
            c.push(m, 0x4e5141);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.push(m,0xfu32);
            pc = 0x4e55cf; continue;
            }
            0x4e514b => {
            c.push(m,0x6dd768u32);
            c.push(m, 0x4e5155);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x7u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd769u32);
            c.push(m, 0x4e516e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.push(m,c.get(1,32));
            pc = 0x4e55cc; continue;
            }
            0x4e5178 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)));
            pc = 0x4e5182; continue;
            }
            0x4e5180 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4e5182; continue;
            }
            0x4e5182 => {
            c.set(0,32,rm(m,c.r[0],32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4e4c0c; continue; }
            pc = 0x4e5194; continue;
            }
            0x4e5194 => {
            c.set(1,32,rm(m,0x6de4acu32,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            pc = 0x4e4c05; continue;
            }
            0x4e51a1 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4e51b2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xcu32);
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x6de4b8),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6ddbb0),16));
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4e51f4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            { let a = 0x6dfc04u32; wm(m,a,32,0x0u32); }
            c.push(m, 0x4e520b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd76au32);
            c.push(m, 0x4e521c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x8u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd76bu32);
            pc = 0x4e5230; continue;
            }
            0x4e5230 => {
            c.push(m, 0x4e5235);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.set(2,16,rm(m,0x6de4aau32,16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x4e55cd; continue;
            }
            0x4e5247 => {
            c.set(0,32,rm(m,0x6de4acu32,32));
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4e5265; continue; }
            pc = 0x4e5256; continue;
            }
            0x4e5256 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0x1u32);
            pc = 0x4e55e4; continue;
            }
            0x4e5265 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4e5276);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb30u32,32));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x4e5283);
            { let args = []; c.r[0] = host.call(m, 0x4220b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x19),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x6de4b8),8));
            c.push(m,c.get(1,32));
            c.set(0,32,c.get(2,32));
            c.push(m,0xcu32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6ddbb0),16));
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4e52c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            { let a = 0x6dfc04u32; wm(m,a,32,0x0u32); }
            c.push(m, 0x4e52e0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.set(1,32,c.get(4,32));
            if c.cond(5) { pc = 0x4e5325; continue; }
            pc = 0x4e52f8; continue;
            }
            0x4e52f8 => {
            c.push(m,0x6dd76cu32);
            c.push(m, 0x4e5302);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x9u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd76du32);
            c.push(m, 0x4e531b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.push(m,0xfu32);
            pc = 0x4e55cf; continue;
            }
            0x4e5325 => {
            c.push(m,0x6dd76eu32);
            c.push(m, 0x4e532f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xau32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd76fu32);
            pc = 0x4e55c2; continue;
            }
            0x4e5348 => {
            c.set(0,32,rm(m,0x6de4acu32,32));
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4e5366; continue; }
            pc = 0x4e5357; continue;
            }
            0x4e5357 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0x1u32);
            pc = 0x4e55e4; continue;
            }
            0x4e5366 => {
            c.set(3,32,0x14u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,0x0u32); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(3,16),16); }
            c.set(6,32,0x80000000u32);
            if c.cond(6) { pc = 0x4e53c5; continue; }
            pc = 0x4e537e; continue;
            }
            0x4e537e => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4e5389);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0xd),8); c.sub(a,0x3u32,8); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,c.get(1,16),16); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.or(a,c.get(1,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x4e53b5; continue; }
            pc = 0x4e53a5; continue;
            }
            0x4e53a5 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x46c),32));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4e53b5; continue; }
            pc = 0x4e53af; continue;
            }
            0x4e53af => {
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(3,8)); }
            pc = 0x4e53b5; continue;
            }
            0x4e53b5 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4e537e; continue; }
            pc = 0x4e53c3; continue;
            }
            0x4e53c3 => {
            pc = 0x4e53c9; continue;
            }
            0x4e53c5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            pc = 0x4e53c9; continue;
            }
            0x4e53c9 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xcu32);
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x6de4b8),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6ddbb0),16));
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4e540b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            { let a = 0x6dfc04u32; wm(m,a,32,0x0u32); }
            c.push(m, 0x4e5422);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.set(1,32,c.get(4,32));
            if c.cond(5) { pc = 0x4e546f; continue; }
            pc = 0x4e543c; continue;
            }
            0x4e543c => {
            c.push(m,0x6dd770u32);
            c.push(m, 0x4e5446);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xbu32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd771u32);
            c.push(m, 0x4e545f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x3c),8));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,0xfu32);
            pc = 0x4e55cf; continue;
            }
            0x4e546f => {
            c.push(m,0x6dd772u32);
            c.push(m, 0x4e5479);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xcu32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd773u32);
            c.push(m, 0x4e5492);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x3c),8));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x4e55cd; continue;
            }
            0x4e54a3 => {
            c.set(6,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(6) { pc = 0x4e54cc; continue; }
            pc = 0x4e54b1; continue;
            }
            0x4e54b1 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(2,32); let r = c.add(a,0x2d4u32,32); c.set(2,32,r); }
            pc = 0x4e54bd; continue;
            }
            0x4e54bd => {
            c.set(3,32,rm(m,c.r[2],32));
            { let a = c.get(2,32); let r = c.add(a,0x9f8u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x4e54bd; continue; }
            pc = 0x4e54cc; continue;
            }
            0x4e54cc => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4e54e9; continue; }
            pc = 0x4e54d0; continue;
            }
            0x4e54d0 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let r = c.add(a,0x100u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0x1u32);
            pc = 0x4e55e4; continue;
            }
            0x4e54e9 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4e54fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb30u32,32));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x4e5507);
            { let args = []; c.r[0] = host.call(m, 0x4220b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x19),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x6de4b8),8));
            c.push(m,c.get(1,32));
            c.push(m,0xcu32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6ddbb0),16));
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4e554b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            { let a = 0x6dfc04u32; wm(m,a,32,0x0u32); }
            c.push(m, 0x4e5562);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(4,32)); }
            c.set(1,32,c.get(4,32));
            if c.cond(5) { pc = 0x4e55a4; continue; }
            pc = 0x4e557a; continue;
            }
            0x4e557a => {
            c.push(m,0x6dd774u32);
            c.push(m, 0x4e5584);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xdu32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd775u32);
            c.push(m, 0x4e559d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.push(m,0xfu32);
            pc = 0x4e55cf; continue;
            }
            0x4e55a4 => {
            c.push(m,0x6dd776u32);
            c.push(m, 0x4e55ae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xeu32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd777u32);
            pc = 0x4e55c2; continue;
            }
            0x4e55c2 => {
            c.push(m, 0x4e55c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.push(m,c.get(2,32));
            pc = 0x4e55cc; continue;
            }
            0x4e55cc => {
            c.push(m,c.get(6,32));
            pc = 0x4e55cd; continue;
            }
            0x4e55cd => {
            c.push(m,0x10u32);
            pc = 0x4e55cf; continue;
            }
            0x4e55cf => {
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4e55e2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4e55e2; continue;
            }
            0x4e55e2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4e55e4; continue;
            }
            0x4e55e4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn st_alderman_mission(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4eafe4;
    loop {
        match pc {
            0x4eafe4 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x662d9au32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x38u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(6,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x4eb0eb; continue; }
            pc = 0x4eb026; continue;
            }
            0x4eb026 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.sub(a,0x5u32,8); }
            if c.cond(3) { pc = 0x4ebf0f; continue; }
            pc = 0x4eb030; continue;
            }
            0x4eb030 => {
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4eb03b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x532c50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4ebf0f; continue; }
            pc = 0x4eb043; continue;
            }
            0x4eb043 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xe),8));
            { let a = c.get(0,8); c.sub(a,0x3fu32,8); }
            if c.cond(3) { pc = 0x4eb070; continue; }
            pc = 0x4eb04a; continue;
            }
            0x4eb04a => {
            { let a = c.get(0,8); let r = c.add(a,0x2u32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0xe); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,0x200u32,32); c.set(0,32,r); }
            pc = 0x4eb056; continue;
            }
            0x4eb056 => {
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4eb058; continue;
            }
            0x4eb058 => {
            c.set(0,32,0x1u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4eb070 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,0x6de4aau32,16));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x4eb0af; continue; }
            pc = 0x4eb07d; continue;
            }
            0x4eb07d => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(3,32));
            c.push(m, 0x4eb089);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x4eb0ab; continue; }
            pc = 0x4eb090; continue;
            }
            0x4eb090 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x37),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(6) { pc = 0x4eb0ab; continue; }
            pc = 0x4eb097; continue;
            }
            0x4eb097 => {
            { let a = c.get(1,8); c.sub(a,0x4u32,8); }
            if c.cond(3) { pc = 0x4eb0ab; continue; }
            pc = 0x4eb09c; continue;
            }
            0x4eb09c => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4eb0a3);
            { let args = []; c.r[0] = host.call(m, 0x4f73d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(14) { pc = 0x4eb0ab; continue; }
            pc = 0x4eb0a8; continue;
            }
            0x4eb0a8 => {
            { let a = c.r[6].wrapping_add(0xd); wm(m,a,8,c.get(3,8)); }
            pc = 0x4eb0ab; continue;
            }
            0x4eb0ab => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(7) { pc = 0x4eb07d; continue; }
            pc = 0x4eb0af; continue;
            }
            0x4eb0af => {
            c.set(7,32,rm(m,c.r[6],32));
            { let a = c.r[6].wrapping_add(0xf); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,0x6de4a4u32,32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd),8));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x300u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(7,32)); }
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4eb0eb => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xf),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4eb13a; continue; }
            pc = 0x4eb0f2; continue;
            }
            0x4eb0f2 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.push(m, 0x4eb101);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4ebf54, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,0x1u32); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.add(a,0x100u32,32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(0,32,0x1u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4eb13a => {
            c.set(3,32,rm(m,c.r[6],32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4eb3eb; continue; }
            pc = 0x4eb147; continue;
            }
            0x4eb147 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4eb157);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4eb160);
            { let args = []; c.r[0] = host.call(m, 0x4f9170, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4eb3ba; continue; }
            pc = 0x4eb16b; continue;
            }
            0x4eb16b => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x4eb174);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,0x6de52eu32,8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x70u32); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x19),8));
            { let a = c.r[4].wrapping_add(0x2d); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x0u32); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x39c),8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4eb1a9);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1d),8));
            c.set(2,8,c.get(0,8));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.r[4].wrapping_add(0x33); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,0x6de52du32,8));
            { let a = c.get(0,8); let n = 0x5u32; let r = c.shl(a,n,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.add(a,c.get(0,8),8); c.set(2,8,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1c),8));
            { let a = c.r[4].wrapping_add(0x31); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,8,c.get(0,8)); }
            c.set(2,8,c.get(1,8));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4eb20c; continue; }
            pc = 0x4eb1df; continue;
            }
            0x4eb1df => {
            c.set(0,8,c.get(1,8));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4eb1ec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1c),8));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            { let a = c.r[4].wrapping_add(0x35); wm(m,a,8,c.get(2,8)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4eb20a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4eb23e; continue;
            }
            0x4eb20c => {
            c.set(0,8,rm(m,0x6de52eu32,8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6d4),8));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x6d8),8));
            { let a = c.r[4].wrapping_add(0x35); wm(m,a,8,c.get(2,8)); }
            pc = 0x4eb23e; continue;
            }
            0x4eb23e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x4eb31b; continue; }
            pc = 0x4eb24c; continue;
            }
            0x4eb24c => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4ebf24)); continue;
            }
            0x4eb253 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x16),8));
            { let a = c.r[4].wrapping_add(0x36); wm(m,a,8,c.get(0,8)); }
            pc = 0x4eb316; continue;
            }
            0x4eb25f => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x10),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x11),8));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x36); wm(m,a,8,c.get(1,8)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x3b); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x4eb27f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x8),16));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,0x1u32); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(6,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x6),16));
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,0xba00u32,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,8,0x1u32); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x10),8));
            { let a = c.r[0].wrapping_add(0xd); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xe),8));
            { let a = c.r[0].wrapping_add(0xe); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xf),8));
            { let a = c.r[0].wrapping_add(0xf); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x10),8));
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x11),8));
            { let a = c.r[0].wrapping_add(0x11); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x12),8));
            { let a = c.r[0].wrapping_add(0x12); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[0].wrapping_add(0x13); wm(m,a,8,0xffu32); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            pc = 0x4eb31b; continue;
            }
            0x4eb2da => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.r[4].wrapping_add(0x36); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x37); wm(m,a,8,c.get(1,8)); }
            pc = 0x4eb31b; continue;
            }
            0x4eb2ea => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x11),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x3a); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x3b); wm(m,a,8,0x0u32); }
            c.push(m, 0x4eb306);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0xe1); wm(m,a,8,0xffu32); }
            pc = 0x4eb31b; continue;
            }
            0x4eb30f => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x12),8));
            { let a = c.r[4].wrapping_add(0x36); wm(m,a,8,c.get(1,8)); }
            pc = 0x4eb316; continue;
            }
            0x4eb316 => {
            { let a = c.r[4].wrapping_add(0x37); wm(m,a,8,0x0u32); }
            pc = 0x4eb31b; continue;
            }
            0x4eb31b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4eb330);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(1,16,rm(m,0x6de52eu32,8));
            c.push(m,c.get(0,32));
            c.push(m,0x1eu32);
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4eb350);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xd),8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4eb3ad; continue; }
            pc = 0x4eb35e; continue;
            }
            0x4eb35e => {
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            c.push(m, 0x4eb36b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd77eu32);
            c.push(m, 0x4eb37c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd77fu32);
            c.push(m, 0x4eb392);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0x24u32);
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            c.push(m, 0x4eb3ad);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4eb3ad; continue;
            }
            0x4eb3ad => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x4eb3ba);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4eb3ba; continue;
            }
            0x4eb3ba => {
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,8,0x5u32); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(0,32,c.get(1,32));
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x1c00u32,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0x1u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4eb3eb => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(1,32); c.sub(a,0x5u32,32); }
            if c.cond(7) { pc = 0x4ebf0f; continue; }
            pc = 0x4eb3f9; continue;
            }
            0x4eb3f9 => {
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x4ebf38)); continue;
            }
            0x4eb400 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x16),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x76c),32));
            { let a = c.get(5,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4eb42a; continue; }
            pc = 0x4eb41e; continue;
            }
            0x4eb41e => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4eb425);
            { let args = []; c.r[0] = host.call(m, 0x4ec354, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4ebf0f; continue;
            }
            0x4eb42a => {
            { let a = c.get(3,32); let r = c.add(a,0x200u32,32); c.set(3,32,r); }
            c.set(0,32,0x1u32);
            { let a = c.r[6]; wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4eb44a => {
            c.set(7,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x10),8));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4ebf0f; continue; }
            pc = 0x4eb45d; continue;
            }
            0x4eb45d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x11),8));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4ebf0f; continue; }
            pc = 0x4eb46a; continue;
            }
            0x4eb46a => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x4eb62d; continue; }
            pc = 0x4eb475; continue;
            }
            0x4eb475 => {
            c.set(7,32,c.r[6].wrapping_add(0x14));
            { let a = c.get(3,32); let r = c.add(a,0x180u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,0x672bf0u32,32); c.set(7,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            pc = 0x4eb48c; continue;
            }
            0x4eb48c => {
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x13),8));
            { let a = c.get(1,8); let r = c.add(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x672bf0),8));
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672bf0),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4eb058; continue; }
            pc = 0x4eb4b8; continue;
            }
            0x4eb4b8 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4eb48c; continue; }
            pc = 0x4eb4be; continue;
            }
            0x4eb4be => {
            c.set(0,32,0x672bf0u32);
            c.set(1,32,c.r[6].wrapping_add(0x14));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(5,32,0x8u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x4eb4d3; continue;
            }
            0x4eb4d3 => {
            c.set(1,8,c.get(2,8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x13),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,8); let r = c.add(a,c.get(1,8),8); c.set(1,8,r); }
            c.set(7,32,c.r[6].wrapping_add(c.r[2]).wrapping_add(0x14));
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[3]),8));
            c.set(3,8,0x3u32);
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7],8));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[2]));
            { let a = c.get(3,8); let n = c.get(1,8); let r = c.shl(a,n,8); c.set(3,8,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x13),8));
            { let a = c.get(0,32); let r = c.and(a,0x300u32,32); c.set(0,32,r); }
            { let a = c.get(3,8); c.set(3,8,!a & 0xffu32); }
            { let a = c.get(3,8); let r = c.and(a,c.get(1,8),8); c.set(3,8,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,0x2u32,32); c.set(5,32,r); }
            { let a = c.get(3,8); let r = c.or(a,c.get(0,8),8); c.set(3,8,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[6].wrapping_add(0x13); wm(m,a,8,c.get(3,8)); }
            if c.cond(15) { pc = 0x4eb4d3; continue; }
            pc = 0x4eb51f; continue;
            }
            0x4eb51f => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12),8));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x4eb5c1; continue; }
            pc = 0x4eb52d; continue;
            }
            0x4eb52d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x10),8));
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,0x27u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[7].wrapping_add(0x7a4));
            c.push(m, 0x4eb55d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f9c80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,0x0u32);
            c.push(m,0x27u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0xffu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4eb577);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x521900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x11),8));
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,0x27u32);
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[7].wrapping_add(0x7a4));
            c.push(m, 0x4eb5a7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f9c80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,0x0u32);
            c.push(m,0x27u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0xffu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4eb5c1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x521900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4eb5c1; continue;
            }
            0x4eb5c1 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4eb615; continue; }
            pc = 0x4eb5ca; continue;
            }
            0x4eb5ca => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x12),8));
            c.set(7,32,c.get(1,32));
            pc = 0x4eb5d7; continue;
            }
            0x4eb5d7 => {
            c.set(1,8,c.get(0,8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x13),8));
            { let a = c.get(1,8); let r = c.add(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.get(3,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x672bf0),8));
            { let a = c.get(3,32); let r = c.and(a,0x3u32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672bf0),8));
            { let a = c.get(1,32); let r = c.imul(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4eb058; continue; }
            pc = 0x4eb606; continue;
            }
            0x4eb606 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x4eb5d7; continue; }
            pc = 0x4eb60c; continue;
            }
            0x4eb60c => {
            { let a = c.get(2,32); let r = c.add(a,0x200u32,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4eb615; continue;
            }
            0x4eb615 => {
            c.set(0,32,0x1u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4eb62d => {
            c.set(0,32,c.r[3].wrapping_add(0x200));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4eb63d; continue; }
            pc = 0x4eb639; continue;
            }
            0x4eb639 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x4eb63d; continue;
            }
            0x4eb63d => {
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x764),16));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x776),16));
            c.set(3,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4eb68e; continue; }
            pc = 0x4eb665; continue;
            }
            0x4eb665 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4eb66d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x27u32,8); }
            if c.cond(4) { pc = 0x4eb6a6; continue; }
            pc = 0x4eb673; continue;
            }
            0x4eb673 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4eb67b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x776),16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4eb665; continue; }
            pc = 0x4eb68e; continue;
            }
            0x4eb68e => {
            c.set(0,32,0x1u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4eb6a6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x11),8));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x776),16));
            c.set(6,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x764),16));
            c.set(7,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4eb701; continue; }
            pc = 0x4eb6d4; continue;
            }
            0x4eb6d4 => {
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4eb6dc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x27u32,8); }
            if c.cond(4) { pc = 0x4ebe73; continue; }
            pc = 0x4eb6e6; continue;
            }
            0x4eb6e6 => {
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4eb6ee);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x4eb6d4; continue; }
            pc = 0x4eb701; continue;
            }
            0x4eb701 => {
            c.set(0,32,0x1u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4eb719 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x16),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4eb7a5; continue; }
            pc = 0x4eb724; continue;
            }
            0x4eb724 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4eb781; continue; }
            pc = 0x4eb731; continue;
            }
            0x4eb731 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0x12u32,16); }
            if c.cond(5) { pc = 0x4eb781; continue; }
            pc = 0x4eb74e; continue;
            }
            0x4eb74e => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x42),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x4eb781; continue; }
            pc = 0x4eb75b; continue;
            }
            0x4eb75b => {
            c.set(2,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2),8));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x12),16),16); }
            if c.cond(5) { pc = 0x4eb781; continue; }
            pc = 0x4eb776; continue;
            }
            0x4eb776 => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x14),16),16); }
            if c.cond(4) { pc = 0x4eb785; continue; }
            pc = 0x4eb781; continue;
            }
            0x4eb781 => {
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,8,0x2u32); }
            pc = 0x4eb785; continue;
            }
            0x4eb785 => {
            { let a = c.get(3,32); let r = c.add(a,0x400u32,32); c.set(3,32,r); }
            c.set(0,32,0x1u32);
            { let a = c.r[6]; wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4eb7a5 => {
            { let a = c.get(1,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4eb7b6; continue; }
            pc = 0x4eb7aa; continue;
            }
            0x4eb7aa => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4eb7b1);
            { let args = []; c.r[0] = host.call(m, 0x4ec354, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4ebf0f; continue;
            }
            0x4eb7b6 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4eb7c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4ebf0f; continue; }
            pc = 0x4eb7d1; continue;
            }
            0x4eb7d1 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(1,16,rm(m,0x6de52eu32,8));
            c.push(m,c.get(0,32));
            c.push(m,0x1eu32);
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4eb7f1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xd),8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4ebf0f; continue; }
            pc = 0x4eb803; continue;
            }
            0x4eb803 => {
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            c.push(m, 0x4eb810);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd780u32);
            c.push(m, 0x4eb821);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0x2u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd781u32);
            c.push(m, 0x4eb83a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,0x24u32);
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4eb858);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4ebf0f; continue;
            }
            0x4eb85d => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x12),8));
            { let a = c.get(1,32); let r = c.sub(a,0x0u32,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x4ebce3; continue; }
            pc = 0x4eb86b; continue;
            }
            0x4eb86b => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x4eb951; continue; }
            pc = 0x4eb872; continue;
            }
            0x4eb872 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            if c.cond(4) { pc = 0x4eb939; continue; }
            pc = 0x4eb884; continue;
            }
            0x4eb884 => {
            c.push(m, 0x4eb889);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0xe1); wm(m,a,8,0xffu32); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xd),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4eb8a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4ebf0f; continue; }
            pc = 0x4eb8ab; continue;
            }
            0x4eb8ab => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(2,16,rm(m,0x6de52eu32,8));
            c.push(m,c.get(0,32));
            c.push(m,0x1eu32);
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4eb8cb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4ebf0f; continue; }
            pc = 0x4eb8de; continue;
            }
            0x4eb8de => {
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            c.push(m, 0x4eb8eb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd782u32);
            c.push(m, 0x4eb8fc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0x3u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd783u32);
            c.push(m, 0x4eb915);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x24u32);
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4eb934);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4ebf0f; continue;
            }
            0x4eb939 => {
            c.push(m, 0x4eb93e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            { let a = c.r[0].wrapping_add(0xe1); wm(m,a,8,0xffu32); }
            c.push(m, 0x4eb94c);
            { let args = []; c.r[0] = host.call(m, 0x4ec354, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4ebf0f; continue;
            }
            0x4eb951 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x10),8));
            { let a = c.get(1,8); c.sub(a,0x5u32,8); }
            if c.cond(3) { pc = 0x4eb972; continue; }
            pc = 0x4eb959; continue;
            }
            0x4eb959 => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4eb972; continue; }
            pc = 0x4eb96a; continue;
            }
            0x4eb96a => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x11),8));
            { let a = rm(m,c.r[1].wrapping_add(0xe),8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4eb991; continue; }
            pc = 0x4eb972; continue;
            }
            0x4eb972 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,8,0x3u32); }
            { let a = c.r[6]; wm(m,a,32,c.get(3,32)); }
            c.set(0,32,0x1u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4eb991 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4eb9df; continue; }
            pc = 0x4eb99e; continue;
            }
            0x4eb99e => {
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(7,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(7,32,sx(rm(m,c.r[1].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[2].wrapping_add(0x6ddbb0),32),32); }
            if c.cond(5) { pc = 0x4eb9d9; continue; }
            pc = 0x4eb9cd; continue;
            }
            0x4eb9cd => {
            c.set(1,32,sx(rm(m,c.r[1].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[2].wrapping_add(0x6ddbb4),32),32); }
            if c.cond(4) { pc = 0x4eb9df; continue; }
            pc = 0x4eb9d9; continue;
            }
            0x4eb9d9 => {
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,0xffffu32); }
            pc = 0x4eb9df; continue;
            }
            0x4eb9df => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x1fu32); }
            c.push(m, 0x4eb9f7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4ebcc1; continue; }
            pc = 0x4eba11; continue;
            }
            0x4eba11 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4ebca7; continue; }
            pc = 0x4eba2d; continue;
            }
            0x4eba2d => {
            { let a = rm(m,c.r[7].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x4ebca7; continue; }
            pc = 0x4eba3b; continue;
            }
            0x4eba3b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x11),8));
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x6ddbb0),32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x6ddbb4),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x4ebc76; continue; }
            pc = 0x4eba71; continue;
            }
            0x4eba71 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x4ebb20; continue; }
            pc = 0x4eba87; continue;
            }
            0x4eba87 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4eba96);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50a4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4ebca7; continue; }
            pc = 0x4eba9e; continue;
            }
            0x4eba9e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,0x20u32);
            c.set(5,32,0x14u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x18),32));
            c.set(1,16,rm(m,c.r[2].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.or(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(0x12); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[2].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4ebb0d; continue; }
            pc = 0x4ebae0; continue;
            }
            0x4ebae0 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = rm(m,c.r[0].wrapping_add(0x3c),16); let r = c.or(a,c.get(7,16),16); { let a = c.r[0].wrapping_add(0x3c); wm(m,a,16,r); } }
            { let a = c.r[0].wrapping_add(0x134); wm(m,a,16,c.get(5,16)); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4ebae0; continue; }
            pc = 0x4ebb0d; continue;
            }
            0x4ebb0d => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x10),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4ebb1e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4ebb72; continue;
            }
            0x4ebb20 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x4ebb26);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x505b60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4ebca7; continue; }
            pc = 0x4ebb2e; continue;
            }
            0x4ebb2e => {
            { let a = rm(m,c.r[7].wrapping_add(0x3c),8); let r = c.or(a,0x20u32,8); { let a = c.r[7].wrapping_add(0x3c); wm(m,a,8,r); } }
            c.push(m,c.get(3,32));
            c.set(1,32,0x6dd7a0u32);
            { let a = c.r[7].wrapping_add(0x134); wm(m,a,16,0x14u32); }
            c.push(m, 0x4ebb46);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4ebb56);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5062f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14); wm(m,a,16,0x12u32); }
            pc = 0x4ebb72; continue;
            }
            0x4ebb72 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x11),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4ebb85);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x50e280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x10),8));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            c.push(m, 0x4ebb96);
            { let args = []; c.r[0] = host.call(m, 0x515d00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4ebc02; continue; }
            pc = 0x4ebb9a; continue;
            }
            0x4ebb9a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x10),8));
            c.set(2,8,0xfeu32);
            c.push(m,c.get(1,32));
            { let a = c.get(2,8); let r = c.sub(a,c.get(0,8),8); c.set(2,8,r); }
            c.set(1,32,0x6e55d0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4ebbc4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6130b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.sub(a,0xffu32,32); }
            if c.cond(13) { pc = 0x4ebc02; continue; }
            pc = 0x4ebbd3; continue;
            }
            0x4ebbd3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(1,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6e55d0),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4ebbfa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x60234c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); c.sub(a,0xffu32,32); }
            if c.cond(12) { pc = 0x4ebc57; continue; }
            pc = 0x4ebc02; continue;
            }
            0x4ebc02 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(3,16)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x10),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            c.push(m, 0x4ebc17);
            { let args = []; c.r[0] = host.call(m, 0x514530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x6e55d0u32);
            c.push(m, 0x4ebc21);
            { let args = []; c.r[0] = host.call(m, 0x611d80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,8,0xfeu32);
            c.set(1,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x10),8));
            { let a = c.get(2,8); let r = c.sub(a,c.get(0,8),8); c.set(2,8,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6e55d0),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x4ebc57);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5fbfe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4ebc57; continue;
            }
            0x4ebc57 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,0x40u32,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0x1u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4ebc76 => {
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(13) { pc = 0x4ebc85; continue; }
            pc = 0x4ebc7b; continue;
            }
            0x4ebc7b => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            pc = 0x4ebca7; continue;
            }
            0x4ebc85 => {
            { let a = c.get(0,32); c.sub(a,0xe1u32,32); }
            if c.cond(13) { pc = 0x4ebc95; continue; }
            pc = 0x4ebc8c; continue;
            }
            0x4ebc8c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            pc = 0x4ebca3; continue;
            }
            0x4ebc95 => {
            { let a = c.get(0,32); c.sub(a,0x640u32,32); }
            if c.cond(13) { pc = 0x4ebca7; continue; }
            pc = 0x4ebc9c; continue;
            }
            0x4ebc9c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.and(a,0x7u32,32); c.set(0,32,r); }
            pc = 0x4ebca3; continue;
            }
            0x4ebca3 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x4ebca7; continue;
            }
            0x4ebca7 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4eba11; continue; }
            pc = 0x4ebcc1; continue;
            }
            0x4ebcc1 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0x1u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4ebce3 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x10),8));
            { let a = c.get(1,8); c.sub(a,0x5u32,8); }
            if c.cond(3) { pc = 0x4ebd06; continue; }
            pc = 0x4ebceb; continue;
            }
            0x4ebceb => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4ebd06; continue; }
            pc = 0x4ebcfc; continue;
            }
            0x4ebcfc => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xe),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x11),8));
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4ebd2a; continue; }
            pc = 0x4ebd06; continue;
            }
            0x4ebd06 => {
            { let a = c.get(3,32); let r = c.add(a,0x100u32,32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,8,0x3u32); }
            { let a = c.r[6]; wm(m,a,32,c.get(3,32)); }
            c.set(0,32,0x1u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4ebd2a => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.set(3,32,0x40u32);
            c.push(m, 0x4ebd3f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4ebe33; continue; }
            pc = 0x4ebd57; continue;
            }
            0x4ebd57 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x11),8));
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x6ddbb0),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x6ddbb4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xc8u32,32); }
            if c.cond(13) { pc = 0x4ebde3; continue; }
            pc = 0x4ebda2; continue;
            }
            0x4ebda2 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xd),8));
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,8,0x1u32); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4ebdb6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x11),8));
            c.set(5,32,0x6dd7acu32);
            { let a = c.r[0].wrapping_add(0xe1); wm(m,a,8,c.get(1,8)); }
            pc = 0x4ebdc4; continue;
            }
            0x4ebdc4 => {
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4ebdd6; continue; }
            pc = 0x4ebdcb; continue;
            }
            0x4ebdcb => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x11),8));
            c.push(m,c.get(2,32));
            c.push(m, 0x4ebdd6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5144a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4ebdd6; continue;
            }
            0x4ebdd6 => {
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x6dd7c0u32,32); }
            if c.cond(12) { pc = 0x4ebdc4; continue; }
            pc = 0x4ebde1; continue;
            }
            0x4ebde1 => {
            pc = 0x4ebe1f; continue;
            }
            0x4ebde3 => {
            { let a = c.get(0,32); c.sub(a,0x1000u32,32); }
            if c.cond(13) { pc = 0x4ebe1f; continue; }
            pc = 0x4ebdea; continue;
            }
            0x4ebdea => {
            { let a = c.get(0,32); c.sub(a,0x400u32,32); }
            if c.cond(14) { pc = 0x4ebdf8; continue; }
            pc = 0x4ebdf1; continue;
            }
            0x4ebdf1 => {
            c.set(0,32,0x16u32);
            pc = 0x4ebe19; continue;
            }
            0x4ebdf8 => {
            { let a = c.get(0,32); c.sub(a,0x200u32,32); }
            if c.cond(14) { pc = 0x4ebe06; continue; }
            pc = 0x4ebdff; continue;
            }
            0x4ebdff => {
            c.set(0,32,0xcu32);
            pc = 0x4ebe19; continue;
            }
            0x4ebe06 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x100u32,32); }
            c.set(1,8,u32::from(c.cond(15)));
            c.set(1,32,c.r[1].wrapping_mul(4).wrapping_add(0x2));
            c.set(0,32,c.get(1,32));
            pc = 0x4ebe19; continue;
            }
            0x4ebe19 => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x4ebe1f; continue; }
            pc = 0x4ebe1d; continue;
            }
            0x4ebe1d => {
            c.set(3,32,c.get(0,32));
            pc = 0x4ebe1f; continue;
            }
            0x4ebe1f => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4ebd57; continue; }
            pc = 0x4ebe33; continue;
            }
            0x4ebe33 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0x1u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4ebe51 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2d4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4ebe7f; continue; }
            pc = 0x4ebe73; continue;
            }
            0x4ebe73 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4ebe7a);
            { let args = []; c.r[0] = host.call(m, 0x4ec354, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4ebf0f; continue;
            }
            0x4ebe7f => {
            c.set(0,32,c.r[3].wrapping_add(0x700));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x4eb056; continue; }
            pc = 0x4ebe8d; continue;
            }
            0x4ebe8d => {
            c.set(0,32,c.r[3].wrapping_add(0x1));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x4eb058; continue; }
            pc = 0x4ebe9a; continue;
            }
            0x4ebe9a => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4eb058; continue;
            }
            0x4ebea2 => {
            c.set(1,32,rm(m,0x6e558cu32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4ebf0f; continue; }
            pc = 0x4ebeac; continue;
            }
            0x4ebeac => {
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xc8),32); c.push(m, 0x4ebeb4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4ebf0f; continue; }
            pc = 0x4ebeb8; continue;
            }
            0x4ebeb8 => {
            c.set(1,32,rm(m,0x6e558cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x4ebec3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e558cu32,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x4ebed2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e558cu32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x4ebee1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,rm(m,0x6e558cu32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x4ebef6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.push(m, 0x4ebf0f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4ebf0f; continue;
            }
            0x4ebf0f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x44u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_alderman_mission_completed(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ec354;
    loop {
        match pc {
            0x4ec354 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662dc6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x4ec37b);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0xd),8));
            c.set(6,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ec3a2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,8,rm(m,0x6de52eu32,8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x6fu32); }
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x37),8));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            c.set(0,8,c.get(2,8));
            { let a = c.r[7].wrapping_add(0x37); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(5) { pc = 0x4ec3cc; continue; }
            pc = 0x4ec3c7; continue;
            }
            0x4ec3c7 => {
            { let a = rm(m,c.r[4].wrapping_add(0x20),8); let r = c.or(a,0x80u32,8); { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,r); } }
            pc = 0x4ec3cc; continue;
            }
            0x4ec3cc => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x19),8));
            c.set(3,8,rm(m,c.r[2].wrapping_add(c.r[7]).wrapping_add(0x39c),8));
            c.push(m, 0x4ec3df);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,8); let n = 0x5u32; let r = c.shl(a,n,8); c.set(3,8,r); }
            { let a = c.get(0,8); let r = c.add(a,c.get(3,8),8); c.set(0,8,r); }
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            { let a = c.r[4].wrapping_add(0x21); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1c),8));
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1d),8));
            { let a = c.r[4].wrapping_add(0x23); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,0x6de52du32,8));
            c.set(2,8,c.get(1,8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4ec43b; continue; }
            pc = 0x4ec40c; continue;
            }
            0x4ec40c => {
            c.set(0,8,c.get(1,8));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ec419);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1c),8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4ec437);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            pc = 0x4ec46e; continue;
            }
            0x4ec43b => {
            c.set(0,8,rm(m,0x6de52eu32,8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.or(a,0xffffffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6d4),8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x6d8),8));
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,c.get(2,8)); }
            pc = 0x4ec46e; continue;
            }
            0x4ec46e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x4ec4f0; continue; }
            pc = 0x4ec47a; continue;
            }
            0x4ec47a => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4ec5e0)); continue;
            }
            0x4ec481 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x16),8));
            { let a = c.r[4].wrapping_add(0x27); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x26); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x37),8));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            c.set(0,8,c.get(1,8));
            { let a = c.r[7].wrapping_add(0x37); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(5) { pc = 0x4ec4a0; continue; }
            pc = 0x4ec49b; continue;
            }
            0x4ec49b => {
            { let a = rm(m,c.r[4].wrapping_add(0x20),8); let r = c.or(a,0x80u32,8); { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,r); } }
            pc = 0x4ec4a0; continue;
            }
            0x4ec4a0 => {
            c.set(5,32,0x1fu32);
            pc = 0x4ec4f0; continue;
            }
            0x4ec4a7 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x10),8));
            c.set(5,32,0x20u32);
            { let a = c.r[4].wrapping_add(0x26); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x11),8));
            { let a = c.r[4].wrapping_add(0x27); wm(m,a,8,c.get(2,8)); }
            pc = 0x4ec4f0; continue;
            }
            0x4ec4bc => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12),8));
            c.set(5,32,0x21u32);
            { let a = c.r[4].wrapping_add(0x26); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.r[4].wrapping_add(0x27); wm(m,a,8,c.get(1,8)); }
            pc = 0x4ec4f0; continue;
            }
            0x4ec4d1 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x11),8));
            c.set(5,32,0x22u32);
            { let a = c.r[4].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            pc = 0x4ec4eb; continue;
            }
            0x4ec4df => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12),8));
            c.set(5,32,0x23u32);
            { let a = c.r[4].wrapping_add(0x26); wm(m,a,8,c.get(0,8)); }
            pc = 0x4ec4eb; continue;
            }
            0x4ec4eb => {
            { let a = c.r[4].wrapping_add(0x27); wm(m,a,8,0x0u32); }
            pc = 0x4ec4f0; continue;
            }
            0x4ec4f0 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,0xfffffffeu32);
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4ec501);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4ec5bb; continue; }
            pc = 0x4ec50c; continue;
            }
            0x4ec50c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4ec521);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(1,16,rm(m,0x6de52eu32,8));
            c.push(m,c.get(0,32));
            c.push(m,0x1fu32);
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4ec541);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xd),8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4ec5bb; continue; }
            pc = 0x4ec54f; continue;
            }
            0x4ec54f => {
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x0u32);
            c.push(m, 0x4ec55c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd784u32);
            c.push(m, 0x4ec56d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x1u32); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dd785u32);
            c.push(m, 0x4ec583);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x2e),8));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x2f),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x0u32); }
            c.push(m, 0x4ec5ac);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,0x6e558cu32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4ec5bb; continue; }
            pc = 0x4ec5b6; continue;
            }
            0x4ec5b6 => {
            c.push(m, 0x4ec5bb);
            { let args = []; c.r[0] = host.call(m, 0x5e94b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4ec5bb; continue;
            }
            0x4ec5bb => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4ec5cc);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn class22_on_click_map_object(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x594dd0;
    loop {
        match pc {
            0x594dd0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x30u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[7].wrapping_add(0xc314),32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[6],32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xc308),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),16));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x98),32));
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,c.get(5,32));
            c.set(0,32,rm(m,c.r[5],32));
            { let target = rm(m,c.r[0].wrapping_add(0x50),32); c.push(m, 0x594e0d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc324),32));
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x59551e; continue; }
            pc = 0x594e25; continue;
            }
            0x594e25 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(2,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x5952f6; continue; }
            pc = 0x594e32; continue;
            }
            0x594e32 => {
            { let a = c.get(0,32); c.sub(a,0x69u32,32); }
            if c.cond(7) { pc = 0x5952bb; continue; }
            pc = 0x594e3b; continue;
            }
            0x594e3b => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x595644),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x5955d8)); continue;
            }
            0x594e4a => {
            c.set(1,32,rm(m,0x6e5524u32,32));
            c.push(m, 0x594e55);
            { let args = []; c.r[0] = host.call(m, 0x5ad120, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e5524u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x594e65); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x594e6f => {
            c.set(1,32,rm(m,0x6e54f8u32,32));
            c.push(m, 0x594e7a);
            { let args = []; c.r[0] = host.call(m, 0x5a5ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e54f8u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x594e8a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x594e94 => {
            c.set(1,32,rm(m,0x6e54f0u32,32));
            c.push(m, 0x594e9f);
            { let args = []; c.r[0] = host.call(m, 0x5a5090, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e54f0u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x594eaf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x594eb9 => {
            c.set(1,32,rm(m,0x6e54e0u32,32));
            c.push(m, 0x594ec4);
            { let args = []; c.r[0] = host.call(m, 0x5a3330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e54e0u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x594ed4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x594ede => {
            c.set(1,32,rm(m,0x6e5550u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x594eec); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x594ef6 => {
            c.set(1,32,rm(m,0x6e551cu32,32));
            c.push(m, 0x594f01);
            { let args = []; c.r[0] = host.call(m, 0x5ac550, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e551cu32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x594f11); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x594f1b => {
            c.set(1,32,rm(m,0x6e54e8u32,32));
            c.push(m, 0x594f26);
            { let args = []; c.r[0] = host.call(m, 0x5a4270, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e54e8u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x594f36); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x594f40 => {
            c.set(1,32,rm(m,0x6e54d8u32,32));
            c.push(m, 0x594f4b);
            { let args = []; c.r[0] = host.call(m, 0x5a2740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e54d8u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x594f5b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x594f65 => {
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x594f6d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(15) { pc = 0x5955cc; continue; }
            pc = 0x594f76; continue;
            }
            0x594f76 => {
            c.push(m,0x8u32);
            c.push(m, 0x594f7d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6],32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[3].wrapping_add(0x4));
            { let a = c.r[3]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x594f99);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x590960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e5528u32,32));
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x594fa8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x594fb2 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e553cu32,32));
            c.push(m, 0x594fc0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5b7390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x594fca => {
            c.push(m,0x8u32);
            c.push(m, 0x594fd1);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6],32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[3].wrapping_add(0x4));
            { let a = c.r[3]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x594fed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x590960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e5548u32,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x594ff9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5bae40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x595003 => {
            c.push(m,0x8u32);
            c.push(m, 0x59500a);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[3].wrapping_add(0x4));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x595026);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x590960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e55b4u32,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x595032);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5ee180, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x59503c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[7].wrapping_add(0xc314),32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc310),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x3),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e5538u32,32));
            c.push(m, 0x595060);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5b6c40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x59506a => {
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x595072); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(15) { pc = 0x5955cc; continue; }
            pc = 0x59507b; continue;
            }
            0x59507b => {
            c.push(m,0x8u32);
            c.push(m, 0x595082);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[3].wrapping_add(0x4));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x59509e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x590960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e54ccu32,32));
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x5950ad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5950b7 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5950be);
            { let args = []; c.r[0] = host.call(m, 0x59ae40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5950c8 => {
            c.set(0,16,rm(m,0x6dfc14u32,16));
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,0x66ecf6u32,8));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6cc950u32,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            c.push(m, 0x5950f9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x475e40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x595103 => {
            c.set(1,32,rm(m,0x6e5500u32,32));
            c.push(m, 0x59510e);
            { let args = []; c.r[0] = host.call(m, 0x5a8c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e5500u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x59511e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x595128 => {
            c.set(1,32,rm(m,0x6e54c8u32,32));
            c.push(m, 0x595133);
            { let args = []; c.r[0] = host.call(m, 0x59fdb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e54c8u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x595143); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x59514d => {
            c.set(1,32,rm(m,0x6e552cu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x59515b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x595165 => {
            c.set(1,32,rm(m,0x6e550cu32,32));
            c.push(m, 0x595170);
            { let args = []; c.r[0] = host.call(m, 0x5aa7f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e550cu32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x595180); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x59518a => {
            c.set(1,32,rm(m,0x6e5514u32,32));
            c.push(m, 0x595195);
            { let args = []; c.r[0] = host.call(m, 0x5ab340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e5514u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x5951a5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5951af => {
            c.set(1,32,rm(m,0x6e5508u32,32));
            c.push(m, 0x5951ba);
            { let args = []; c.r[0] = host.call(m, 0x5a9be0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e5508u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x5951ca); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5951d4 => {
            c.push(m,0x8u32);
            c.push(m, 0x5951db);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[3].wrapping_add(0x4));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5951f7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x590960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e55b0u32,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x595203);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5ed9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x59520d => {
            c.push(m,0x8u32);
            c.push(m, 0x595214);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[3].wrapping_add(0x4));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x595230);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x590960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e5564u32,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x59523c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5c7dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x595246 => {
            c.push(m,0x8u32);
            c.push(m, 0x59524d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[3].wrapping_add(0x4));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x595269);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x590960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e5588u32,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x595275);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5def10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x59527f => {
            c.push(m,0x8u32);
            c.push(m, 0x595286);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[3].wrapping_add(0x4));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5952a2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x590960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e54fcu32,32));
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x5952b1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5952bb => {
            c.set(0,8,rm(m,0x6dcbd1u32,8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            { let a = 0x6dcbd1u32; wm(m,a,8,c.get(2,8)); }
            if c.cond(5) { pc = 0x5955cc; continue; }
            pc = 0x5952dd; continue;
            }
            0x5952dd => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x134),32); c.push(m, 0x5952ec); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5952f6 => {
            { let a = c.get(2,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x5955cc; continue; }
            pc = 0x5952ff; continue;
            }
            0x5952ff => {
            { let a = c.get(0,32); c.sub(a,0xeu32,32); }
            if c.cond(4) { pc = 0x59544a; continue; }
            pc = 0x595308; continue;
            }
            0x595308 => {
            { let a = c.get(0,32); c.sub(a,0x16u32,32); }
            if c.cond(5) { pc = 0x5955cc; continue; }
            pc = 0x595311; continue;
            }
            0x595311 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x50));
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m, 0x595325);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x5953e8; continue; }
            pc = 0x59532e; continue;
            }
            0x59532e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2],16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[0],8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x39),8));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc324),32));
            c.set(7,32,rm(m,0x6dfc14u32,32));
            c.set(6,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x595383; continue; }
            pc = 0x59536e; continue;
            }
            0x59536e => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m, 0x595379);
            { let args = []; c.r[0] = host.call(m, 0x492430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x595383 => {
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x5955cc; continue; }
            pc = 0x59538b; continue;
            }
            0x59538b => {
            c.set(1,32,rm(m,0x6a1284u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.set(0,8,0xffu32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x3a); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x33); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x31); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x3ecccccdu32); }
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,c.get(3,16)); }
            c.push(m, 0x5953de);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5953e8 => {
            if c.cond(14) { pc = 0x5955cc; continue; }
            pc = 0x5953ee; continue;
            }
            0x5953ee => {
            c.set(0,8,0xffu32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x2f); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6a1284u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb10u32,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x3a); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x31); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x3ecccccdu32); }
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,c.get(3,16)); }
            c.push(m, 0x595440);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x59544a => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.set(2,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x595462);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x5955cc; continue; }
            pc = 0x595470; continue;
            }
            0x595470 => {
            c.set(6,32,rm(m,c.r[7].wrapping_add(0xc324),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            pc = 0x595480; continue;
            }
            0x595480 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x38),8));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x5954a9; continue; }
            pc = 0x595496; continue;
            }
            0x595496 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x595480; continue; }
            pc = 0x59549f; continue;
            }
            0x59549f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x5954a9 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6de4b8),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0x29e4),16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbb0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x5954d6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc324),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6de4b8),8));
            c.set(0,32,c.get(1,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.set(6,32,sx(rm(m,c.r[1].wrapping_add(0x29e6),16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbb4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x595514);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x455b50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x59551e => {
            { let a = rm(m,c.r[4].wrapping_add(0x50),32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(5) { pc = 0x595558; continue; }
            pc = 0x595525; continue;
            }
            0x595525 => {
            c.set(1,8,rm(m,0x6dcbd1u32,8));
            c.set(3,8,c.get(1,8));
            c.set(2,8,c.get(1,8));
            { let a = c.get(3,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(3,8,r); }
            { let a = c.get(3,8); let r = c.and(a,0x7eu32,8); c.set(3,8,r); }
            { let a = c.get(1,8); let r = c.and(a,0xfeu32,8); c.set(1,8,r); }
            { let a = c.get(3,8); let r = c.or(a,c.get(1,8),8); c.set(3,8,r); }
            { let a = 0x6dcbd1u32; wm(m,a,8,c.get(3,8)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x595558; continue; }
            pc = 0x595545; continue;
            }
            0x595545 => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x134),32); c.push(m, 0x595554); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x48),32));
            pc = 0x595558; continue;
            }
            0x595558 => {
            { let a = c.get(0,32); c.sub(a,0x29u32,32); }
            if c.cond(5) { pc = 0x5955cc; continue; }
            pc = 0x59555d; continue;
            }
            0x59555d => {
            { let a = rm(m,c.r[4].wrapping_add(0x50),32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x5955cc; continue; }
            pc = 0x595564; continue;
            }
            0x595564 => {
            c.set(0,32,rm(m,0x6e59ccu32,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6e55d0),32));
            c.push(m, 0x595581);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6016cb, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x1u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5955a1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x590960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5955b2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x58dac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x510),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[5].wrapping_add(0xfffffffc));
            c.push(m,c.get(2,32));
            c.push(m, 0x5955cc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x580bd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5955cc; continue;
            }
            0x5955cc => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn hanse_content_window_draw(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5c1780;
    loop {
        match pc {
            0x5c1780 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12fe),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x5c19d7; continue; }
            pc = 0x5c17c2; continue;
            }
            0x5c17c2 => {
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.push(m, 0x5c17cc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb7c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5c19d7; continue; }
            pc = 0x5c17d7; continue;
            }
            0x5c17d7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5c17e1);
            { let args = []; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x5c17ea);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd8),8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(7) { pc = 0x5c19c8; continue; }
            pc = 0x5c1800; continue;
            }
            0x5c1800 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x5c19e0)); continue;
            }
            0x5c1807 => {
            c.set(2,32,rm(m,0x69c18cu32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5c181b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xe0));
            c.push(m, 0x5c1826);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x420c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5c1832);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5c1a10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5c19c8; continue;
            }
            0x5c1837 => {
            c.set(2,32,rm(m,0x69c190u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5c184b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xe0));
            c.push(m, 0x5c1856);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x420c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5c1862);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5c1b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5c19c8; continue;
            }
            0x5c1867 => {
            c.set(2,32,rm(m,0x69c194u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5c187b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xe0));
            c.push(m, 0x5c1886);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x420c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xd),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5c18a3; continue; }
            pc = 0x5c1892; continue;
            }
            0x5c1892 => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5c189e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5c3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5c19c8; continue;
            }
            0x5c18a3 => {
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5c18af);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5c3640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5c19c8; continue;
            }
            0x5c18b4 => {
            c.set(0,32,rm(m,0x69c890u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5c18c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xe0));
            c.push(m, 0x5c18d2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x420c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5c18de);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5c5370, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5c19c8; continue;
            }
            0x5c18e3 => {
            c.set(2,32,rm(m,0x69c19cu32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5c18f7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xe0));
            c.push(m, 0x5c1902);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x420c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5c190e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5c59d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5c19c8; continue;
            }
            0x5c1913 => {
            c.set(2,32,rm(m,0x6c2eccu32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5c1927);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xe0));
            c.push(m, 0x5c1932);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x420c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5c193e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5c5f00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5c19c8; continue;
            }
            0x5c1943 => {
            c.set(2,32,rm(m,0x69c194u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5c1957);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xe0));
            c.push(m, 0x5c1962);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x420c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5c196e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5c2f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5c19c8; continue;
            }
            0x5c1970 => {
            c.set(2,32,rm(m,0x69c18cu32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5c1984);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xe0));
            c.push(m, 0x5c198f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x420c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5c199b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5c6200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5c19c8; continue;
            }
            0x5c199d => {
            c.set(2,32,rm(m,0x6b2008u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5c19b1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xe0));
            c.push(m, 0x5c19bc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x420c70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5c19c8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5c74c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5c19c8; continue;
            }
            0x5c19c8 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x5c19ce);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x5c19d4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bb9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x5c19d7; continue;
            }
            0x5c19d7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_siege_begin(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x629c60;
    loop {
        match pc {
            0x629c60 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x6684aau32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x30u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x3a),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(4) { pc = 0x629c8f; continue; }
            pc = 0x629c89; continue;
            }
            0x629c89 => {
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x6a),16),16); }
            pc = 0x629c96; continue;
            }
            0x629c8f => {
            { let a = rm(m,c.r[6].wrapping_add(0x21e),16); c.sub(a,c.get(5,16),16); }
            pc = 0x629c96; continue;
            }
            0x629c96 => {
            c.set(1,8,u32::from(c.cond(7)));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x62b28f; continue; }
            pc = 0x629ca1; continue;
            }
            0x629ca1 => {
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(5) { pc = 0x62b28f; continue; }
            pc = 0x629caa; continue;
            }
            0x629caa => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x629cc0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[0].wrapping_add(0x19),8),8); }
            if c.cond(4) { pc = 0x629cea; continue; }
            pc = 0x629cc5; continue;
            }
            0x629cc5 => {
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x6f1),8));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x629d3b; continue; }
            pc = 0x629cea; continue;
            }
            0x629cea => {
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x629cf6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x469380, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6ee118u32);
            c.push(m, 0x629d07);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6ee119u32);
            c.push(m, 0x629d1c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc7e8u32,32));
            c.push(m,0x15u32);
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x629d3b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x469af0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x629d3b; continue;
            }
            0x629d3b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5d8),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x629d4e; continue; }
            pc = 0x629d45; continue;
            }
            0x629d45 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x629d4b);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x629d4e; continue;
            }
            0x629d4e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x59c),32));
            { let a = c.r[6].wrapping_add(0x5d8); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[6].wrapping_add(0x590); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[6].wrapping_add(0x592); wm(m,a,16,c.get(5,16)); }
            if c.cond(4) { pc = 0x629d75; continue; }
            pc = 0x629d6c; continue;
            }
            0x629d6c => {
            c.push(m,c.get(0,32));
            c.push(m, 0x629d72);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x629d75; continue;
            }
            0x629d75 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x250),32));
            { let a = c.r[6].wrapping_add(0x59c); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[6].wrapping_add(0x77); wm(m,a,8,0x0u32); }
            if c.cond(4) { pc = 0x629d92; continue; }
            pc = 0x629d89; continue;
            }
            0x629d89 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x629d8f);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x629d92; continue;
            }
            0x629d92 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x58),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.r[6].wrapping_add(0x250); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x78); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x61c); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x629dc4);
            { let args = []; c.r[0] = host.call(m, 0x5252f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(0,8)); }
            c.set(7,32,c.r[6].wrapping_add(0x5dc));
            c.set(3,32,0x8u32);
            pc = 0x629dd3; continue;
            }
            0x629dd3 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x629df5; continue; }
            pc = 0x629dd9; continue;
            }
            0x629dd9 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x629df3; continue; }
            pc = 0x629de3; continue;
            }
            0x629de3 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x3f8),32));
            { let a = c.get(1,32); let r = c.add(a,0x3f8u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x629df3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x629df3; continue;
            }
            0x629df3 => {
            { let a = c.r[7]; wm(m,a,32,c.get(5,32)); }
            pc = 0x629df5; continue;
            }
            0x629df5 => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x629dd3; continue; }
            pc = 0x629dfb; continue;
            }
            0x629dfb => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x584); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(5,32)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x16),8));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            c.set(7,32,0x67b604u32);
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2d4),32),32); c.set(1,32,r); }
            c.set(0,32,0xcd9a6735u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x21c));
            { let a = c.get(2,32); let n = 0xbu32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(5,32,c.r[1].wrapping_mul(4).wrapping_add(0x67b5c8));
            pc = 0x629e86; continue;
            }
            0x629e86 => {
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x54),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x7fffu32,32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,16,rm(m,c.r[7],16));
            { let a = rm(m,c.r[0].wrapping_add(0x2),16); c.sub(a,c.get(1,16),16); }
            if c.cond(6) { pc = 0x629ea3; continue; }
            pc = 0x629e9f; continue;
            }
            0x629e9f => {
            { let a = c.r[0].wrapping_add(0x2); wm(m,a,16,c.get(1,16)); }
            pc = 0x629ea3; continue;
            }
            0x629ea3 => {
            { let a = c.get(2,8); c.sub(a,0x4u32,8); }
            { let a = c.r[0]; wm(m,a,16,0xffffu32); }
            if c.cond(3) { pc = 0x629ed7; continue; }
            pc = 0x629ead; continue;
            }
            0x629ead => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(3,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(3,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[3].wrapping_mul(8)));
            c.set(3,32,rm(m,0x6de508u32,32));
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[3]).wrapping_add(0x998),8));
            { let a = c.r[0].wrapping_add(0x16); wm(m,a,16,c.get(1,16)); }
            pc = 0x629ed7; continue;
            }
            0x629ed7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(3,32,0xffffu32);
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,16,c.get(3,16)); }
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(2,8); c.sub(a,0x5u32,8); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            if c.cond(2) { pc = 0x629e86; continue; }
            pc = 0x629ef9; continue;
            }
            0x629ef9 => {
            { let a = rm(m,c.r[6].wrapping_add(0x21e),16); c.sub(a,0x3u32,16); }
            c.set(0,32,c.r[6].wrapping_add(0x21e));
            if c.cond(3) { pc = 0x629f0e; continue; }
            pc = 0x629f09; continue;
            }
            0x629f09 => {
            { let a = c.r[0]; wm(m,a,16,0x3u32); }
            pc = 0x629f0e; continue;
            }
            0x629f0e => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(2,32,0x4u32);
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(5,32)); }
            c.set(1,16,c.get(5,16));
            pc = 0x629f1c; continue;
            }
            0x629f1c => {
            { let a = c.get(1,16); let r = c.add(a,rm(m,c.r[0],16),16); c.set(1,16,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x629f1c; continue; }
            pc = 0x629f25; continue;
            }
            0x629f25 => {
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x629f33);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.set(7,32,0x1u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            c.push(m,0xfffffffeu32);
            c.set(1,32,0x6dd730u32);
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x3d); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x53u32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x629f68);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x62a115; continue; }
            pc = 0x629f78; continue;
            }
            0x629f78 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.r[6].wrapping_add(0x79); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x78); wm(m,a,8,0x1u32); }
            { let a = c.r[6]; wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[6].wrapping_add(0x22e); wm(m,a,16,c.get(5,16)); }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x242); wm(m,a,16,c.get(5,16)); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,0x3u32);
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2d4),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,c.get(3,16)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,0x1e00u32);
            { let a = c.r[6].wrapping_add(0x6e); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x66); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[6].wrapping_add(0x42); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[6].wrapping_add(0x44); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[6].wrapping_add(0x6a); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[6].wrapping_add(0x68); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x6d); wm(m,a,8,c.get(2,8)); }
            pc = 0x629fdf; continue;
            }
            0x629fdf => {
            { let a = c.r[6].wrapping_add(0x6f); wm(m,a,8,0xffu32); }
            pc = 0x629fe3; continue;
            }
            0x629fe3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x21e),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x226),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x22e),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x222),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x22a),16));
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x232),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x57c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x23a),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x23e),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x236),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)));
            { let a = c.r[6].wrapping_add(0x580); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x62b1b1; continue; }
            pc = 0x62a08f; continue;
            }
            0x62a08f => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            c.set(5,32,0x708u32);
            pc = 0x62a09d; continue;
            }
            0x62a09d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x250),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7]).wrapping_add(0x8),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a0c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x6),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x62b1a7; continue; }
            pc = 0x62a0d1; continue;
            }
            0x62a0d1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x250),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x62a0f9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x580),32));
            { let a = c.get(1,8); c.sub(a,0x32u32,8); }
            if c.cond(5) { pc = 0x62b19f; continue; }
            pc = 0x62a10b; continue;
            }
            0x62a10b => {
            { let a = c.get(0,32); let r = c.add(a,0x2d0u32,32); c.set(0,32,r); }
            pc = 0x62b1a1; continue;
            }
            0x62a115 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffu32); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(3,8)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x10000u32); }
            { let a = c.r[6].wrapping_add(0x79); wm(m,a,8,c.get(3,8)); }
            c.set(7,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.r[6]; wm(m,a,32,0x10000u32); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x768),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x244); wm(m,a,32,c.get(0,32)); }
            pc = 0x62a167; continue;
            }
            0x62a163 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x58),8));
            pc = 0x62a167; continue;
            }
            0x62a167 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x244),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x62a217; continue; }
            pc = 0x62a18d; continue;
            }
            0x62a18d => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x62a195);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,0x37u32,8); }
            if c.cond(4) { pc = 0x62a1ac; continue; }
            pc = 0x62a19d; continue;
            }
            0x62a19d => {
            { let a = c.get(1,8); c.sub(a,0x38u32,8); }
            if c.cond(4) { pc = 0x62a1ac; continue; }
            pc = 0x62a1a2; continue;
            }
            0x62a1a2 => {
            { let a = c.get(1,8); c.sub(a,0x34u32,8); }
            if c.cond(4) { pc = 0x62a1ac; continue; }
            pc = 0x62a1a7; continue;
            }
            0x62a1a7 => {
            { let a = c.get(1,8); c.sub(a,0x35u32,8); }
            if c.cond(5) { pc = 0x62a1c6; continue; }
            pc = 0x62a1ac; continue;
            }
            0x62a1ac => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,0xffu32);
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x2),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x62a213; continue; }
            pc = 0x62a1c6; continue;
            }
            0x62a1c6 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x62a1ee; continue; }
            pc = 0x62a1ca; continue;
            }
            0x62a1ca => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x62a1ee; continue; }
            pc = 0x62a1d7; continue;
            }
            0x62a1d7 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x762),16));
            c.set(3,8,0x1u32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x244); wm(m,a,32,c.get(0,32)); }
            pc = 0x62a1fb; continue;
            }
            0x62a1ee => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x244); wm(m,a,32,c.get(2,32)); }
            pc = 0x62a1fb; continue;
            }
            0x62a1fb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x244),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x62a18d; continue; }
            pc = 0x62a213; continue;
            }
            0x62a213 => {
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(3,8)); }
            pc = 0x62a217; continue;
            }
            0x62a217 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x244),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x62a414; continue; }
            pc = 0x62a22f; continue;
            }
            0x62a22f => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x62a237);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x34u32,8); }
            if c.cond(4) { pc = 0x62a24c; continue; }
            pc = 0x62a240; continue;
            }
            0x62a240 => {
            { let a = c.get(0,8); c.sub(a,0x35u32,8); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x0u32); }
            if c.cond(5) { pc = 0x62a254; continue; }
            pc = 0x62a24c; continue;
            }
            0x62a24c => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x1u32); }
            pc = 0x62a254; continue;
            }
            0x62a254 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x768),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m,c.get(0,32));
            c.push(m, 0x62a288);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62a36f; continue; }
            pc = 0x62a290; continue;
            }
            0x62a290 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,0x32u32,8); }
            if c.cond(4) { pc = 0x62a2dc; continue; }
            pc = 0x62a298; continue;
            }
            0x62a298 => {
            { let a = c.get(1,8); c.sub(a,0x33u32,8); }
            if c.cond(4) { pc = 0x62a2dc; continue; }
            pc = 0x62a29d; continue;
            }
            0x62a29d => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x776),16),16); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            if c.cond(3) { pc = 0x62a36f; continue; }
            pc = 0x62a2c8; continue;
            }
            0x62a2c8 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x62a2d3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x62a290; continue; }
            pc = 0x62a2d7; continue;
            }
            0x62a2d7 => {
            pc = 0x62a36f; continue;
            }
            0x62a2dc => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[0],8));
            c.set(1,8,rm(m,c.r[3],8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x2u32,32); c.set(2,32,r); }
            c.set(5,32,c.get(2,32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(2,32),32); c.set(5,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x48u32,32); }
            if c.cond(15) { pc = 0x62a32b; continue; }
            pc = 0x62a30a; continue;
            }
            0x62a30a => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,0x32u32,8); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            if c.cond(5) { pc = 0x62a31b; continue; }
            pc = 0x62a316; continue;
            }
            0x62a316 => {
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            pc = 0x62a31e; continue;
            }
            0x62a31b => {
            { let a = c.get(1,32); let r = c.add(a,0x3u32,32); c.set(1,32,r); }
            pc = 0x62a31e; continue;
            }
            0x62a31e => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x62a32b; continue;
            }
            0x62a32b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x776),16),16); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            if c.cond(3) { pc = 0x62a36b; continue; }
            pc = 0x62a352; continue;
            }
            0x62a352 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x62a35d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x62a290; continue; }
            pc = 0x62a369; continue;
            }
            0x62a369 => {
            pc = 0x62a36f; continue;
            }
            0x62a36b => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x62a36f; continue;
            }
            0x62a36f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x62a387; continue; }
            pc = 0x62a37c; continue;
            }
            0x62a37c => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            pc = 0x62a3c0; continue;
            }
            0x62a387 => {
            if c.cond(5) { pc = 0x62a3cf; continue; }
            pc = 0x62a389; continue;
            }
            0x62a389 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2d4),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x62a3cf; continue; }
            pc = 0x62a3c0; continue;
            }
            0x62a3c0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x54),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x6d); wm(m,a,8,c.get(2,8)); }
            pc = 0x62a3cf; continue;
            }
            0x62a3cf => {
            { let a = rm(m,c.r[4].wrapping_add(0x54),8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x62a419; continue; }
            pc = 0x62a3d6; continue;
            }
            0x62a3d6 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x58),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x62a405; continue; }
            pc = 0x62a3de; continue;
            }
            0x62a3de => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x62a405; continue; }
            pc = 0x62a3eb; continue;
            }
            0x62a3eb => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x762),16));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x248); wm(m,a,32,c.get(0,32)); }
            pc = 0x62a419; continue;
            }
            0x62a405 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x4),16));
            { let a = c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0x248); wm(m,a,32,c.get(1,32)); }
            pc = 0x62a419; continue;
            }
            0x62a414 => {
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x2u32); }
            pc = 0x62a419; continue;
            }
            0x62a419 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x54),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x62a163; continue; }
            pc = 0x62a42b; continue;
            }
            0x62a42b => {
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x10),16));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x242); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x62a43e);
            { let args = []; c.r[0] = host.call(m, 0x62b2b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x62a445);
            { let args = []; c.r[0] = host.call(m, 0x632780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x6e); wm(m,a,8,0x0u32); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a467);
            { let args = []; c.r[0] = host.call(m, 0x525220, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0],16); c.sub(a,0x1u32,16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            if c.cond(2) { pc = 0x62a5d2; continue; }
            pc = 0x62a47d; continue;
            }
            0x62a47d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0x7fffffffu32); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x0u32); }
            c.set(3,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[6].wrapping_add(0x244));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x62a4a9; continue;
            }
            0x62a4a9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[2],32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a4ce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3],16));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a4f4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[0],8));
            c.set(1,8,rm(m,c.r[5],8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.get(1,32));
            c.set(0,16,rm(m,c.r[3],16));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a525);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(7,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x62a54f; continue; }
            pc = 0x62a543; continue;
            }
            0x62a543 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x54),8));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(1,8)); }
            pc = 0x62a54f; continue;
            }
            0x62a54f => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x54),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            if c.cond(2) { pc = 0x62a4a9; continue; }
            pc = 0x62a56c; continue;
            }
            0x62a56c => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x50),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6d),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x62a5ba; continue; }
            pc = 0x62a577; continue;
            }
            0x62a577 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,16,rm(m,c.r[3],16));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a59b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,0x5u32,8); }
            if c.cond(4) { pc = 0x62a5ba; continue; }
            pc = 0x62a5a1; continue;
            }
            0x62a5a1 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            c.set(0,16,rm(m,c.r[3],16));
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x258); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x6e); wm(m,a,8,c.get(0,8)); }
            pc = 0x62a5ba; continue;
            }
            0x62a5ba => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[2],16),16); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x62a47d; continue; }
            pc = 0x62a5d0; continue;
            }
            0x62a5d0 => {
            c.set(0,32,c.get(2,32));
            pc = 0x62a5d2; continue;
            }
            0x62a5d2 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x62a5d8);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(6) { pc = 0x62a808; continue; }
            pc = 0x62a5e6; continue;
            }
            0x62a5e6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2d8),32));
            c.set(1,32,0x5u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,8,c.get(3,8));
            { let a = c.get(0,8); let n = 0x2u32; let r = c.shr(a,n,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.add(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(2,8)); }
            if c.cond(6) { pc = 0x62a620; continue; }
            pc = 0x62a61c; continue;
            }
            0x62a61c => {
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(3,8)); }
            pc = 0x62a620; continue;
            }
            0x62a620 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x0u32); }
            if c.cond(6) { pc = 0x62a69a; continue; }
            pc = 0x62a629; continue;
            }
            0x62a629 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            pc = 0x62a633; continue;
            }
            0x62a633 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(5,32,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2d8),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x62a68b; continue; }
            pc = 0x62a66f; continue;
            }
            0x62a66f => {
            c.set(0,8,c.get(3,8));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x6e); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x258),32));
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x258); wm(m,a,32,c.get(2,32)); }
            pc = 0x62a68f; continue;
            }
            0x62a68b => {
            { let a = rm(m,c.r[4].wrapping_add(0x54),8); let r = c.inc(a,8); { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,r); } }
            pc = 0x62a68f; continue;
            }
            0x62a68f => {
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x54),8));
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(2) { pc = 0x62a633; continue; }
            pc = 0x62a69a; continue;
            }
            0x62a69a => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            c.set(1,32,c.get(0,32));
            c.set(3,32,0x7fffffffu32);
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x62a808; continue; }
            pc = 0x62a6b9; continue;
            }
            0x62a6b9 => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            pc = 0x62a6c1; continue;
            }
            0x62a6c1 => {
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0x80000000u32); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(2,8)); }
            if c.cond(3) { pc = 0x62a79a; continue; }
            pc = 0x62a6d5; continue;
            }
            0x62a6d5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x258),32));
            c.set(5,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x258));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a70c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let r = c.add(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a73c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(5) { pc = 0x62a757; continue; }
            pc = 0x62a751; continue;
            }
            0x62a751 => {
            c.set(1,32,c.get(0,32));
            c.set(2,32,c.get(7,32));
            pc = 0x62a75d; continue;
            }
            0x62a757 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,c.get(3,32));
            pc = 0x62a75d; continue;
            }
            0x62a75d => {
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x62a781; continue; }
            pc = 0x62a76d; continue;
            }
            0x62a76d => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x54),8));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x62a781; continue;
            }
            0x62a781 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x54),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x62a6d5; continue; }
            pc = 0x62a796; continue;
            }
            0x62a796 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            pc = 0x62a79a; continue;
            }
            0x62a79a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x50),8));
            { let a = c.get(0,8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x62a7e0; continue; }
            pc = 0x62a7ae; continue;
            }
            0x62a7ae => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x258),32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x258),16));
            c.set(1,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x258));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x258); wm(m,a,32,c.get(0,32)); }
            pc = 0x62a7e0; continue;
            }
            0x62a7e0 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            c.set(7,32,c.get(0,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x62a6c1; continue; }
            pc = 0x62a808; continue;
            }
            0x62a808 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x62acbf; continue; }
            pc = 0x62a813; continue;
            }
            0x62a813 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x258),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a838);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x258),32));
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(7,32,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let r = c.add(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a865);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,32,c.get(1,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x62a893);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62a8a1);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x258),32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a8cc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x258),32));
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(7,32,0x7fu32);
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a8fe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,32,c.get(1,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x62a92c);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62a93a);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x62a944; continue; }
            pc = 0x62a93e; continue;
            }
            0x62a93e => {
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            pc = 0x62a944; continue;
            }
            0x62a944 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x258),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a969);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x258),32));
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(7,32,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let r = c.add(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62a996);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,32,0xe1u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x62a9c9);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62a9d7);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x62a9e1; continue; }
            pc = 0x62a9db; continue;
            }
            0x62a9db => {
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            pc = 0x62a9e1; continue;
            }
            0x62a9e1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x258),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62aa06);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x258),32));
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(7,32,0x7fu32);
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62aa38);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,32,0xe1u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x62aa6b);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62aa79);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x62aa83; continue; }
            pc = 0x62aa7d; continue;
            }
            0x62aa7d => {
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            pc = 0x62aa83; continue;
            }
            0x62aa83 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x250),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x62aaad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x250),32));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let r = c.add(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62aae6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x62ab1a);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62ab28);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x28));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x258),32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62ab51);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0],8));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x258),32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x62ab7e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x1u32); }
            if c.cond(6) { pc = 0x62adfd; continue; }
            pc = 0x62ab98; continue;
            }
            0x62ab98 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x258),32));
            c.set(5,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x258));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62abcf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(1,8,rm(m,c.r[0],8));
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(7,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(7,32); let r = c.add(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62ac05);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(7,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x62ac3f);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62ac4d);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(3,32,c.r[3].wrapping_add(c.r[0]).wrapping_add(0xa));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62ac73);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0],8));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x62ac9c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x54),8));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(2) { pc = 0x62ab98; continue; }
            pc = 0x62acba; continue;
            }
            0x62acba => {
            pc = 0x62adfd; continue;
            }
            0x62acbf => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(2,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x62acf4);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            c.push(m, 0x62acff);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(3,32,c.get(0,32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(0,32,0x7fu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x60));
            c.fstp64(m,c.r[4].wrapping_add(0x30));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x62ad3d);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.push(m, 0x62ad4b);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x62ad51; continue; }
            pc = 0x62ad4f; continue;
            }
            0x62ad4f => {
            c.set(3,32,c.get(0,32));
            pc = 0x62ad51; continue;
            }
            0x62ad51 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(2,32,c.get(1,32));
            c.set(1,32,0xe1u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x62ad8f);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62ad9d);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x62ada3; continue; }
            pc = 0x62ada1; continue;
            }
            0x62ada1 => {
            c.set(3,32,c.get(0,32));
            pc = 0x62ada3; continue;
            }
            0x62ada3 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x7fu32);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,0xe1u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x62ade6);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62adf4);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x62adfa; continue; }
            pc = 0x62adf8; continue;
            }
            0x62adf8 => {
            c.set(3,32,c.get(0,32));
            pc = 0x62adfa; continue;
            }
            0x62adfa => {
            { let a = c.get(3,32); let r = c.add(a,0x1eu32,32); c.set(3,32,r); }
            pc = 0x62adfd; continue;
            }
            0x62adfd => {
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); c.sub(a,0x900u32,16); }
            { let a = c.r[6].wrapping_add(0x66); wm(m,a,16,c.get(0,16)); }
            if c.cond(3) { pc = 0x62ae17; continue; }
            pc = 0x62ae11; continue;
            }
            0x62ae11 => {
            { let a = c.r[6].wrapping_add(0x66); wm(m,a,16,0x900u32); }
            pc = 0x62ae17; continue;
            }
            0x62ae17 => {
            c.set(5,32,0x1300u32);
            { let a = rm(m,c.r[6].wrapping_add(0x66),16); c.sub(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x62ae26; continue; }
            pc = 0x62ae22; continue;
            }
            0x62ae22 => {
            { let a = c.r[6].wrapping_add(0x66); wm(m,a,16,c.get(5,16)); }
            pc = 0x62ae26; continue;
            }
            0x62ae26 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x66),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.r[6].wrapping_add(0x42); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x62ae63);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            c.push(m, 0x62ae6e);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(0,32,0x7fu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x60));
            c.fstp64(m,c.r[4].wrapping_add(0x30));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x62aeac);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.push(m, 0x62aeba);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x62aec0; continue; }
            pc = 0x62aebe; continue;
            }
            0x62aebe => {
            c.set(7,32,c.get(0,32));
            pc = 0x62aec0; continue;
            }
            0x62aec0 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(2,32,c.get(1,32));
            c.set(1,32,0xe1u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x62aefe);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62af0c);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x62af12; continue; }
            pc = 0x62af10; continue;
            }
            0x62af10 => {
            c.set(7,32,c.get(0,32));
            pc = 0x62af12; continue;
            }
            0x62af12 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x7fu32);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,0xe1u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x62af55);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62af63);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x62af69; continue; }
            pc = 0x62af67; continue;
            }
            0x62af67 => {
            c.set(7,32,c.get(0,32));
            pc = 0x62af69; continue;
            }
            0x62af69 => {
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,16); c.sub(a,0x900u32,16); }
            { let a = c.r[6].wrapping_add(0x6a); wm(m,a,16,c.get(0,16)); }
            if c.cond(3) { pc = 0x62af83; continue; }
            pc = 0x62af7d; continue;
            }
            0x62af7d => {
            { let a = c.r[6].wrapping_add(0x6a); wm(m,a,16,0x900u32); }
            pc = 0x62af83; continue;
            }
            0x62af83 => {
            { let a = rm(m,c.r[6].wrapping_add(0x6a),16); c.sub(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x62af8d; continue; }
            pc = 0x62af89; continue;
            }
            0x62af89 => {
            { let a = c.r[6].wrapping_add(0x6a); wm(m,a,16,c.get(5,16)); }
            pc = 0x62af8d; continue;
            }
            0x62af8d => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x20),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x9u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x68); wm(m,a,16,c.get(1,16)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x66),16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            { let a = c.get(3,32); let r = c.sub(a,0x1eu32,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0x44); wm(m,a,16,c.get(1,16)); }
            if c.cond(6) { pc = 0x629fdf; continue; }
            pc = 0x62afc8; continue;
            }
            0x62afc8 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x258),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62b006);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0],8));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x258),32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x62b033);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x0u32); }
            if c.cond(6) { pc = 0x62b196; continue; }
            pc = 0x62b04d; continue;
            }
            0x62b04d => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x258),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62b07d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(1,8,rm(m,c.r[0],8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x258),32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(5,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(5,32); let r = c.sub(a,c.get(2,32),32); c.set(5,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(5,32); let r = c.add(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62b0b7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(5,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x58));
            c.fstp64(m,c.r[4].wrapping_add(0x28));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x62b0f1);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.ld(m,0x66a7d0u32,8); c.fop('+',v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62b0ff);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.get(5,32));
            { let a = c.get(1,32); let r = c.add(a,0xau32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x258),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x254); wm(m,a,16,c.get(5,16)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x62b14b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0],8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x258),32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x62b178);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x54),8));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x6e),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x62b04d; continue; }
            pc = 0x62b196; continue;
            }
            0x62b196 => {
            { let a = c.r[6].wrapping_add(0x6f); wm(m,a,8,0x0u32); }
            pc = 0x629fe3; continue;
            }
            0x62b19f => {
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            pc = 0x62b1a1; continue;
            }
            0x62b1a1 => {
            { let a = c.r[6].wrapping_add(0x580); wm(m,a,32,c.get(0,32)); }
            pc = 0x62b1a7; continue;
            }
            0x62b1a7 => {
            { let a = c.get(7,32); let r = c.add(a,0xcu32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x62a09d; continue; }
            pc = 0x62b1b1; continue;
            }
            0x62b1b1 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x62b1b8);
            { let args = []; c.r[0] = host.call(m, 0x62e810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x226),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x222),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x22a),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x21e),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,0x2aaaaaabu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,8); c.sub(a,0x1u32,8); }
            { let a = c.r[6].wrapping_add(0x5d4); wm(m,a,8,c.get(2,8)); }
            if c.cond(3) { pc = 0x62b204; continue; }
            pc = 0x62b1fb; continue;
            }
            0x62b1fb => {
            { let a = c.r[6].wrapping_add(0x5d4); wm(m,a,8,0x1u32); }
            pc = 0x62b210; continue;
            }
            0x62b204 => {
            { let a = c.get(2,8); c.sub(a,0x6u32,8); }
            if c.cond(6) { pc = 0x62b210; continue; }
            pc = 0x62b209; continue;
            }
            0x62b209 => {
            { let a = c.r[6].wrapping_add(0x5d4); wm(m,a,8,0x6u32); }
            pc = 0x62b210; continue;
            }
            0x62b210 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8));
            { let a = c.get(7,32); let r = c.or(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5d3),8));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc324),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x62b261; continue; }
            pc = 0x62b251; continue;
            }
            0x62b251 => {
            c.push(m, 0x62b256);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x62b261; continue; }
            pc = 0x62b25a; continue;
            }
            0x62b25a => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x62b261);
            { let args = []; c.r[0] = host.call(m, 0x62c730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x62b261; continue;
            }
            0x62b261 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[6].wrapping_add(0x578); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x620); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x624); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x62c); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x62b28f);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x62b28f; continue;
            }
            0x62b28f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "multiplayer_screen_start_game_tail" => multiplayer_screen_start_game_tail(m, host, regs),
        "multiplayer_screen_update_hotseat_setup_tail_enter_town" => multiplayer_screen_update_hotseat_setup_tail_enter_town(m, host, regs),
        "nav_map_on_mouse_event" => nav_map_on_mouse_event(m, host, regs),
        "std_string_copy_alloc_tail" => std_string_copy_alloc_tail(m, host, regs),
        "event_window_show_next_queued_event" => event_window_show_next_queued_event(m, host, regs),
        "route_save_to_file_failure_tail" => route_save_to_file_failure_tail(m, host, regs),
        "st_check_game_end" => st_check_game_end(m, host, regs),
        "st_alderman_mission" => st_alderman_mission(m, host, regs),
        "scheduled_tasks_alderman_mission_completed" => scheduled_tasks_alderman_mission_completed(m, host, regs),
        "class22_on_click_map_object" => class22_on_click_map_object(m, host, regs),
        "hanse_content_window_draw" => hanse_content_window_draw(m, host, regs),
        "town_siege_begin" => town_siege_begin(m, host, regs),
        _ => return None,
    })
}
