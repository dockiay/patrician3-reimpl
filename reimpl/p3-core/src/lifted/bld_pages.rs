//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn tavern_content_window_open_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5ced00;
    loop {
        match pc {
            0x5ced00 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x666c8cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x2cu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(7,32,c.r[6].wrapping_add(0x17c));
            { let a = c.r[6].wrapping_add(0x1bf4); wm(m,a,32,c.get(1,32)); }
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ced4f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10e4),32));
            c.set(3,32,c.r[6].wrapping_add(0x10e4));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5ced65); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x11cc),32));
            c.set(5,32,c.r[6].wrapping_add(0x11cc));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ced7b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[4].wrapping_add(0x4c),32); c.sub(a,0x9u32,32); }
            if c.cond(5) { pc = 0x5ced9a; continue; }
            pc = 0x5ced82; continue;
            }
            0x5ced82 => {
            c.set(1,32,rm(m,0x6e54f8u32,32));
            c.push(m, 0x5ced8d);
            { let args = []; c.r[0] = host.call(m, 0x5a74b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd736u32,16),16); }
            if c.cond(3) { pc = 0x5ced9a; continue; }
            pc = 0x5ced96; continue;
            }
            0x5ced96 => {
            c.set(0,8,0x1u32);
            pc = 0x5ced9c; continue;
            }
            0x5ced9a => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x5ced9c; continue;
            }
            0x5ced9c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2188),32));
            c.set(1,32,c.r[6].wrapping_add(0x2188));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cedaf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dd894u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,c.r[6].wrapping_add(0x94));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,0x1u32);
            { let a = c.r[6].wrapping_add(0x1c30); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cedd3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x69db2c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cede6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5cedef); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5cedf8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5cee02); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb74),32));
            c.set(1,32,c.r[6].wrapping_add(0xb74));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cee16); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc5c),32));
            c.set(1,32,c.r[6].wrapping_add(0xc5c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cee2a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1448),32));
            c.set(1,32,c.r[6].wrapping_add(0x1448));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cee3e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x12b8),32));
            c.set(1,32,c.r[6].wrapping_add(0x12b8));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cee52); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9a4),32));
            c.set(5,32,c.r[6].wrapping_add(0x9a4));
            { let a = c.get(1,32); let r = c.add(a,0x17cu32,32); c.set(1,32,r); }
            c.push(m,0x64u32);
            c.push(m,c.get(1,32));
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5cee7d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.get(3,32));
            c.set(1,32,c.get(5,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.sub(a,0x6eu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5cee8e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            c.set(1,32,c.r[6].wrapping_add(0xa8c));
            { let a = c.get(0,32); let r = c.add(a,0x17cu32,32); c.set(0,32,r); }
            c.push(m,0x64u32);
            { let a = c.get(3,32); let r = c.add(a,0x6eu32,32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x5ceeac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ceeb9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            c.set(1,32,c.r[6].wrapping_add(0xa8c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5ceecd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xd44),32));
            c.set(1,32,c.r[6].wrapping_add(0xd44));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x1ca6); wm(m,a,8,0x0u32); }
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ceee8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe2c),32));
            c.set(1,32,c.r[6].wrapping_add(0xe2c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5ceefc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xf14),32));
            c.set(1,32,c.r[6].wrapping_add(0xf14));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cef10); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xffc),32));
            c.set(1,32,c.r[6].wrapping_add(0xffc));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cef24); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x17dc),32));
            c.set(3,32,c.r[6].wrapping_add(0x17dc));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cef3a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1be4),32));
            { let a = c.r[6].wrapping_add(0x1ca8); wm(m,a,8,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x1ca6); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x1caa); wm(m,a,8,0x0u32); }
            if c.cond(4) { pc = 0x5cef62; continue; }
            pc = 0x5cef59; continue;
            }
            0x5cef59 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5cef5f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5cef62; continue;
            }
            0x5cef62 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2010),32));
            c.set(1,32,c.r[6].wrapping_add(0x2010));
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x1be4); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cef80); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1e80),32));
            c.set(1,32,c.r[6].wrapping_add(0x1e80));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cef94); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1cb0),32));
            c.set(1,32,c.r[6].wrapping_add(0x1cb0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cefa8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1d98),32));
            c.set(1,32,c.r[6].wrapping_add(0x1d98));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cefbc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cc7f4u32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x4u32); }
            { let a = c.r[0].wrapping_add(0x3a); wm(m,a,8,0x1u32); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x1c88); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1c84); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[6].wrapping_add(0x604));
            { let a = c.r[6].wrapping_add(0x1cad); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x1cae); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x5ceff4; continue;
            }
            0x5ceff4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.add(a,0xfffffc60u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cf008); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cf016); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,0xe8u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5ceff4; continue; }
            pc = 0x5cf02f; continue;
            }
            0x5cf02f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bf4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x11u32,32); }
            c.set(1,8,u32::from(c.cond(14)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(2,8,u32::from(c.cond(13)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5cf12b; continue; }
            pc = 0x5cf04c; continue;
            }
            0x5cf04c => {
            { let a = c.get(0,32); c.sub(a,0x9u32,32); }
            if c.cond(7) { pc = 0x5cf0cf; continue; }
            pc = 0x5cf051; continue;
            }
            0x5cf051 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x5cf7d4)); continue;
            }
            0x5cf058 => {
            c.push(m,0x7920u32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5cf067);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cf0d3; continue;
            }
            0x5cf069 => {
            c.push(m,0x7921u32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5cf078);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cf0d3; continue;
            }
            0x5cf07a => {
            c.push(m,0x7922u32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5cf089);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cf0d3; continue;
            }
            0x5cf08b => {
            c.push(m,0x791fu32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5cf09a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cf0d3; continue;
            }
            0x5cf09c => {
            c.push(m,0x797bu32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5cf0ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cf0d3; continue;
            }
            0x5cf0ad => {
            c.push(m,0x797cu32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5cf0bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cf0d3; continue;
            }
            0x5cf0be => {
            c.push(m,0x7923u32);
            c.set(1,32,0x6da820u32);
            c.push(m, 0x5cf0cd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cf0d3; continue;
            }
            0x5cf0cf => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            pc = 0x5cf0d3; continue;
            }
            0x5cf0d3 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cf0e1);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x66e100u32,8));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x20),32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6cc7e0u32,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x1c),32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0xa4u32,32); c.set(2,32,r); }
            { let a = c.r[2]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.r[2].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x5cf12b; continue;
            }
            0x5cf12b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(0,32); c.sub(a,0x9u32,32); }
            if c.cond(7) { pc = 0x5cf7a4; continue; }
            pc = 0x5cf138; continue;
            }
            0x5cf138 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x5cf7fc)); continue;
            }
            0x5cf13f => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cf14b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,0x0u32);
            c.push(m,0x3u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x5cf167);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x504ac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0x1be4); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[0],16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x5cf17f; continue; }
            pc = 0x5cf173; continue;
            }
            0x5cf173 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.r[6].wrapping_add(0x1c30); wm(m,a,32,c.get(2,32)); }
            pc = 0x5cf17f; continue;
            }
            0x5cf17f => {
            c.set(7,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,8); c.sub(a,0xcu32,8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0xbu32,8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5cf1a4; continue; }
            pc = 0x5cf19a; continue;
            }
            0x5cf19a => {
            c.push(m,c.get(6,32));
            c.push(m,0xbu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cf1a4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46b710, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5cf1a4; continue;
            }
            0x5cf1a4 => {
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5cf1c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5269a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1c34); wm(m,a,32,c.get(0,32)); }
            pc = 0x5cf7a4; continue;
            }
            0x5cf1d2 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cf1de); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x5cf1fa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x504ac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0x1be4); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[0],16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x5cf212; continue; }
            pc = 0x5cf206; continue;
            }
            0x5cf206 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.r[6].wrapping_add(0x1c30); wm(m,a,32,c.get(1,32)); }
            pc = 0x5cf212; continue;
            }
            0x5cf212 => {
            c.set(7,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,8); c.sub(a,0xau32,8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.and(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5cf240; continue; }
            pc = 0x5cf236; continue;
            }
            0x5cf236 => {
            c.push(m,c.get(6,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cf240);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46b710, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5cf240; continue;
            }
            0x5cf240 => {
            c.set(2,32,rm(m,0x6dccf8u32,32));
            { let a = c.get(2,32); let r = c.add(a,0x1b58u32,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x1be0); wm(m,a,32,c.get(2,32)); }
            pc = 0x5cf7a4; continue;
            }
            0x5cf257 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cf263); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.push(m,0x0u32);
            c.push(m,0x5u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x5cf27f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x504ac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0x1be4); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[0],16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x5cf297; continue; }
            pc = 0x5cf28b; continue;
            }
            0x5cf28b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.r[6].wrapping_add(0x1c30); wm(m,a,32,c.get(1,32)); }
            pc = 0x5cf297; continue;
            }
            0x5cf297 => {
            c.set(7,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5cf7a4; continue; }
            pc = 0x5cf2b6; continue;
            }
            0x5cf2b6 => {
            c.push(m,c.get(6,32));
            c.push(m,0x4u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cf2c0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46b710, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5cf7a4; continue;
            }
            0x5cf2c5 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cf2d1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x2154));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5cf2f4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x526b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5cf7a4; continue; }
            pc = 0x5cf313; continue;
            }
            0x5cf313 => {
            c.push(m,c.get(6,32));
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cf31d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46b710, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5cf7a4; continue;
            }
            0x5cf322 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let r = c.add(a,0x12cu32,32); c.set(0,32,r); }
            c.push(m,0x64u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[3].wrapping_add(0x44),32); c.push(m, 0x5cf335); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x6eu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x5cf34a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,0x12cu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            c.push(m,0x64u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,c.r[6].wrapping_add(0xa8c));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x6e));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x5cf371); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cf37d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x604));
            c.set(3,32,0x4u32);
            pc = 0x5cf388; continue;
            }
            0x5cf388 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xfffffc60),32));
            c.set(1,32,c.r[7].wrapping_add(0xfffffc60));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cf39c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cf3a8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5cf388; continue; }
            pc = 0x5cf3b1; continue;
            }
            0x5cf3b1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd44),32));
            c.set(1,32,c.r[6].wrapping_add(0xd44));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cf3c5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xe2c),32));
            c.set(1,32,c.r[6].wrapping_add(0xe2c));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cf3d9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf14),32));
            c.set(1,32,c.r[6].wrapping_add(0xf14));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cf3ed); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xffc),32));
            c.set(1,32,c.r[6].wrapping_add(0xffc));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cf401); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,0x0u32);
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x5cf41d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x504ac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0x1be4); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[0],16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x5cf6cc; continue; }
            pc = 0x5cf42d; continue;
            }
            0x5cf42d => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.r[6].wrapping_add(0x1c30); wm(m,a,32,c.get(2,32)); }
            pc = 0x5cf6cc; continue;
            }
            0x5cf43e => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cf44a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c9c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xfffffff3));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x3c));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c7c),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x5cf497); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c7c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c80),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5cf4b6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c80),32));
            c.set(1,32,c.r[6].wrapping_add(0x17d4));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cf4c8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1c80),32));
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.get(7,32); c.sub(a,0x6u32,32); }
            c.set(1,32,c.get(3,32));
            c.set(0,8,u32::from(c.cond(15)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cf4df); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,8); c.sub(a,0x10u32,8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0xfu32,8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5cf7a4; continue; }
            pc = 0x5cf4fe; continue;
            }
            0x5cf4fe => {
            c.push(m,c.get(6,32));
            c.push(m,0xfu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cf508);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46b710, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5cf7a4; continue;
            }
            0x5cf50d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,0x0u32);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x838),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x1c74); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cf53a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x5cf556);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x504ac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0x1be4); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[0],16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x5cf56e; continue; }
            pc = 0x5cf562; continue;
            }
            0x5cf562 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.r[6].wrapping_add(0x1c30); wm(m,a,32,c.get(2,32)); }
            pc = 0x5cf56e; continue;
            }
            0x5cf56e => {
            c.set(7,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,8); c.sub(a,0x12u32,8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0x11u32,8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5cf7a4; continue; }
            pc = 0x5cf58d; continue;
            }
            0x5cf58d => {
            c.push(m,c.get(6,32));
            c.push(m,0x11u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cf597);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46b710, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5cf7a4; continue;
            }
            0x5cf59c => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cf5a8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2170),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5cf5d4; continue; }
            pc = 0x5cf5b2; continue;
            }
            0x5cf5b2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5cf5ce);
            { let args = []; c.r[0] = host.call(m, 0x526650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x2170); wm(m,a,32,c.get(0,32)); }
            pc = 0x5cf5d4; continue;
            }
            0x5cf5d4 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2170),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5cf613; continue; }
            pc = 0x5cf5de; continue;
            }
            0x5cf5de => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xe),8));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            if c.cond(4) { pc = 0x5cf604; continue; }
            pc = 0x5cf5f6; continue;
            }
            0x5cf5f6 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x1c98); wm(m,a,32,c.get(1,32)); }
            pc = 0x5cf613; continue;
            }
            0x5cf604 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x1c98); wm(m,a,32,c.get(2,32)); }
            pc = 0x5cf613; continue;
            }
            0x5cf613 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c98),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x1ca9); wm(m,a,8,0x0u32); }
            c.push(m, 0x5cf628);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5d7de0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1be4); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[0],16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x5cf640; continue; }
            pc = 0x5cf634; continue;
            }
            0x5cf634 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.r[6].wrapping_add(0x1c30); wm(m,a,32,c.get(1,32)); }
            pc = 0x5cf640; continue;
            }
            0x5cf640 => {
            { let a = c.r[6].wrapping_add(0x1ca6); wm(m,a,8,0x0u32); }
            c.set(7,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,8); c.sub(a,0xeu32,8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); c.sub(a,0xdu32,8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); c.and(a,c.get(2,32),32); }
            pc = 0x5cf6e5; continue;
            }
            0x5cf665 => {
            c.set(1,32,rm(m,0x6e54f8u32,32));
            c.push(m, 0x5cf670);
            { let args = []; c.r[0] = host.call(m, 0x5a7320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0xb),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cf6cc; continue; }
            pc = 0x5cf67b; continue;
            }
            0x5cf67b => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cf687); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5cf690);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[3].wrapping_add(c.r[0]).wrapping_add(0x1));
            c.push(m,0x7cu32);
            c.push(m,c.get(3,32));
            c.push(m, 0x5cf69c);
            { let args = []; c.r[0] = host.call(m, 0x63c920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x2184));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5cf6b2; continue; }
            pc = 0x5cf6a9; continue;
            }
            0x5cf6a9 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5cf6b0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cf6b7; continue;
            }
            0x5cf6b2 => {
            c.push(m, 0x5cf6b7);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5cf6b7; continue;
            }
            0x5cf6b7 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x5cf6bd);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[3].wrapping_add(c.r[0]).wrapping_add(0x1));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cf6cc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cf6cc; continue;
            }
            0x5cf6cc => {
            c.set(7,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,8); c.sub(a,0xeu32,8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0xdu32,8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            pc = 0x5cf6e5; continue;
            }
            0x5cf6e5 => {
            if c.cond(4) { pc = 0x5cf7a4; continue; }
            pc = 0x5cf6eb; continue;
            }
            0x5cf6eb => {
            c.push(m,c.get(6,32));
            c.push(m,0xdu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cf6f5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46b710, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5cf7a4; continue;
            }
            0x5cf6fa => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cf706); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2010),32));
            c.set(1,32,c.r[6].wrapping_add(0x2010));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cf71a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1e80),32));
            c.set(1,32,c.r[6].wrapping_add(0x1e80));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cf72e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1cb0),32));
            c.set(1,32,c.r[6].wrapping_add(0x1cb0));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cf742); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1d98),32));
            c.set(1,32,c.r[6].wrapping_add(0x1d98));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cf756); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,0x6cc7f4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x38),8));
            { let a = c.get(0,8); c.sub(a,0xeu32,8); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0xdu32,8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5cf77b; continue; }
            pc = 0x5cf771; continue;
            }
            0x5cf771 => {
            c.push(m,c.get(6,32));
            c.push(m,0xdu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cf77b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46b710, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5cf77b; continue;
            }
            0x5cf77b => {
            c.set(0,32,c.r[6].wrapping_add(0x1bf0));
            c.set(1,32,c.r[6].wrapping_add(0x2174));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x5cf7a4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5268d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5cf7a4; continue;
            }
            0x5cf7a4 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x5cf7ae); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5cf7bf);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn church_get_donation_tier(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4fe360;
    loop {
        match pc {
            0x4fe360 => {
            c.set(0,32,rm(m,0x6de52cu32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            if c.cond(14) { pc = 0x4fe38e; continue; }
            pc = 0x4fe389; continue;
            }
            0x4fe389 => {
            c.set(0,32,0x5u32);
            pc = 0x4fe38e; continue;
            }
            0x4fe38e => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn church_content_window_update_feed_poor_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5caab0;
    loop {
        match pc {
            0x5caab0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6667a6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x3cu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x1d38),16));
            c.set(2,16,rm(m,0x6dfc14u32,16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x5caaf8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[6].wrapping_add(0x16a8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[6].wrapping_add(0xed0));
            c.set(3,32,c.get(0,32));
            c.set(0,32,0xffffe318u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(7,32,c.r[6].wrapping_add(0x1d3c));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x5cab2f; continue;
            }
            0x5cab27 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x5cab2f; continue;
            }
            0x5cab2f => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[7]; wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x5cab42; continue; }
            pc = 0x5cab39; continue;
            }
            0x5cab39 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffb0));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            { let a = c.r[7]; wm(m,a,32,c.get(1,32)); }
            pc = 0x5cab42; continue;
            }
            0x5cab42 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5cab53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xe),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x5cabbd; continue; }
            pc = 0x5cab6b; continue;
            }
            0x5cab6b => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(5,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x136),8));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x39),8));
            { let a = c.get(0,8); c.set(0,8,!a & 0xffu32); }
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x1d38),32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5caba7; continue; }
            pc = 0x5cab98; continue;
            }
            0x5cab98 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[5]),32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            pc = 0x5caba7; continue;
            }
            0x5caba7 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x5cab6b; continue; }
            pc = 0x5cabbd; continue;
            }
            0x5cabbd => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,rm(m,c.r[7],32));
            if c.cond(4) { pc = 0x5cabe0; continue; }
            pc = 0x5cabcd; continue;
            }
            0x5cabcd => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            pc = 0x5cabf1; continue;
            }
            0x5cabe0 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            pc = 0x5cabf1; continue;
            }
            0x5cabf1 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xfffffff8),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[7]; wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x5cac11; continue; }
            pc = 0x5cabfc; continue;
            }
            0x5cabfc => {
            c.set(1,32,rm(m,c.r[3],32));
            { let a = c.r[3].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x5cac11; continue; }
            pc = 0x5cac05; continue;
            }
            0x5cac05 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[3].wrapping_add(0xfffffe78));
            c.push(m, 0x5cac11);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cac11; continue;
            }
            0x5cac11 => {
            c.set(5,32,c.r[3].wrapping_add(0xfffffe78));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5cac1e);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cac26);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(1,32)); }
            if c.cond(9) { pc = 0x5cac43; continue; }
            pc = 0x5cac31; continue;
            }
            0x5cac31 => {
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5cac3d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[7]; wm(m,a,32,0x0u32); }
            pc = 0x5cac43; continue;
            }
            0x5cac43 => {
            c.set(1,32,rm(m,c.r[7],32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x6c64ccu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5cac55);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cac68);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x190u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x100u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            if c.cond(12) { pc = 0x5cab27; continue; }
            pc = 0x5cac8d; continue;
            }
            0x5cac8d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[6].wrapping_add(0x1d4c); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x5caca6; continue; }
            pc = 0x5cac9d; continue;
            }
            0x5cac9d => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x20),32));
            { let a = c.r[6].wrapping_add(0x1d4c); wm(m,a,32,c.get(1,32)); }
            pc = 0x5caca6; continue;
            }
            0x5caca6 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5cacb7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x5cad1f; continue; }
            pc = 0x5caccb; continue;
            }
            0x5caccb => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1d38),32));
            pc = 0x5cacd1; continue;
            }
            0x5cacd1 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x136),8));
            { let a = c.get(1,8); c.set(1,8,!a & 0xffu32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x5cad0b; continue; }
            pc = 0x5cacfa; continue;
            }
            0x5cacfa => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x70),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1d4c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x1d4c); wm(m,a,32,c.get(1,32)); }
            pc = 0x5cad0b; continue;
            }
            0x5cad0b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x5cacd1; continue; }
            pc = 0x5cad1f; continue;
            }
            0x5cad1f => {
            c.set(0,8,rm(m,0x672c1bu32,8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1d4c),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cad41; continue; }
            pc = 0x5cad2e; continue;
            }
            0x5cad2e => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            pc = 0x5cad52; continue;
            }
            0x5cad41 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            pc = 0x5cad52; continue;
            }
            0x5cad52 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1508),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[6].wrapping_add(0x1d4c); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x5cad80; continue; }
            pc = 0x5cad64; continue;
            }
            0x5cad64 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1510),32));
            c.set(1,32,c.r[6].wrapping_add(0x1388));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[1].wrapping_add(0x180); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x5cad80; continue; }
            pc = 0x5cad7a; continue;
            }
            0x5cad7a => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5cad80);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cad80; continue;
            }
            0x5cad80 => {
            c.set(7,32,c.r[6].wrapping_add(0x1388));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cad8d);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cad95);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1d4c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x1d4c); wm(m,a,32,c.get(1,32)); }
            if c.cond(9) { pc = 0x5cadbe; continue; }
            pc = 0x5cada8; continue;
            }
            0x5cada8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1510),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cadb8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1d4c); wm(m,a,32,c.get(5,32)); }
            pc = 0x5cadbe; continue;
            }
            0x5cadbe => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1d4c),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x6c64d0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5cadd4);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.r[6].wrapping_add(0x1aa8));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cade7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x16a8));
            c.set(7,32,c.r[6].wrapping_add(0x264));
            c.set(5,32,c.r[6].wrapping_add(0xd48));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x5u32); }
            pc = 0x5cae05; continue;
            }
            0x5cae05 => {
            c.push(m, 0x5cae0a);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cae12);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cae20);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cae5b; continue; }
            pc = 0x5cae24; continue;
            }
            0x5cae24 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5cae5b; continue; }
            pc = 0x5cae2c; continue;
            }
            0x5cae2c => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5cae33);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cae3b);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x6c64d4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x5cae4c);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5cae5b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cae5b; continue;
            }
            0x5cae5b => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5cae62);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x180),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cae70);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(1,32,c.get(7,32));
            if c.cond(4) { pc = 0x5cae82; continue; }
            pc = 0x5cae79; continue;
            }
            0x5cae79 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5cae80); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cae89; continue;
            }
            0x5cae82 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5cae89); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cae89; continue;
            }
            0x5cae89 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.add(a,0x100u32,32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x190u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5cae05; continue; }
            pc = 0x5caeb2; continue;
            }
            0x5caeb2 => {
            c.set(7,32,c.r[6].wrapping_add(0x6ec));
            c.set(5,32,c.r[6].wrapping_add(0xd48));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x5u32); }
            pc = 0x5caec6; continue;
            }
            0x5caec6 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5caecd);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5caed5);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5caee3);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5caf11; continue; }
            pc = 0x5caeeb; continue;
            }
            0x5caeeb => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x5caf13; continue; }
            pc = 0x5caeef; continue;
            }
            0x5caeef => {
            c.set(0,32,c.r[3].wrapping_add(0xffffffff));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0x6c64d8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x5caf02);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5caf11);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5caf11; continue;
            }
            0x5caf11 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            pc = 0x5caf13; continue;
            }
            0x5caf13 => {
            c.set(1,32,c.get(7,32));
            if c.cond(4) { pc = 0x5caf20; continue; }
            pc = 0x5caf17; continue;
            }
            0x5caf17 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5caf1e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5caf27; continue;
            }
            0x5caf20 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5caf27); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5caf27; continue;
            }
            0x5caf27 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(5,32); let r = c.add(a,0x190u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5caec6; continue; }
            pc = 0x5caf3e; continue;
            }
            0x5caf3e => {
            c.set(7,32,c.r[6].wrapping_add(0xed0));
            c.set(3,32,0x5u32);
            c.set(5,32,0x1869fu32);
            pc = 0x5caf4e; continue;
            }
            0x5caf4e => {
            { let a = rm(m,c.r[7].wrapping_add(0xfffffff8),32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x5caf68; continue; }
            pc = 0x5caf53; continue;
            }
            0x5caf53 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[7].wrapping_add(0xfffffff8); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x5caf68; continue; }
            pc = 0x5caf5c; continue;
            }
            0x5caf5c => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[7].wrapping_add(0xfffffe78));
            c.push(m, 0x5caf68);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5caf68; continue;
            }
            0x5caf68 => {
            { let a = c.get(7,32); let r = c.add(a,0x190u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5caf4e; continue; }
            pc = 0x5caf71; continue;
            }
            0x5caf71 => {
            { let a = c.r[6].wrapping_add(0x1d50); wm(m,a,32,0x0u32); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0xd48));
            pc = 0x5caf83; continue;
            }
            0x5caf83 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cafaa; continue; }
            pc = 0x5caf8f; continue;
            }
            0x5caf8f => {
            c.push(m, 0x5caf94);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5caf9c);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            pc = 0x5cafc6; continue;
            }
            0x5cafaa => {
            c.push(m, 0x5cafaf);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cafb7);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            pc = 0x5cafc6; continue;
            }
            0x5cafc6 => {
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1d38),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x5cafd9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1d50),32));
            { let a = c.get(7,32); let r = c.add(a,0x190u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x5u32,32); }
            { let a = c.r[6].wrapping_add(0x1d50); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.get(1,32));
            if c.cond(12) { pc = 0x5caf83; continue; }
            pc = 0x5caff5; continue;
            }
            0x5caff5 => {
            c.set(7,32,c.r[6].wrapping_add(0x17c));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,32,c.get(7,32));
            if c.cond(4) { pc = 0x5cb00a; continue; }
            pc = 0x5cb001; continue;
            }
            0x5cb001 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5cb008); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb011; continue;
            }
            0x5cb00a => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5cb011); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb011; continue;
            }
            0x5cb011 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cb018);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cb254; continue; }
            pc = 0x5cb020; continue;
            }
            0x5cb020 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1d38),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,sx(rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x304),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[0].wrapping_add(0x2d4),32),32); c.set(1,32,r); }
            c.set(0,32,0x38e38e39u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x20));
            c.fstp64(m,c.r[4].wrapping_add(0x20));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5cb072);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x5cb07a);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1d50),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            c.push(m,0x0u32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x1d30); wm(m,a,32,0x4u32); }
            { let a = c.r[6].wrapping_add(0x1d50); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cb0a4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5cb0d3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54f0u32,32));
            c.push(m, 0x5cb0de);
            { let args = []; c.r[0] = host.call(m, 0x5a5c50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m, 0x5cb0e7);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,0x6dfc14u32,8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1d38),8));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1d50),32));
            c.set(0,32,0xffu32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x30u32); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x3d); wm(m,a,8,c.get(2,8)); }
            if c.cond(15) { pc = 0x5cb116; continue; }
            pc = 0x5cb112; continue;
            }
            0x5cb112 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,c.get(1,8));
            pc = 0x5cb116; continue;
            }
            0x5cb116 => {
            c.set(7,32,c.r[6].wrapping_add(0xd48));
            { let a = c.r[4].wrapping_add(0x3e); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cb127);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb12f);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x11f8));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x5cb142);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb14a);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1068));
            { let a = c.r[4].wrapping_add(0x42); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x5cb15d);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb165);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0xed8));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x5cb178);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb180);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1388));
            { let a = c.r[4].wrapping_add(0x46); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x5cb193);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb19b);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb1b2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0x6ec));
            c.set(3,32,c.r[6].wrapping_add(0x16a8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x5u32); }
            pc = 0x5cb1c6; continue;
            }
            0x5cb1c6 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cb1d2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cb1de); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c64dcu32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x5cb1ec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x1u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cb1fd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x0u32); }
            c.push(m, 0x5cb20b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffb78),32));
            c.set(1,32,c.r[5].wrapping_add(0xfffffb78));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cb21f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cb22c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); let r = c.add(a,0x100u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x190u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0xe8u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5cb1c6; continue; }
            pc = 0x5cb24d; continue;
            }
            0x5cb24d => {
            { let a = c.r[6].wrapping_add(0x1d5f); wm(m,a,8,0x1u32); }
            pc = 0x5cb254; continue;
            }
            0x5cb254 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5cb265);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn church_content_window_update_redecorate_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5cb280;
    loop {
        match pc {
            0x5cb280 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6667b8u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.r[6].wrapping_add(0x1518));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5cb2aa);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb2b2);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1698),32));
            c.set(1,32,0xf423fu32);
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x0u32); }
            if c.cond(14) { pc = 0x5cb2ef; continue; }
            pc = 0x5cb2d7; continue;
            }
            0x5cb2d7 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x188),32));
            { let a = c.r[3].wrapping_add(0x180); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x5cb2ef; continue; }
            pc = 0x5cb2e7; continue;
            }
            0x5cb2e7 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5cb2ef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb2ef; continue;
            }
            0x5cb2ef => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1698),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5cb31a; continue; }
            pc = 0x5cb2f9; continue;
            }
            0x5cb2f9 => {
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(7,32));
            c.push(m,0x6c64e0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x5cb30b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5cb31a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb31a; continue;
            }
            0x5cb31a => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.push(m,c.get(5,32));
            if c.cond(13) { pc = 0x5cb361; continue; }
            pc = 0x5cb31f; continue;
            }
            0x5cb31f => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(7,32));
            c.push(m,0x6c64e4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb331);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5cb33c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb33c; continue;
            }
            0x5cb33c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(5,32,c.r[6].wrapping_add(0x17c));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5cb34f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb34f; continue;
            }
            0x5cb34f => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(1,32,c.r[6].wrapping_add(0xb74));
            if c.cond(5) { pc = 0x5cb378; continue; }
            pc = 0x5cb359; continue;
            }
            0x5cb359 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5cb35f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb37f; continue;
            }
            0x5cb361 => {
            if c.cond(14) { pc = 0x5cb33c; continue; }
            pc = 0x5cb363; continue;
            }
            0x5cb363 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(5,32,c.r[6].wrapping_add(0x17c));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5cb376); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb34f; continue;
            }
            0x5cb378 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5cb37f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb37f; continue;
            }
            0x5cb37f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1698),32));
            c.set(3,32,c.r[6].wrapping_add(0xc5c));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,c.get(3,32));
            if c.cond(5) { pc = 0x5cb39a; continue; }
            pc = 0x5cb391; continue;
            }
            0x5cb391 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5cb398); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb3a1; continue;
            }
            0x5cb39a => {
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5cb3a1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb3a1; continue;
            }
            0x5cb3a1 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5cb3a8);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let v = c.pop(m); c.set(5,32,v); }
            if c.cond(4) { pc = 0x5cb432; continue; }
            pc = 0x5cb3b1; continue;
            }
            0x5cb3b1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5cb3e0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54f0u32,32));
            c.push(m, 0x5cb3eb);
            { let args = []; c.r[0] = host.call(m, 0x5a5c50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e54f0u32,32));
            c.push(m, 0x5cb3f6);
            { let args = []; c.r[0] = host.call(m, 0x5a5c80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x5cb3ff);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1d38),32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x32u32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x5cb42e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x5cb4ac; continue;
            }
            0x5cb432 => {
            c.set(1,32,c.r[6].wrapping_add(0xb74));
            c.push(m, 0x5cb43d);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cb46c; continue; }
            pc = 0x5cb441; continue;
            }
            0x5cb441 => {
            { let a = c.get(7,32); let r = c.sub(a,0x1f4u32,32); c.set(7,32,r); }
            if c.cond(9) { pc = 0x5cb44b; continue; }
            pc = 0x5cb449; continue;
            }
            0x5cb449 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x5cb44b; continue;
            }
            0x5cb44b => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x6c64e8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x5cb45b);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1518));
            c.push(m,c.get(7,32));
            c.push(m, 0x5cb46a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb4ac; continue;
            }
            0x5cb46c => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5cb473);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cb4ac; continue; }
            pc = 0x5cb477; continue;
            }
            0x5cb477 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1698),32));
            { let a = c.get(7,32); let r = c.add(a,0x1f4u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5cb489; continue; }
            pc = 0x5cb487; continue;
            }
            0x5cb487 => {
            c.set(7,32,c.get(0,32));
            pc = 0x5cb489; continue;
            }
            0x5cb489 => {
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,0x6c64ecu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5cb499);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.set(1,32,c.r[6].wrapping_add(0x1518));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb4ac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb4ac; continue;
            }
            0x5cb4ac => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[6].wrapping_add(0x1d58); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5cb4c3);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn church_content_window_update_extension_page(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5cb4e0;
    loop {
        match pc {
            0x5cb4e0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6667cau32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(5,32,c.r[6].wrapping_add(0x1518));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5cb50b);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb513);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1698),32));
            c.set(1,32,0xf423fu32);
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x0u32); }
            if c.cond(14) { pc = 0x5cb550; continue; }
            pc = 0x5cb538; continue;
            }
            0x5cb538 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x188),32));
            { let a = c.r[5].wrapping_add(0x180); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x5cb550; continue; }
            pc = 0x5cb548; continue;
            }
            0x5cb548 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5cb550);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb550; continue;
            }
            0x5cb550 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1d38),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x794));
            c.push(m, 0x5cb571);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fea97, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1d5d),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5cb5b2; continue; }
            pc = 0x5cb57d; continue;
            }
            0x5cb57d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0x1d5d); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cb5b2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb5b2; continue;
            }
            0x5cb5b2 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1d5d),8));
            c.set(1,32,c.r[6].wrapping_add(0xb74));
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            c.set(0,32,rm(m,c.r[1],32));
            if c.cond(3) { pc = 0x5cb5fa; continue; }
            pc = 0x5cb5c4; continue;
            }
            0x5cb5c4 => {
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cb5cc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc5c),32));
            c.set(1,32,c.r[6].wrapping_add(0xc5c));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cb5e0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(3,32,c.r[6].wrapping_add(0x17c));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cb5f6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            pc = 0x5cb62e; continue;
            }
            0x5cb5fa => {
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cb602); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc5c),32));
            c.set(1,32,c.r[6].wrapping_add(0xc5c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cb616); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(3,32,c.r[6].wrapping_add(0x17c));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cb62c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            pc = 0x5cb62e; continue;
            }
            0x5cb62e => {
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cb639); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1698),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5cb664; continue; }
            pc = 0x5cb643; continue;
            }
            0x5cb643 => {
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(7,32));
            c.push(m,0x6c64f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb655);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5cb664);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb664; continue;
            }
            0x5cb664 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x5cb6a4; continue; }
            pc = 0x5cb668; continue;
            }
            0x5cb668 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(7,32));
            c.push(m,0x6c64f4u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5cb67a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb689);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb689; continue;
            }
            0x5cb689 => {
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5cb692); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb692; continue;
            }
            0x5cb692 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(1,32,c.r[6].wrapping_add(0xb74));
            if c.cond(5) { pc = 0x5cb6b1; continue; }
            pc = 0x5cb69c; continue;
            }
            0x5cb69c => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5cb6a2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb6b8; continue;
            }
            0x5cb6a4 => {
            if c.cond(14) { pc = 0x5cb689; continue; }
            pc = 0x5cb6a6; continue;
            }
            0x5cb6a6 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5cb6af); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb692; continue;
            }
            0x5cb6b1 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5cb6b8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb6b8; continue;
            }
            0x5cb6b8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1698),32));
            c.set(1,32,c.r[6].wrapping_add(0xc5c));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5cb6d1; continue; }
            pc = 0x5cb6c8; continue;
            }
            0x5cb6c8 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5cb6cf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb6d8; continue;
            }
            0x5cb6d1 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5cb6d8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb6d8; continue;
            }
            0x5cb6d8 => {
            c.set(1,32,c.r[6].wrapping_add(0xb74));
            c.push(m, 0x5cb6e3);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cb715; continue; }
            pc = 0x5cb6e7; continue;
            }
            0x5cb6e7 => {
            { let a = c.get(7,32); let r = c.sub(a,0x1f4u32,32); c.set(7,32,r); }
            if c.cond(9) { pc = 0x5cb6f1; continue; }
            pc = 0x5cb6ef; continue;
            }
            0x5cb6ef => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x5cb6f1; continue;
            }
            0x5cb6f1 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6c64f8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x5cb701);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5cb710);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb8d4; continue;
            }
            0x5cb715 => {
            c.set(1,32,c.r[6].wrapping_add(0xc5c));
            c.push(m, 0x5cb720);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cb75a; continue; }
            pc = 0x5cb724; continue;
            }
            0x5cb724 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1698),32));
            { let a = c.get(7,32); let r = c.add(a,0x1f4u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5cb736; continue; }
            pc = 0x5cb734; continue;
            }
            0x5cb734 => {
            c.set(7,32,c.get(0,32));
            pc = 0x5cb736; continue;
            }
            0x5cb736 => {
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6c64fcu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5cb746);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5cb755);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5cb8d4; continue;
            }
            0x5cb75a => {
            c.set(1,32,c.r[6].wrapping_add(0x17c));
            c.push(m, 0x5cb765);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cb8d4; continue; }
            pc = 0x5cb76d; continue;
            }
            0x5cb76d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cb79c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let a = c.r[6].wrapping_add(0x1d30); wm(m,a,32,0x5u32); }
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cb7b3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xb74),32));
            c.set(1,32,c.r[6].wrapping_add(0xb74));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cb7c7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc5c),32));
            c.set(1,32,c.r[6].wrapping_add(0xc5c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cb7db); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            c.set(1,32,c.r[6].wrapping_add(0x17c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cb7ef); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x5cb7f8);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1d38),32));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x31u32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x5cb826);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e54f0u32,32));
            c.push(m, 0x5cb831);
            { let args = []; c.r[0] = host.call(m, 0x5a5c50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,0x1u32);
            { let a = c.get(7,32); c.sub(a,0x3e8u32,32); }
            { let a = c.r[6].wrapping_add(0x1d5f); wm(m,a,8,c.get(0,8)); }
            if c.cond(13) { pc = 0x5cb848; continue; }
            pc = 0x5cb841; continue;
            }
            0x5cb841 => {
            { let a = c.r[6].wrapping_add(0x1d5c); wm(m,a,8,0x0u32); }
            pc = 0x5cb848; continue;
            }
            0x5cb848 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,0x1388u32,32); }
            c.set(1,8,u32::from(c.cond(14)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,0x3e8u32,32); }
            c.set(2,8,u32::from(c.cond(13)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5cb868; continue; }
            pc = 0x5cb862; continue;
            }
            0x5cb862 => {
            { let a = c.r[6].wrapping_add(0x1d5c); wm(m,a,8,c.get(0,8)); }
            pc = 0x5cb868; continue;
            }
            0x5cb868 => {
            { let a = c.get(7,32); c.sub(a,0x1388u32,32); }
            if c.cond(14) { pc = 0x5cb877; continue; }
            pc = 0x5cb870; continue;
            }
            0x5cb870 => {
            { let a = c.r[6].wrapping_add(0x1d5c); wm(m,a,8,0x2u32); }
            pc = 0x5cb877; continue;
            }
            0x5cb877 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1d38),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,0x6cc3e8u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x7a3),8));
            c.set(1,32,c.get(2,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x5));
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xe),8));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x5cb8d2; continue; }
            pc = 0x5cb8cb; continue;
            }
            0x5cb8cb => {
            { let a = c.r[6].wrapping_add(0x1d5c); wm(m,a,8,0x3u32); }
            pc = 0x5cb8d2; continue;
            }
            0x5cb8d2 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x5cb8d4; continue;
            }
            0x5cb8d4 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[6].wrapping_add(0x1d54); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5cb8eb);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_build_loan_applicant_operations(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51d160;
    loop {
        match pc {
            0x51d160 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[3]; wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,c.r[5].wrapping_add(0x9db));
            c.set(6,32,c.r[1].wrapping_add(0xb));
            pc = 0x51d18b; continue;
            }
            0x51d18b => {
            { let a = rm(m,c.r[7].wrapping_add(0x1),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x51d245; continue; }
            pc = 0x51d196; continue;
            }
            0x51d196 => {
            { let a = rm(m,c.r[7],8); c.sub(a,0x0u32,8); }
            if c.cond(6) { pc = 0x51d245; continue; }
            pc = 0x51d19f; continue;
            }
            0x51d19f => {
            c.set(2,8,rm(m,0x6dfc14u32,8));
            { let a = c.r[6]; wm(m,a,8,0x0u32); }
            { let a = c.get(0,8); let n = 0x4u32; let r = c.shl(a,n,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x1); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x2c0),8));
            { let a = c.get(0,8); let r = c.add(a,0x32u32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x3); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,8,c.get(0,8)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x1),16));
            { let a = rm(m,c.r[6].wrapping_add(0xfffffffd),16); c.sub(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x51d1cd; continue; }
            pc = 0x51d1c6; continue;
            }
            0x51d1c6 => {
            { let a = c.r[6].wrapping_add(0xfffffffd); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[3]; wm(m,a,8,0x1u32); }
            pc = 0x51d1cd; continue;
            }
            0x51d1cd => {
            c.set(0,8,rm(m,c.r[7],8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xffffffff),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x51d1dc; continue; }
            pc = 0x51d1d6; continue;
            }
            0x51d1d6 => {
            { let a = c.r[6].wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[3]; wm(m,a,8,0x1u32); }
            pc = 0x51d1dc; continue;
            }
            0x51d1dc => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xfffffff5),32));
            c.set(5,32,c.r[6].wrapping_add(0xfffffff5));
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x51d1ec);
            { let args = []; c.r[0] = host.call(m, 0x51d270, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x51d1fd; continue; }
            pc = 0x51d1fa; continue;
            }
            0x51d1fa => {
            { let a = c.r[3]; wm(m,a,8,0x1u32); }
            pc = 0x51d1fd; continue;
            }
            0x51d1fd => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xfffffffd),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xfffffffa),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x51d20d; continue; }
            pc = 0x51d207; continue;
            }
            0x51d207 => {
            { let a = c.r[6].wrapping_add(0xfffffffa); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[3]; wm(m,a,8,0x1u32); }
            pc = 0x51d20d; continue;
            }
            0x51d20d => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xfffffffe),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xfffffffb),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x51d21d; continue; }
            pc = 0x51d217; continue;
            }
            0x51d217 => {
            { let a = c.r[6].wrapping_add(0xfffffffb); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[3]; wm(m,a,8,0x1u32); }
            pc = 0x51d21d; continue;
            }
            0x51d21d => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xffffffff),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xfffffffc),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x51d22d; continue; }
            pc = 0x51d227; continue;
            }
            0x51d227 => {
            { let a = c.r[6].wrapping_add(0xfffffffc); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[3]; wm(m,a,8,0x1u32); }
            pc = 0x51d22d; continue;
            }
            0x51d22d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(6,32); let r = c.add(a,0x10u32,32); c.set(6,32,r); }
            pc = 0x51d245; continue;
            }
            0x51d245 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x6u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x51d18b; continue; }
            pc = 0x51d256; continue;
            }
            0x51d256 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_get_guild_entry_fee(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51b7e0;
    loop {
        match pc {
            0x51b7e0 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x2c0),8));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x19),8));
            { let a = c.get(1,8); let r = c.sub(a,c.get(0,8),8); c.set(1,8,r); }
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x2710u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x4e20u32,32); c.set(1,32,r); }
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x51b810);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f9580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x51b825; continue; }
            pc = 0x51b817; continue;
            }
            0x51b817 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            pc = 0x51b82e; continue;
            }
            0x51b825 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            pc = 0x51b82e; continue;
            }
            0x51b82e => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x3a98u32,32); }
            if c.cond(14) { pc = 0x51b841; continue; }
            pc = 0x51b83c; continue;
            }
            0x51b83c => {
            c.set(1,32,0x3a98u32);
            pc = 0x51b841; continue;
            }
            0x51b841 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x46c),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x51b876; continue; }
            pc = 0x51b84b; continue;
            }
            0x51b84b => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.imul(a,0x32u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xea60u32,32); }
            if c.cond(14) { pc = 0x51b878; continue; }
            pc = 0x51b868; continue;
            }
            0x51b868 => {
            c.set(0,32,0xea60u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x51b876 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x51b878; continue;
            }
            0x51b878 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_collect_visible_auctions(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4da1a0;
    loop {
        match pc {
            0x4da1a0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[5]; wm(m,a,32,0x0u32); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            if c.cond(3) { pc = 0x4da266; continue; }
            pc = 0x4da1d4; continue;
            }
            0x4da1d4 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(c.r[2]).wrapping_add(0x6),16));
            c.set(0,32,c.r[7].wrapping_add(c.r[2]));
            { let a = c.get(1,16); c.sub(a,0xau32,16); }
            if c.cond(4) { pc = 0x4da1f0; continue; }
            pc = 0x4da1ea; continue;
            }
            0x4da1ea => {
            { let a = c.get(1,16); c.sub(a,0x36u32,16); }
            if c.cond(5) { pc = 0x4da247; continue; }
            pc = 0x4da1f0; continue;
            }
            0x4da1f0 => {
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x15),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x4da247; continue; }
            pc = 0x4da1ff; continue;
            }
            0x4da1ff => {
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(2,32); let r = c.and(a,0xc0u32,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x4da247; continue; }
            pc = 0x4da21e; continue;
            }
            0x4da21e => {
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x14),8));
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            if c.cond(4) { pc = 0x4da244; continue; }
            pc = 0x4da225; continue;
            }
            0x4da225 => {
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x4da244; continue; }
            pc = 0x4da229; continue;
            }
            0x4da229 => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(3,8,rm(m,c.r[2].wrapping_add(0xd),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4da247; continue; }
            pc = 0x4da244; continue;
            }
            0x4da244 => {
            { let a = rm(m,c.r[5],32); let r = c.inc(a,32); { let a = c.r[5]; wm(m,a,32,r); } }
            pc = 0x4da247; continue;
            }
            0x4da247 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x4),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4da1d4; continue; }
            pc = 0x4da25e; continue;
            }
            0x4da25e => {
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(13) { pc = 0x4da272; continue; }
            pc = 0x4da266; continue;
            }
            0x4da266 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4da272 => {
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(14) { pc = 0x4da27e; continue; }
            pc = 0x4da277; continue;
            }
            0x4da277 => {
            { let a = c.r[5]; wm(m,a,32,0x3u32); }
            pc = 0x4da27e; continue;
            }
            0x4da27e => {
            c.set(0,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4da28d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4da2a5; continue; }
            pc = 0x4da298; continue;
            }
            0x4da298 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4da2a5 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xc),16));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4da475; continue; }
            pc = 0x4da2b9; continue;
            }
            0x4da2b9 => {
            c.set(1,32,0xfffffff0u32);
            c.set(5,32,c.r[0].wrapping_add(0x10));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            pc = 0x4da2c7; continue;
            }
            0x4da2c7 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(3,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,0xau32,16); }
            if c.cond(4) { pc = 0x4da2e9; continue; }
            pc = 0x4da2df; continue;
            }
            0x4da2df => {
            { let a = c.get(0,16); c.sub(a,0x36u32,16); }
            if c.cond(5) { pc = 0x4da456; continue; }
            pc = 0x4da2e9; continue;
            }
            0x4da2e9 => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x15),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x4da456; continue; }
            pc = 0x4da2fc; continue;
            }
            0x4da2fc => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(0,32); let r = c.and(a,0xc0u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4da456; continue; }
            pc = 0x4da31e; continue;
            }
            0x4da31e => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x14),8));
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            if c.cond(4) { pc = 0x4da346; continue; }
            pc = 0x4da325; continue;
            }
            0x4da325 => {
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x4da346; continue; }
            pc = 0x4da329; continue;
            }
            0x4da329 => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(0xd),8); c.sub(a,c.get(1,8),8); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4da452; continue; }
            pc = 0x4da346; continue;
            }
            0x4da346 => {
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x14),8));
            { let a = c.get(1,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4da35e; continue; }
            pc = 0x4da353; continue;
            }
            0x4da353 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0xe),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(1,32)); }
            pc = 0x4da35e; continue;
            }
            0x4da35e => {
            { let a = rm(m,c.r[3].wrapping_add(0x14),8); c.sub(a,0x2u32,8); }
            if c.cond(7) { pc = 0x4da36b; continue; }
            pc = 0x4da364; continue;
            }
            0x4da364 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff0),32); let r = c.add(a,0x100u32,32); { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,r); } }
            pc = 0x4da36b; continue;
            }
            0x4da36b => {
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x15),8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0xfffffffa); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[5]; wm(m,a,32,0x0u32); }
            c.set(7,32,c.r[5].wrapping_add(0xfffffff4));
            c.set(6,32,c.r[3].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x3u32); }
            pc = 0x4da38c; continue;
            }
            0x4da38c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x6),16); c.sub(a,0xau32,16); }
            if c.cond(5) { pc = 0x4da3e2; continue; }
            pc = 0x4da39e; continue;
            }
            0x4da39e => {
            c.set(0,16,rm(m,c.r[6],16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x4da422; continue; }
            pc = 0x4da3aa; continue;
            }
            0x4da3aa => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            c.set(0,32,c.r[2].wrapping_add(c.r[5]));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x10));
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            c.set(1,16,rm(m,c.r[6],16));
            { let a = c.r[7]; wm(m,a,16,c.get(1,16)); }
            pc = 0x4da427; continue;
            }
            0x4da3e2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x15),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(0,16,rm(m,c.r[6],16));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0x7fffu32,32); c.set(2,32,r); }
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[1].wrapping_add(0x776),16),16); }
            if c.cond(3) { pc = 0x4da422; continue; }
            pc = 0x4da40b; continue;
            }
            0x4da40b => {
            { let a = c.get(0,32); let r = c.and(a,0x7fffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4da416);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5]; wm(m,a,8,0xffu32); }
            c.set(0,16,rm(m,c.r[6],16));
            { let a = c.r[7]; wm(m,a,16,c.get(0,16)); }
            pc = 0x4da427; continue;
            }
            0x4da422 => {
            { let a = c.r[7]; wm(m,a,16,0xffffu32); }
            pc = 0x4da427; continue;
            }
            0x4da427 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4da38c; continue; }
            pc = 0x4da43c; continue;
            }
            0x4da43c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x14u32,32); c.set(5,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4da471; continue; }
            pc = 0x4da452; continue;
            }
            0x4da452 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4da456; continue;
            }
            0x4da456 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x4),16));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xc),16));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x4da2c7; continue; }
            pc = 0x4da471; continue;
            }
            0x4da471 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4da475; continue;
            }
            0x4da475 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn merchant_render_world_fog_map(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f7490;
    loop {
        match pc {
            0x4f7490 => {
            { let a = c.get(4,32); let r = c.sub(a,0x64u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x4f749f);
            { let args = []; c.r[0] = host.call(m, 0x534750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[6].wrapping_add(0x4d8));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m, 0x4f74af);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x534920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x468),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f74c8; continue; }
            pc = 0x4f74ba; continue;
            }
            0x4f74ba => {
            c.push(m,0x6df278u32);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x4f74c8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x534920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4f74c8; continue;
            }
            0x4f74c8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x4f74db);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x534a40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(4,32); let r = c.add(a,0x64u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_reset_loadout_for_pirate(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x515750;
    loop {
        match pc {
            0x515750 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(5,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            if c.cond(5) { pc = 0x5157e3; continue; }
            pc = 0x51576a; continue;
            }
            0x51576a => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,0x6de4aau32,16));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x5157dd; continue; }
            pc = 0x515777; continue;
            }
            0x515777 => {
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x515783);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x5157ca; continue; }
            pc = 0x515799; continue;
            }
            0x515799 => {
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            pc = 0x51579f; continue;
            }
            0x51579f => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x120),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5157c6; continue; }
            pc = 0x5157b7; continue;
            }
            0x5157b7 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x5157c6; continue;
            }
            0x5157c6 => {
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x51579f; continue; }
            pc = 0x5157ca; continue;
            }
            0x5157ca => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(15) { pc = 0x515777; continue; }
            pc = 0x5157ce; continue;
            }
            0x5157ce => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x5157dd; continue; }
            pc = 0x5157d2; continue;
            }
            0x5157d2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            pc = 0x5157e3; continue;
            }
            0x5157dd => {
            { let a = c.r[3]; wm(m,a,32,0x0u32); }
            pc = 0x5157e3; continue;
            }
            0x5157e3 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5159b5; continue; }
            pc = 0x5157fb; continue;
            }
            0x5157fb => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.push(m,0x60u32);
            c.push(m,0x0u32);
            c.set(2,32,c.r[6].wrapping_add(0x54));
            { let a = c.r[6].wrapping_add(0x154); wm(m,a,16,0x0u32); }
            c.push(m,c.get(2,32));
            c.push(m, 0x51581f);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x60u32);
            c.set(0,32,c.r[6].wrapping_add(0xb4));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x51582f);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x156); wm(m,a,8,0x0u32); }
            c.push(m,0x4u32);
            c.push(m, 0x515842);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x157),8));
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            if c.cond(6) { pc = 0x515856; continue; }
            pc = 0x51584c; continue;
            }
            0x51584c => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x157); wm(m,a,8,c.get(0,8)); }
            pc = 0x51585d; continue;
            }
            0x515856 => {
            { let a = c.r[6].wrapping_add(0x157); wm(m,a,8,0x0u32); }
            pc = 0x51585d; continue;
            }
            0x51585d => {
            c.push(m,0x4u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x515866);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x18u32);
            c.set(1,32,c.r[6].wrapping_add(0x13c));
            c.push(m,0xffu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x515879);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x515883);
            { let args = []; c.r[0] = host.call(m, 0x5181d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x51588a);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x12),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[3],32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,0x6de52eu32,8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5158be);
            { let args = []; c.r[0] = host.call(m, 0x52bb00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5158c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x51a4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5158d0);
            { let args = []; c.r[0] = host.call(m, 0x5185c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            if c.cond(3) { pc = 0x515921; continue; }
            pc = 0x5158e7; continue;
            }
            0x5158e7 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x12),8));
            { let a = c.get(1,32); let r = c.sub(a,0x0u32,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x515913; continue; }
            pc = 0x5158f1; continue;
            }
            0x5158f1 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x515903; continue; }
            pc = 0x5158f4; continue;
            }
            0x5158f4 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,c.get(0,16)); }
            pc = 0x51595c; continue;
            }
            0x515903 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,c.get(1,16)); }
            pc = 0x51595c; continue;
            }
            0x515913 => {
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            pc = 0x515956; continue;
            }
            0x515921 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x12),8));
            { let a = c.get(1,32); let r = c.sub(a,0x0u32,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x515950; continue; }
            pc = 0x51592b; continue;
            }
            0x51592b => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x51593c; continue; }
            pc = 0x51592e; continue;
            }
            0x51592e => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,c.get(0,16)); }
            pc = 0x51595c; continue;
            }
            0x51593c => {
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,c.get(1,16)); }
            pc = 0x51595c; continue;
            }
            0x515950 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            pc = 0x515956; continue;
            }
            0x515956 => {
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,c.get(2,16)); }
            pc = 0x51595c; continue;
            }
            0x51595c => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x40),16));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(1,16); let r = c.add(a,0x63u32,16); c.set(1,16,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x5159a0; continue; }
            pc = 0x51598f; continue;
            }
            0x51598f => {
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(7,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            pc = 0x5159a0; continue;
            }
            0x5159a0 => {
            { let a = c.r[6].wrapping_add(0x154); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5159b5 => {
            { let v = c.pop(m); c.set(7,32,v); }
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

pub fn scheduled_tasks_find_council_meeting_for_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4da140;
    loop {
        match pc {
            0x4da140 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,16,rm(m,c.r[1].wrapping_add(0xc),16));
            c.set(2,32,c.r[0].wrapping_add(0x1e00));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x4da189; continue; }
            pc = 0x4da15b; continue;
            }
            0x4da15b => {
            c.set(6,32,rm(m,c.r[1],32));
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x10),8));
            pc = 0x4da161; continue;
            }
            0x4da161 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x6),16); c.sub(a,0x2eu32,16); }
            c.set(1,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)));
            if c.cond(5) { pc = 0x4da17c; continue; }
            pc = 0x4da177; continue;
            }
            0x4da177 => {
            { let a = rm(m,c.r[1].wrapping_add(0xc),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4da18d; continue; }
            pc = 0x4da17c; continue;
            }
            0x4da17c => {
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4da189; continue; }
            pc = 0x4da180; continue;
            }
            0x4da180 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x4da161; continue; }
            pc = 0x4da189; continue;
            }
            0x4da189 => {
            { let a = c.get(0,16); let r = c.or(a,0xffffu32,16); c.set(0,16,r); }
            pc = 0x4da18d; continue;
            }
            0x4da18d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn merchant_get_mayor_candidacy_flag(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f9360;
    loop {
        match pc {
            0x4f9360 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x4f9370; continue; }
            pc = 0x4f9369; continue;
            }
            0x4f9369 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x118),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4f9370 => {
            { let a = c.get(0,8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x4f9381; continue; }
            pc = 0x4f9374; continue;
            }
            0x4f9374 => {
            { let a = c.r[1].wrapping_add(0x118); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4f9381 => {
            c.set(0,32,0x1u32);
            { let a = c.r[1].wrapping_add(0x118); wm(m,a,32,c.get(0,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn name_bank_get_given_name(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x512a60;
    loop {
        match pc {
            0x512a60 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0x80u32,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.sub(a,0x80u32,8); }
            if c.cond(5) { pc = 0x512aa9; continue; }
            pc = 0x512a72; continue;
            }
            0x512a72 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x7fu32,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xd2),16));
            { let a = c.get(2,32); let r = c.add(a,0x14u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x512a8f; continue; }
            pc = 0x512a85; continue;
            }
            0x512a85 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xac),32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x512a8f => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc0),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xac),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x512aa9 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xd0),16));
            { let a = c.get(2,32); let r = c.add(a,0x14u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x512ac3; continue; }
            pc = 0x512ab9; continue;
            }
            0x512ab9 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xa8),32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x512ac3 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xbc),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xa8),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn name_bank_get_family_name(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x512ae0;
    loop {
        match pc {
            0x512ae0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xd6),16));
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.add(a,0x14u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x512aff; continue; }
            pc = 0x512af6; continue;
            }
            0x512af6 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xb0),32));
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x512aff => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc4),32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xb0),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_collect_mayor_candidates(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x528cf0;
    loop {
        match pc {
            0x528cf0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(5,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(6,32,c.r[7].wrapping_add(0x3));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            pc = 0x528d0a; continue;
            }
            0x528d0a => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x4u32,32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x6737b4),8));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,8); let r = c.or(a,0xffu32,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.sub(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,8,c.get(2,8)); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x528d0a; continue; }
            pc = 0x528d26; continue;
            }
            0x528d26 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,0x6de4aau32,16));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x528dbd; continue; }
            pc = 0x528d37; continue;
            }
            0x528d37 => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(3,32));
            c.push(m, 0x528d43);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x528d4c);
            { let args = []; c.r[0] = host.call(m, 0x4f9360, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x528db5; continue; }
            pc = 0x528d50; continue;
            }
            0x528d50 => {
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x2c0),8));
            c.set(0,32,0x1u32);
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x468),32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x528db5; continue; }
            pc = 0x528d6d; continue;
            }
            0x528d6d => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x19),8));
            { let a = c.get(0,8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x528db5; continue; }
            pc = 0x528d74; continue;
            }
            0x528d74 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x39c),8); c.sub(a,0x5u32,8); }
            if c.cond(2) { pc = 0x528db5; continue; }
            pc = 0x528d83; continue;
            }
            0x528d83 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x528d8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f9580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.set(0,32,0x3u32);
            pc = 0x528d93; continue;
            }
            0x528d93 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14),32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x528db5; continue; }
            pc = 0x528d9b; continue;
            }
            0x528d9b => {
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x528daf; continue; }
            pc = 0x528da8; continue;
            }
            0x528da8 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[7]),8));
            { let a = c.r[0].wrapping_add(c.r[7]).wrapping_add(0x1); wm(m,a,8,c.get(1,8)); }
            pc = 0x528daf; continue;
            }
            0x528daf => {
            { let a = c.r[0].wrapping_add(c.r[7]); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x528d93; continue; }
            pc = 0x528db5; continue;
            }
            0x528db5 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(15) { pc = 0x528d37; continue; }
            pc = 0x528dbd; continue;
            }
            0x528dbd => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x528dbf; continue;
            }
            0x528dbf => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[7]),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(1,8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x528e12; continue; }
            pc = 0x528dd0; continue;
            }
            0x528dd0 => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x528de1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x528dea);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0x4); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x19),8));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x39c),8));
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0x8); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x1c),8));
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1d),8));
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            pc = 0x528e46; continue;
            }
            0x528e12 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x6e0),8));
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x6e4),8));
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0x8); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x6d4),8));
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0xc); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x6d8),8));
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0x10); wm(m,a,8,c.get(2,8)); }
            pc = 0x528e46; continue;
            }
            0x528e46 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x528dbf; continue; }
            pc = 0x528e50; continue;
            }
            0x528e50 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_get_next_mayor_election_date(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5295f0;
    loop {
        match pc {
            0x5295f0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x2c0),8));
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x529603);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d9f10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x529626; continue; }
            pc = 0x52960b; continue;
            }
            0x52960b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[1]; wm(m,a,8,0x1fu32); }
            { let a = c.r[2]; wm(m,a,8,0xcu32); }
            c.set(1,16,rm(m,0x6de4a2u32,16));
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x529626 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xf),8));
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x10),8));
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            c.set(2,16,rm(m,0x6de4a4u32,16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x8),16),16); }
            c.set(0,16,rm(m,0x6de4a2u32,16));
            if c.cond(6) { pc = 0x529653; continue; }
            pc = 0x529647; continue;
            }
            0x529647 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,16); let r = c.inc(a,16); c.set(0,16,r); }
            { let a = c.r[1]; wm(m,a,16,c.get(0,16)); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x529653 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[2]; wm(m,a,16,c.get(0,16)); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_get_mayor_epithet_index(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5296b0;
    loop {
        match pc {
            0x5296b0 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x6f1),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x5296e1; continue; }
            pc = 0x5296c4; continue;
            }
            0x5296c4 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5296d4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5296db);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5296e1 => {
            { let a = c.get(2,32); let r = c.or(a,0xffffffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x6e0),8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_find_hanse_diet_vote(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d9cd0;
    loop {
        match pc {
            0x4d9cd0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.push(m,c.get(6,32));
            if c.cond(3) { pc = 0x4d9cfb; continue; }
            pc = 0x4d9ce1; continue;
            }
            0x4d9ce1 => {
            c.set(6,32,rm(m,c.r[1],32));
            pc = 0x4d9ce3; continue;
            }
            0x4d9ce3 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x6),16); c.sub(a,0x21u32,16); }
            c.set(1,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)));
            if c.cond(4) { pc = 0x4d9cff; continue; }
            pc = 0x4d9cf1; continue;
            }
            0x4d9cf1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4d9ce3; continue; }
            pc = 0x4d9cfb; continue;
            }
            0x4d9cfb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4d9cff => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_find_alderman_election(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d9d10;
    loop {
        match pc {
            0x4d9d10 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xc),16));
            c.push(m,c.get(6,32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x4d9d30; continue; }
            pc = 0x4d9d1d; continue;
            }
            0x4d9d1d => {
            c.set(6,32,rm(m,c.r[1],32));
            c.set(1,32,c.r[6].wrapping_add(0x6));
            pc = 0x4d9d22; continue;
            }
            0x4d9d22 => {
            { let a = rm(m,c.r[1],16); c.sub(a,0x20u32,16); }
            if c.cond(4) { pc = 0x4d9d34; continue; }
            pc = 0x4d9d28; continue;
            }
            0x4d9d28 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x18u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4d9d22; continue; }
            pc = 0x4d9d30; continue;
            }
            0x4d9d30 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4d9d34 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn collect_alderman_candidates(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x529700;
    loop {
        match pc {
            0x529700 => {
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x54u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,0x6de52eu32,8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.push(m,c.get(7,32));
            c.set(5,32,0xffu32);
            if c.cond(3) { pc = 0x52977c; continue; }
            pc = 0x529721; continue;
            }
            0x529721 => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,0x6de52du32,8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x529763; continue; }
            pc = 0x529738; continue;
            }
            0x529738 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,8,c.get(0,8));
            c.push(m,c.get(2,32));
            c.push(m, 0x529747);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x52974e);
            { let args = []; c.r[0] = host.call(m, 0x4f73d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,0x6de52du32,8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x52976f; continue;
            }
            0x529763 => {
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xc8u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            pc = 0x52976f; continue;
            }
            0x52976f => {
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0xffffffu32); }
            c.set(1,32,0x1u32);
            pc = 0x52977c; continue;
            }
            0x52977c => {
            c.set(0,32,c.r[1].wrapping_mul(4));
            pc = 0x529783; continue;
            }
            0x529783 => {
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x529783; continue; }
            pc = 0x52979b; continue;
            }
            0x52979b => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x529849; continue; }
            pc = 0x5297aa; continue;
            }
            0x5297aa => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(6,32,c.r[2].wrapping_add(0x6f1));
            pc = 0x5297b6; continue;
            }
            0x5297b6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xfffffbd7),32));
            { let a = c.get(0,32); let r = c.and(a,0x40u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); c.sub(a,0x40u32,8); }
            if c.cond(5) { pc = 0x529835; continue; }
            pc = 0x5297c3; continue;
            }
            0x5297c3 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x6de52eu32,8));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x529835; continue; }
            pc = 0x5297cf; continue;
            }
            0x5297cf => {
            c.set(2,16,rm(m,c.r[6],8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x529835; continue; }
            pc = 0x5297dc; continue;
            }
            0x5297dc => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xfffffbe3),32));
            c.set(0,32,0x3u32);
            pc = 0x5297e7; continue;
            }
            0x5297e7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3c),32));
            { let a = c.get(1,32); c.sub(a,0xc8u32,32); }
            if c.cond(12) { pc = 0x529800; continue; }
            pc = 0x5297f3; continue;
            }
            0x5297f3 => {
            if c.cond(5) { pc = 0x5297fb; continue; }
            pc = 0x5297f5; continue;
            }
            0x5297f5 => {
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x50),32),32); }
            if c.cond(15) { pc = 0x529800; continue; }
            pc = 0x5297fb; continue;
            }
            0x5297fb => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x529832; continue;
            }
            0x529800 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x50),32));
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3c); wm(m,a,32,0xc8u32); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x50); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x529832; continue;
            }
            0x529832 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x5297e7; continue; }
            pc = 0x529835; continue;
            }
            0x529835 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x9f8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x5297b6; continue; }
            pc = 0x529849; continue;
            }
            0x529849 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x5298fa; continue; }
            pc = 0x529858; continue;
            }
            0x529858 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x6de52du32,8));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5298e8; continue; }
            pc = 0x529868; continue;
            }
            0x529868 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x529873);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(2,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(2,32); let r = c.and(a,0x40u32,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.sub(a,0x40u32,8); }
            if c.cond(5) { pc = 0x5298e8; continue; }
            pc = 0x529897; continue;
            }
            0x529897 => {
            { let a = rm(m,c.r[0].wrapping_add(0x37),8); c.sub(a,0x4u32,8); }
            if c.cond(2) { pc = 0x5298e8; continue; }
            pc = 0x52989d; continue;
            }
            0x52989d => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5298a4);
            { let args = []; c.r[0] = host.call(m, 0x4f73d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.set(0,32,0x3u32);
            pc = 0x5298ac; continue;
            }
            0x5298ac => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3c),32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x5298e2; continue; }
            pc = 0x5298b4; continue;
            }
            0x5298b4 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x50),32));
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            pc = 0x5298e5; continue;
            }
            0x5298e2 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x5298e5; continue;
            }
            0x5298e5 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(9) { pc = 0x5298ac; continue; }
            pc = 0x5298e8; continue;
            }
            0x5298e8 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x529858; continue; }
            pc = 0x5298fa; continue;
            }
            0x5298fa => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.set(3,32,0xfffffff8u32);
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(6,32,c.r[0].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x529912; continue;
            }
            0x529912 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(c.r[5]).wrapping_add(0x28),8));
            { let a = c.r[6].wrapping_add(0xfffffff8); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[5]).wrapping_add(0x28),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[5]).wrapping_add(0x14),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x529977; continue; }
            pc = 0x52993f; continue;
            }
            0x52993f => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x52994a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x529953);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0xfffffffc); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x19),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x39c),8));
            { let a = c.r[6]; wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x1c),8));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1d),8));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x68),32));
            pc = 0x5299c6; continue;
            }
            0x529977 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x6f1),8));
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(0,8,c.get(2,8));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x529991; continue; }
            pc = 0x52998b; continue;
            }
            0x52998b => {
            c.set(2,8,rm(m,0x6de52du32,8));
            pc = 0x529991; continue;
            }
            0x529991 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x6e0),8));
            { let a = c.r[6].wrapping_add(0xfffffffc); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x6de52eu32,8));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,8); let r = c.add(a,0x6u32,8); c.set(2,8,r); }
            { let a = c.r[6]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x6d4),8));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x6d8),8));
            pc = 0x5299c6; continue;
            }
            0x5299c6 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(c.r[5]).wrapping_add(0x14),8));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            c.set(2,32,c.r[3].wrapping_add(c.r[6]));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(2,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x529912; continue; }
            pc = 0x5299e4; continue;
            }
            0x5299e4 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x54u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn get_alderman_epithet_index(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x529dc0;
    loop {
        match pc {
            0x529dc0 => {
            c.set(0,8,rm(m,0x6de52du32,8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x529def; continue; }
            pc = 0x529dd3; continue;
            }
            0x529dd3 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,8,c.get(0,8));
            c.push(m,c.get(2,32));
            c.push(m, 0x529de2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x529de9);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x529def => {
            { let a = c.get(1,32); let r = c.or(a,0xffffffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x6de52eu32,8));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x6e0),8));
            c.set(0,32,c.get(1,32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_find_council_meeting_vote_state(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4da060;
    loop {
        match pc {
            0x4da060 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[0]; wm(m,a,8,0x0u32); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(7,32); let r = c.or(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.r[2]; wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(0xc),16),16); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0x1u32); }
            if c.cond(3) { pc = 0x4da137; continue; }
            pc = 0x4da094; continue;
            }
            0x4da094 => {
            c.set(5,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            pc = 0x4da099; continue;
            }
            0x4da099 => {
            c.set(6,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2],16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(6,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = rm(m,c.r[6],32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x4da0b1; continue; }
            pc = 0x4da0aa; continue;
            }
            0x4da0aa => {
            { let a = c.r[2]; wm(m,a,16,0xffffu32); }
            pc = 0x4da0c9; continue;
            }
            0x4da0b1 => {
            { let a = rm(m,c.r[6].wrapping_add(0x6),16); c.sub(a,0x2eu32,16); }
            if c.cond(5) { pc = 0x4da0c2; continue; }
            pc = 0x4da0b8; continue;
            }
            0x4da0b8 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(4) { pc = 0x4da0d9; continue; }
            pc = 0x4da0c2; continue;
            }
            0x4da0c2 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.r[2]; wm(m,a,16,c.get(0,16)); }
            pc = 0x4da0c9; continue;
            }
            0x4da0c9 => {
            c.set(0,16,rm(m,c.r[2],16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(0xc),16),16); }
            if c.cond(2) { pc = 0x4da099; continue; }
            pc = 0x4da0d2; continue;
            }
            0x4da0d2 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4da0d9 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(7,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4da137; continue; }
            pc = 0x4da0e6; continue;
            }
            0x4da0e6 => {
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4da0f7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4da137; continue; }
            pc = 0x4da0fe; continue;
            }
            0x4da0fe => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x4da125; continue; }
            pc = 0x4da104; continue;
            }
            0x4da104 => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4da10f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4da120; continue; }
            pc = 0x4da116; continue;
            }
            0x4da116 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x4da120; continue;
            }
            0x4da120 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x4da104; continue; }
            pc = 0x4da125; continue;
            }
            0x4da125 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0xc),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x13),8));
            { let a = c.get(1,8); c.and(a,c.get(0,8),8); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            pc = 0x4da137; continue;
            }
            0x4da137 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_record_is_siege_active(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x633ae0;
    loop {
        match pc {
            0x633ae0 => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x633aec; continue; }
            pc = 0x633ae7; continue;
            }
            0x633ae7 => {
            { let args = []; c.r[0] = host.call(m, 0x629c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x633aec => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_get_armament_lot_price(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5285b0;
    loop {
        match pc {
            0x5285b0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); c.sub(a,0x6u32,32); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,8,0x2u32); }
            { let a = c.r[4].wrapping_add(0x5); wm(m,a,8,0x3u32); }
            { let a = c.r[4].wrapping_add(0x6); wm(m,a,8,0x5u32); }
            { let a = c.r[4].wrapping_add(0x7); wm(m,a,8,0x8u32); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,0xbu32); }
            { let a = c.r[4].wrapping_add(0x9); wm(m,a,8,0xeu32); }
            if c.cond(3) { pc = 0x528614; continue; }
            pc = 0x5285db; continue;
            }
            0x5285db => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x2c0),8));
            c.set(1,32,rm(m,0x6de4a4u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,0x5u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(c.r[6]).wrapping_add(0x4),8));
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x528614 => {
            c.set(0,32,sx(rm(m,0x673a78u32,8),8)&0xffffffffu32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn storage_get_weapon_piece_count(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x52e030;
    loop {
        match pc {
            0x52e030 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            if c.cond(2) { pc = 0x52e042; continue; }
            pc = 0x52e039; continue;
            }
            0x52e039 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x2bc),32));
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x52e042 => {
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672cb4),16));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x124),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_record_get_displeasure_level(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5f3750;
    loop {
        match pc {
            0x5f3750 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x1018),16));
            { let a = c.get(0,16); c.sub(a,0x3fffu32,16); }
            if c.cond(6) { pc = 0x5f3763; continue; }
            pc = 0x5f375d; continue;
            }
            0x5f375d => {
            c.set(0,32,0x4u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5f3763 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0xcu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_record_get_urgent_demanded_ware(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x635660;
    loop {
        match pc {
            0x635660 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,8,rm(m,0x6de4a1u32,8));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0xffu32); }
            c.set(6,32,c.r[7].wrapping_add(0x1010));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x4u32); }
            pc = 0x635684; continue;
            }
            0x635684 => {
            { let a = rm(m,c.r[6].wrapping_add(0x4),8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x6356b2; continue; }
            pc = 0x63568a; continue;
            }
            0x63568a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            c.set(2,8,c.get(3,8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(1,8)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let b = c.get(5,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x6356b2; continue; }
            pc = 0x6356ac; continue;
            }
            0x6356ac => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            pc = 0x6356b2; continue;
            }
            0x6356b2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x635684; continue; }
            pc = 0x6356be; continue;
            }
            0x6356be => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_record_find_delivery_agreement(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x636590;
    loop {
        match pc {
            0x636590 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x101c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6365aa; continue; }
            pc = 0x63659a; continue;
            }
            0x63659a => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x4),8));
            pc = 0x63659e; continue;
            }
            0x63659e => {
            { let a = rm(m,c.r[0].wrapping_add(0x3),8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x6365aa; continue; }
            pc = 0x6365a3; continue;
            }
            0x6365a3 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x63659e; continue; }
            pc = 0x6365aa; continue;
            }
            0x6365aa => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_get_overland_route_partner_index(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x525630;
    loop {
        match pc {
            0x525630 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x2c0),8));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x76c),32));
            { let a = c.get(1,32); let r = c.and(a,0x8000u32,32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x6de4b8),8));
            { let a = c.get(1,32); c.sub(a,0x8000u32,32); }
            { let a = c.r[4].wrapping_add(0xb); wm(m,a,8,0xffu32); }
            if c.cond(5) { pc = 0x5256d5; continue; }
            pc = 0x52565c; continue;
            }
            0x52565c => {
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            pc = 0x525662; continue;
            }
            0x525662 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672bf8),8));
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x525686; continue; }
            pc = 0x525678; continue;
            }
            0x525678 => {
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x672bf9),8));
            c.set(2,8,0x7u32);
            { let a = c.r[4].wrapping_add(0xb); wm(m,a,8,c.get(0,8)); }
            pc = 0x52569c; continue;
            }
            0x525686 => {
            { let a = rm(m,c.r[0].wrapping_add(0x672bf9),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x525698; continue; }
            pc = 0x52568e; continue;
            }
            0x52568e => {
            c.set(0,8,c.get(1,8));
            c.set(2,8,0x7u32);
            { let a = c.r[4].wrapping_add(0xb); wm(m,a,8,c.get(0,8)); }
            pc = 0x52569c; continue;
            }
            0x525698 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xb),8));
            pc = 0x52569c; continue;
            }
            0x52569c => {
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x7u32,8); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            if c.cond(2) { pc = 0x525662; continue; }
            pc = 0x5256a7; continue;
            }
            0x5256a7 => {
            { let a = c.get(0,8); c.sub(a,0x28u32,8); }
            if c.cond(3) { pc = 0x5256d5; continue; }
            pc = 0x5256ab; continue;
            }
            0x5256ab => {
            c.set(6,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(6) { pc = 0x5256d5; continue; }
            pc = 0x5256b7; continue;
            }
            0x5256b7 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x5256b9; continue;
            }
            0x5256b9 => {
            { let a = c.get(0,8); c.sub(a,rm(m,c.r[1].wrapping_add(0x6de4b8),8),8); }
            if c.cond(4) { pc = 0x5256dd; continue; }
            pc = 0x5256c1; continue;
            }
            0x5256c1 => {
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x5256b9; continue; }
            pc = 0x5256d5; continue;
            }
            0x5256d5 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.or(a,0xffu32,8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5256dd => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,c.get(2,8));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "tavern_content_window_open_page" => tavern_content_window_open_page(m, host, regs),
        "church_get_donation_tier" => church_get_donation_tier(m, host, regs),
        "church_content_window_update_feed_poor_page" => church_content_window_update_feed_poor_page(m, host, regs),
        "church_content_window_update_redecorate_page" => church_content_window_update_redecorate_page(m, host, regs),
        "church_content_window_update_extension_page" => church_content_window_update_extension_page(m, host, regs),
        "town_build_loan_applicant_operations" => town_build_loan_applicant_operations(m, host, regs),
        "town_get_guild_entry_fee" => town_get_guild_entry_fee(m, host, regs),
        "scheduled_tasks_collect_visible_auctions" => scheduled_tasks_collect_visible_auctions(m, host, regs),
        "merchant_render_world_fog_map" => merchant_render_world_fog_map(m, host, regs),
        "ship_reset_loadout_for_pirate" => ship_reset_loadout_for_pirate(m, host, regs),
        "scheduled_tasks_find_council_meeting_for_town" => scheduled_tasks_find_council_meeting_for_town(m, host, regs),
        "merchant_get_mayor_candidacy_flag" => merchant_get_mayor_candidacy_flag(m, host, regs),
        "name_bank_get_given_name" => name_bank_get_given_name(m, host, regs),
        "name_bank_get_family_name" => name_bank_get_family_name(m, host, regs),
        "town_collect_mayor_candidates" => town_collect_mayor_candidates(m, host, regs),
        "town_get_next_mayor_election_date" => town_get_next_mayor_election_date(m, host, regs),
        "town_get_mayor_epithet_index" => town_get_mayor_epithet_index(m, host, regs),
        "scheduled_tasks_find_hanse_diet_vote" => scheduled_tasks_find_hanse_diet_vote(m, host, regs),
        "scheduled_tasks_find_alderman_election" => scheduled_tasks_find_alderman_election(m, host, regs),
        "collect_alderman_candidates" => collect_alderman_candidates(m, host, regs),
        "get_alderman_epithet_index" => get_alderman_epithet_index(m, host, regs),
        "scheduled_tasks_find_council_meeting_vote_state" => scheduled_tasks_find_council_meeting_vote_state(m, host, regs),
        "siege_record_is_siege_active" => siege_record_is_siege_active(m, host, regs),
        "town_get_armament_lot_price" => town_get_armament_lot_price(m, host, regs),
        "storage_get_weapon_piece_count" => storage_get_weapon_piece_count(m, host, regs),
        "siege_record_get_displeasure_level" => siege_record_get_displeasure_level(m, host, regs),
        "siege_record_get_urgent_demanded_ware" => siege_record_get_urgent_demanded_ware(m, host, regs),
        "siege_record_find_delivery_agreement" => siege_record_find_delivery_agreement(m, host, regs),
        "town_get_overland_route_partner_index" => town_get_overland_route_partner_index(m, host, regs),
        _ => return None,
    })
}
