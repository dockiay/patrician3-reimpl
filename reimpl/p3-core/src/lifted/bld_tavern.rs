//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn tavern_content_window_show(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5cc120;
    loop {
        match pc {
            0x5cc120 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x666c60u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x74u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x5cc148);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cc152);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x60),32); c.push(m, 0x5cc162); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x5cc172); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,0x64u32);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x5cc1a7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5cc1ae);
            { let args = []; c.r[0] = host.call(m, 0x462390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[6].wrapping_add(0x94));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5cc1bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[3],32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x48),32); c.push(m, 0x5cc1c5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xfffffffd));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5cc1d9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xfffffffd));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x5cc1eb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cc1f7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bc8),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1bcc),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            c.set(3,32,c.r[6].wrapping_add(0x17c));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5cc223);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5cc22f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cc23b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0x2188));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5cc249);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(2,32); let r = c.add(a,0x14u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5cc262); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cc26e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cc27a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x2184));
            c.push(m, 0x5cc285);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(3,32,c.r[6].wrapping_add(0x9a4));
            c.push(m,0x64u32);
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc324),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[6].wrapping_add(0x1bfc); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x17cu32,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x1c30); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            c.set(1,32,c.get(3,32));
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5cc2d0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.get(5,32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(2,32); let r = c.sub(a,0x6eu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5cc2e1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa8c),32));
            c.set(1,32,c.r[6].wrapping_add(0xa8c));
            { let a = c.get(0,32); let r = c.add(a,0x17cu32,32); c.set(0,32,r); }
            c.push(m,0x64u32);
            { let a = c.get(5,32); let r = c.add(a,0x6eu32,32); c.set(5,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x5cc2ff); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cc307);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0xa8c));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5cc315);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cc321); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cc32e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xb74),32));
            c.set(3,32,c.r[6].wrapping_add(0xb74));
            { let a = c.get(1,32); let r = c.add(a,0x122u32,32); c.set(1,32,r); }
            c.push(m,0x64u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x44),32); c.push(m, 0x5cc34b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x5cc362); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc5c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(5,32,c.r[6].wrapping_add(0xc5c));
            { let a = c.get(2,32); let r = c.add(a,0x122u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.push(m,0x64u32);
            c.push(m,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xa));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5cc397); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cc39f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cc3a7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cc3b3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cc3c0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1448),32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x5cc3cf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1448),32));
            c.set(1,32,c.r[6].wrapping_add(0x1448));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5cc3df); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0xfffffff6u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5cc3f1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x1448));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x5cc401); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x12b8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc5c),32));
            c.set(3,32,c.r[6].wrapping_add(0xc5c));
            c.push(m,0x64u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x5cc41a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5cc422); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5cc42d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(3,32,c.r[6].wrapping_add(0x12b8));
            c.set(0,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xa));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x5cc441); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,c.r[6].wrapping_add(0x1448));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5cc44f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cc457);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cc464); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cc470); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5cc479);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x5cc47e);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x34));
            c.fstp32(m,c.r[4].wrapping_add(0x34));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x34),4));
            { let v = c.ld(m,0x66cb34u32,4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x66dcd8u32,4); c.fop('*',v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0x1ca0); wm(m,a,32,c.get(1,32)); }
            { let v = c.ld(m,0x679ca0u32,8); c.fop('+',v); }
            c.push(m, 0x5cc4b7);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x1c3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(2,16,rm(m,0x6dfc14u32,16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5cc4e1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5269a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1c34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(5,32,c.r[6].wrapping_add(0x1c00));
            c.push(m,0x10u32);
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x98); wm(m,a,32,0x0u32); }
            c.push(m, 0x5cc50b);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x1c64));
            c.set(1,32,c.r[6].wrapping_add(0x1c54));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1ca4),8));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.set(2,32,c.r[6].wrapping_add(0x1c44));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5cc549);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x526470, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(5,32,c.r[6].wrapping_add(0x264));
            { let a = c.get(0,32); let r = c.add(a,0x15eu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x4u32); }
            pc = 0x5cc56b; continue;
            }
            0x5cc56b => {
            c.set(3,32,c.r[6].wrapping_add(0x1b8c));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5cc578);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cc583);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(5,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,0x64u32);
            c.set(3,32,c.r[6].wrapping_add(0x1b8c));
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cc5a8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cc5b5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x1b8c));
            c.push(m, 0x5cc5cc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x1b8c));
            c.push(m, 0x5cc5e3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x5cc5fa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,c.r[3].wrapping_add(0x3a0));
            c.push(m,0x64u32);
            c.set(3,32,c.r[6].wrapping_add(0x1b8c));
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(2,32));
            c.push(m,0x2u32);
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cc61c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(3,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x5cc627);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5cc632); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(3,32,c.r[5].wrapping_add(0x3a0));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5cc648); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cc650);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cc658);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cc665); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cc671); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xfffffff0),32));
            c.push(m,c.get(2,32));
            c.push(m,0x6c657cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5cc688);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5cc697);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,c.get(2,32));
            c.push(m,0x6c6580u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5cc6ad);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5cc6bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0xe8u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5cc56b; continue; }
            pc = 0x5cc6dc; continue;
            }
            0x5cc6dc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bd8),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1bd4),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0xd44));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x1ca6); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cc70f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5cc71b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5cc722); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(5,32,c.r[6].wrapping_add(0xe2c));
            c.push(m,c.get(5,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x2));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5cc73c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x5cc749); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5cc751); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x2));
            c.set(0,32,c.r[6].wrapping_add(0xf14));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5cc76b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf14),32));
            c.set(1,32,c.r[6].wrapping_add(0xf14));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x5cc77f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf14),32));
            c.set(1,32,c.r[6].wrapping_add(0xf14));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5cc78e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x2));
            c.set(0,32,c.r[6].wrapping_add(0xffc));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5cc7a8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xffc),32));
            c.set(1,32,c.r[6].wrapping_add(0xffc));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x5cc7bc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cc7c8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cc7d5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf14),32));
            c.set(1,32,c.r[6].wrapping_add(0xf14));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cc7e9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xffc),32));
            c.set(1,32,c.r[6].wrapping_add(0xffc));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cc7fd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.r[6].wrapping_add(0x1ca4); wm(m,a,8,0x0u32); }
            c.push(m,0x1u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x124),32); c.push(m, 0x5cc810); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x5cc81d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf14),32));
            c.set(1,32,c.r[6].wrapping_add(0xf14));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x124),32); c.push(m, 0x5cc831); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xffc),32));
            c.set(1,32,c.r[6].wrapping_add(0xffc));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x124),32); c.push(m, 0x5cc845); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10e4),32));
            c.set(5,32,c.r[6].wrapping_add(0x10e4));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5cc856); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x17c),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x17c));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x5cc878); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cc884);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x5cc891); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cc89e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5cc8a9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(5,32,c.r[6].wrapping_add(0x11cc));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(3,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m,c.get(5,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5cc8cb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5cc8d8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cc8e6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(0,32,rm(m,0x6b1470u32,32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,8,0x1u32); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5cc915);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5cc924);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x2154));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5cc947);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x526b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcdd0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x58));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcdd0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x5cc95a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.r[6].wrapping_add(0x1c7c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1be8),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5cc977; continue; }
            pc = 0x5cc96e; continue;
            }
            0x5cc96e => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5cc974);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5cc977; continue;
            }
            0x5cc977 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bec),32));
            { let a = c.r[6].wrapping_add(0x1be8); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5cc990; continue; }
            pc = 0x5cc987; continue;
            }
            0x5cc987 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5cc98d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5cc990; continue;
            }
            0x5cc990 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            { let a = c.r[6].wrapping_add(0x1bec); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5cc9a7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x531470, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1be8); wm(m,a,32,c.get(0,32)); }
            c.set(5,16,rm(m,c.r[0],16));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,8,0x5u32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,c.r[5].wrapping_mul(4).wrapping_add(0x2));
            c.push(m,c.get(2,32));
            c.push(m, 0x5cc9e8);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(3,32,c.get(0,32));
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x38),16));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            { let a = c.r[3]; wm(m,a,16,c.get(0,16)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x1u32); }
            if c.cond(14) { pc = 0x5ccbf0; continue; }
            pc = 0x5cca07; continue;
            }
            0x5cca07 => {
            c.set(1,32,c.r[5].wrapping_add(c.r[5]));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x2u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x5cca17; continue;
            }
            0x5cca17 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1be8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2]),16));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5cca32);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xe4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5cca44);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xe8),32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cca54);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x5cca5d);
            { let args = []; c.r[0] = host.call(m, 0x64f709, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x5cca66);
            { let args = []; c.r[0] = host.call(m, 0x64f709, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,0x1u32);
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(14) { pc = 0x5ccb91; continue; }
            pc = 0x5cca78; continue;
            }
            0x5cca78 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(c.r[5].wrapping_mul(2)),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5cca89);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cca9d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x5ccab1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m, 0x5ccaba);
            { let args = []; c.r[0] = host.call(m, 0x64f709, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x5ccac3);
            { let args = []; c.r[0] = host.call(m, 0x64f709, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ccad2);
            { let args = []; c.r[0] = host.call(m, 0x639cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x5ccb03; continue; }
            pc = 0x5ccad9; continue;
            }
            0x5ccad9 => {
            if c.cond(5) { pc = 0x5ccaf1; continue; }
            pc = 0x5ccadb; continue;
            }
            0x5ccadb => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5ccaea);
            { let args = []; c.r[0] = host.call(m, 0x639cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x5ccb2a; continue; }
            pc = 0x5ccaf1; continue;
            }
            0x5ccaf1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x5cca78; continue; }
            pc = 0x5ccafe; continue;
            }
            0x5ccafe => {
            pc = 0x5ccb91; continue;
            }
            0x5ccb03 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[3].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[3].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x2));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ccb1e);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[5]));
            c.set(0,32,c.r[3].wrapping_add(c.r[2].wrapping_mul(2)));
            pc = 0x5ccb4f; continue;
            }
            0x5ccb2a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[3].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[3].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x2));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5ccb45);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[5]));
            c.set(0,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(2)));
            pc = 0x5ccb4f; continue;
            }
            0x5ccb4f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_add(0x1));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(2)));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ccb67);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1be8),32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1]),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[3].wrapping_add(c.r[5].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1be8),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1]),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.r[1]; wm(m,a,16,c.get(0,16)); }
            pc = 0x5ccb91; continue;
            }
            0x5ccb91 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x5ccbc3; continue; }
            pc = 0x5ccb99; continue;
            }
            0x5ccb99 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1be8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[2]),16));
            { let a = c.r[3].wrapping_add(c.r[5].wrapping_mul(2)); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1be8),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[5]),16));
            { let a = c.r[3].wrapping_add(c.r[2].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            pc = 0x5ccbc3; continue;
            }
            0x5ccbc3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,0x2u32);
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,c.r[2].wrapping_add(0x1));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x5cca17; continue; }
            pc = 0x5ccbf0; continue;
            }
            0x5ccbf0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1be8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5ccc03; continue; }
            pc = 0x5ccbfa; continue;
            }
            0x5ccbfa => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5ccc00);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5ccc03; continue;
            }
            0x5ccc03 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x1be8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1c9c); wm(m,a,32,0x1eu32); }
            c.set(0,16,rm(m,c.r[3],16));
            { let a = c.r[6].wrapping_add(0x1c80); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5ccc26);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x1bec); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c80),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            if c.cond(14) { pc = 0x5ccd94; continue; }
            pc = 0x5ccc49; continue;
            }
            0x5ccc49 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1be8),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(2)).wrapping_add(0x2),16));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5ccc61);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x46c),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x5ccc7b; continue; }
            pc = 0x5ccc6d; continue;
            }
            0x5ccc6d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bec),32));
            { let a = c.r[0].wrapping_add(c.r[5].wrapping_mul(2)); wm(m,a,16,0x1f4u32); }
            pc = 0x5cccb9; continue;
            }
            0x5ccc7b => {
            c.set(0,32,0x68db8badu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xcu32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x1));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xfde8u32,32); }
            if c.cond(6) { pc = 0x5cccaf; continue; }
            pc = 0x5ccca1; continue;
            }
            0x5ccca1 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1bec),32));
            { let a = c.r[2].wrapping_add(c.r[5].wrapping_mul(2)); wm(m,a,16,0xfde8u32); }
            pc = 0x5cccb9; continue;
            }
            0x5cccaf => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1bec),32));
            { let a = c.r[1].wrapping_add(c.r[5].wrapping_mul(2)); wm(m,a,16,c.get(0,16)); }
            pc = 0x5cccb9; continue;
            }
            0x5cccb9 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5cccc0);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x1c88),32),32); }
            if c.cond(5) { pc = 0x5cccdf; continue; }
            pc = 0x5cccd0; continue;
            }
            0x5cccd0 => {
            c.set(2,32,rm(m,0x66e104u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5cccdc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5cccdf; continue;
            }
            0x5cccdf => {
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x1c84),32),32); }
            if c.cond(5) { pc = 0x5cccf5; continue; }
            pc = 0x5ccce7; continue;
            }
            0x5ccce7 => {
            c.set(0,32,rm(m,0x66e108u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cccf2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5cccf5; continue;
            }
            0x5cccf5 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1a),8));
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x19),8));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(c.r[1]).wrapping_add(0x39c),8));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0xe4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xe8),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0ef8),32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a0c18),32));
            c.set(1,32,rm(m,0x69d960u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x30));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5ccd61);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcdd0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5ccd73);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x5ccd81; continue; }
            pc = 0x5ccd7b; continue;
            }
            0x5ccd7b => {
            c.set(2,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            pc = 0x5ccd81; continue;
            }
            0x5ccd81 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c80),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            if c.cond(12) { pc = 0x5ccc49; continue; }
            pc = 0x5ccd94; continue;
            }
            0x5ccd94 => {
            { let a = rm(m,c.r[6].wrapping_add(0x1c80),32); c.sub(a,0x6u32,32); }
            if c.cond(14) { pc = 0x5ccda0; continue; }
            pc = 0x5ccd9d; continue;
            }
            0x5ccd9d => {
            { let a = c.get(2,32); let r = c.add(a,0x1eu32,32); c.set(2,32,r); }
            pc = 0x5ccda0; continue;
            }
            0x5ccda0 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x1c9c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xfffffff3));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x3c));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c7c),32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,c.r[4].wrapping_add(0x74));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[4].wrapping_add(0x5c));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(1,32)); }
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x5ccdf9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c7c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c80),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.set(3,32,c.r[6].wrapping_add(0x17dc));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x60));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5cce1e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c80),32));
            c.set(1,32,c.r[6].wrapping_add(0x17d4));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cce30);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cce3c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x1c88); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1c84); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5cce50);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x38));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            { let v = c.ld(m,0x66cb34u32,4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x66fd54u32,4); c.fop('*',v); }
            c.push(m, 0x5cce79);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x1c90); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x5cce9b);
            { let args = []; c.r[0] = host.call(m, 0x526650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x2170); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5ccea6);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x38));
            c.fstp32(m,c.r[4].wrapping_add(0x38));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x38),4));
            { let v = c.ld(m,0x66cb34u32,4); c.fop('/',v); }
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,0x66fd54u32,4); c.fop('*',v); }
            c.push(m, 0x5ccecf);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x1ca5); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6cc380u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ccee8);
            { let args = []; c.r[0] = host.call(m, 0x4bbb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1e80));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.push(m,0x64u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(5,32),32); c.set(3,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x2010),32));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5ccf15); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x70),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x2010));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x5ccf36); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1d98),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1d98));
            c.push(m,0x64u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xa));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x5ccf58); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1cb0),32));
            c.set(5,32,c.r[6].wrapping_add(0x1cb0));
            c.push(m,0x64u32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5ccf70); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xa));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x64),32); c.push(m, 0x5ccf8d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1e80),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1d98),32));
            c.set(1,32,c.r[6].wrapping_add(0x1d98));
            c.push(m,0x64u32);
            c.push(m,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x5ccfa9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5ccfb3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(5,32,c.r[6].wrapping_add(0x1e80));
            c.set(2,32,c.r[3].wrapping_add(c.r[1]).wrapping_add(0x14));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x5ccfd6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(3,32,c.r[6].wrapping_add(0x2010));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x5ccfe4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5ccfec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x1cb0));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ccffa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x1d98));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5cd008);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cd014); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cd021); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1cb0),32));
            c.set(1,32,c.r[6].wrapping_add(0x1cb0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5cd035); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1d98),32));
            c.set(1,32,c.r[6].wrapping_add(0x1d98));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5cd049); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c6584u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x5cd057);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x90); wm(m,a,8,0x6u32); }
            c.push(m, 0x5cd06b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d2ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,8,0x5u32);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x5cd07d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6c6586u32);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x5cd08b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,8,0x7u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5cd09f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x38));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x5cd0af);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x1bfc),16));
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5cd0ca);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5cd0e0; continue; }
            pc = 0x5cd0ce; continue;
            }
            0x5cd0ce => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2175),8));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0x2150); wm(m,a,32,c.get(2,32)); }
            pc = 0x5cd0e0; continue;
            }
            0x5cd0e0 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5cd0f0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xe),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x5cd167; continue; }
            pc = 0x5cd108; continue;
            }
            0x5cd108 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1bfc),32));
            pc = 0x5cd10e; continue;
            }
            0x5cd10e => {
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x136),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x39),8));
            { let a = c.get(0,8); c.set(0,8,!a & 0xffu32); }
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5cd151; continue; }
            pc = 0x5cd137; continue;
            }
            0x5cd137 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2175),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x54),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2150),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x2150); wm(m,a,32,c.get(0,32)); }
            pc = 0x5cd151; continue;
            }
            0x5cd151 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x5cd10e; continue; }
            pc = 0x5cd167; continue;
            }
            0x5cd167 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2175),8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2150),32));
            if c.cond(4) { pc = 0x5cd192; continue; }
            pc = 0x5cd17f; continue;
            }
            0x5cd17f => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            pc = 0x5cd1a3; continue;
            }
            0x5cd192 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            pc = 0x5cd1a3; continue;
            }
            0x5cd1a3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[6].wrapping_add(0x2150); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x1bf4); wm(m,a,32,c.get(7,32)); }
            c.set(2,8,rm(m,0x66e100u32,8));
            c.push(m,0x64u32);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x1624),32));
            c.set(2,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xa));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x1624));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x41));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x5cd1e3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x66e100u32,8));
            c.push(m,0x64u32);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x16fc),32));
            c.set(1,32,c.r[6].wrapping_add(0x16fc));
            c.set(2,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x3e));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.push(m,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x3d));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x5cd214); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,8,0x4u32); }
            c.push(m, 0x5cd225);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,8,0x3u32); }
            c.push(m, 0x5cd236);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,8,0x2u32); }
            c.push(m, 0x5cd247);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,8,0x1u32); }
            c.push(m, 0x5cd258);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,8,0x0u32); }
            c.push(m, 0x5cd269);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x5cd279);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x84),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x80u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_find_tavern_captain_for_merchant(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5269a0;
    loop {
        match pc {
            0x5269a0 => {
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
            if c.cond(3) { pc = 0x5269f5; continue; }
            pc = 0x5269bd; continue;
            }
            0x5269bd => {
            c.set(2,32,rm(m,0x6dd7a0u32,32));
            c.set(7,32,c.get(6,32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5269d1);
            { let args = []; c.r[0] = host.call(m, 0x4fe150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5269e1; continue; }
            pc = 0x5269d5; continue;
            }
            0x5269d5 => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xf),8));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x10),16),16); }
            if c.cond(4) { pc = 0x526a42; continue; }
            pc = 0x5269e1; continue;
            }
            0x5269e1 => {
            c.set(0,16,rm(m,0x6dd892u32,16));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[7],16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,c.get(0,16));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5269bd; continue; }
            pc = 0x5269f5; continue;
            }
            0x5269f5 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(6,16,rm(m,c.r[3].wrapping_add(0x82e),16));
            c.set(2,16,c.get(0,16));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x526a37; continue; }
            pc = 0x526a07; continue;
            }
            0x526a07 => {
            c.set(3,8,0xffu32);
            pc = 0x526a09; continue;
            }
            0x526a09 => {
            c.set(2,32,rm(m,0x6dd7a0u32,32));
            c.set(7,32,c.get(6,32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x526a1d);
            { let args = []; c.r[0] = host.call(m, 0x4fe150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x526a26; continue; }
            pc = 0x526a21; continue;
            }
            0x526a21 => {
            { let a = rm(m,c.r[7].wrapping_add(0xf),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x526a42; continue; }
            pc = 0x526a26; continue;
            }
            0x526a26 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(6,16,rm(m,c.r[7],16));
            c.set(0,16,rm(m,0x6dd892u32,16));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x526a09; continue; }
            pc = 0x526a37; continue;
            }
            0x526a37 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,0xffffu32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x526a42 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_calc_weapon_trade_table(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x526470;
    loop {
        match pc {
            0x526470 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,c.r[5].wrapping_add(0x54));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(6,32),32); c.set(3,32,r); }
            pc = 0x5264a8; continue;
            }
            0x5264a8 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[3].wrapping_add(c.r[6]));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5264c1; continue; }
            pc = 0x5264b3; continue;
            }
            0x5264b3 => {
            { let a = c.r[6]; wm(m,a,32,0xau32); }
            { let a = c.r[1]; wm(m,a,32,0xau32); }
            pc = 0x526516; continue;
            }
            0x5264c1 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x5264d4; continue; }
            pc = 0x5264c6; continue;
            }
            0x5264c6 => {
            { let a = c.r[6]; wm(m,a,32,0x32u32); }
            { let a = c.r[1]; wm(m,a,32,0x32u32); }
            pc = 0x526516; continue;
            }
            0x5264d4 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x5264e7; continue; }
            pc = 0x5264d9; continue;
            }
            0x5264d9 => {
            { let a = c.r[6]; wm(m,a,32,0x1f4u32); }
            { let a = c.r[1]; wm(m,a,32,0x1f4u32); }
            pc = 0x526516; continue;
            }
            0x5264e7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,0x66666667u32);
            c.set(2,32,rm(m,c.r[2],32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[7]));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x526516; continue;
            }
            0x526516 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,0x66666667u32);
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[2],32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x52653d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5263d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5265b0; continue; }
            pc = 0x526543; continue;
            }
            0x526543 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x2d4),32));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x673784),8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,0x1f4u32,32); }
            if c.cond(13) { pc = 0x526580; continue; }
            pc = 0x52656b; continue;
            }
            0x52656b => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            pc = 0x526588; continue;
            }
            0x526580 => {
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            pc = 0x526588; continue;
            }
            0x526588 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x673780),8));
            c.set(7,32,c.get(2,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x673788),8));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x5265a8; continue; }
            pc = 0x5265a0; continue;
            }
            0x5265a0 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x5265b6; continue;
            }
            0x5265a8 => {
            { let a = c.r[6]; wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x5265b6; continue;
            }
            0x5265b0 => {
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            pc = 0x5265b6; continue;
            }
            0x5265b6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            if c.cond(2) { pc = 0x5264a8; continue; }
            pc = 0x5265d6; continue;
            }
            0x5265d6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_create_tavern_ware_offer(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x526650;
    loop {
        match pc {
            0x526650 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffu32); }
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(6,32,c.r[1].wrapping_add(0x4f4));
            c.set(5,32,0x1u32);
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            pc = 0x526679; continue;
            }
            0x526679 => {
            { let a = c.get(5,32); c.and(a,0x47fafu32,32); }
            if c.cond(4) { pc = 0x526697; continue; }
            pc = 0x526681; continue;
            }
            0x526681 => {
            c.set(7,32,rm(m,c.r[1],32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,0x9c40u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x526697; continue; }
            pc = 0x526691; continue;
            }
            0x526691 => {
            c.set(3,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x526697; continue;
            }
            0x526697 => {
            { let a = c.get(5,32); let r = c.add(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x10u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x526679; continue; }
            pc = 0x5266a5; continue;
            }
            0x5266a5 => {
            { let a = rm(m,c.r[4].wrapping_add(0x14),32); c.sub(a,0x14u32,32); }
            if c.cond(12) { pc = 0x5266b6; continue; }
            pc = 0x5266ac; continue;
            }
            0x5266ac => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5266b6 => {
            c.set(5,32,rm(m,0x6dd888u32,32));
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x526746; continue; }
            pc = 0x5266d0; continue;
            }
            0x5266d0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x2c0),8));
            pc = 0x5266da; continue;
            }
            0x5266da => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(6,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x39),8));
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x52672d; continue; }
            pc = 0x5266f6; continue;
            }
            0x5266f6 => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x52672d; continue; }
            pc = 0x526706; continue;
            }
            0x526706 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x526716);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x2c0),8));
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x52672d; continue; }
            pc = 0x526727; continue;
            }
            0x526727 => {
            { let a = rm(m,c.r[0].wrapping_add(0x8),8); c.and(a,0x4u32,8); }
            if c.cond(5) { pc = 0x52673f; continue; }
            pc = 0x52672d; continue;
            }
            0x52672d => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x5266da; continue; }
            pc = 0x52673d; continue;
            }
            0x52673d => {
            pc = 0x526746; continue;
            }
            0x52673f => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[6],8));
            pc = 0x526746; continue;
            }
            0x526746 => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x526765; continue; }
            pc = 0x526758; continue;
            }
            0x526758 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(7,32,0x7530u32);
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x2c0),8));
            pc = 0x526765; continue;
            }
            0x526765 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x4f4),32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x52677f; continue; }
            pc = 0x52677d; continue;
            }
            0x52677d => {
            c.set(7,32,c.get(0,32));
            pc = 0x52677f; continue;
            }
            0x52677f => {
            c.push(m,0x10u32);
            c.push(m, 0x526786);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x52686c; continue; }
            pc = 0x526793; continue;
            }
            0x526793 => {
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0xe); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5267e8; continue; }
            pc = 0x5267b6; continue;
            }
            0x5267b6 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(7,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,16); c.sub(a,0x1u32,16); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,c.get(2,16)); }
            if c.cond(3) { pc = 0x5267d7; continue; }
            pc = 0x5267d1; continue;
            }
            0x5267d1 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,0x1u32); }
            pc = 0x5267d7; continue;
            }
            0x5267d7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            pc = 0x52681b; continue;
            }
            0x5267e8 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(7,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,16); c.sub(a,0x1u32,16); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,c.get(2,16)); }
            if c.cond(3) { pc = 0x526809; continue; }
            pc = 0x526803; continue;
            }
            0x526803 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,0x1u32); }
            pc = 0x526809; continue;
            }
            0x526809 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            pc = 0x52681b; continue;
            }
            0x52681b => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x2c0),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xe),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x526839);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x2c0),8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de3d8u32);
            c.push(m, 0x526852);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x52e430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,0x6dfc14u32,8));
            { let a = c.r[6].wrapping_add(0xd); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[6].wrapping_add(0xa); wm(m,a,16,0xffffu32); }
            pc = 0x52686c; continue;
            }
            0x52686c => {
            c.set(0,32,c.get(6,32));
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

pub fn town_build_tavern_news_message(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x526b40;
    loop {
        match pc {
            0x526b40 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x663298u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x34u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x2c0),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,0x3u32);
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x526ca4; continue; }
            pc = 0x526b99; continue;
            }
            0x526b99 => {
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.set(2,32,c.r[4].wrapping_add(0x3c));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x526bb5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4d75d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd736u32,16),16); }
            if c.cond(3) { pc = 0x526ca4; continue; }
            pc = 0x526bc6; continue;
            }
            0x526bc6 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[6]; wm(m,a,32,0x55u32); }
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x526bf1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x39c),8));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x526c06);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0xbu32);
            { let a = c.get(6,32); let r = c.add(a,0x18u32,32); c.set(6,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xfffffff4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x2c0),8));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xfffffff8); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6c29f8u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x526c54);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(6,32));
            c.push(m,0x22u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x526c64);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64cbf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x22u32);
            c.set(2,32,c.r[4].wrapping_add(0x58));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            c.push(m, 0x526c76);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64cb60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x2u32); }
            c.push(m, 0x526c83);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x54));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x1u32); }
            c.push(m, 0x526c91);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x0u32); }
            c.push(m, 0x526c9f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x527859; continue;
            }
            0x526ca4 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(6,32,0xffffu32);
            { let a = c.r[7]; wm(m,a,32,0x4cu32); }
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x526ccf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x39c),8));
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x14); wm(m,a,16,c.get(5,16)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x16); wm(m,a,16,c.get(5,16)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[7].wrapping_add(0x18));
            c.push(m, 0x526d08);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x82c),8));
            { let a = c.get(0,8); c.sub(a,0x14u32,8); }
            if c.cond(2) { pc = 0x526d20; continue; }
            pc = 0x526d12; continue;
            }
            0x526d12 => {
            { let a = c.r[7].wrapping_add(0x14); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[7].wrapping_add(0x16); wm(m,a,16,c.get(5,16)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            pc = 0x526d5c; continue;
            }
            0x526d20 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x673732),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x14); wm(m,a,16,c.get(2,16)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x82c),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x673733),8));
            { let a = c.r[7].wrapping_add(0x16); wm(m,a,16,c.get(1,16)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x82c),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x673731),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x526d5c; continue;
            }
            0x526d5c => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            c.set(3,32,0xau32);
            if c.cond(6) { pc = 0x5270cb; continue; }
            pc = 0x526d70; continue;
            }
            0x526d70 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            pc = 0x526d74; continue;
            }
            0x526d74 => {
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x2c1),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,8,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6ddbc0),8));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5270a6; continue; }
            pc = 0x526dae; continue;
            }
            0x526dae => {
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de588u32);
            c.push(m, 0x526db9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5353c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(14) { pc = 0x5270a6; continue; }
            pc = 0x526dc2; continue;
            }
            0x526dc2 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(5,32));
            c.set(1,32,0x6de588u32);
            c.set(6,32,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x2c8),32));
            c.push(m, 0x526dde);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x535300, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0x10u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x526e40; continue; }
            pc = 0x526dec; continue;
            }
            0x526dec => {
            c.set(0,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(1,32,rm(m,0x6c25c8u32,32));
            c.push(m,c.get(0,32));
            if c.cond(5) { pc = 0x526e08; continue; }
            pc = 0x526e02; continue;
            }
            0x526e02 => {
            c.set(1,32,rm(m,0x6c25ccu32,32));
            pc = 0x526e08; continue;
            }
            0x526e08 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x526e0f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1f),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x526e29; continue; }
            pc = 0x526e1a; continue;
            }
            0x526e1a => {
            c.set(0,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            c.push(m,c.get(0,32));
            pc = 0x52709b; continue;
            }
            0x526e29 => {
            c.push(m,0x6c07fcu32);
            c.set(1,32,c.r[7].wrapping_add(0x18));
            c.push(m, 0x526e36);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            pc = 0x52709b; continue;
            }
            0x526e40 => {
            c.set(1,32,c.get(6,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0x8u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0x8u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x526ea2; continue; }
            pc = 0x526e4e; continue;
            }
            0x526e4e => {
            c.set(0,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(1,32,rm(m,0x6c25c8u32,32));
            c.push(m,c.get(0,32));
            if c.cond(5) { pc = 0x526e6a; continue; }
            pc = 0x526e64; continue;
            }
            0x526e64 => {
            c.set(1,32,rm(m,0x6c25ccu32,32));
            pc = 0x526e6a; continue;
            }
            0x526e6a => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x526e71);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1f),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x526e8b; continue; }
            pc = 0x526e7c; continue;
            }
            0x526e7c => {
            c.set(0,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            c.push(m,c.get(0,32));
            pc = 0x52709b; continue;
            }
            0x526e8b => {
            c.push(m,0x6c07feu32);
            c.set(1,32,c.r[7].wrapping_add(0x18));
            c.push(m, 0x526e98);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            pc = 0x52709b; continue;
            }
            0x526ea2 => {
            c.set(1,32,c.get(6,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0x8000u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0x8000u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x526f0a; continue; }
            pc = 0x526eb6; continue;
            }
            0x526eb6 => {
            c.set(0,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(1,32,rm(m,0x6c26c0u32,32));
            c.push(m,c.get(0,32));
            if c.cond(5) { pc = 0x526ed2; continue; }
            pc = 0x526ecc; continue;
            }
            0x526ecc => {
            c.set(1,32,rm(m,0x6c26c4u32,32));
            pc = 0x526ed2; continue;
            }
            0x526ed2 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x526ed9);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1f),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x526ef3; continue; }
            pc = 0x526ee4; continue;
            }
            0x526ee4 => {
            c.set(0,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            c.push(m,c.get(0,32));
            pc = 0x52709b; continue;
            }
            0x526ef3 => {
            c.push(m,0x6c0800u32);
            c.set(1,32,c.r[7].wrapping_add(0x18));
            c.push(m, 0x526f00);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            pc = 0x52709b; continue;
            }
            0x526f0a => {
            c.set(1,32,c.get(6,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0x1000u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0x1000u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x526f72; continue; }
            pc = 0x526f1e; continue;
            }
            0x526f1e => {
            c.set(0,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(1,32,rm(m,0x6c25c8u32,32));
            c.push(m,c.get(0,32));
            if c.cond(5) { pc = 0x526f3a; continue; }
            pc = 0x526f34; continue;
            }
            0x526f34 => {
            c.set(1,32,rm(m,0x6c25ccu32,32));
            pc = 0x526f3a; continue;
            }
            0x526f3a => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x526f41);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1f),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x526f5b; continue; }
            pc = 0x526f4c; continue;
            }
            0x526f4c => {
            c.set(0,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            c.push(m,c.get(0,32));
            pc = 0x52709b; continue;
            }
            0x526f5b => {
            c.push(m,0x6c0802u32);
            c.set(1,32,c.r[7].wrapping_add(0x18));
            c.push(m, 0x526f68);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            pc = 0x52709b; continue;
            }
            0x526f72 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,0x7c0000u32,32); }
            if c.cond(4) { pc = 0x526fff; continue; }
            pc = 0x526f7f; continue;
            }
            0x526f7f => {
            { let a = c.get(6,32); let n = 0x12u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0x1fu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x14u32,32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(6,32)); }
            if c.cond(13) { pc = 0x5270a6; continue; }
            pc = 0x526f92; continue;
            }
            0x526f92 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x66e0d4),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x526fbc; continue; }
            pc = 0x526f9c; continue;
            }
            0x526f9c => {
            c.set(0,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.get(6,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(1,32,rm(m,0x6c276cu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x526fdb; continue;
            }
            0x526fbc => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(0,32,rm(m,0x6c2770u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x526fdb; continue;
            }
            0x526fdb => {
            c.push(m, 0x526fe0);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x23),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x526ff5; continue; }
            pc = 0x526feb; continue;
            }
            0x526feb => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x527096; continue;
            }
            0x526ff5 => {
            c.push(m,0x6c0804u32);
            pc = 0x52708e; continue;
            }
            0x526fff => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0x2000u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x2000u32,32); }
            if c.cond(5) { pc = 0x527051; continue; }
            pc = 0x52700d; continue;
            }
            0x52700d => {
            c.set(1,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,rm(m,0x6c272cu32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x527026);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1f),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x52703d; continue; }
            pc = 0x527031; continue;
            }
            0x527031 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            c.push(m,c.get(1,32));
            pc = 0x52709b; continue;
            }
            0x52703d => {
            c.push(m,0x6c0806u32);
            c.set(1,32,c.r[7].wrapping_add(0x18));
            c.push(m, 0x52704a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            pc = 0x52709b; continue;
            }
            0x527051 => {
            { let a = c.get(6,32); let r = c.and(a,0x4000u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x4000u32,32); }
            if c.cond(5) { pc = 0x5270a6; continue; }
            pc = 0x52705f; continue;
            }
            0x52705f => {
            c.set(2,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,0x6c2730u32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x527077);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x1f),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x527089; continue; }
            pc = 0x527082; continue;
            }
            0x527082 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x527096; continue;
            }
            0x527089 => {
            c.push(m,0x6c0808u32);
            pc = 0x52708e; continue;
            }
            0x52708e => {
            c.set(1,32,c.r[7].wrapping_add(0x18));
            c.push(m, 0x527096);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x527096; continue;
            }
            0x527096 => {
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            pc = 0x52709b; continue;
            }
            0x52709b => {
            c.set(1,32,c.r[7].wrapping_add(0x18));
            c.push(m, 0x5270a3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[7].wrapping_add(0xc),32); let r = c.add(a,c.get(3,32),32); { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x5270a6; continue;
            }
            0x5270a6 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x9f8u32,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            if c.cond(2) { pc = 0x526d74; continue; }
            pc = 0x5270cb; continue;
            }
            0x5270cb => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5270e4; continue; }
            pc = 0x5270d5; continue;
            }
            0x5270d5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x2c0),8));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x5270e4; continue;
            }
            0x5270e4 => {
            c.set(5,32,rm(m,0x6de4b0u32,32));
            c.set(0,32,0xffu32);
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x100u32); }
            if c.cond(14) { pc = 0x5271af; continue; }
            pc = 0x527107; continue;
            }
            0x527107 => {
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x630));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            pc = 0x527121; continue;
            }
            0x527121 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x9f8u32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xfffffc91),8));
            c.set(0,32,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,8,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbc0),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5271a7; continue; }
            pc = 0x52715c; continue;
            }
            0x52715c => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(6,32,0x14u32);
            c.set(3,32,c.get(7,32));
            { let a = c.get(7,32); let r = c.add(a,0xfffffa24u32,32); c.set(7,32,r); }
            pc = 0x52716d; continue;
            }
            0x52716d => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xfffffffc),32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xfffffff0),32));
            { let a = c.get(7,32); let r = c.sub(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,0x10u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x5271a3; continue; }
            pc = 0x52717e; continue;
            }
            0x52717e => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x52718d; continue; }
            pc = 0x527182; continue;
            }
            0x527182 => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            pc = 0x527191; continue;
            }
            0x52718d => {
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            pc = 0x527191; continue;
            }
            0x527191 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x20),32),32); }
            if c.cond(13) { pc = 0x5271a3; continue; }
            pc = 0x527197; continue;
            }
            0x527197 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            pc = 0x5271a3; continue;
            }
            0x5271a3 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(15) { pc = 0x52716d; continue; }
            pc = 0x5271a7; continue;
            }
            0x5271a7 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(15) { pc = 0x527121; continue; }
            pc = 0x5271af; continue;
            }
            0x5271af => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x527218; continue; }
            pc = 0x5271bd; continue;
            }
            0x5271bd => {
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6c27e0u32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5271e4);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x23),8));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5271fa; continue; }
            pc = 0x5271f3; continue;
            }
            0x5271f3 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x527207; continue;
            }
            0x5271fa => {
            c.push(m,0x6c080au32);
            c.set(1,32,c.r[6].wrapping_add(0x18));
            c.push(m, 0x527207);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x527207; continue;
            }
            0x527207 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x18));
            c.push(m, 0x527214);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.add(a,0xau32,32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x527218; continue;
            }
            0x527218 => {
            c.set(5,32,rm(m,0x6de4b0u32,32));
            c.set(0,32,0xffu32);
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            if c.cond(14) { pc = 0x52732c; continue; }
            pc = 0x52723b; continue;
            }
            0x52723b => {
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(2,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x76c));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            pc = 0x527254; continue;
            }
            0x527254 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x9f8u32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xfffffb55),8));
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,8,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6ddbc0),8));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x527320; continue; }
            pc = 0x527293; continue;
            }
            0x527293 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.get(0,32); let r = c.and(a,0x100u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x100u32,32); }
            if c.cond(5) { pc = 0x527320; continue; }
            pc = 0x5272a5; continue;
            }
            0x5272a5 => {
            c.set(7,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(3,32,c.r[7].wrapping_add(0xfffffd88));
            { let a = c.get(7,32); let r = c.add(a,0xfffff898u32,32); c.set(7,32,r); }
            pc = 0x5272b5; continue;
            }
            0x5272b5 => {
            c.set(0,32,0x1u32);
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,0x4472eu32,32); }
            if c.cond(4) { pc = 0x527314; continue; }
            pc = 0x5272c3; continue;
            }
            0x5272c3 => {
            c.set(6,32,rm(m,c.r[3],32));
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[6].wrapping_add(0x4e20));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x527314; continue; }
            pc = 0x5272d1; continue;
            }
            0x5272d1 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5272f9; continue; }
            pc = 0x5272d8; continue;
            }
            0x5272d8 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x5272f1; continue; }
            pc = 0x5272e4; continue;
            }
            0x5272e4 => {
            c.set(0,32,c.r[2].wrapping_add(c.r[2]));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.add(a,0x100u32,32); c.set(0,32,r); }
            pc = 0x527302; continue;
            }
            0x5272f1 => {
            c.set(0,32,c.r[2].wrapping_add(0x100));
            pc = 0x527302; continue;
            }
            0x5272f9 => {
            c.set(0,32,c.r[2].wrapping_add(c.r[2]));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(6,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let b = c.get(6,32); c.idiv32(b); }
            pc = 0x527302; continue;
            }
            0x527302 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x20),32),32); }
            if c.cond(14) { pc = 0x527314; continue; }
            pc = 0x527308; continue;
            }
            0x527308 => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            pc = 0x527314; continue;
            }
            0x527314 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x10u32,32); c.set(3,32,r); }
            { let a = c.get(1,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x5272b5; continue; }
            pc = 0x527320; continue;
            }
            0x527320 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(15) { pc = 0x527254; continue; }
            pc = 0x527328; continue;
            }
            0x527328 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x52732c; continue;
            }
            0x52732c => {
            { let a = c.get(0,32); c.sub(a,rm(m,0x6de4b0u32,32),32); }
            if c.cond(3) { pc = 0x52738f; continue; }
            pc = 0x527334; continue;
            }
            0x527334 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6c28a8u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x527359);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x23),8));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x52736f; continue; }
            pc = 0x527368; continue;
            }
            0x527368 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x52737c; continue;
            }
            0x52736f => {
            c.push(m,0x6c080cu32);
            c.set(1,32,c.r[5].wrapping_add(0x18));
            c.push(m, 0x52737c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x52737c; continue;
            }
            0x52737c => {
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.r[5].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m, 0x527389);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); let r = c.add(a,0xau32,32); { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x527393; continue;
            }
            0x52738f => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x54),32));
            pc = 0x527393; continue;
            }
            0x527393 => {
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x0u32); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x5273a0; continue;
            }
            0x5273a0 => {
            c.set(1,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5273d5; continue; }
            pc = 0x5273ab; continue;
            }
            0x5273ab => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x16),16));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x5273d5; continue; }
            pc = 0x5273b5; continue;
            }
            0x5273b5 => {
            c.set(7,32,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x5273ca);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x515ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            c.set(6,16,rm(m,c.r[1].wrapping_add(0xc),16));
            pc = 0x5273d5; continue;
            }
            0x5273d5 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x5u32,32); }
            if c.cond(12) { pc = 0x5273a0; continue; }
            pc = 0x5273db; continue;
            }
            0x5273db => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5273ec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4ec694, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x527421; continue; }
            pc = 0x5273f2; continue;
            }
            0x5273f2 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x527402);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0xc),8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x527421; continue; }
            pc = 0x527408; continue;
            }
            0x527408 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x16),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x527421; continue; }
            pc = 0x52740f; continue;
            }
            0x52740f => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12),8));
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x10),16));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x14),8));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(2,8)); }
            pc = 0x527421; continue;
            }
            0x527421 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5275df; continue; }
            pc = 0x527439; continue;
            }
            0x527439 => {
            { let a = c.get(6,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x5275df; continue; }
            pc = 0x527446; continue;
            }
            0x527446 => {
            c.set(3,32,0xau32);
            pc = 0x52744b; continue;
            }
            0x52744b => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x42),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x5275ce; continue; }
            pc = 0x527471; continue;
            }
            0x527471 => {
            c.set(1,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = rm(m,c.r[0].wrapping_add(0x2),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5275ce; continue; }
            pc = 0x52748e; continue;
            }
            0x52748e => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x28),8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x5275ce; continue; }
            pc = 0x52749d; continue;
            }
            0x52749d => {
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5274a9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x448610, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(0,8)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x42),16));
            c.set(1,32,0x3u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.sub(a,0x0u32,32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x52755c; continue; }
            pc = 0x5274d0; continue;
            }
            0x5274d0 => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x52751b; continue; }
            pc = 0x5274d3; continue;
            }
            0x5274d3 => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x5275a2; continue; }
            pc = 0x5274da; continue;
            }
            0x5274da => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x5274f6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x52750c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6b1468u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x52759a; continue;
            }
            0x52751b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x527538);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x52754d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b1464u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x52759a; continue;
            }
            0x52755c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x527570);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x527586);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b1460u32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x52759a; continue;
            }
            0x52759a => {
            c.push(m, 0x52759f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x5275a2; continue;
            }
            0x5275a2 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5275b1; continue; }
            pc = 0x5275aa; continue;
            }
            0x5275aa => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x5275be; continue;
            }
            0x5275b1 => {
            c.push(m,0x6c080eu32);
            c.set(1,32,c.r[5].wrapping_add(0x18));
            c.push(m, 0x5275be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5275be; continue;
            }
            0x5275be => {
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.r[5].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m, 0x5275cb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); let r = c.add(a,c.get(3,32),32); { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x5275ce; continue;
            }
            0x5275ce => {
            c.set(6,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(6,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x52744b; continue; }
            pc = 0x5275df; continue;
            }
            0x5275df => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x18),32));
            c.set(6,32,c.r[5].wrapping_add(0x18));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x527639; continue; }
            pc = 0x5275ec; continue;
            }
            0x5275ec => {
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m,0x22u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x5275f9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64cbf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x22u32);
            c.set(2,32,c.r[4].wrapping_add(0x58));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,0x3u32); }
            c.push(m, 0x52760b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64cb60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x4u32); }
            c.push(m, 0x527618);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x54));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x3u32); }
            c.push(m, 0x527626);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x0u32); }
            c.push(m, 0x527634);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x527859; continue;
            }
            0x527639 => {
            c.push(m, 0x52763e);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5276a5; continue; }
            pc = 0x52764a; continue;
            }
            0x52764a => {
            c.set(6,32,rm(m,0x6de508u32,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2c1),8));
            c.set(2,32,c.get(3,32));
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,c.r[2].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,8,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6ddbc0),8));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x52768b; continue; }
            pc = 0x527686; continue;
            }
            0x527686 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            pc = 0x52768b; continue;
            }
            0x52768b => {
            c.set(6,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x5276a1; continue; }
            pc = 0x527696; continue;
            }
            0x527696 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x5276d4; continue; }
            pc = 0x52769a; continue;
            }
            0x52769a => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,0x1u32);
            pc = 0x5276a1; continue;
            }
            0x5276a1 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x52764a; continue; }
            pc = 0x5276a5; continue;
            }
            0x5276a5 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x5276ad; continue;
            }
            0x5276ad => {
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(6,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x5277ea; continue; }
            pc = 0x5276cd; continue;
            }
            0x5276cd => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x527880)); continue;
            }
            0x5276d4 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x82c),8));
            { let b = c.get(6,32); c.div32(b); }
            c.set(7,32,c.get(2,32));
            pc = 0x5276ad; continue;
            }
            0x5276e8 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2e4),32));
            c.set(0,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2d4),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6c29b8u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[5].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x52770e);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x5277ea; continue;
            }
            0x527716 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6f1),8));
            c.set(2,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.push(m,c.get(2,32));
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x52777a; continue; }
            pc = 0x527732; continue;
            }
            0x527732 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x527742);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x527749);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6f1),8));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x52775d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x527764);
            { let args = []; c.r[0] = host.call(m, 0x4f6420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6c29bcu32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x527775);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x5277ea; continue;
            }
            0x52777a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6d8),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x52778d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x6d4),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x5277a1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6c29bcu32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5277b2);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x5277ea; continue;
            }
            0x5277b7 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x31c),32));
            c.set(2,32,rm(m,0x6c29c0u32,32));
            pc = 0x5277d4; continue;
            }
            0x5277c5 => {
            c.set(2,32,rm(m,0x6c29c4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2c6),16));
            pc = 0x5277d4; continue;
            }
            0x5277d4 => {
            c.set(0,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[5].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5277e7);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x5277ea; continue;
            }
            0x5277ea => {
            c.set(6,32,c.r[5].wrapping_add(0x18));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(6,32));
            c.push(m,0x22u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5277fa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64cbf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,0x22u32);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,0x5u32); }
            c.push(m, 0x52780c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64cb60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,0x6u32); }
            c.push(m, 0x527819);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x5u32); }
            c.push(m, 0x527827);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x0u32); }
            c.push(m, 0x527835);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x54),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x7u32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x2c0),8));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x527859; continue;
            }
            0x527859 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x52786a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn list_other_merchants_with_office_in_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x531470;
    loop {
        match pc {
            0x531470 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(1,32,c.r[0].wrapping_mul(4).wrapping_add(0x2));
            c.push(m,c.get(1,32));
            c.push(m, 0x53148b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(5,32,c.get(0,32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x53149b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x5315ad; continue; }
            pc = 0x5314ac; continue;
            }
            0x5314ac => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5315ad; continue; }
            pc = 0x5314b4; continue;
            }
            0x5314b4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[5]; wm(m,a,16,0x0u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.r[5].wrapping_add(0x2));
            if c.cond(3) { pc = 0x531583; continue; }
            pc = 0x5314d1; continue;
            }
            0x5314d1 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x68),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x784),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x531583; continue; }
            pc = 0x5314f4; continue;
            }
            0x5314f4 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x5314fa; continue;
            }
            0x5314fa => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x74),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x2c4),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xa),16),16); }
            if c.cond(3) { pc = 0x53156c; continue; }
            pc = 0x53151a; continue;
            }
            0x53151a => {
            c.set(0,32,c.get(2,32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x78),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(7,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[3]).wrapping_add(0x8),8); c.and(a,0x4u32,8); }
            if c.cond(5) { pc = 0x531568; continue; }
            pc = 0x531537; continue;
            }
            0x531537 => {
            { let a = c.get(0,32); c.sub(a,rm(m,0x6dfc14u32,32),32); }
            if c.cond(4) { pc = 0x531568; continue; }
            pc = 0x53153f; continue;
            }
            0x53153f => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,16,c.get(2,16)); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x2d4),16));
            { let a = c.get(2,16); let r = c.add(a,rm(m,c.r[1].wrapping_add(0x2d2),16),16); c.set(2,16,r); }
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0xfffffffe); wm(m,a,16,c.get(2,16)); }
            { let a = rm(m,c.r[5],16); let r = c.inc(a,16); { let a = c.r[5]; wm(m,a,16,r); } }
            pc = 0x531568; continue;
            }
            0x531568 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x53156c; continue;
            }
            0x53156c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x2ca),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5314fa; continue; }
            pc = 0x531583; continue;
            }
            0x531583 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5],16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0]));
            c.set(0,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x2));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x531598);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x53159e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,c.get(5,32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x5315ad => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn auto_trader_is_pirate_kind(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4fe150;
    loop {
        match pc {
            0x4fe150 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x8),8));
            c.set(0,8,0x20u32);
            { let a = c.get(0,8); c.sub(a,c.get(2,8),8); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn pick_type_45_46_message_bit(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d75d0;
    loop {
        match pc {
            0x4d75d0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            if c.cond(3) { pc = 0x4d7709; continue; }
            pc = 0x4d75ea; continue;
            }
            0x4d75ea => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4d75fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xa),16));
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[5]; wm(m,a,16,0xffffu32); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4d7679; continue; }
            pc = 0x4d761c; continue;
            }
            0x4d761c => {
            c.set(2,32,c.get(3,32));
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x4d7679; continue; }
            pc = 0x4d7623; continue;
            }
            0x4d7623 => {
            c.set(7,32,rm(m,c.r[6],32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.get(2,8); c.sub(a,0x45u32,8); }
            if c.cond(4) { pc = 0x4d764d; continue; }
            pc = 0x4d7634; continue;
            }
            0x4d7634 => {
            { let a = c.get(2,8); c.sub(a,0x46u32,8); }
            if c.cond(5) { pc = 0x4d7641; continue; }
            pc = 0x4d7639; continue;
            }
            0x4d7639 => {
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); c.sub(a,0x1ffu32,16); }
            if c.cond(2) { pc = 0x4d7664; continue; }
            pc = 0x4d7641; continue;
            }
            0x4d7641 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x4d761c; continue; }
            pc = 0x4d764b; continue;
            }
            0x4d764b => {
            pc = 0x4d7679; continue;
            }
            0x4d764d => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[5]; wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xa),16));
            { let a = c.r[7]; wm(m,a,16,c.get(2,16)); }
            pc = 0x4d767d; continue;
            }
            0x4d7664 => {
            { let a = c.r[5]; wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xa),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[0]; wm(m,a,16,c.get(2,16)); }
            pc = 0x4d7679; continue;
            }
            0x4d7679 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x30),32));
            pc = 0x4d767d; continue;
            }
            0x4d767d => {
            c.set(0,16,rm(m,c.r[5],16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x6),16),16); }
            if c.cond(3) { pc = 0x4d76fe; continue; }
            pc = 0x4d7687; continue;
            }
            0x4d7687 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4d76fe; continue; }
            pc = 0x4d768b; continue;
            }
            0x4d768b => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xa),16); c.sub(a,0x1ffu32,16); }
            if c.cond(3) { pc = 0x4d7704; continue; }
            pc = 0x4d769e; continue;
            }
            0x4d769e => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7],16));
            pc = 0x4d76a7; continue;
            }
            0x4d76a7 => {
            c.set(2,32,0x1u32);
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4d76bf; continue; }
            pc = 0x4d76b2; continue;
            }
            0x4d76b2 => {
            c.set(2,32,0x1u32);
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(c.r[6].wrapping_mul(2)).wrapping_add(0xe); wm(m,a,16,c.get(2,16)); }
            pc = 0x4d76bf; continue;
            }
            0x4d76bf => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x9u32,32); }
            if c.cond(12) { pc = 0x4d76a7; continue; }
            pc = 0x4d76c5; continue;
            }
            0x4d76c5 => {
            { let a = c.get(6,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x4d76dc; continue; }
            pc = 0x4d76ca; continue;
            }
            0x4d76ca => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x10),16));
            { let a = c.r[7]; wm(m,a,16,c.get(0,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4d76dc => {
            c.push(m, 0x4d76e1);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[4].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x10),16));
            { let a = c.r[7]; wm(m,a,16,c.get(1,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4d76fe => {
            { let a = c.r[5]; wm(m,a,16,0xffffu32); }
            pc = 0x4d7704; continue;
            }
            0x4d7704 => {
            { let a = c.r[7]; wm(m,a,16,0x0u32); }
            pc = 0x4d7709; continue;
            }
            0x4d7709 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_calc_weapon_buy_price(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5263d0;
    loop {
        match pc {
            0x5263d0 => {
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.and(a,0x3u32,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(0,32); c.sub(a,0xau32,32); }
            if c.cond(13) { pc = 0x5263ef; continue; }
            pc = 0x5263e1; continue;
            }
            0x5263e1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x5263ef => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[3],32));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x526405; continue; }
            pc = 0x5263fb; continue;
            }
            0x5263fb => {
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            pc = 0x52640f; continue;
            }
            0x526405 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(2,32,c.get(6,32));
            pc = 0x52640f; continue;
            }
            0x52640f => {
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_mul(2).wrapping_add(0x673794),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x2d4),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,0x1f4u32,32); }
            c.set(2,32,c.get(0,32));
            if c.cond(13) { pc = 0x526445; continue; }
            pc = 0x526430; continue;
            }
            0x526430 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            pc = 0x52644d; continue;
            }
            0x526445 => {
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            pc = 0x52644d; continue;
            }
            0x52644d => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_mul(2).wrapping_add(0x67378c),16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x67379c),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x526467; continue; }
            pc = 0x526465; continue;
            }
            0x526465 => {
            c.set(1,32,c.get(0,32));
            pc = 0x526467; continue;
            }
            0x526467 => {
            { let a = c.r[3]; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn get_merchant_alderman_mission_task_index(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ec694;
    loop {
        match pc {
            0x4ec694 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.set(0,16,rm(m,0x6de4aau32,16));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            c.set(7,32,c.get(1,32));
            if c.cond(3) { pc = 0x4ec6ee; continue; }
            pc = 0x4ec6a9; continue;
            }
            0x4ec6a9 => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ec6b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x37),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(6) { pc = 0x4ec6ee; continue; }
            pc = 0x4ec6bb; continue;
            }
            0x4ec6bb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(6,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x4ec6ee; continue; }
            pc = 0x4ec6cb; continue;
            }
            0x4ec6cb => {
            c.set(7,32,rm(m,c.r[7],32));
            pc = 0x4ec6cd; continue;
            }
            0x4ec6cd => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x6),16); c.sub(a,0x32u32,16); }
            c.set(1,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(8)));
            if c.cond(5) { pc = 0x4ec6e4; continue; }
            pc = 0x4ec6db; continue;
            }
            0x4ec6db => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xd),8));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4ec6f3; continue; }
            pc = 0x4ec6e4; continue;
            }
            0x4ec6e4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x4ec6cd; continue; }
            pc = 0x4ec6ee; continue;
            }
            0x4ec6ee => {
            c.set(0,32,0xffffu32);
            pc = 0x4ec6f3; continue;
            }
            0x4ec6f3 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_info_history_get_local_player_age(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5353c0;
    loop {
        match pc {
            0x5353c0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5353c9; continue;
            }
            0x5353c9 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xa00),8));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x5353e5; continue; }
            pc = 0x5353d6; continue;
            }
            0x5353d6 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(2) { pc = 0x5353c9; continue; }
            pc = 0x5353dc; continue;
            }
            0x5353dc => {
            c.set(0,32,0xfu32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x5353e5 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa08),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5353dc; continue; }
            pc = 0x5353f0; continue;
            }
            0x5353f0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2]),8));
            c.set(0,32,c.get(1,32));
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_info_history_get_known_flags(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x535300;
    loop {
        match pc {
            0x535300 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(5,32));
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[3].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),32));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xfu32,32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[6].wrapping_add(0xa00),8));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.or(a,0x7e0000u32,32); c.set(5,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x53535a; continue; }
            pc = 0x535347; continue;
            }
            0x535347 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x8u32,32); }
            if c.cond(13) { pc = 0x535380; continue; }
            pc = 0x53534d; continue;
            }
            0x53534d => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[6].wrapping_add(c.r[7]).wrapping_add(0xa00),8));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x535347; continue; }
            pc = 0x53535a; continue;
            }
            0x53535a => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xa08),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x535380; continue; }
            pc = 0x535365; continue;
            }
            0x535365 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[2]),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xfu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.and(a,c.get(5,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x535380 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); let r = c.and(a,0xfu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.and(a,c.get(5,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "tavern_content_window_show" => tavern_content_window_show(m, host, regs),
        "town_find_tavern_captain_for_merchant" => town_find_tavern_captain_for_merchant(m, host, regs),
        "town_calc_weapon_trade_table" => town_calc_weapon_trade_table(m, host, regs),
        "town_create_tavern_ware_offer" => town_create_tavern_ware_offer(m, host, regs),
        "town_build_tavern_news_message" => town_build_tavern_news_message(m, host, regs),
        "list_other_merchants_with_office_in_town" => list_other_merchants_with_office_in_town(m, host, regs),
        "auto_trader_is_pirate_kind" => auto_trader_is_pirate_kind(m, host, regs),
        "pick_type_45_46_message_bit" => pick_type_45_46_message_bit(m, host, regs),
        "town_calc_weapon_buy_price" => town_calc_weapon_buy_price(m, host, regs),
        "get_merchant_alderman_mission_task_index" => get_merchant_alderman_mission_task_index(m, host, regs),
        "town_info_history_get_local_player_age" => town_info_history_get_local_player_age(m, host, regs),
        "town_info_history_get_known_flags" => town_info_history_get_known_flags(m, host, regs),
        _ => return None,
    })
}
