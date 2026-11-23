//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn town_get_player_supply_contract_record(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5268d0;
    loop {
        match pc {
            0x5268d0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(7,32,c.r[5].wrapping_add(0x830));
            pc = 0x5268e0; continue;
            }
            0x5268e0 => {
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd748u32,16),16); }
            if c.cond(3) { pc = 0x52690a; continue; }
            pc = 0x5268ec; continue;
            }
            0x5268ec => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5268fc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x16),8));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x52695f; continue; }
            pc = 0x52690a; continue;
            }
            0x52690a => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); c.sub(a,0x2u32,32); }
            if c.cond(12) { pc = 0x5268e0; continue; }
            pc = 0x526913; continue;
            }
            0x526913 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x2c0),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            { let a = c.r[0].wrapping_add(0x1); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[0].wrapping_add(0x2); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[0].wrapping_add(0x3); wm(m,a,8,c.get(3,8)); }
            c.set(2,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,c.get(3,16)); }
            { let a = c.get(2,32); let r = c.add(a,0x16d00u32,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0xa); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,16,c.get(3,16)); }
            c.set(1,8,rm(m,0x6dfc14u32,8));
            { let a = c.r[0].wrapping_add(0xe); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[0].wrapping_add(0xf); wm(m,a,8,c.get(3,8)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[2]; wm(m,a,16,0xffffu32); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x52695f => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x10u32);
            c.set(2,16,rm(m,c.r[5].wrapping_add(c.r[6].wrapping_mul(2)).wrapping_add(0x830),16));
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x526976);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x526984);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,rm(m,c.r[5].wrapping_add(c.r[6].wrapping_mul(2)).wrapping_add(0x830),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[2]; wm(m,a,16,c.get(1,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_player_has_supply_contract(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x526880;
    loop {
        match pc {
            0x526880 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(6,32,c.r[1].wrapping_add(0x830));
            pc = 0x52688a; continue;
            }
            0x52688a => {
            c.set(0,16,rm(m,c.r[6],16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd748u32,16),16); }
            if c.cond(3) { pc = 0x5268b7; continue; }
            pc = 0x526896; continue;
            }
            0x526896 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5268a6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5268c5; continue; }
            pc = 0x5268b7; continue;
            }
            0x5268b7 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,0x2u32,32); }
            if c.cond(12) { pc = 0x52688a; continue; }
            pc = 0x5268c0; continue;
            }
            0x5268c0 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5268c5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_tavern_ware_offer_available(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5265e0;
    loop {
        match pc {
            0x5265e0 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x83e),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(0,8));
            c.push(m,c.get(3,32));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x526606; continue; }
            pc = 0x5265f7; continue;
            }
            0x5265f7 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x52663a; continue; }
            pc = 0x526606; continue;
            }
            0x526606 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(0x4));
            c.set(6,32,0x1u32);
            { let a = c.get(1,32); let r = c.add(a,0x4f4u32,32); c.set(1,32,r); }
            pc = 0x526616; continue;
            }
            0x526616 => {
            { let a = c.get(6,32); c.and(a,0x47fafu32,32); }
            if c.cond(4) { pc = 0x52662c; continue; }
            pc = 0x52661e; continue;
            }
            0x52661e => {
            c.set(7,32,rm(m,c.r[1],32));
            c.set(3,32,rm(m,c.r[2],32));
            { let a = c.get(7,32); let r = c.add(a,0x9c40u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            if c.cond(15) { pc = 0x526640; continue; }
            pc = 0x52662c; continue;
            }
            0x52662c => {
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x526616; continue; }
            pc = 0x52663a; continue;
            }
            0x52663a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x526640 => {
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

pub fn merchant_get_hireable_sailors(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f6ca0;
    loop {
        match pc {
            0x4f6ca0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2e4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(13) { pc = 0x4f6cc6; continue; }
            pc = 0x4f6cc1; continue;
            }
            0x4f6cc1 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4f6cc6 => {
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xf0),8));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4f6cdb; continue; }
            pc = 0x4f6cd9; continue;
            }
            0x4f6cd9 => {
            c.set(0,8,c.get(2,8));
            pc = 0x4f6cdb; continue;
            }
            0x4f6cdb => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tavern_content_window_update_news_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5d5970;
    loop {
        match pc {
            0x5d5970 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x666e38u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x3cu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x82c),8); c.sub(a,0x14u32,8); }
            if c.cond(2) { pc = 0x5d59e4; continue; }
            pc = 0x5d59aa; continue;
            }
            0x5d59aa => {
            c.set(1,32,rm(m,0x6e54f8u32,32));
            c.push(m, 0x5d59b5);
            { let args = []; c.r[0] = host.call(m, 0x5a6e20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m, 0x5d59c0);
            { let args = []; c.r[0] = host.call(m, 0x466320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5d59c9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5ced00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xd4),32); c.push(m, 0x5d59d3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5d59e4 => {
            c.set(3,32,c.r[6].wrapping_add(0x17c));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5d59f1);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5d5c40; continue; }
            pc = 0x5d59f9; continue;
            }
            0x5d59f9 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5d5a0a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2160),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[6].wrapping_add(0x1ca7); wm(m,a,8,0x0u32); }
            if c.cond(12) { pc = 0x5d5bcb; continue; }
            pc = 0x5d5a21; continue;
            }
            0x5d5a21 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m, 0x5d5a2c);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2154),32));
            c.set(0,32,c.r[6].wrapping_add(0x2158));
            c.push(m,0x10u32);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5d5a49);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5d5a5b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1ca7); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x216c),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x6c6748u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,0x0u32); }
            c.push(m, 0x5d5a89);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x8cu32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x1608));
            c.push(m, 0x5d5aa7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x462520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x17dc),32));
            c.set(7,32,c.r[6].wrapping_add(0x17dc));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.push(m,0x64u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xffffffe2));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xffffffbf));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x5d5ad5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x5d5ade); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c7c),32));
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x5d5aef); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x5c),32); c.push(m, 0x5d5af7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,c.r[6].wrapping_add(0x17c));
            c.set(1,32,c.r[0].wrapping_add(0x46));
            c.set(0,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffd3));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffe2));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x5d5b2c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1c7c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x5d5b43); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x5d5b6d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c7c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1610),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5d5b8c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1610),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x17d4));
            c.push(m, 0x5d5b9e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1610),32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,8,u32::from(c.cond(7)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5d5bb8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5d5bc9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x5d5bcb; continue;
            }
            0x5d5bcb => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x5d5bd5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9a4),32));
            c.set(1,32,c.r[6].wrapping_add(0x9a4));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x1bf4); wm(m,a,32,0xcu32); }
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5d5bf3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            c.set(1,32,c.r[6].wrapping_add(0xa8c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5d5c07); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5d5c13); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54f8u32,32));
            c.push(m, 0x5d5c1e);
            { let args = []; c.r[0] = host.call(m, 0x5a6de0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2154));
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(0,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x5d5c40);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x526b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5d5c40; continue;
            }
            0x5d5c40 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_tavern_ship_task_available_to_player(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5261a0;
    loop {
        match pc {
            0x5261a0 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x83f),8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(2) { pc = 0x5261ad; continue; }
            pc = 0x5261aa; continue;
            }
            0x5261aa => {
            c.set(0,8,0x1u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5261ad => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.set(1,8,u32::from(c.cond(4)));
            c.set(0,8,c.get(1,8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_tavern_weapons_dealer_available_to_player(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x526280;
    loop {
        match pc {
            0x526280 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x2c8),8));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(5) { pc = 0x5262de; continue; }
            pc = 0x526291; continue;
            }
            0x526291 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x2c0),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x874),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x5262de; continue; }
            pc = 0x5262b1; continue;
            }
            0x5262b1 => {
            c.set(0,32,0x14u32);
            c.set(2,32,c.r[1].wrapping_add(0x54));
            pc = 0x5262b9; continue;
            }
            0x5262b9 => {
            { let a = rm(m,c.r[2],32); c.sub(a,0xau32,32); }
            if c.cond(13) { pc = 0x5262c9; continue; }
            pc = 0x5262be; continue;
            }
            0x5262be => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x5262b9; continue; }
            pc = 0x5262c7; continue;
            }
            0x5262c7 => {
            pc = 0x5262ce; continue;
            }
            0x5262c9 => {
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x5262de; continue; }
            pc = 0x5262ce; continue;
            }
            0x5262ce => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x83d),8));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5262de; continue; }
            pc = 0x5262da; continue;
            }
            0x5262da => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5262de => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x83d),8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(2) { pc = 0x5262ec; continue; }
            pc = 0x5262e8; continue;
            }
            0x5262e8 => {
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5262ec => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            c.set(1,8,u32::from(c.cond(4)));
            c.set(0,8,c.get(1,8));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_tavern_thief_available_to_player(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x526120;
    loop {
        match pc {
            0x526120 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x83c),8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(2) { pc = 0x526151; continue; }
            pc = 0x52612a; continue;
            }
            0x52612a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x784),16));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x52613e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2ca),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4a8u32,16),16); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x526151 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            c.set(0,8,c.get(2,8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_find_tavern_pirate_captain_for_player(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5261d0;
    loop {
        match pc {
            0x5261d0 => {
            c.set(0,16,rm(m,0x6dd892u32,16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(6,16,rm(m,c.r[3].wrapping_add(0x82e),16));
            c.set(1,16,c.get(0,16));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x526227; continue; }
            pc = 0x5261ed; continue;
            }
            0x5261ed => {
            c.set(2,32,rm(m,0x6dd7a0u32,32));
            c.set(7,32,c.get(6,32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x526201);
            { let args = []; c.r[0] = host.call(m, 0x4fe150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x526213; continue; }
            pc = 0x526205; continue;
            }
            0x526205 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xf),8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x526272; continue; }
            pc = 0x526213; continue;
            }
            0x526213 => {
            c.set(0,16,rm(m,0x6dd892u32,16));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[7],16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,c.get(0,16));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5261ed; continue; }
            pc = 0x526227; continue;
            }
            0x526227 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(6,16,rm(m,c.r[3].wrapping_add(0x82e),16));
            c.set(2,16,c.get(0,16));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x526269; continue; }
            pc = 0x526239; continue;
            }
            0x526239 => {
            c.set(3,8,0xffu32);
            pc = 0x52623b; continue;
            }
            0x52623b => {
            c.set(2,32,rm(m,0x6dd7a0u32,32));
            c.set(7,32,c.get(6,32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x52624f);
            { let args = []; c.r[0] = host.call(m, 0x4fe150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x526258; continue; }
            pc = 0x526253; continue;
            }
            0x526253 => {
            { let a = rm(m,c.r[7].wrapping_add(0xf),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x526272; continue; }
            pc = 0x526258; continue;
            }
            0x526258 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(6,16,rm(m,c.r[7],16));
            c.set(0,16,rm(m,0x6dd892u32,16));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x52623b; continue; }
            pc = 0x526269; continue;
            }
            0x526269 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,0xffffu32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x526272 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn church_get_extension_stage(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4fea97;
    loop {
        match pc {
            0x4fea97 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(1,8,rm(m,c.r[3].wrapping_add(0xf),8));
            c.push(m,c.get(7,32));
            { let a = c.get(1,8); c.sub(a,0x3u32,8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            if c.cond(2) { pc = 0x4feab7; continue; }
            pc = 0x4feaaa; continue;
            }
            0x4feaaa => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0x5u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4feab7 => {
            { let a = rm(m,c.r[3].wrapping_add(0x8),32); c.sub(a,0x64u32,32); }
            if c.cond(7) { pc = 0x4feaca; continue; }
            pc = 0x4feabd; continue;
            }
            0x4feabd => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0x4u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4feaca => {
            c.set(0,32,rm(m,0x6de52cu32,32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.sub(a,0x0u32,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x4feb0b; continue; }
            pc = 0x4feae3; continue;
            }
            0x4feae3 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x4feaf7; continue; }
            pc = 0x4feae6; continue;
            }
            0x4feae6 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x4feb1a; continue;
            }
            0x4feaf7 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x4feb1a; continue;
            }
            0x4feb0b => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x4feb1a; continue;
            }
            0x4feb1a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_mul(2).wrapping_add(0x6734b4),16));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x4feb3a; continue; }
            pc = 0x4feb2d; continue;
            }
            0x4feb2d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0x3u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4feb3a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x762),16));
            c.set(5,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x776),16));
            c.set(7,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x4feb89; continue; }
            pc = 0x4feb64; continue;
            }
            0x4feb64 => {
            c.set(3,8,0x1fu32);
            pc = 0x4feb66; continue;
            }
            0x4feb66 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4feb73);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4feabd; continue; }
            pc = 0x4feb7c; continue;
            }
            0x4feb7c => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x4feb66; continue; }
            pc = 0x4feb85; continue;
            }
            0x4feb85 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4feb89; continue;
            }
            0x4feb89 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn loan_compute_interest_and_default(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51d270;
    loop {
        match pc {
            0x51d270 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('*',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fld(c.ld(m,0x66bc80u32,4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x18),4); c.fop('/',v); }
            c.fstp64(m,c.r[4]);
            c.push(m, 0x51d2bc);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            c.fstp32(m,c.r[4].wrapping_add(0x18));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x18),4));
            { let v = c.ld(m,0x6736c8u32,4); c.fop('*',v); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xf),8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xfu32,32); c.set(2,32,r); }
            { let v = c.ld(m,0x672dbcu32,4); c.fop('+',v); }
            { let a = c.get(2,32); let r = c.add(a,0x8u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fild32(m,c.r[4].wrapping_add(0x8));
            c.fstp32(m,c.r[4].wrapping_add(0x8));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x8),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x10),4); c.fop('*',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            c.push(m, 0x51d301);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,8,c.get(0,8)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xa),8));
            c.fstp32(m,c.r[4].wrapping_add(0x8));
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x10),4));
            { let v = c.ld(m,0x67355cu32,4); c.fop('*',v); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let v = c.ld(m,0x66bc80u32,4); c.fop('+',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            if c.cond(14) { pc = 0x51d35b; continue; }
            pc = 0x51d34c; continue;
            }
            0x51d34c => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x4),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x10),4); c.fop('*',v); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.fstp32(m,c.r[4].wrapping_add(0x4));
            if c.cond(5) { pc = 0x51d34c; continue; }
            pc = 0x51d35b; continue;
            }
            0x51d35b => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x4),4));
            { let v = c.ld(m,0x6736bcu32,4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(0,32); let r = c.and(a,0x4100u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x51d38f; continue; }
            pc = 0x51d36e; continue;
            }
            0x51d36e => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x4),4));
            { let v = c.ld(m,c.r[4].wrapping_add(0x8),4); c.fcomp(v); }
            c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();
            { let a = c.get(4,8); c.and(a,0x5u32,8); }
            if c.cond(11) { pc = 0x51d38f; continue; }
            pc = 0x51d37d; continue;
            }
            0x51d37d => {
            c.fld(c.ld(m,c.r[4].wrapping_add(0x4),4));
            c.push(m, 0x51d386);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x51d395; continue;
            }
            0x51d38f => {
            { let a = c.r[6]; wm(m,a,32,0xfde8u32); }
            pc = 0x51d395; continue;
            }
            0x51d395 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            { let a = c.r[6].wrapping_add(0xb); wm(m,a,8,0x0u32); }
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x51d403; continue; }
            pc = 0x51d3a0; continue;
            }
            0x51d3a0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6],32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x51d403; continue; }
            pc = 0x51d3bb; continue;
            }
            0x51d3bb => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0xd1b71759u32);
            { let a = c.get(1,32); let r = c.add(a,0x4e2u32,32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(7,32,c.get(2,32));
            { let a = c.get(7,32); let n = 0xau32; let r = c.shr(a,n,32); c.set(7,32,r); }
            c.push(m, 0x51d3d5);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.and(a,0x3ffu32,32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x51d403; continue; }
            pc = 0x51d3eb; continue;
            }
            0x51d3eb => {
            c.push(m, 0x51d3f0);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x4eu32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,8); let r = c.add(a,0x33u32,8); c.set(2,8,r); }
            { let a = c.r[6].wrapping_add(0xb); wm(m,a,8,c.get(2,8)); }
            pc = 0x51d403; continue;
            }
            0x51d403 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn map_discovery_construct(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x534750;
    loop {
        match pc {
            0x534750 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0x30u32);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x60); wm(m,a,32,0x0u32); }
            c.push(m, 0x534768);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x30u32);
            c.set(0,32,c.r[6].wrapping_add(0x30));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x534775);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6df2e0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5347e8; continue; }
            pc = 0x534781; continue;
            }
            0x534781 => {
            c.set(0,32,rm(m,0x6dccc8u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x6c363cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53479b);
            { let args = []; c.r[0] = host.call(m, 0x6389ee, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5347e8; continue; }
            pc = 0x5347a4; continue;
            }
            0x5347a4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let a = 0x6df2e4u32; wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x5347c3);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = 0x6df2e0u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5347d8);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5347e5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x6389f4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x5347e8; continue;
            }
            0x5347e8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn map_discovery_merge(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x534920;
    loop {
        match pc {
            0x534920 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x60),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(6,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x60),32));
            { let a = c.get(2,32); let r = c.or(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[1].wrapping_add(0x30));
            { let a = c.r[1].wrapping_add(0x60); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            pc = 0x53493b; continue;
            }
            0x53493b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xffffffd0),32));
            { let a = c.get(3,32); let r = c.or(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.r[0].wrapping_add(0xffffffd0); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(3,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.or(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(2,32); c.sub(a,0xcu32,32); }
            if c.cond(2) { pc = 0x53493b; continue; }
            pc = 0x534959; continue;
            }
            0x534959 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_find_mayor_election(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d9f10;
    loop {
        match pc {
            0x4d9f10 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.set(6,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x4d9f4a; continue; }
            pc = 0x4d9f23; continue;
            }
            0x4d9f23 => {
            c.set(7,32,rm(m,c.r[1],32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4d9f29; continue;
            }
            0x4d9f29 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x6),16); c.sub(a,0x2u32,16); }
            c.set(1,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)));
            if c.cond(5) { pc = 0x4d9f40; continue; }
            pc = 0x4d9f37; continue;
            }
            0x4d9f37 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(0xe),8));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4d9f52; continue; }
            pc = 0x4d9f40; continue;
            }
            0x4d9f40 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x4d9f29; continue; }
            pc = 0x4d9f4a; continue;
            }
            0x4d9f4a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4d9f52 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "town_get_player_supply_contract_record" => town_get_player_supply_contract_record(m, host, regs),
        "town_player_has_supply_contract" => town_player_has_supply_contract(m, host, regs),
        "town_tavern_ware_offer_available" => town_tavern_ware_offer_available(m, host, regs),
        "merchant_get_hireable_sailors" => merchant_get_hireable_sailors(m, host, regs),
        "tavern_content_window_update_news_page" => tavern_content_window_update_news_page(m, host, regs),
        "town_tavern_ship_task_available_to_player" => town_tavern_ship_task_available_to_player(m, host, regs),
        "town_tavern_weapons_dealer_available_to_player" => town_tavern_weapons_dealer_available_to_player(m, host, regs),
        "town_tavern_thief_available_to_player" => town_tavern_thief_available_to_player(m, host, regs),
        "town_find_tavern_pirate_captain_for_player" => town_find_tavern_pirate_captain_for_player(m, host, regs),
        "church_get_extension_stage" => church_get_extension_stage(m, host, regs),
        "loan_compute_interest_and_default" => loan_compute_interest_and_default(m, host, regs),
        "map_discovery_construct" => map_discovery_construct(m, host, regs),
        "map_discovery_merge" => map_discovery_merge(m, host, regs),
        "scheduled_tasks_find_mayor_election" => scheduled_tasks_find_mayor_election(m, host, regs),
        _ => return None,
    })
}
