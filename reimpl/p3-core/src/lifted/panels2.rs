//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

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

pub fn game_screen_draw_at_origin(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x428590;
    loop {
        match pc {
            0x428590 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[1],32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(4,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[3].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            { let target = rm(m,c.r[7].wrapping_add(0x9c),32); c.push(m, 0x4285b3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn game_screen_apply_resolution_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x429a40;
    loop {
        match pc {
            0x429a40 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[6].wrapping_add(0x8e0));
            c.push(m,c.get(0,32));
            c.push(m, 0x429a53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x9b8));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x429a61);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xa90));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x429a6f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xd18));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x429a7d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xdf0));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x429a8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0xb68));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x429a99);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0xc40));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x429aa7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0x808));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x429ab5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cba64u32,32));
            c.push(m, 0x429ac0);
            { let args = []; c.r[0] = host.call(m, 0x401160, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dcce0u32,32));
            { let a = c.get(0,32); c.sub(a,0x320u32,32); }
            if c.cond(4) { pc = 0x429c32; continue; }
            pc = 0x429ad0; continue;
            }
            0x429ad0 => {
            { let a = c.get(0,32); c.sub(a,0x400u32,32); }
            if c.cond(4) { pc = 0x429b8a; continue; }
            pc = 0x429adb; continue;
            }
            0x429adb => {
            { let a = c.get(0,32); c.sub(a,0x500u32,32); }
            if c.cond(5) { pc = 0x429ce5; continue; }
            pc = 0x429ae6; continue;
            }
            0x429ae6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa90),32));
            c.set(5,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[6].wrapping_add(0xa90));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429af9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x429aff); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429b08); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xa90));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429b14); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x429b1a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb68),32));
            c.set(1,32,c.r[6].wrapping_add(0xb68));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x429b30); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429b38); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0xa90));
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429b45); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.r[6].wrapping_add(0xb68));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x429b57); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xa90));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x429b67);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[6].wrapping_add(0xdf0));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x429b77);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696f28u32);
            pc = 0x429cc5; continue;
            }
            0x429b8a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9b8),32));
            c.set(5,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[6].wrapping_add(0x9b8));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429b9d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x429ba3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429bac); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x9b8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429bb8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x429bbe); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb68),32));
            c.set(1,32,c.r[6].wrapping_add(0xb68));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x429bd4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9b8),32));
            c.set(1,32,c.r[6].wrapping_add(0x9b8));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429be4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429bed); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.r[6].wrapping_add(0xb68));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x429bff); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x9b8));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x429c0f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[6].wrapping_add(0xd18));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x429c1f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696f10u32);
            pc = 0x429cc5; continue;
            }
            0x429c32 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8e0),32));
            c.set(5,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429c45); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x429c4b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429c54); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429c60); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[5].wrapping_add(0x78),32); c.push(m, 0x429c66); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb68),32));
            c.set(1,32,c.r[6].wrapping_add(0xb68));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x429c7c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429c8c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429c95); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.r[6].wrapping_add(0xb68));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x78),32); c.push(m, 0x429ca7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x8e0));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x429cb7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x696ef8u32);
            pc = 0x429cc5; continue;
            }
            0x429cc5 => {
            c.push(m, 0x429cca);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cba64u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x429cd5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cba64u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x429ce5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x429ce5; continue;
            }
            0x429ce5 => {
            c.push(m,c.get(5,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x429cef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x429cf9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(3,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x429d03);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            c.push(m,0x0u32);
            c.set(5,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429d1b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6dcce4u32,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x88),32); c.push(m, 0x429d2e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x4c),32); c.push(m, 0x429d38); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc640),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc644),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x429d51);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x590ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.push(m, 0x429d5e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x593910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbdd8u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x8e0));
            c.push(m,0x0u32);
            c.set(5,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429d76); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6dcce4u32,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x88),32); c.push(m, 0x429d89); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x4c),32); c.push(m, 0x429d93); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x429d9e);
            { let args = []; c.r[0] = host.call(m, 0x44f640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e51c0u32,32));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            c.set(5,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429dae); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x88),32); c.push(m, 0x429db9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            c.set(3,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429dc2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51c0u32,32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x429dce); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1078),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xec8),32));
            c.set(1,32,c.r[6].wrapping_add(0x1078));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x429de5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x808),32));
            c.set(1,32,c.r[6].wrapping_add(0x808));
            c.set(5,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429df6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x88),32); c.push(m, 0x429e03); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            c.set(5,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429e0c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0xec8));
            c.push(m,c.get(5,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x429e18); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x808),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1078),32));
            c.set(1,32,c.r[6].wrapping_add(0x808));
            c.set(3,32,c.r[6].wrapping_add(0x1078));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x429e35); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xec8),32));
            c.set(1,32,c.r[6].wrapping_add(0xec8));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x429e45); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x429e4b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xfa0),32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x429e5a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429e65); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xfa0),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0xfa0));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x429e7b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1078),32));
            c.set(1,32,c.r[6].wrapping_add(0x1078));
            c.set(3,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x429e8c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1078),32));
            c.set(1,32,c.r[6].wrapping_add(0x1078));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429e9f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xfa0));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x429eab); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec8),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1150),32));
            c.set(3,32,c.r[6].wrapping_add(0xec8));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x70),32); c.push(m, 0x429ec4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429ecf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,c.r[6].wrapping_add(0x1078));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x429ee3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x1150));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x429eed); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(5,32,rm(m,c.r[1],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x429efe); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec8),32));
            c.set(1,32,c.r[6].wrapping_add(0xec8));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x429f0e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x429f18); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xb68),32));
            c.set(3,32,c.r[6].wrapping_add(0xb68));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x429f29); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8c),32); c.push(m, 0x429f34); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            c.set(5,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x48),32); c.push(m, 0x429f3d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x88),32); c.push(m, 0x429f4a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.push(m, 0x429f50);
            { let args = []; c.r[0] = host.call(m, 0x441750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x2640),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x2640));
            c.set(1,32,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,0x12u32);
            { let target = rm(m,c.r[2].wrapping_add(0x88),32); c.push(m, 0x429f6d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[5].wrapping_add(0x4c),32); c.push(m, 0x429f73); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6dcce4u32,32));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.add(a,0xffffffeeu32,32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x429f88); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6ee120u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x429f9c; continue; }
            pc = 0x429f92; continue;
            }
            0x429f92 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x429f9c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x429f9c; continue;
            }
            0x429f9c => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x429fa3);
            { let args = []; c.r[0] = host.call(m, 0x429fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb10u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x429fcb; continue; }
            pc = 0x429fad; continue;
            }
            0x429fad => {
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x429fb5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x429fcb; continue; }
            pc = 0x429fb9; continue;
            }
            0x429fb9 => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let args = []; c.r[0] = host.call(m, 0x41e860, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x429fcb => {
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

pub fn game_screen_push_letter_ticker_line(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42bb20;
    loop {
        match pc {
            0x42bb20 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d1c1u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42bb4f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(3,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7b4),8));
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            if c.cond(3) { pc = 0x42bcb6; continue; }
            pc = 0x42bb5f; continue;
            }
            0x42bb5f => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x494),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[6]));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x52c),32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x42bb98);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7b4),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x494));
            { let target = rm(m,c.r[7].wrapping_add(0x58),32); c.push(m, 0x42bbb4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,0x64u32);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7b4),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(5,32,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x494),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x494));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7b4),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x494));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[5].wrapping_add(0x70),32); c.push(m, 0x42bbec); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42bbf4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7b4),8));
            c.push(m,c.get(3,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x494));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x42bc0f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7b4),8));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(7,32,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x494));
            c.set(1,32,c.r[7].wrapping_add(0x9c));
            c.push(m, 0x42bc34);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xd4),32); c.push(m, 0x42bc3e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dccf0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x7b4),8));
            { let a = c.get(1,32); let r = c.add(a,0x2ee0u32,32); c.set(1,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x494));
            { let a = c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x7e8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(7,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x42bc66); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x7b8); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x42bc73); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42bc7c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7b4),8));
            c.set(1,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x7bc); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.push(m,0x1u32);
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x494));
            { let a = c.r[6].wrapping_add(0x7b4); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x42bcb1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x42be10; continue;
            }
            0x42bcb6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x494),32));
            c.set(7,32,c.r[6].wrapping_add(0x494));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x42bcc7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x7b8); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x42bcd4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42bcdd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x4u32); }
            { let a = c.r[6].wrapping_add(0x7bc); wm(m,a,32,c.get(5,32)); }
            pc = 0x42bced; continue;
            }
            0x42bced => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x42bcf7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x13c),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x98),32));
            c.push(m,c.get(0,32));
            c.set(5,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x42bd1e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x58),32); c.push(m, 0x42bd27); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[7],32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[5].wrapping_add(0x70),32); c.push(m, 0x42bd30); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x42bd38); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[5].wrapping_add(0x64),32); c.push(m, 0x42bd42); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x13c),32));
            c.set(1,32,c.r[7].wrapping_add(0x9c));
            c.push(m,c.get(0,32));
            c.push(m, 0x42bd54);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x42bd5e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.add(a,0xa0u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x42bced; continue; }
            pc = 0x42bd73; continue;
            }
            0x42bd73 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x714),32));
            c.set(7,32,c.r[6].wrapping_add(0x714));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xd4),32); c.push(m, 0x42bd87); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x7ec));
            c.push(m,0x10u32);
            c.set(2,32,c.r[6].wrapping_add(0x7e8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x42bd9c);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dccf0u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.add(a,0x2ee0u32,32); c.set(0,32,r); }
            c.set(5,32,rm(m,c.r[7],32));
            { let a = c.r[6].wrapping_add(0x7f8); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x7ac),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.r[6].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x42bdd4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[5].wrapping_add(0x58),32); c.push(m, 0x42bddd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,c.r[7],32));
            c.push(m,0x64u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[6].wrapping_add(0x70),32); c.push(m, 0x42bde6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42bdee); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[6].wrapping_add(0x64),32); c.push(m, 0x42bdf6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[7].wrapping_add(0x9c));
            c.push(m, 0x42be06);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x42be10); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x42be10; continue;
            }
            0x42be10 => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x42be21);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_list_window_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x472c60;
    loop {
        match pc {
            0x472c60 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65f6ddu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x34u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x6b9874u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,0x1u32); }
            c.push(m, 0x472ca8);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9880u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472cc6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b988cu32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472ce3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b989cu32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.set(3,32,c.get(0,32));
            c.push(m, 0x472d00);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b98a8u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472d1f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b98b8u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472d3e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b98c0u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x472d5d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b98ccu32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472d7c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b98d8u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472d9b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b98e4u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x472db8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b98f0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472dd7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9900u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472df6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b990cu32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x472e15);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9918u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472e34);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9924u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472e53);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9934u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x472e72);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9940u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472e8d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b994cu32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472eac);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9958u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x472ecb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9964u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472ee6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0xcd0); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x472efb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,0x6da820u32);
            { let a = c.r[6].wrapping_add(0xa8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x472f0c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,0x6da820u32);
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x472f1d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xac); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x472f32);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6b9974u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x8c8));
            { let a = c.r[6].wrapping_add(0xb0); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x472f52);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9980u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x472f6d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x5c));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x8d4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x472f85);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd8),32));
            c.set(1,32,c.r[6].wrapping_add(0xd8));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x472f94); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x5c));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x472fa6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x48c),32));
            c.set(1,32,c.r[6].wrapping_add(0x48c));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x472fb5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0x6b998cu32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x904));
            c.push(m, 0x472fcf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,c.r[6].wrapping_add(0x910));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de59u32,8));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x472ff5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x47300b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xdbc),32));
            c.set(1,32,c.r[6].wrapping_add(0xdbc));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x47301a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x473030);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            c.set(1,32,c.r[6].wrapping_add(0xcd4));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x47303f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x473055);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa04),32));
            c.set(1,32,c.r[6].wrapping_add(0xa04));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x473064); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x47307a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xaec),32));
            c.set(1,32,c.r[6].wrapping_add(0xaec));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x473089); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6b999cu32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x9f8));
            c.push(m, 0x4730a3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4730b2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[6].wrapping_add(0xb4); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x4730c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0xb8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4730dc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xbc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x4730f1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x473106);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x47311b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0xcc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,c.get(0,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6cc954u32);
            c.push(m,0x6b99acu32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x473149);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4731e8; continue; }
            pc = 0x473151; continue;
            }
            0x473151 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dbb58u32);
            c.push(m, 0x473167);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x838));
            c.push(m, 0x473172);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x40a560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6dbb58u32);
            c.push(m, 0x473184);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x874));
            c.push(m, 0x47318f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x40a560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x4731c2; continue; }
            pc = 0x47319c; continue;
            }
            0x47319c => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4731c2; continue; }
            pc = 0x4731a3; continue;
            }
            0x4731a3 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(2,32,c.r[6].wrapping_add(0x8b8));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[2]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[2].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x4731c2; continue;
            }
            0x4731c2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xbd8u32,32); c.set(6,32,r); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x4731e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4731e8; continue;
            }
            0x4731e8 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4731f6);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x54));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x473207);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_list_window_layout_controls(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4751d0;
    loop {
        match pc {
            0x4751d0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x68u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x4751e3);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x6da5f0u32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x4751ef);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcc8),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x475224; continue; }
            pc = 0x4751ff; continue;
            }
            0x4751ff => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x475201; continue;
            }
            0x475201 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbd4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x475213);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcc8),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0xe8u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x475201; continue; }
            pc = 0x475224; continue;
            }
            0x475224 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xbd4),32));
            { let a = c.r[6].wrapping_add(0xcc8); wm(m,a,32,0x0u32); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x47523e; continue; }
            pc = 0x475238; continue;
            }
            0x475238 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x47523e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47523e; continue;
            }
            0x47523e => {
            { let a = c.r[6].wrapping_add(0xbd4); wm(m,a,32,0x0u32); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x60),32); c.push(m, 0x475258); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x475268); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0x838));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x475275);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x47527e);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.get(3,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x8ec),32),32); c.set(3,32,r); }
            c.push(m, 0x47528e);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x874));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m, 0x47529b);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x874));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x4752a8);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x874));
            { let a = c.get(5,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x8f0),32),32); c.set(5,32,r); }
            c.push(m, 0x4752bc);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x908),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8cc),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8f0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x475317; continue; }
            pc = 0x4752dc; continue;
            }
            0x4752dc => {
            c.set(1,32,c.r[6].wrapping_add(0x874));
            c.push(m, 0x4752e7);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x838));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x4752f4);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x838));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x475305);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x475317; continue;
            }
            0x475317 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xdbc),32));
            c.set(1,32,c.r[6].wrapping_add(0xdbc));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x475326); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[6].wrapping_add(0x838));
            c.set(3,32,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x475335);
            { let args = []; c.r[0] = host.call(m, 0x40abc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0xd8));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m, 0x475342);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb00u32,32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(1,32,rm(m,0x6cbb00u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x47538f; continue; }
            pc = 0x475369; continue;
            }
            0x475369 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[2]));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8ec),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x475383);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8f0),32));
            c.push(m,c.get(1,32));
            pc = 0x4754b7; continue;
            }
            0x47538f => {
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8f0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x475464; continue; }
            pc = 0x4753a6; continue;
            }
            0x4753a6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8ec),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x475464; continue; }
            pc = 0x4753b4; continue;
            }
            0x4753b4 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4753bb);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x908),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8cc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x874));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m, 0x4753db);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m, 0x4753e4);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m, 0x4753ed);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m, 0x4753f6);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x475401);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,c.r[6].wrapping_add(0x838));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8ec),32));
            c.set(3,32,c.get(0,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x475438; continue; }
            pc = 0x475427; continue;
            }
            0x475427 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x47542f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x8ec),32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            pc = 0x4754b7; continue;
            }
            0x475438 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8f0),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x475455; continue; }
            pc = 0x475442; continue;
            }
            0x475442 => {
            { let a = c.get(3,32); let r = c.sub(a,c.get(1,32),32); c.set(3,32,r); }
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x47544c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8f0),32));
            c.push(m,c.get(2,32));
            pc = 0x4754b7; continue;
            }
            0x475455 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x47545d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            pc = 0x4754b7; continue;
            }
            0x475464 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x47546b);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x475474);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m, 0x47547d);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x908),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8cc),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8f0),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4754af; continue; }
            pc = 0x4754a5; continue;
            }
            0x4754a5 => {
            c.push(m,0x0u32);
            c.push(m, 0x4754ac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            pc = 0x4754b7; continue;
            }
            0x4754af => {
            c.push(m,c.get(3,32));
            c.push(m, 0x4754b5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            pc = 0x4754b7; continue;
            }
            0x4754b7 => {
            c.set(1,32,c.r[6].wrapping_add(0x874));
            c.push(m, 0x4754c2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a900, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x4754e4; continue; }
            pc = 0x4754dd; continue;
            }
            0x4754dd => {
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            pc = 0x4754e4; continue;
            }
            0x4754e4 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6cbb00u32,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x475513; continue; }
            pc = 0x475508; continue;
            }
            0x475508 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            pc = 0x47551c; continue;
            }
            0x475513 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x47551c; continue;
            }
            0x47551c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,8); c.and(a,0x3u32,8); }
            if c.cond(4) { pc = 0x475537; continue; }
            pc = 0x475523; continue;
            }
            0x475523 => {
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0x80000003u32,32); c.set(2,32,r); }
            if c.cond(9) { pc = 0x475532; continue; }
            pc = 0x47552d; continue;
            }
            0x47552d => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.or(a,0xfffffffcu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x475532; continue;
            }
            0x475532 => {
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x475537; continue;
            }
            0x475537 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x475542);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x41c5a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x47554a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x475552);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x910));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x475560);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xbd8));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47556e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0xa04));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x47557c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xaec));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47558a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x34),32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcc0),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x904),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,u32::from(c.cond(4))); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x908),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x90c),32));
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x54),32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x910),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x910));
            c.set(2,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x4755f7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9f8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x9fc),32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa00),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x24),32),32); c.set(2,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x4c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x475631); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[3],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x475640); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9f8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x9fc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa00),32));
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x24),32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[6].wrapping_add(0xaec));
            c.set(0,32,c.r[4].wrapping_add(0x4c));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x47567e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xaec),32));
            c.set(1,32,c.r[6].wrapping_add(0xaec));
            { let a = c.get(0,8); let r = c.xor(a,0x1u32,8); c.set(0,8,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x475697); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x47569f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xaec));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4756ad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x910));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4756bb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x8c8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8cc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8d0),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x4756f4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8ec),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x475718; continue; }
            pc = 0x4756fe; continue;
            }
            0x4756fe => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x475705);
            { let args = []; c.r[0] = host.call(m, 0x40abd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x47570e);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x20),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x475718; continue;
            }
            0x475718 => {
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x874));
            c.push(m, 0x475728);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0xd8));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x475735);
            { let args = []; c.r[0] = host.call(m, 0x461480, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x48c));
            c.push(m, 0x475740);
            { let args = []; c.r[0] = host.call(m, 0x461480, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x83c),32));
            c.set(1,32,rm(m,0x6cbb00u32,32));
            c.set(0,32,0x55555556u32);
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(2,32)); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8ec),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x475817; continue; }
            pc = 0x47577c; continue;
            }
            0x47577c => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x840),32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x475789);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x475796);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4757a7);
            { let args = []; c.r[0] = host.call(m, 0x40abd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x4757b0);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x68));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x58));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x4757ce); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4757d7);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x4757e0);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(3,32));
            c.set(3,32,c.r[6].wrapping_add(0xd8));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4757f7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x475803); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8ec),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0xd0));
            c.push(m, 0x475815);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x475849; continue;
            }
            0x475817 => {
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.set(2,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x47583d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x475849); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x475849; continue;
            }
            0x475849 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8f0),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x475914; continue; }
            pc = 0x475859; continue;
            }
            0x475859 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x87c),32));
            c.set(3,32,c.r[6].wrapping_add(0x874));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(3,32));
            c.push(m, 0x475870);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x475881);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(3,32));
            c.push(m, 0x475892);
            { let args = []; c.r[0] = host.call(m, 0x40abd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x47589d);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x68));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x58));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x4758bf); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,32,c.r[6].wrapping_add(0x874));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4758ce);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4758d9);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x48c));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4758f4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x475900); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8f0),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x484));
            c.push(m, 0x475912);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47594c; continue;
            }
            0x475914 => {
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.set(2,32,c.r[4].wrapping_add(0x70));
            c.set(3,32,c.r[6].wrapping_add(0x48c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x7c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x475940);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x460ef0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x47594c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47594c; continue;
            }
            0x47594c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8ec),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x475964; continue; }
            pc = 0x475956; continue;
            }
            0x475956 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x47595d);
            { let args = []; c.r[0] = host.call(m, 0x475c10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x475964);
            { let args = []; c.r[0] = host.call(m, 0x476dc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x475964; continue;
            }
            0x475964 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x83c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x864),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbd8),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x910));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.push(m,0x0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x47598f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b2aa4u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4759b2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,32,c.r[6].wrapping_add(0xbd8));
            c.set(1,32,c.get(3,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xa));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x4759ca); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xccc),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6cc91c),32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4759e1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4759e9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xeac));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4759f7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xfa4));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x475a05);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4eb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x475a15);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x40aa40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x475a1c);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x475a25);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            c.set(1,32,c.r[6].wrapping_add(0xcd4));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x475a3a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xcd0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,0x6cbaf4u32,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(2,32); c.sub(a,0x1eu32,32); }
            if c.cond(14) { pc = 0x475a9a; continue; }
            pc = 0x475a71; continue;
            }
            0x475a71 => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x475a9a; continue; }
            pc = 0x475a76; continue;
            }
            0x475a76 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.add(a,0x1e0u32,32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[2],32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(2,32)); }
            pc = 0x475a9a; continue;
            }
            0x475a9a => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(3,32,c.r[6].wrapping_add(0xeac));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[3].wrapping_add(0xe8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.set(2,32,c.r[3].wrapping_add(0xec));
            { let a = c.r[3].wrapping_add(0xec); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x475ad3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x54),32); c.push(m, 0x475adf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x448),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(5,32));
            c.push(m, 0x475af0);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,0x6cbaf4u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.set(5,32,c.r[6].wrapping_add(0xfa4));
            c.set(1,32,c.r[5].wrapping_add(0xec));
            { let a = c.r[5].wrapping_add(0xe8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x475b3c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x54),32); c.push(m, 0x475b49); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x475b51);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x475b59);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8b0),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8ec),32));
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x475b72; continue; }
            pc = 0x475b6c; continue;
            }
            0x475b6c => {
            { let a = c.r[6].wrapping_add(0x8b0); wm(m,a,32,c.get(0,32)); }
            pc = 0x475b72; continue;
            }
            0x475b72 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8b4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8f0),32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x475b88; continue; }
            pc = 0x475b82; continue;
            }
            0x475b82 => {
            { let a = c.r[6].wrapping_add(0x8b4); wm(m,a,32,c.get(0,32)); }
            pc = 0x475b88; continue;
            }
            0x475b88 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x68u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_list_window_place_row_buttons(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x476dc0;
    loop {
        match pc {
            0x476dc0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x476dd3);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(3,32,c.r[6].wrapping_add(0x838));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x476de9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x40aa40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xcd4),32));
            c.set(1,32,c.r[6].wrapping_add(0xcd4));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x476df8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xcd0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x476e11);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x476e1a);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x47c),32));
            { let a = c.get(7,32); let r = c.imul(a,rm(m,c.r[6].wrapping_add(0x474),32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8ec),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(5,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            if c.cond(6) { pc = 0x476efc; continue; }
            pc = 0x476e49; continue;
            }
            0x476e49 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x476e4b; continue;
            }
            0x476e4b => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x474),32),32); }
            if c.cond(12) { pc = 0x476e87; continue; }
            pc = 0x476e53; continue;
            }
            0x476e53 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x478),32),32); }
            if c.cond(13) { pc = 0x476e87; continue; }
            pc = 0x476e5b; continue;
            }
            0x476e5b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbd4),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x476e6f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xbd4),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xc8),32); c.push(m, 0x476e7f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x476ec3; continue; }
            pc = 0x476e83; continue;
            }
            0x476e83 => {
            c.push(m,0x1u32);
            pc = 0x476e9f; continue;
            }
            0x476e87 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbd4),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let target = rm(m,c.r[2].wrapping_add(0xc8),32); c.push(m, 0x476e99); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x476ec3; continue; }
            pc = 0x476e9d; continue;
            }
            0x476e9d => {
            c.push(m,0x0u32);
            pc = 0x476e9f; continue;
            }
            0x476e9f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbd4),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x476eb1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xbd4),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[7]),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x476ec3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x476ec3; continue;
            }
            0x476ec3 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x476eca);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(3,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x476ed3);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8ec),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x476e4b; continue; }
            pc = 0x476efc; continue;
            }
            0x476efc => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn build_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x478920;
    loop {
        match pc {
            0x478920 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65f84au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,0x6beee0u32,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(1,32));
            c.push(m,0x6b9a9cu32);
            c.push(m,0x6b9a8cu32);
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x2u32); }
            c.push(m, 0x478982);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x478990);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,0x6beee0u32,32));
            c.push(m,c.get(2,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(0,32));
            c.push(m,0x6b9aa4u32);
            c.push(m,0x6b9a8cu32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4789b9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x4789c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x4789e1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xf4c),32));
            c.set(7,32,c.r[6].wrapping_add(0xf4c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x4789f2); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[6].wrapping_add(0xe64));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x100c); wm(m,a,32,0xff000000u32); }
            c.push(m, 0x478a0a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xc94));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x478a18);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0xd7c));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x478a26);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0x5f4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x8u32); }
            pc = 0x478a34; continue;
            }
            0x478a34 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffff40));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x478a42);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[5]; wm(m,a,32,0xff000000u32); }
            c.set(1,32,rm(m,0x66e104u32,32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(5,32); let r = c.add(a,0xe8u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x478a34; continue; }
            pc = 0x478a63; continue;
            }
            0x478a63 => {
            c.set(2,32,rm(m,0x6c2280u32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x478a71);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c2298u32,32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x478a7e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6c22acu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0xc94));
            c.push(m, 0x478a90);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6c22bcu32,32));
            c.set(1,32,c.r[6].wrapping_add(0xd7c));
            c.push(m,c.get(2,32));
            c.push(m, 0x478aa2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9aacu32);
            c.push(m,0x6b9a8cu32);
            c.push(m, 0x478abd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[6].wrapping_add(0x1034); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9ab8u32);
            c.push(m,0x6b9a8cu32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x478ade);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9ac4u32);
            c.push(m,0x6b9a8cu32);
            { let a = c.r[6].wrapping_add(0x1044); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x478aff);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0x1048); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9ad4u32);
            c.push(m,0x6b9a8cu32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x478b20);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[6].wrapping_add(0x1064); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9ae0u32);
            c.push(m,0x6b9a8cu32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x478b41);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9aecu32);
            c.push(m,0x6b9a8cu32);
            { let a = c.r[6].wrapping_add(0x1068); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x478b62);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0x106c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9af8u32);
            c.push(m,0x6b9a8cu32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x478b83);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.set(3,32,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x478b91);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4417d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0xac));
            pc = 0x478b99; continue;
            }
            0x478b99 => {
            c.set(2,32,c.r[3].wrapping_add(c.r[5]));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x478bae);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x478bb5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x478bc1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,0x5u32,32); }
            if c.cond(12) { pc = 0x478b99; continue; }
            pc = 0x478bcd; continue;
            }
            0x478bcd => {
            c.set(0,32,rm(m,0x6dcf20u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dcf20u32);
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x478bdf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1034),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[6].wrapping_add(0x1038); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6dcd28u32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dcd28u32);
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x478c03); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x1038),32));
            c.push(m,0x9cu32);
            c.set(2,32,c.r[6].wrapping_add(0x107c));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x1040); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x103c); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x478c37);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x130),32); c.push(m, 0x478c44); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.push(m, 0x478c52);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x478c60);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x478c71);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn build_side_menu_show_buttons(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x479150;
    loop {
        match pc {
            0x479150 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.r[1].wrapping_add(0x534));
            c.set(7,32,c.r[1].wrapping_add(0xc74));
            c.set(3,32,0x8u32);
            pc = 0x479164; continue;
            }
            0x479164 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x47917a; continue; }
            pc = 0x47916a; continue;
            }
            0x47916a => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x479176); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            pc = 0x4791cd; continue;
            }
            0x47917a => {
            { let a = c.get(0,32); c.sub(a,0x32u32,32); }
            if c.cond(4) { pc = 0x479199; continue; }
            pc = 0x47917f; continue;
            }
            0x47917f => {
            { let a = c.get(0,32); c.sub(a,0x33u32,32); }
            if c.cond(4) { pc = 0x479199; continue; }
            pc = 0x479184; continue;
            }
            0x479184 => {
            { let a = c.get(0,32); c.sub(a,0x36u32,32); }
            if c.cond(4) { pc = 0x479199; continue; }
            pc = 0x479189; continue;
            }
            0x479189 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x479195); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            pc = 0x4791cd; continue;
            }
            0x479199 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4791a5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc324),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.set(1,8,!a & 0xffu32); }
            { let a = c.get(1,8); let r = c.and(a,0x1u32,8); c.set(1,8,r); }
            c.push(m,c.get(1,32));
            pc = 0x4791cd; continue;
            }
            0x4791cd => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4791d4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x479164; continue; }
            pc = 0x4791e0; continue;
            }
            0x4791e0 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn build_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x479390;
    loop {
        match pc {
            0x479390 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4793a0);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x66d504u32,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xa4),8));
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x4793f1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1054),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4794b1; continue; }
            pc = 0x4793ff; continue;
            }
            0x4793ff => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xf4c),32));
            c.set(7,32,c.r[6].wrapping_add(0xf4c));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x479415); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x479437); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x479443); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1070),32));
            { let a = c.get(0,32); c.sub(a,0x36u32,32); }
            if c.cond(12) { pc = 0x47945b; continue; }
            pc = 0x47944e; continue;
            }
            0x47944e => {
            { let a = c.get(0,32); c.sub(a,0x3au32,32); }
            if c.cond(13) { pc = 0x47945b; continue; }
            pc = 0x479453; continue;
            }
            0x479453 => {
            c.set(7,32,c.r[6].wrapping_add(0xe64));
            pc = 0x479466; continue;
            }
            0x47945b => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x4794a4; continue; }
            pc = 0x479460; continue;
            }
            0x479460 => {
            c.set(7,32,c.r[6].wrapping_add(0xc94));
            pc = 0x479466; continue;
            }
            0x479466 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x479472); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,c.get(4,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x479493); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47949f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x479528; continue;
            }
            0x4794a4 => {
            { let a = c.get(0,32); c.sub(a,0x28u32,32); }
            if c.cond(5) { pc = 0x479528; continue; }
            pc = 0x4794a9; continue;
            }
            0x4794a9 => {
            c.set(7,32,c.r[6].wrapping_add(0xd7c));
            pc = 0x479466; continue;
            }
            0x4794b1 => {
            c.set(0,32,c.r[6].wrapping_add(0xc74));
            c.set(7,32,c.r[6].wrapping_add(0x534));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x8u32); }
            pc = 0x4794c9; continue;
            }
            0x4794c9 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(12) { pc = 0x47950c; continue; }
            pc = 0x4794d2; continue;
            }
            0x4794d2 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4794de); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x479500); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47950c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47950c; continue;
            }
            0x47950c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4794c9; continue; }
            pc = 0x479528; continue;
            }
            0x479528 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(1,32));
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x47953c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn build_side_menu_update(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x479f50;
    loop {
        match pc {
            0x479f50 => {
            { let a = c.get(4,32); let r = c.sub(a,0x38u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x479f62);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x479f6f);
            { let args = []; c.r[0] = host.call(m, 0x4415d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6e51acu32,32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xc324),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            if c.cond(2) { pc = 0x479f8d; continue; }
            pc = 0x479f87; continue;
            }
            0x479f87 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            c.set(3,32,c.get(7,32));
            pc = 0x479f8d; continue;
            }
            0x479f8d => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),32));
            c.set(5,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.and(a,0xe0u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a077; continue; }
            pc = 0x479fb6; continue;
            }
            0x479fb6 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.set(7,32,c.r[6].wrapping_add(0xac));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x479fc7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x479fd4; continue; }
            pc = 0x479fcb; continue;
            }
            0x479fcb => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x479fd4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x479fd4; continue;
            }
            0x479fd4 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x364),32));
            c.set(7,32,c.r[6].wrapping_add(0x364));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x479fe5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47a030; continue; }
            pc = 0x479fed; continue;
            }
            0x479fed => {
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x47a001);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x47a030; continue; }
            pc = 0x47a005; continue;
            }
            0x47a005 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x194),32));
            c.set(1,32,c.r[6].wrapping_add(0x194));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47a016); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a01f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x44c),32));
            c.set(1,32,c.r[6].wrapping_add(0x44c));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47a030); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a030; continue;
            }
            0x47a030 => {
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x6f1),8));
            c.set(1,32,c.get(7,32));
            c.set(5,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a041); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,c.get(0,8));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x47a181; continue; }
            pc = 0x47a05d; continue;
            }
            0x47a05d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x194),32));
            c.set(1,32,c.r[6].wrapping_add(0x194));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47a072); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a181; continue;
            }
            0x47a077 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x6f1),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x47a267; continue; }
            pc = 0x47a08c; continue;
            }
            0x47a08c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x194),32));
            c.set(7,32,c.r[6].wrapping_add(0x194));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a09d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47a0aa; continue; }
            pc = 0x47a0a1; continue;
            }
            0x47a0a1 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a0aa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a0aa; continue;
            }
            0x47a0aa => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x27c),32));
            c.set(7,32,c.r[6].wrapping_add(0x27c));
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x76c),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shr(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,8); let r = c.and(a,0x1u32,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(3,8)); }
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a0cf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x47a0df; continue; }
            pc = 0x47a0d3; continue;
            }
            0x47a0d3 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a0df); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a0df; continue;
            }
            0x47a0df => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x47a0f7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.set(7,32,c.r[6].wrapping_add(0xac));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x47a215; continue; }
            pc = 0x47a10b; continue;
            }
            0x47a10b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,rm(m,c.r[1].wrapping_add(0x76c),32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a11a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x47a141; continue; }
            pc = 0x47a12a; continue;
            }
            0x47a12a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x76c),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(1,8,u32::from(c.cond(5)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47a141); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a141; continue;
            }
            0x47a141 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x364),32));
            c.set(7,32,c.r[6].wrapping_add(0x364));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a152); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a15f; continue; }
            pc = 0x47a156; continue;
            }
            0x47a156 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a15f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a15f; continue;
            }
            0x47a15f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x44c),32));
            c.set(7,32,c.r[6].wrapping_add(0x44c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a170); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a17d; continue; }
            pc = 0x47a174; continue;
            }
            0x47a174 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            pc = 0x47a178; continue;
            }
            0x47a178 => {
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a17d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a17d; continue;
            }
            0x47a17d => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x47a181; continue;
            }
            0x47a181 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            pc = 0x47a187; continue;
            }
            0x47a187 => {
            c.set(1,32,c.r[7].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0xac));
            c.push(m, 0x47a19f);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a4d4; continue; }
            pc = 0x47a1a7; continue;
            }
            0x47a1a7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x104c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x47a1b8);
            { let args = []; c.r[0] = host.call(m, 0x47ab20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,0x8u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x1u32); }
            c.set(7,32,c.r[6].wrapping_add(0x534));
            c.set(5,32,c.r[6].wrapping_add(0xc74));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            pc = 0x47a1d5; continue;
            }
            0x47a1d5 => {
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); c.sub(a,0x32u32,32); }
            if c.cond(12) { pc = 0x47a3c3; continue; }
            pc = 0x47a1e1; continue;
            }
            0x47a1e1 => {
            { let a = c.get(0,32); c.sub(a,0x36u32,32); }
            if c.cond(15) { pc = 0x47a3c3; continue; }
            pc = 0x47a1ea; continue;
            }
            0x47a1ea => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47a1f6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x2c8),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.set(2,8,!a & 0xffu32); }
            { let a = c.get(2,8); let r = c.and(a,0x1u32,8); c.set(2,8,r); }
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a210); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a3e8; continue;
            }
            0x47a215 => {
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a21a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47a227; continue; }
            pc = 0x47a21e; continue;
            }
            0x47a21e => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a227); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a227; continue;
            }
            0x47a227 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x364),32));
            c.set(7,32,c.r[6].wrapping_add(0x364));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a238); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47a245; continue; }
            pc = 0x47a23c; continue;
            }
            0x47a23c => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a245); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a245; continue;
            }
            0x47a245 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x44c),32));
            c.set(7,32,c.r[6].wrapping_add(0x44c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a256); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47a17d; continue; }
            pc = 0x47a25e; continue;
            }
            0x47a25e => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            pc = 0x47a178; continue;
            }
            0x47a267 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x76c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.set(7,32,c.r[6].wrapping_add(0xac));
            { let a = c.get(4,8); c.and(a,0x1u32,8); }
            c.set(1,32,c.get(7,32));
            if c.cond(5) { pc = 0x47a315; continue; }
            pc = 0x47a284; continue;
            }
            0x47a284 => {
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a287); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a294; continue; }
            pc = 0x47a28b; continue;
            }
            0x47a28b => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a294); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a294; continue;
            }
            0x47a294 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x194),32));
            c.set(7,32,c.r[6].wrapping_add(0x194));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a2a5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a2b2; continue; }
            pc = 0x47a2a9; continue;
            }
            0x47a2a9 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a2b2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a2b2; continue;
            }
            0x47a2b2 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x27c),32));
            c.set(7,32,c.r[6].wrapping_add(0x27c));
            c.set(1,32,c.get(7,32));
            pc = 0x47a2c0; continue;
            }
            0x47a2c0 => {
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a2c3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a2d0; continue; }
            pc = 0x47a2c7; continue;
            }
            0x47a2c7 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a2d0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a2d0; continue;
            }
            0x47a2d0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x364),32));
            c.set(7,32,c.r[6].wrapping_add(0x364));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a2e1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a2ee; continue; }
            pc = 0x47a2e5; continue;
            }
            0x47a2e5 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a2ee); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a2ee; continue;
            }
            0x47a2ee => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x44c),32));
            c.set(7,32,c.r[6].wrapping_add(0x44c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a2ff); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a181; continue; }
            pc = 0x47a307; continue;
            }
            0x47a307 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a310); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a181; continue;
            }
            0x47a315 => {
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a318); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47a325; continue; }
            pc = 0x47a31c; continue;
            }
            0x47a31c => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a325); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a325; continue;
            }
            0x47a325 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x27c),32));
            c.set(7,32,c.r[6].wrapping_add(0x27c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a336); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a343; continue; }
            pc = 0x47a33a; continue;
            }
            0x47a33a => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a343); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a343; continue;
            }
            0x47a343 => {
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x47a358);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x194),32));
            c.set(7,32,c.r[6].wrapping_add(0x194));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,32,c.get(7,32));
            if c.cond(4) { pc = 0x47a2c0; continue; }
            pc = 0x47a36e; continue;
            }
            0x47a36e => {
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a371); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47a37e; continue; }
            pc = 0x47a375; continue;
            }
            0x47a375 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a37e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a37e; continue;
            }
            0x47a37e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x364),32));
            c.set(7,32,c.r[6].wrapping_add(0x364));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a38f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47a39c; continue; }
            pc = 0x47a393; continue;
            }
            0x47a393 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a39c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a39c; continue;
            }
            0x47a39c => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x44c),32));
            c.set(7,32,c.r[6].wrapping_add(0x44c));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x47a3ad); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47a181; continue; }
            pc = 0x47a3b5; continue;
            }
            0x47a3b5 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a3be); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a181; continue;
            }
            0x47a3c3 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            if c.cond(12) { pc = 0x47a3d7; continue; }
            pc = 0x47a3cb; continue;
            }
            0x47a3cb => {
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x47a3d3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            pc = 0x47a3e1; continue;
            }
            0x47a3d7 => {
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x47a3df); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            pc = 0x47a3e1; continue;
            }
            0x47a3e1 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47a3e8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x47a3e8; continue;
            }
            0x47a3e8 => {
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x47a1d5; continue; }
            pc = 0x47a3f8; continue;
            }
            0x47a3f8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1054),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x47a450; continue; }
            pc = 0x47a402; continue;
            }
            0x47a402 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1058),32));
            { let a = c.r[6].wrapping_add(0x1054); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x47a42d; continue; }
            pc = 0x47a416; continue;
            }
            0x47a416 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.push(m, 0x47a423);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58cc90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1058); wm(m,a,32,0x0u32); }
            pc = 0x47a42d; continue;
            }
            0x47a42d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x105c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x47a450; continue; }
            pc = 0x47a437; continue;
            }
            0x47a437 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x47a446);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x593800, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[6].wrapping_add(0x105c); wm(m,a,32,0x0u32); }
            pc = 0x47a450; continue;
            }
            0x47a450 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe64),32));
            c.set(7,32,c.r[6].wrapping_add(0xe64));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a463); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47a46f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf4c),32));
            c.set(7,32,c.r[6].wrapping_add(0xf4c));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a482); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47a48e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc94),32));
            c.set(7,32,c.r[6].wrapping_add(0xc94));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a4a1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47a4ad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd7c),32));
            c.set(7,32,c.r[6].wrapping_add(0xd7c));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a4c0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47a4cc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x47a4d4; continue;
            }
            0x47a4d4 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x5u32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            if c.cond(12) { pc = 0x47a187; continue; }
            pc = 0x47a4e2; continue;
            }
            0x47a4e2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1054),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x47a7af; continue; }
            pc = 0x47a4f0; continue;
            }
            0x47a4f0 => {
            c.set(7,32,c.r[6].wrapping_add(0xf4c));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x47a4fd);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a643; continue; }
            pc = 0x47a505; continue;
            }
            0x47a505 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x1054); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a518); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47a524); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe64),32));
            c.set(7,32,c.r[6].wrapping_add(0xe64));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a537); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47a543); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc94),32));
            c.set(7,32,c.r[6].wrapping_add(0xc94));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a556); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47a562); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd7c),32));
            c.set(7,32,c.r[6].wrapping_add(0xd7c));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a575); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47a581); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x534));
            c.set(5,32,c.r[6].wrapping_add(0xc74));
            c.set(3,32,0x8u32);
            pc = 0x47a592; continue;
            }
            0x47a592 => {
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); c.sub(a,0x32u32,32); }
            if c.cond(4) { pc = 0x47a5c4; continue; }
            pc = 0x47a59a; continue;
            }
            0x47a59a => {
            { let a = c.get(0,32); c.sub(a,0x33u32,32); }
            if c.cond(4) { pc = 0x47a5c4; continue; }
            pc = 0x47a59f; continue;
            }
            0x47a59f => {
            { let a = c.get(0,32); c.sub(a,0x36u32,32); }
            if c.cond(4) { pc = 0x47a5c4; continue; }
            pc = 0x47a5a4; continue;
            }
            0x47a5a4 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            if c.cond(12) { pc = 0x47a5b8; continue; }
            pc = 0x47a5ac; continue;
            }
            0x47a5ac => {
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x47a5b4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            pc = 0x47a5e3; continue;
            }
            0x47a5b8 => {
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x47a5c0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            pc = 0x47a5e3; continue;
            }
            0x47a5c4 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x47a5d0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x2c8),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.set(1,8,!a & 0xffu32); }
            { let a = c.get(1,8); let r = c.and(a,0x1u32,8); c.set(1,8,r); }
            c.push(m,c.get(1,32));
            pc = 0x47a5e3; continue;
            }
            0x47a5e3 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47a5ea); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x47a592; continue; }
            pc = 0x47a5f6; continue;
            }
            0x47a5f6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1058),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x47a617; continue; }
            pc = 0x47a600; continue;
            }
            0x47a600 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.push(m, 0x47a60d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58cc90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1058); wm(m,a,32,0x0u32); }
            pc = 0x47a617; continue;
            }
            0x47a617 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x105c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x47a778; continue; }
            pc = 0x47a625; continue;
            }
            0x47a625 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x47a634);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x593800, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[6].wrapping_add(0x105c); wm(m,a,32,0x0u32); }
            pc = 0x47a778; continue;
            }
            0x47a643 => {
            c.set(1,32,c.r[6].wrapping_add(0xe64));
            c.push(m, 0x47a64e);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a6a5; continue; }
            pc = 0x47a652; continue;
            }
            0x47a652 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x766),16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x776),16),16); }
            if c.cond(3) { pc = 0x47a770; continue; }
            pc = 0x47a66a; continue;
            }
            0x47a66a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1070),32));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,0x36u32,32); }
            c.set(1,8,u32::from(c.cond(5)));
            c.set(2,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x24u32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x47a6a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a778; continue;
            }
            0x47a6a5 => {
            c.set(1,32,c.r[6].wrapping_add(0xc94));
            c.push(m, 0x47a6b0);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a6dd; continue; }
            pc = 0x47a6b4; continue;
            }
            0x47a6b4 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0x25u32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x47a6d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a778; continue;
            }
            0x47a6dd => {
            c.set(1,32,c.r[6].wrapping_add(0xd7c));
            c.push(m, 0x47a6e8);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a730; continue; }
            pc = 0x47a6ec; continue;
            }
            0x47a6ec => {
            c.set(2,8,rm(m,0x6dfc14u32,8));
            c.set(0,32,c.r[4].wrapping_add(0x3b));
            c.set(1,32,c.r[4].wrapping_add(0x3a));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x22u32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x41); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,0x28u32); }
            { let a = c.r[4].wrapping_add(0x45); wm(m,a,8,0x0u32); }
            c.push(m, 0x47a71f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5250d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x47a72e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a778; continue;
            }
            0x47a730 => {
            c.set(0,32,rm(m,0x6dfc18u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x47a757; continue; }
            pc = 0x47a739; continue;
            }
            0x47a739 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x107c),32));
            c.set(1,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x47a757; continue; }
            pc = 0x47a749; continue;
            }
            0x47a749 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x7a4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x47a770; continue; }
            pc = 0x47a757; continue;
            }
            0x47a757 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[2].wrapping_add(0x7a4); wm(m,a,32,0x0u32); }
            c.push(m, 0x47a76c);
            { let args = []; c.r[0] = host.call(m, 0x47b3c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x20),32); let r = c.or(a,c.get(0,32),32); { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,r); } }
            pc = 0x47a770; continue;
            }
            0x47a770 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x47a7a7; continue; }
            pc = 0x47a778; continue;
            }
            0x47a778 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x47a7a7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a7a7; continue;
            }
            0x47a7a7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x47a7af => {
            c.set(1,32,c.r[6].wrapping_add(0x534));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0xc74));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            pc = 0x47a7c1; continue;
            }
            0x47a7c1 => {
            c.push(m, 0x47a7c6);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47a7cf; continue; }
            pc = 0x47a7ca; continue;
            }
            0x47a7ca => {
            { let a = rm(m,c.r[7],32); c.sub(a,0x0u32,32); }
            if c.cond(13) { pc = 0x47a7e8; continue; }
            pc = 0x47a7cf; continue;
            }
            0x47a7cf => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xe8u32,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,0x8u32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x47a7c1; continue; }
            pc = 0x47a7e6; continue;
            }
            0x47a7e6 => {
            pc = 0x47a770; continue;
            }
            0x47a7e8 => {
            c.set(7,32,c.r[6].wrapping_add(0x534));
            c.set(3,32,0x8u32);
            pc = 0x47a7f3; continue;
            }
            0x47a7f3 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a7fc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47a808); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x47a7f3; continue; }
            pc = 0x47a811; continue;
            }
            0x47a811 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xf4c),32));
            c.set(7,32,c.r[6].wrapping_add(0xf4c));
            c.set(3,32,0x1u32);
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47a828); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47a833); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0xc74),32));
            c.set(1,16,c.get(0,16));
            { let a = c.r[6].wrapping_add(0x1070); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1154); wm(m,a,16,c.get(1,16)); }
            c.set(2,16,rm(m,0x6dfc14u32,16));
            c.set(1,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x1156); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(1,32); c.sub(a,0x36u32,32); }
            if c.cond(13) { pc = 0x47aa02; continue; }
            pc = 0x47a863; continue;
            }
            0x47a863 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x47a871; continue; }
            pc = 0x47a869; continue;
            }
            0x47a869 => {
            c.set(0,32,rm(m,0x6c22e8u32,32));
            c.push(m,c.get(0,32));
            pc = 0x47a8a2; continue;
            }
            0x47a871 => {
            { let a = c.get(1,32); c.sub(a,0x34u32,32); }
            if c.cond(4) { pc = 0x47a89b; continue; }
            pc = 0x47a876; continue;
            }
            0x47a876 => {
            { let a = c.get(1,32); c.sub(a,0x35u32,32); }
            if c.cond(4) { pc = 0x47a89b; continue; }
            pc = 0x47a87b; continue;
            }
            0x47a87b => {
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a57c8),32));
            c.set(2,32,rm(m,0x6c22ccu32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[6].wrapping_add(0x1078));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47a896);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x47a8ad; continue;
            }
            0x47a89b => {
            c.set(1,32,rm(m,0x6b23bcu32,32));
            c.push(m,c.get(1,32));
            pc = 0x47a8a2; continue;
            }
            0x47a8a2 => {
            c.set(1,32,c.r[6].wrapping_add(0x1078));
            c.push(m, 0x47a8ad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x47a8ad; continue;
            }
            0x47a8ad => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[6].wrapping_add(0x1118));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x47a8bf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1070),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x47a8fd; continue; }
            pc = 0x47a8cf; continue;
            }
            0x47a8cf => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0x1058); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x47a8f3; continue; }
            pc = 0x47a8d9; continue;
            }
            0x47a8d9 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x792),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x790),16),16); }
            if c.cond(6) { pc = 0x47a8f3; continue; }
            pc = 0x47a8e9; continue;
            }
            0x47a8e9 => {
            c.set(0,8,0x1u32);
            c.set(7,32,c.r[6].wrapping_add(0xc94));
            pc = 0x47a936; continue;
            }
            0x47a8f3 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            c.set(7,32,c.r[6].wrapping_add(0xc94));
            pc = 0x47a936; continue;
            }
            0x47a8fd => {
            { let a = c.get(1,32); c.sub(a,0x28u32,32); }
            if c.cond(5) { pc = 0x47a95b; continue; }
            pc = 0x47a902; continue;
            }
            0x47a902 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0x1058); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x47a92e; continue; }
            pc = 0x47a90c; continue;
            }
            0x47a90c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x789),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x2d4),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x47a92e; continue; }
            pc = 0x47a92a; continue;
            }
            0x47a92a => {
            c.set(0,8,0x1u32);
            pc = 0x47a930; continue;
            }
            0x47a92e => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x47a930; continue;
            }
            0x47a930 => {
            c.set(7,32,c.r[6].wrapping_add(0xd7c));
            pc = 0x47a936; continue;
            }
            0x47a936 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47a93e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x47a949); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x47a950);
            { let args = []; c.r[0] = host.call(m, 0x47b270, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x1054); wm(m,a,32,c.get(3,32)); }
            pc = 0x47a778; continue;
            }
            0x47a95b => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47ab06; continue; }
            pc = 0x47a963; continue;
            }
            0x47a963 => {
            { let a = c.r[6].wrapping_add(0x1058); wm(m,a,32,c.get(3,32)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x672fc0),8));
            { let a = c.get(0,8); c.sub(a,0x2cu32,8); }
            c.push(m,c.get(3,32));
            if c.cond(5) { pc = 0x47a99b; continue; }
            pc = 0x47a974; continue;
            }
            0x47a974 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x47a98a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x47a991);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f74f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x2cu32);
            pc = 0x47a9ce; continue;
            }
            0x47a99b => {
            { let a = c.get(1,32); c.sub(a,0x1fu32,32); }
            if c.cond(5) { pc = 0x47a9a3; continue; }
            pc = 0x47a9a0; continue;
            }
            0x47a9a0 => {
            c.push(m,c.get(5,32));
            pc = 0x47a9ca; continue;
            }
            0x47a9a3 => {
            { let a = c.get(1,32); c.sub(a,0x1eu32,32); }
            if c.cond(5) { pc = 0x47a9b0; continue; }
            pc = 0x47a9a8; continue;
            }
            0x47a9a8 => {
            c.set(1,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,0x20u32);
            c.push(m,c.get(1,32));
            pc = 0x47a9ce; continue;
            }
            0x47a9b0 => {
            { let a = c.get(0,8); c.sub(a,0x29u32,8); }
            if c.cond(5) { pc = 0x47a9b9; continue; }
            pc = 0x47a9b4; continue;
            }
            0x47a9b4 => {
            c.push(m,c.get(5,32));
            c.push(m,0x29u32);
            pc = 0x47a9ce; continue;
            }
            0x47a9b9 => {
            { let a = c.get(1,32); c.sub(a,0x34u32,32); }
            if c.cond(5) { pc = 0x47a9f7; continue; }
            pc = 0x47a9be; continue;
            }
            0x47a9be => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x677929u32,8));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            pc = 0x47a9ca; continue;
            }
            0x47a9ca => {
            c.set(0,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            pc = 0x47a9ce; continue;
            }
            0x47a9ce => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x47a9d9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5928a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47a9e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58cc90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x47a9ec);
            { let args = []; c.r[0] = host.call(m, 0x47b270, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x1054); wm(m,a,32,c.get(3,32)); }
            pc = 0x47a778; continue;
            }
            0x47a9f7 => {
            c.set(1,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,0x200000u32);
            c.push(m,c.get(1,32));
            pc = 0x47a9ce; continue;
            }
            0x47aa02 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x766),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x776),16),16); }
            if c.cond(3) { pc = 0x47aa8d; continue; }
            pc = 0x47aa16; continue;
            }
            0x47aa16 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x1074); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x47aa29);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x3),8));
            c.set(0,16,c.get(2,16));
            c.set(1,32,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x1070); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x1154); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a57c8),32));
            c.set(0,32,rm(m,0x6c22ccu32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x1078));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x47aa5a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[6].wrapping_add(0x1118));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x47aa6b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x2c8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.set(1,8,!a & 0xffu32); }
            { let a = c.get(1,8); let r = c.and(a,0x1u32,8); c.set(1,8,r); }
            { let a = c.get(0,8); let r = c.and(a,c.get(1,8),8); c.set(0,8,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x47aa8b);
            { let args = []; c.r[0] = host.call(m, 0x522a00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x47aac8; continue;
            }
            0x47aa8d => {
            c.set(2,32,c.r[6].wrapping_add(0x1118));
            c.set(5,32,c.r[1].wrapping_add(0xffffffff));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(5,32); let r = c.and(a,0x3u32,32); c.set(5,32,r); }
            c.push(m, 0x47aaa1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5226c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c230cu32,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[6].wrapping_add(0x1078));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x47aab4);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            c.set(1,32,c.r[7].wrapping_add(0x7a4));
            c.push(m,c.get(5,32));
            c.push(m, 0x47aac4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fcfe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            pc = 0x47aac8; continue;
            }
            0x47aac8 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xe64),32));
            c.set(7,32,c.r[6].wrapping_add(0xe64));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x105c); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47aae4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x47aaef); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.push(m, 0x47aafc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58cc90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x1058); wm(m,a,32,0x0u32); }
            pc = 0x47ab06; continue;
            }
            0x47ab06 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x47ab0d);
            { let args = []; c.r[0] = host.call(m, 0x47b270, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x1054); wm(m,a,32,c.get(3,32)); }
            pc = 0x47a778; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn personal_letters_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x47c9b0;
    loop {
        match pc {
            0x47c9b0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65faa8u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,0x6beee0u32,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(1,32));
            c.push(m,0x6b9ba4u32);
            c.push(m,0x6b9bacu32);
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0x0u32); }
            c.push(m, 0x47ca00);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x47ca0e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,0x6beee0u32,32));
            c.push(m,c.get(2,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(0,32));
            c.push(m,0x6b9bbcu32);
            c.push(m,0x6b9bc4u32);
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x1u32); }
            c.push(m, 0x47ca3c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x47ca4a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x2u32); }
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9bd4u32);
            c.push(m,0x6b9be4u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x47ca6a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x47ca78);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4417d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,0x1u32);
            c.set(7,32,c.r[6].wrapping_add(0x1c0));
            pc = 0x47ca83; continue;
            }
            0x47ca83 => {
            c.set(0,32,c.r[3].wrapping_add(c.r[5]));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x47ca98);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x47ca9f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x47caab); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,0x6u32,32); }
            if c.cond(12) { pc = 0x47ca83; continue; }
            pc = 0x47cab7; continue;
            }
            0x47cab7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9bf4u32);
            c.push(m,0x6b9c00u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x47cad2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x47cae4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x650),32));
            c.set(7,32,c.r[6].wrapping_add(0x650));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x47caf5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(1,32));
            c.push(m,0x1u32);
            c.push(m,0x6b9c10u32);
            c.push(m,0x6b9c1cu32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x47cb10);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9c2cu32);
            c.push(m,0x6b9c38u32);
            { let a = c.r[6].wrapping_add(0xc4); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x47cb32);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[6].wrapping_add(0xc6); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9c48u32);
            c.push(m,0x6b9c54u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x47cb54);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x47cb66);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x9fc),32));
            c.set(1,32,c.r[6].wrapping_add(0x9fc));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x47cb75); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc6),16));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x47cba0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x2c),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc6),16));
            { let a = c.r[6].wrapping_add(0xc2); wm(m,a,16,c.get(1,16)); }
            c.push(m,0x6b9c64u32);
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x47cbd2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            c.push(m,0x6b9c68u32);
            { let a = c.r[6].wrapping_add(0xbc); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xc4),16));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x47cc02);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2c),16));
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0xbc),16),16); c.set(0,16,r); }
            { let a = c.get(1,16); let r = c.sub(a,0x6u32,16); c.set(1,16,r); }
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xca); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0xbe); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x47cc29);
            { let args = []; c.r[0] = host.call(m, 0x461fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x2c),16));
            { let a = c.get(2,16); let r = c.sub(a,c.get(0,16),16); c.set(2,16,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x8u32,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,0x6cbd5cu32,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6cbd54u32,32));
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xc2),16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x1d64); wm(m,a,8,0x0u32); }
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,16,c.get(0,16)); }
            { let target = rm(m,c.r[2].wrapping_add(0x130),32); c.push(m, 0x47cc7a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.push(m, 0x47cc88);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x47cc96);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x47cca7);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn personal_letters_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x47daa0;
    loop {
        match pc {
            0x47daa0 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x47dab0);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa4),8));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(4,32));
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x47db08); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbd94u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x650));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x47db27);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x461fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x47db45);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
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

pub fn ship_side_menu_hide(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x488460;
    loop {
        match pc {
            0x488460 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0xau32);
            c.push(m, 0x488470);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42d6d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cba98u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x48847e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x488490; continue; }
            pc = 0x488482; continue;
            }
            0x488482 => {
            c.set(1,32,rm(m,0x6cba98u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x488490); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x488490; continue;
            }
            0x488490 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xb8),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4884ed; continue; }
            pc = 0x4884a0; continue;
            }
            0x4884a0 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(2,32,rm(m,0x66d504u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4884f7; continue; }
            pc = 0x4884b0; continue;
            }
            0x4884b0 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc0),32),32); }
            if c.cond(4) { pc = 0x4884d3; continue; }
            pc = 0x4884b8; continue;
            }
            0x4884b8 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc8),32),32); }
            if c.cond(4) { pc = 0x4884d3; continue; }
            pc = 0x4884c0; continue;
            }
            0x4884c0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2],16));
            { let a = c.r[6].wrapping_add(0x3868); wm(m,a,32,c.get(0,32)); }
            pc = 0x4884f7; continue;
            }
            0x4884d3 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x5238),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            { let a = c.r[6].wrapping_add(0x3868); wm(m,a,32,c.get(0,32)); }
            pc = 0x4884f7; continue;
            }
            0x4884ed => {
            { let a = c.r[6].wrapping_add(0x3868); wm(m,a,32,0xffffffffu32); }
            pc = 0x4884f7; continue;
            }
            0x4884f7 => {
            c.set(1,32,rm(m,0x6d9584u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x488505); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb10u32,32));
            { let a = rm(m,c.r[1].wrapping_add(0x5ac),32); c.sub(a,0x4950d0u32,32); }
            if c.cond(5) { pc = 0x48851f; continue; }
            pc = 0x488517; continue;
            }
            0x488517 => {
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x48851f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x48851f; continue;
            }
            0x48851f => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x488526);
            { let args = []; c.r[0] = host.call(m, 0x440ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x48852f);
            { let args = []; c.r[0] = host.call(m, 0x492580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x488600;
    loop {
        match pc {
            0x488600 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6603cfu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xd0u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0xe8); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf4),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,0x6b9edcu32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0xf4); wm(m,a,8,0x1u32); }
            c.push(m, 0x488654);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xfc),32));
            c.set(1,32,rm(m,0x6beee0u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(1,32));
            c.push(m,0x6b9eecu32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488684);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x488692);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(1,32,rm(m,0x6beee0u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(1,32));
            c.push(m,0x6b9ef4u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x100); wm(m,a,8,0x2u32); }
            c.push(m, 0x4886c7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x4886d5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9f04u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0xf8); wm(m,a,8,0x3u32); }
            c.push(m, 0x4886fb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9f10u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x48871b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9f1cu32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.set(5,32,c.get(0,32));
            c.push(m, 0x48873b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0xd4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9f2cu32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488760);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0xdc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9f40u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488785);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9f54u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4887a7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0xc8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9f64u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4887c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9f70u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4887e9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9f7cu32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x48880b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9f84u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x48882d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9f90u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x48884f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9fa0u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488871);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0xb8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9fb0u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488896);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0x90); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9fc0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4888bb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9fccu32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4888dd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0xd8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9fdcu32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488902);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0x98); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9fe8u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488927);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6b9ff8u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488949);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0xa0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba008u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x48896e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba01cu32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488990);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba028u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0xd0); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4889b5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba034u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4889d7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0xa8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba044u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4889fc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba054u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488a1e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x78); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba068u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488a40);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba078u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488a65);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba084u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0xc0); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488a8a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba094u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488aaf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba0a0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488ad1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba0b8u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0xdc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488af6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba0c4u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488b18);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba0d0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488b3a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba0dcu32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488b5c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba0e8u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488b7e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba0f4u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488ba0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba100u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488bc2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba110u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488be4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba11cu32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488c06);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba128u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488c2b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0x84); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba138u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488c50);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba144u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488c75);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba154u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0xa4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488c9a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x9c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba160u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488cbf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba170u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488ce4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba184u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0xbc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488d09);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0xb4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba194u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488d2e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[4].wrapping_add(0xa4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba1a4u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488d53);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba1b4u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0xd4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488d78);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.r[4].wrapping_add(0xd0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba1c4u32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488d9d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[6].wrapping_add(0x582c); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba1d0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488dc2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba1e0u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[6].wrapping_add(0x582e); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x488de7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0x3864); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba1ecu32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488e0b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[6].wrapping_add(0x3874); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba200u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488e2f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba214u32);
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            { let a = c.r[6].wrapping_add(0x386c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488e53);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0x3870); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba220u32);
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x488e77);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x3878); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x488e8f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2bc),32));
            c.set(7,32,c.r[6].wrapping_add(0x2bc));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x488ea0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x488eb2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            c.set(5,32,c.r[6].wrapping_add(0xec));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x488ec3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x488ed5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1d4),32));
            c.set(3,32,c.r[6].wrapping_add(0x1d4));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x488ee6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xd0),32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x488eff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2258),32));
            c.set(1,32,c.r[6].wrapping_add(0x2258));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x488f0e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xd4),32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x488f27);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x574),32));
            c.set(1,32,c.r[6].wrapping_add(0x574));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x488f36); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xdc),32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x488f4f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3a4),32));
            c.set(1,32,c.r[6].wrapping_add(0x3a4));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x488f5e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x488f74);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x48c),32));
            c.set(1,32,c.r[6].wrapping_add(0x48c));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x488f83); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc8),32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x488f9c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x65c),32));
            c.set(1,32,c.r[6].wrapping_add(0x65c));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x488fab); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x488fc1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3938),32));
            c.set(1,32,c.r[6].wrapping_add(0x3938));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x488fd0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            c.set(2,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x488fe6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3cec),32));
            c.set(1,32,c.r[6].wrapping_add(0x3cec));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x488ff5); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x48900b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5428),32));
            c.set(1,32,c.r[6].wrapping_add(0x5428));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x48901a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,c.r[4].wrapping_add(0xf0));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x489033);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x387c));
            c.push(m, 0x48903e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x40a560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x90),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x489050);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.r[6].wrapping_add(0x3810); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x489065);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xb8),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x3814); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x48907d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x3818); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x489092);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x98),32));
            { let a = c.r[6].wrapping_add(0x380c); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x4890aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x3824); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4890bf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x381c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xd8),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x4890d7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.r[6].wrapping_add(0x3820); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x4890ec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xa0),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x3828); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x489104);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x382c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x489119);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc0),32));
            { let a = c.r[6].wrapping_add(0x3830); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x489131);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x3834); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x489146);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x3838); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xa8),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0xf8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x489168);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x38b8));
            c.push(m, 0x489173);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x40a560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x78),32));
            c.set(2,32,c.r[4].wrapping_add(0xf0));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x48918c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(0x38f4));
            c.push(m, 0x489197);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x40a560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4891a6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x80),32));
            { let a = c.r[6].wrapping_add(0x383c); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x4891be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xb0),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x3840); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4891d6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x3844); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x88),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x4891ee);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[6].wrapping_add(0x3848); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x489203);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x384c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x489218);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x3854); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x48922d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.r[6].wrapping_add(0x3850); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x489242);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x5824); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x489257);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x5858); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x48926c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let a = c.r[6].wrapping_add(0x5854); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x489281);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x585c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x489296);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x5850); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x4892ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x7c),32));
            { let a = c.r[6].wrapping_add(0x5870); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x4892c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x5868); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4892d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x5864); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x4892f0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x94),32));
            { let a = c.r[6].wrapping_add(0x586c); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x489308);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x9c),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x5860); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x489320);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x5880); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xa4),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x489338);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xac),32));
            { let a = c.r[6].wrapping_add(0x588c); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x489350);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xb4),32));
            c.set(1,32,0x6da820u32);
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x5884); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x489368);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x5888); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xbc),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x489380);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc4),32));
            { let a = c.r[6].wrapping_add(0x5890); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da820u32);
            c.push(m, 0x489398);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b3dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            { let a = c.r[6].wrapping_add(0x385c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x6ba238u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x4893b9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4893c6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6ba244u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x4893e1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x4893ed); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,0x6ba250u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x489408);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x489414); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6ba25cu32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x48942f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3a4),32));
            c.set(1,32,c.r[6].wrapping_add(0x3a4));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x489443); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6ba270u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x48945e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x48c),32));
            c.set(1,32,c.r[6].wrapping_add(0x48c));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x489472); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6ba284u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x48948d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x65c),32));
            c.set(1,32,c.r[6].wrapping_add(0x65c));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x4894a1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6ba294u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x4894bc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x574),32));
            c.set(1,32,c.r[6].wrapping_add(0x574));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4894d0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6ba2a4u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x5818));
            c.push(m, 0x4894ed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6ba2b0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x580c));
            c.push(m, 0x48950a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(2,32));
            c.push(m,0x6ba2bcu32);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[6].wrapping_add(0x57ec));
            c.push(m,c.get(0,32));
            c.push(m, 0x489527);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6ba2c8u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0x57e0));
            c.push(m, 0x489544);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6ba2d4u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x57f8));
            c.push(m, 0x489561);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bd0e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x744));
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x489587);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x489590);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0x82c));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x48959e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.r[6].wrapping_add(0x914));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4895ac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x3724));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4895ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[6].wrapping_add(0x36a0));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x4895c8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[6].wrapping_add(0x36c0));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x4895d6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[6].wrapping_add(0x36e0));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x4895e5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3724),32));
            c.set(1,32,c.r[6].wrapping_add(0x3724));
            c.set(0,32,c.r[6].wrapping_add(0x3700));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x4895fb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x1d04));
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x489621);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x1dec));
            c.set(0,8,rm(m,0x66de57u32,8));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x489647);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6ba2ecu32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x489653);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6ba2eeu32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x48965f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x489668);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x489671);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xcc),32));
            c.set(2,32,c.r[4].wrapping_add(0xf0));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x48968d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1ed8),32));
            c.set(7,32,c.r[6].wrapping_add(0x1ed8));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x48969e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[6].wrapping_add(0x2238));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x4896ac); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38a8),32));
            c.set(2,32,rm(m,0x6999b4u32,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4896d1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            if c.cond(15) { pc = 0x4896e0; continue; }
            pc = 0x4896d6; continue;
            }
            0x4896d6 => {
            c.set(0,8,rm(m,0x66de58u32,8));
            pc = 0x489776; continue;
            }
            0x4896e0 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38a8),32));
            c.set(2,32,rm(m,0x6999b4u32,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x489705);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x20u32,32); }
            if c.cond(15) { pc = 0x489711; continue; }
            pc = 0x48970a; continue;
            }
            0x48970a => {
            c.set(0,8,rm(m,0x66de57u32,8));
            pc = 0x489776; continue;
            }
            0x489711 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38a8),32));
            c.set(2,32,rm(m,0x6999b4u32,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x489736);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x30u32,32); }
            if c.cond(15) { pc = 0x489742; continue; }
            pc = 0x48973b; continue;
            }
            0x48973b => {
            c.set(0,8,rm(m,0x66de56u32,8));
            pc = 0x489776; continue;
            }
            0x489742 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38a8),32));
            c.set(2,32,rm(m,0x6999b4u32,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x489767);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x60u32,32); }
            c.set(0,8,rm(m,0x66de55u32,8));
            if c.cond(14) { pc = 0x489776; continue; }
            pc = 0x489771; continue;
            }
            0x489771 => {
            c.set(0,8,rm(m,0x66de54u32,8));
            pc = 0x489776; continue;
            }
            0x489776 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x2068));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x48979a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6999b4u32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4897a7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38a8),32));
            c.set(2,32,rm(m,0x6999c8u32,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x4897cc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            if c.cond(15) { pc = 0x4897db; continue; }
            pc = 0x4897d1; continue;
            }
            0x4897d1 => {
            c.set(0,8,rm(m,0x66de58u32,8));
            pc = 0x489871; continue;
            }
            0x4897db => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38a8),32));
            c.set(2,32,rm(m,0x6999c8u32,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x489800);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x20u32,32); }
            if c.cond(15) { pc = 0x48980c; continue; }
            pc = 0x489805; continue;
            }
            0x489805 => {
            c.set(0,8,rm(m,0x66de57u32,8));
            pc = 0x489871; continue;
            }
            0x48980c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38a8),32));
            c.set(2,32,rm(m,0x6999c8u32,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x489831);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x30u32,32); }
            if c.cond(15) { pc = 0x48983d; continue; }
            pc = 0x489836; continue;
            }
            0x489836 => {
            c.set(0,8,rm(m,0x66de56u32,8));
            pc = 0x489871; continue;
            }
            0x48983d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38a8),32));
            c.set(2,32,rm(m,0x6999c8u32,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x489862);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c1b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x60u32,32); }
            c.set(0,8,rm(m,0x66de55u32,8));
            if c.cond(14) { pc = 0x489871; continue; }
            pc = 0x48986c; continue;
            }
            0x48986c => {
            c.set(0,8,rm(m,0x66de54u32,8));
            pc = 0x489871; continue;
            }
            0x489871 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x1c1c));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(3,32,c.r[0].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x489897);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6999c8u32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4898a5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x2150));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4898b3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x69c98cu32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4898c0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x6ba2f0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4898d1);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dd524u32);
            c.push(m,0x6ba308u32);
            c.push(m,0x0u32);
            c.push(m,0x6ba318u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4898ef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,0xbu32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6ba328u32);
            c.push(m, 0x489902);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2340),32));
            c.set(7,32,c.r[6].wrapping_add(0x2340));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x489913); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xec),32); c.push(m, 0x48991f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x2380); wm(m,a,8,0x5u32); }
            c.set(3,32,c.r[6].wrapping_add(0x2438));
            c.set(5,32,0x13u32);
            pc = 0x489931; continue;
            }
            0x489931 => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x489939);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cdc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.add(a,0xf8u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x489931; continue; }
            pc = 0x489942; continue;
            }
            0x489942 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0x9fc));
            c.set(0,8,rm(m,0x66de55u32,8));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x489968);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x69b338u32,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x489975);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[6].wrapping_add(0xae4));
            c.set(5,32,0x13u32);
            pc = 0x489980; continue;
            }
            0x489980 => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x489988);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.add(a,0xe8u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x489980; continue; }
            pc = 0x489991; continue;
            }
            0x489991 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xf0),32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba354u32);
            c.push(m,0x6ba340u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4899af);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0xf8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4899c4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4170),32));
            c.set(1,32,c.r[6].wrapping_add(0x4170));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x4899d3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x130),32); c.push(m, 0x4899dd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[6].wrapping_add(0x5260));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,0x66de58u32,8));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x489a03);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1du32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6ba360u32);
            c.push(m, 0x489a16);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5348),32));
            c.set(1,32,c.r[6].wrapping_add(0x5348));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x489a25); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x6d50));
            c.set(5,32,c.r[6].wrapping_add(0x7f70));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x14u32); }
            pc = 0x489a39; continue;
            }
            0x489a39 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,c.r[7].wrapping_add(0xffffeb4c));
            c.set(0,8,rm(m,0x66de58u32,8));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x489a5f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,0x66de58u32,8));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6cbe08));
            c.push(m, 0x489a7f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.r[5].wrapping_add(0xffffeb60));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x489a8d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x489a95); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.add(a,0x20u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x489a39; continue; }
            pc = 0x489aa9; continue;
            }
            0x489aa9 => {
            c.push(m,0x1du32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6ba378u32);
            c.push(m, 0x489abc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x81f0),32));
            c.set(1,32,c.r[6].wrapping_add(0x81f0));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x489acb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66de56u32,8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x489ae5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x82d0),32));
            { let a = c.get(6,32); let r = c.add(a,0x82d0u32,32); c.set(6,32,r); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x489af6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x489b02); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x4e57u32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x104),32); c.push(m, 0x489b13); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0xe8); wm(m,a,8,0x2u32); }
            c.push(m, 0x489b24);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0xe8); wm(m,a,8,0x1u32); }
            c.push(m, 0x489b35);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0xe8); wm(m,a,8,0x0u32); }
            c.push(m, 0x489b46);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0xf0));
            { let a = c.r[4].wrapping_add(0xe8); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x489b5d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xe0),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xdcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x48ada0;
    loop {
        match pc {
            0x48ada0 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x48adb0);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            c.push(m, 0x48adf4);
            { let args = []; c.r[0] = host.call(m, 0x492bc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x48adfb);
            { let args = []; c.r[0] = host.call(m, 0x493d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc8),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x48ae31; continue; }
            pc = 0x48ae0b; continue;
            }
            0x48ae0b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x5238),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1]),8));
            c.push(m,c.get(2,32));
            pc = 0x48ae38; continue;
            }
            0x48ae31 => {
            c.set(0,16,rm(m,0x6dfc14u32,16));
            c.push(m,c.get(0,32));
            pc = 0x48ae38; continue;
            }
            0x48ae38 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x48ae3f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x492150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbd94u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x48ae5d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.set(2,32,c.get(4,32));
            c.set(1,32,c.r[6].wrapping_add(0x3938));
            c.push(m,c.get(0,32));
            { let a = c.r[2]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x48ae7b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x461fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x3cec));
            c.push(m, 0x48ae9a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x461fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6cbd94u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x5428));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x48aeb9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x461fe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x744),32));
            c.set(0,32,c.get(4,32));
            c.set(1,32,c.r[6].wrapping_add(0x744));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x48aede); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x82c),32));
            c.set(0,32,c.get(4,32));
            c.set(1,32,c.r[6].wrapping_add(0x82c));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x48af03); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x914),32));
            c.set(0,32,c.get(4,32));
            c.set(1,32,c.r[6].wrapping_add(0x914));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x48af28); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1dec),32));
            c.set(0,32,c.get(4,32));
            c.set(1,32,c.r[6].wrapping_add(0x1dec));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x48af4d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1d04),32));
            c.set(1,32,c.r[6].wrapping_add(0x1d04));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.get(4,32));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x48af72); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ed8),32));
            c.set(0,32,c.get(4,32));
            c.set(1,32,c.r[6].wrapping_add(0x1ed8));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x48af97); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2068),32));
            c.set(0,32,c.get(4,32));
            c.set(1,32,c.r[6].wrapping_add(0x2068));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x48afbc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c1c),32));
            c.set(0,32,c.get(4,32));
            c.set(1,32,c.r[6].wrapping_add(0x1c1c));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x48afe1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2150),32));
            c.set(0,32,c.get(4,32));
            c.set(1,32,c.r[6].wrapping_add(0x2150));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x48b006); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2258),32));
            c.set(0,32,c.get(4,32));
            c.set(1,32,c.r[6].wrapping_add(0x2258));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x48b02b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x48b049);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
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

pub fn ship_side_menu_draw_route_list(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x48f2c0;
    loop {
        match pc {
            0x48f2c0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x660460u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1u32);
            c.set(5,32,c.get(1,32));
            c.push(m, 0x48f2e5);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x38f8),32));
            c.set(6,32,c.r[5].wrapping_add(0x38f4));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,c.get(6,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x38fc),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x48f318);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x48f321);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xa0),32));
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x48f381; continue; }
            pc = 0x48f35d; continue;
            }
            0x48f35d => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            pc = 0x48f381; continue;
            }
            0x48f381 => {
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x132),16));
            c.set(1,32,0x6dcd28u32);
            c.set(6,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x48f39c);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x48f3a2);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6b04f4u32,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x48f3b4);
            { let args = []; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x2u32);
            c.push(m, 0x48f3bb);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6a4510u32,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x5260),32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(7,32,c.r[5].wrapping_add(0x5260));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x48f3d6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x48f3de); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x48f3e7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6dcdd0u32);
            c.push(m, 0x48f3f4);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x48f3fa);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xff000000u32);
            c.push(m, 0x48f404);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x3u32);
            c.push(m, 0x48f40b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,16); c.sub(a,rm(m,0x6dd72au32,16),16); }
            if c.cond(3) { pc = 0x48f4df; continue; }
            pc = 0x48f41b; continue;
            }
            0x48f41b => {
            c.set(1,32,rm(m,0x6dd72cu32,32));
            c.set(2,8,0x4u32);
            pc = 0x48f423; continue;
            }
            0x48f423 => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3),8); c.and(a,c.get(2,8),8); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            if c.cond(5) { pc = 0x48f446; continue; }
            pc = 0x48f43c; continue;
            }
            0x48f43c => {
            c.set(6,16,rm(m,c.r[0],16));
            { let a = c.get(3,16); c.sub(a,c.get(6,16),16); }
            if c.cond(5) { pc = 0x48f423; continue; }
            pc = 0x48f444; continue;
            }
            0x48f444 => {
            pc = 0x48f448; continue;
            }
            0x48f446 => {
            c.set(3,32,c.get(6,32));
            pc = 0x48f448; continue;
            }
            0x48f448 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(7,32,c.r[1].wrapping_add(0x1));
            c.push(m,c.get(7,32));
            c.push(m,0x6ba3fcu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x48f45f);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(0x14),16); c.sub(a,c.get(6,16),16); }
            if c.cond(5) { pc = 0x48f470; continue; }
            pc = 0x48f469; continue;
            }
            0x48f469 => {
            c.push(m,0xffff0000u32);
            pc = 0x48f475; continue;
            }
            0x48f470 => {
            c.push(m,0xff000000u32);
            pc = 0x48f475; continue;
            }
            0x48f475 => {
            c.push(m, 0x48f47a);
            { let args = []; c.r[0] = host.call(m, 0x4bb870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x57d8),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            c.set(1,8,u32::from(c.cond(12)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(2,8,u32::from(c.cond(13)));
            { let a = c.get(2,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x48f4b9; continue; }
            pc = 0x48f49c; continue;
            }
            0x48f49c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x18),32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x24),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x48f4b6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bb3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48f4b9; continue;
            }
            0x48f4b9 => {
            c.set(0,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd72cu32,32));
            c.set(6,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.get(3,16); c.sub(a,c.get(6,16),16); }
            if c.cond(5) { pc = 0x48f448; continue; }
            pc = 0x48f4df; continue;
            }
            0x48f4df => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x48f4f0);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_side_menu_update_mode_controls(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x492580;
    loop {
        match pc {
            0x492580 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x4),8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            if c.cond(4) { pc = 0x49273c; continue; }
            pc = 0x492592; continue;
            }
            0x492592 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3cec),32));
            c.set(1,32,c.r[6].wrapping_add(0x3cec));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4925a6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3938),32));
            c.set(1,32,c.r[6].wrapping_add(0x3938));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4925ba); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5428),32));
            c.set(1,32,c.r[6].wrapping_add(0x5428));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4925ce); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x82c),32));
            c.set(1,32,c.r[6].wrapping_add(0x82c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4925e2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x914),32));
            c.set(1,32,c.r[6].wrapping_add(0x914));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4925f6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x744),32));
            c.set(1,32,c.r[6].wrapping_add(0x744));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x49260a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3724),32));
            c.set(1,32,c.r[6].wrapping_add(0x3724));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49261e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x589c));
            c.set(3,32,c.r[6].wrapping_add(0x2340));
            c.set(5,32,0x14u32);
            pc = 0x49262f; continue;
            }
            0x49262f => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xffffb160),32));
            c.set(1,32,c.r[7].wrapping_add(0xffffb160));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492643); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49264f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x49265b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.add(a,0xf8u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x49262f; continue; }
            pc = 0x49266a; continue;
            }
            0x49266a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5348),32));
            c.set(1,32,c.r[6].wrapping_add(0x5348));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49267e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x5260),32));
            c.set(1,32,c.r[6].wrapping_add(0x5260));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492692); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x82d0),32));
            c.set(1,32,c.r[6].wrapping_add(0x82d0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4926a6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1dec),32));
            c.set(1,32,c.r[6].wrapping_add(0x1dec));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4926ba); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1d04),32));
            c.set(1,32,c.r[6].wrapping_add(0x1d04));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4926ce); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ed8),32));
            c.set(1,32,c.r[6].wrapping_add(0x1ed8));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4926e2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2068),32));
            c.set(1,32,c.r[6].wrapping_add(0x2068));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4926f6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c1c),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c1c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x49270a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2258),32));
            c.set(1,32,c.r[6].wrapping_add(0x2258));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49271e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2150),32));
            c.set(1,32,c.r[6].wrapping_add(0x2150));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x49273c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xb4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xb8),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x492763; continue; }
            pc = 0x49274c; continue;
            }
            0x49274c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.set(1,32,c.r[6].wrapping_add(0x3930));
            c.push(m,c.get(0,32));
            c.push(m, 0x49275e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x492829; continue;
            }
            0x492763 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc8),32),32); }
            if c.cond(5) { pc = 0x492796; continue; }
            pc = 0x49276b; continue;
            }
            0x49276b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xdc),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x492815; continue; }
            pc = 0x49277f; continue;
            }
            0x49277f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.set(1,32,c.r[6].wrapping_add(0x3930));
            c.push(m,c.get(2,32));
            c.push(m, 0x492791);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x492829; continue;
            }
            0x492796 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc0),32),32); }
            if c.cond(5) { pc = 0x492815; continue; }
            pc = 0x49279e; continue;
            }
            0x49279e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd0),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4927bd; continue; }
            pc = 0x4927ae; continue;
            }
            0x4927ae => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4098),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x3ce4));
            pc = 0x4927d2; continue;
            }
            0x4927bd => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xdc),32),32); }
            if c.cond(5) { pc = 0x4927d7; continue; }
            pc = 0x4927c5; continue;
            }
            0x4927c5 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.set(1,32,c.r[6].wrapping_add(0x3930));
            c.push(m,c.get(2,32));
            pc = 0x4927d2; continue;
            }
            0x4927d2 => {
            c.push(m, 0x4927d7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4927d7; continue;
            }
            0x4927d7 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.set(1,32,rm(m,0x66d4fcu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4927ff; continue; }
            pc = 0x4927e7; continue;
            }
            0x4927e7 => {
            { let a = c.get(0,32); c.sub(a,rm(m,0x66d4f8u32,32),32); }
            if c.cond(4) { pc = 0x4927ff; continue; }
            pc = 0x4927ef; continue;
            }
            0x4927ef => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xdc),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x492829; continue; }
            pc = 0x4927ff; continue;
            }
            0x4927ff => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x3938),32));
            c.set(1,32,c.r[6].wrapping_add(0x3938));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492813); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x492829; continue;
            }
            0x492815 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3938),32));
            c.set(1,32,c.r[6].wrapping_add(0x3938));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x492829); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x492829; continue;
            }
            0x492829 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd0),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x49284d; continue; }
            pc = 0x492839; continue;
            }
            0x492839 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4098),32));
            c.set(1,32,c.r[6].wrapping_add(0x3ce4));
            c.push(m,c.get(2,32));
            c.push(m, 0x49284b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x492861; continue;
            }
            0x49284d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3cec),32));
            c.set(1,32,c.r[6].wrapping_add(0x3cec));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x492861); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x492861; continue;
            }
            0x492861 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe4),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x492911; continue; }
            pc = 0x492875; continue;
            }
            0x492875 => {
            c.set(7,32,c.r[6].wrapping_add(0x589c));
            c.set(3,32,c.r[6].wrapping_add(0x2340));
            c.set(5,32,0x14u32);
            pc = 0x492886; continue;
            }
            0x492886 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xffffb160),32));
            c.set(1,32,c.r[7].wrapping_add(0xffffb160));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x49289a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4928a6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4928b2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.add(a,0xf8u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x492886; continue; }
            pc = 0x4928c1; continue;
            }
            0x4928c1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5428),32));
            c.set(1,32,c.r[6].wrapping_add(0x5428));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4928d5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x5348),32));
            c.set(1,32,c.r[6].wrapping_add(0x5348));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4928e9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5260),32));
            c.set(1,32,c.r[6].wrapping_add(0x5260));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4928fd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x82d0),32));
            c.set(1,32,c.r[6].wrapping_add(0x82d0));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492911); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x492911; continue;
            }
            0x492911 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xd8),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(1,32,c.r[6].wrapping_add(0x2258));
            if c.cond(5) { pc = 0x492988; continue; }
            pc = 0x492927; continue;
            }
            0x492927 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492931); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6e59ccu32,32));
            c.set(3,32,0xffu32);
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x492959; continue; }
            pc = 0x49293f; continue;
            }
            0x49293f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.get(0,32); let r = c.and(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[1],16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6e55d0),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x492957);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6087f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x49295b; continue;
            }
            0x492959 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x49295b; continue;
            }
            0x49295b => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x3858); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x5828); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x5829); wm(m,a,8,c.get(3,8)); }
            if c.cond(4) { pc = 0x49297e; continue; }
            pc = 0x492971; continue;
            }
            0x492971 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x26),8));
            { let a = c.r[6].wrapping_add(0x582a); wm(m,a,8,c.get(1,8)); }
            pc = 0x49299c; continue;
            }
            0x49297e => {
            c.set(1,32,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x582a); wm(m,a,8,c.get(1,8)); }
            pc = 0x49299c; continue;
            }
            0x492988 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x492992); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x3858); wm(m,a,32,0x0u32); }
            pc = 0x49299c; continue;
            }
            0x49299c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xd4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1dec),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,c.r[6].wrapping_add(0x1dec));
            if c.cond(5) { pc = 0x492a8f; continue; }
            pc = 0x4929bc; continue;
            }
            0x4929bc => {
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4929c4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1d04),32));
            c.set(1,32,c.r[6].wrapping_add(0x1d04));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4929d8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ed8),32));
            c.set(1,32,c.r[6].wrapping_add(0x1ed8));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4929ec); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2068),32));
            c.set(1,32,c.r[6].wrapping_add(0x2068));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x492a00); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1],16));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c1c),32));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1c1c));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x42),16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x492a3e; continue; }
            pc = 0x492a32; continue;
            }
            0x492a32 => {
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x492a3a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            pc = 0x492a48; continue;
            }
            0x492a3e => {
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x492a46); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            pc = 0x492a48; continue;
            }
            0x492a48 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2150),32));
            c.set(3,32,c.r[6].wrapping_add(0x2150));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492a5c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[7].wrapping_add(0x134),16); c.sub(a,0x12u32,16); }
            if c.cond(4) { pc = 0x492a7f; continue; }
            pc = 0x492a66; continue;
            }
            0x492a66 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x15c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x492a7f; continue; }
            pc = 0x492a70; continue;
            }
            0x492a70 => {
            c.set(0,32,rm(m,0x69c98cu32,32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x492a7d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x492afb; continue;
            }
            0x492a7f => {
            c.set(1,32,rm(m,0x6999ecu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x492a8d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x492afb; continue;
            }
            0x492a8f => {
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492a97); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1d04),32));
            c.set(1,32,c.r[6].wrapping_add(0x1d04));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x492aab); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ed8),32));
            c.set(1,32,c.r[6].wrapping_add(0x1ed8));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492abf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2068),32));
            c.set(1,32,c.r[6].wrapping_add(0x2068));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x492ad3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c1c),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c1c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492ae7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2150),32));
            c.set(1,32,c.r[6].wrapping_add(0x2150));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x492afb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x492afb; continue;
            }
            0x492afb => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xcc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe0),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x82c),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,c.r[6].wrapping_add(0x82c));
            if c.cond(5) { pc = 0x492b65; continue; }
            pc = 0x492b17; continue;
            }
            0x492b17 => {
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492b1f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x914),32));
            c.set(1,32,c.r[6].wrapping_add(0x914));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x492b33); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x744),32));
            c.set(1,32,c.r[6].wrapping_add(0x744));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492b47); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3724),32));
            c.set(1,32,c.r[6].wrapping_add(0x3724));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x1u32); }
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x492b65 => {
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492b6d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x914),32));
            c.set(1,32,c.r[6].wrapping_add(0x914));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x492b81); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x744),32));
            c.set(1,32,c.r[6].wrapping_add(0x744));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x492b95); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x3724),32));
            c.set(1,32,c.r[6].wrapping_add(0x3724));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_side_menu_refresh_route_controls(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4934b0;
    loop {
        match pc {
            0x4934b0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6604d2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x44u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38f8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(7,32,c.r[6].wrapping_add(0x38f4));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4934e9);
            { let args = []; c.r[0] = host.call(m, 0x40abf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x4934f2);
            { let args = []; c.r[0] = host.call(m, 0x40abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x493501);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x49350c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x49351b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x493526);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x40a940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0],16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x493576; continue; }
            pc = 0x493552; continue;
            }
            0x493552 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            pc = 0x493576; continue;
            }
            0x493576 => {
            c.push(m,c.get(5,32));
            c.set(5,16,rm(m,c.r[7].wrapping_add(0x132),16));
            c.set(1,32,c.r[6].wrapping_add(0x5420));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m, 0x49358f);
            { let args = []; c.r[0] = host.call(m, 0x431260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x57c4),32));
            c.set(2,16,c.get(5,16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd72au32,16),16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x49373f; continue; }
            pc = 0x4935a9; continue;
            }
            0x4935a9 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x136),8));
            c.set(1,32,c.r[6].wrapping_add(0x5348));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4935c5; continue; }
            pc = 0x4935b9; continue;
            }
            0x4935b9 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4935c3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4935cf; continue;
            }
            0x4935c5 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4935cf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4935cf; continue;
            }
            0x4935cf => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5260),32));
            c.set(1,32,c.r[6].wrapping_add(0x5260));
            { let target = rm(m,c.r[0].wrapping_add(0x38),32); c.push(m, 0x4935de); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x493606; continue; }
            pc = 0x4935e2; continue;
            }
            0x4935e2 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x9fc),32));
            c.set(1,32,c.r[6].wrapping_add(0x9fc));
            { let target = rm(m,c.r[2].wrapping_add(0x38),32); c.push(m, 0x4935f1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x493606; continue; }
            pc = 0x4935f5; continue;
            }
            0x4935f5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5260),32));
            c.set(1,32,c.r[6].wrapping_add(0x5260));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x493606); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x493606; continue;
            }
            0x493606 => {
            c.set(1,32,rm(m,0x6dd72cu32,32));
            c.set(2,8,0x4u32);
            pc = 0x49360e; continue;
            }
            0x49360e => {
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3),8); c.and(a,c.get(2,8),8); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            if c.cond(5) { pc = 0x493633; continue; }
            pc = 0x493627; continue;
            }
            0x493627 => {
            c.set(5,16,rm(m,c.r[0],16));
            { let a = rm(m,c.r[4].wrapping_add(0x10),16); c.sub(a,c.get(5,16),16); }
            if c.cond(5) { pc = 0x49360e; continue; }
            pc = 0x493631; continue;
            }
            0x493631 => {
            pc = 0x493637; continue;
            }
            0x493633 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            pc = 0x493637; continue;
            }
            0x493637 => {
            c.set(2,32,c.r[6].wrapping_add(0x2340));
            c.set(7,32,c.r[6].wrapping_add(0x589c));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            pc = 0x49364f; continue;
            }
            0x493649 => {
            c.set(1,32,rm(m,0x6dd72cu32,32));
            pc = 0x49364f; continue;
            }
            0x49364f => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(0,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(5,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(5,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[5]).wrapping_add(0x2),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x49367b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,0x0u32); }
            c.push(m, 0x493691);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dd72cu32,32));
            c.set(1,32,c.get(7,32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x3),8));
            { let a = c.get(0,8); c.and(a,0x8u32,8); }
            if c.cond(4) { pc = 0x4936b9; continue; }
            pc = 0x4936a0; continue;
            }
            0x4936a0 => {
            c.push(m,0x6ba474u32);
            { let a = c.r[6].wrapping_add(c.r[3]).wrapping_add(0x6abc); wm(m,a,8,0x0u32); }
            c.push(m, 0x4936b2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x4e59u32);
            pc = 0x4936f9; continue;
            }
            0x4936b9 => {
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4936e2; continue; }
            pc = 0x4936bd; continue;
            }
            0x4936bd => {
            c.push(m,0x6ba476u32);
            { let a = c.r[6].wrapping_add(c.r[3]).wrapping_add(0x6abc); wm(m,a,8,0x2u32); }
            c.push(m, 0x4936cf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x4e5au32);
            c.push(m,0x3u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x104),32); c.push(m, 0x4936e0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x493705; continue;
            }
            0x4936e2 => {
            c.push(m,0x6ba478u32);
            { let a = c.r[6].wrapping_add(c.r[3]).wrapping_add(0x6abc); wm(m,a,8,0x1u32); }
            c.push(m, 0x4936f4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x4e58u32);
            pc = 0x4936f9; continue;
            }
            0x4936f9 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2].wrapping_add(0x104),32); c.push(m, 0x493705); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x493705; continue;
            }
            0x493705 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,0x6dd72cu32,32));
            { let a = c.get(2,32); let r = c.add(a,0xf8u32,32); c.set(2,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.set(5,16,rm(m,c.r[0].wrapping_add(c.r[5]),16));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x493734);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x10),16); c.sub(a,c.get(5,16),16); }
            if c.cond(5) { pc = 0x493649; continue; }
            pc = 0x49373f; continue;
            }
            0x49373f => {
            { let a = rm(m,c.r[6].wrapping_add(0x57d4),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x493791; continue; }
            pc = 0x493747; continue;
            }
            0x493747 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[6].wrapping_add(0x57d4); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x3884),32));
            c.set(1,32,c.r[6].wrapping_add(0x387c));
            c.push(m, 0x493761);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x493791);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x493791; continue;
            }
            0x493791 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x57d4),32));
            c.set(1,32,c.r[6].wrapping_add(0x5420));
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x4937a3; continue; }
            pc = 0x4937a2; continue;
            }
            0x4937a2 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x4937a3; continue;
            }
            0x4937a3 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4937a9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4311b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x57d4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x5428),32));
            c.set(1,32,c.r[6].wrapping_add(0x5428));
            { let a = c.get(3,32); c.sub(a,0x5u32,32); }
            c.set(0,8,u32::from(c.cond(15)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4937c8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x57d4),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            if c.cond(13) { pc = 0x4937dd; continue; }
            pc = 0x4937d7; continue;
            }
            0x4937d7 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x4937e4; continue;
            }
            0x4937dd => {
            c.set(1,32,c.r[3].wrapping_add(0x6));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            pc = 0x4937e4; continue;
            }
            0x4937e4 => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x57d8),32),32); }
            if c.cond(4) { pc = 0x493a4e; continue; }
            pc = 0x4937f0; continue;
            }
            0x4937f0 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x3884),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,c.r[6].wrapping_add(0x387c));
            { let a = c.r[6].wrapping_add(0x57d8); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x49380a);
            { let args = []; c.r[0] = host.call(m, 0x40ac00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6da5f0u32);
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x49383a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x493892; continue; }
            pc = 0x49383e; continue;
            }
            0x49383e => {
            c.set(7,32,c.r[6].wrapping_add(0x589c));
            c.set(5,32,c.r[6].wrapping_add(0x2340));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x49384e; continue;
            }
            0x49384e => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xffffb160),32));
            c.set(1,32,c.r[7].wrapping_add(0xffffb160));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x493862); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x49386f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49387b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.add(a,0xf8u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x49384e; continue; }
            pc = 0x493892; continue;
            }
            0x493892 => {
            c.set(0,32,c.r[3].wrapping_add(0x6));
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x49390a; continue; }
            pc = 0x49389a; continue;
            }
            0x49389a => {
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x589c));
            c.set(5,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2340));
            c.set(1,32,0x14u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x4938c6; continue;
            }
            0x4938c6 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xffffb160),32));
            c.set(1,32,c.r[7].wrapping_add(0xffffb160));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4938da); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4938e7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4938f3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.add(a,0xf8u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4938c6; continue; }
            pc = 0x49390a; continue;
            }
            0x49390a => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),32),32); }
            if c.cond(13) { pc = 0x49397f; continue; }
            pc = 0x493910; continue;
            }
            0x493910 => {
            c.set(0,32,c.r[3].wrapping_mul(8));
            c.set(2,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x589c));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(5,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2340));
            pc = 0x49393b; continue;
            }
            0x49393b => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xffffb160),32));
            c.set(1,32,c.r[7].wrapping_add(0xffffb160));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49394f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x49395c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x493968); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.add(a,0xf8u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x49393b; continue; }
            pc = 0x49397f; continue;
            }
            0x49397f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(7,32,0x1u32);
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x2c),32),32); c.set(7,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x2340));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(5,32,c.r[6].wrapping_add(0x589c));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x14u32); }
            pc = 0x4939bd; continue;
            }
            0x4939bd => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffb160),32));
            c.set(1,32,c.r[5].wrapping_add(0xffffb160));
            c.push(m,0x0u32);
            c.set(2,32,c.r[7].wrapping_add(c.r[2]).wrapping_add(0x5a));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x4939db); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,0x0u32);
            c.set(2,32,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x5a));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x4939f1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[6].wrapping_add(0x9fc));
            c.push(m,0x0u32);
            c.set(0,32,c.r[7].wrapping_add(c.r[2]).wrapping_add(0x5a));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x493a09); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[3].wrapping_add(0x64),32); c.push(m, 0x493a21); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); let r = c.add(a,0xf8u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0xe8u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4939bd; continue; }
            pc = 0x493a4a; continue;
            }
            0x493a4a => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x493a4e; continue;
            }
            0x493a4e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x57d4),32));
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x493a9d; continue; }
            pc = 0x493a59; continue;
            }
            0x493a59 => {
            c.set(1,32,rm(m,0x6a450cu32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2340));
            c.push(m, 0x493a73);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x57d4),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2340),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2340));
            c.set(0,8,u32::from(c.cond(12)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x493a9d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x493a9d; continue;
            }
            0x493a9d => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x493af5; continue; }
            pc = 0x493aa1; continue;
            }
            0x493aa1 => {
            c.set(7,32,c.r[6].wrapping_add(0x589c));
            c.set(5,32,c.r[6].wrapping_add(0x2340));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            pc = 0x493ab1; continue;
            }
            0x493ab1 => {
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x493abe); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xffffb160),32));
            c.set(1,32,c.r[7].wrapping_add(0xffffb160));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x493ad2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x493ade); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(5,32); let r = c.add(a,0xf8u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x493ab1; continue; }
            pc = 0x493af5; continue;
            }
            0x493af5 => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x57d4),32),32); }
            if c.cond(13) { pc = 0x493b83; continue; }
            pc = 0x493b01; continue;
            }
            0x493b01 => {
            c.set(0,32,c.r[3].wrapping_mul(8));
            c.set(2,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(7,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x589c));
            c.set(5,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2340));
            pc = 0x493b2a; continue;
            }
            0x493b2a => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xffffb160),32));
            c.set(1,32,c.r[7].wrapping_add(0xffffb160));
            { let a = c.get(3,32); c.sub(a,0x6u32,32); }
            c.set(3,8,u32::from(c.cond(12)));
            c.push(m,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x493b47); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x493b53); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x493b5e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(5,32); let r = c.add(a,0xf8u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x57d4),32));
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x493b2a; continue; }
            pc = 0x493b83; continue;
            }
            0x493b83 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x57d4),32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x493bd2; continue; }
            pc = 0x493b8f; continue;
            }
            0x493b8f => {
            c.set(2,32,c.r[0].wrapping_mul(8));
            c.set(3,32,0x14u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(7,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x589c));
            pc = 0x493ba9; continue;
            }
            0x493ba9 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xffffb160),32));
            c.set(1,32,c.r[7].wrapping_add(0xffffb160));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x493bbd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x493bc9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x493ba9; continue; }
            pc = 0x493bd2; continue;
            }
            0x493bd2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x57d4),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x493c08; continue; }
            pc = 0x493bde; continue;
            }
            0x493bde => {
            c.set(1,32,c.get(0,32));
            c.set(7,32,0x14u32);
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(6,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2340));
            pc = 0x493bf3; continue;
            }
            0x493bf3 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x493bff); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0xf8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x493bf3; continue; }
            pc = 0x493c08; continue;
            }
            0x493c08 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x50u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ship_side_menu_update_crew_controls(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x493ed0;
    loop {
        match pc {
            0x493ed0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.r[4].wrapping_add(0xf); wm(m,a,8,0x0u32); }
            c.set(7,16,rm(m,c.r[0],16));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x42),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x493fa3; continue; }
            pc = 0x493f07; continue;
            }
            0x493f07 => {
            c.set(1,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x6719f361u32);
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x5804),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x493f65; continue; }
            pc = 0x493f3e; continue;
            }
            0x493f3e => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x5808),8));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x9),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x493f65; continue; }
            pc = 0x493f4b; continue;
            }
            0x493f4b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x580a),8));
            c.set(2,8,rm(m,c.r[3].wrapping_add(0xb),8));
            { let a = c.get(0,8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x493f65; continue; }
            pc = 0x493f58; continue;
            }
            0x493f58 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x5809),8));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xa),8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x493f8b; continue; }
            pc = 0x493f65; continue;
            }
            0x493f65 => {
            { let a = c.r[6].wrapping_add(0x5804); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xa),8));
            { let a = c.r[6].wrapping_add(0x5809); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0xb),8));
            { let a = c.r[6].wrapping_add(0x580a); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x9),8));
            { let a = c.r[4].wrapping_add(0xf); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x5808); wm(m,a,8,c.get(2,8)); }
            pc = 0x493f8b; continue;
            }
            0x493f8b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c1c),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c1c));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x493f9f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            pc = 0x493fb9; continue;
            }
            0x493fa3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c1c),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c1c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x493fb7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            pc = 0x493fb9; continue;
            }
            0x493fb9 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2150),32));
            c.set(3,32,c.r[6].wrapping_add(0x2150));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x493fcd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x493fd6);
            { let args = []; c.r[0] = host.call(m, 0x518750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x2150));
            { let target = rm(m,c.r[3].wrapping_add(0x34),32); c.push(m, 0x493fe0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x3c),16));
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,8); let r = c.and(a,0x1u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0x1u32,8); }
            if c.cond(4) { pc = 0x494030; continue; }
            pc = 0x493fee; continue;
            }
            0x493fee => {
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x2u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x494030; continue; }
            pc = 0x493ff8; continue;
            }
            0x493ff8 => {
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,8); let r = c.and(a,0x10u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0x10u32,8); }
            if c.cond(4) { pc = 0x494030; continue; }
            pc = 0x494002; continue;
            }
            0x494002 => {
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0x200u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x200u32,32); }
            if c.cond(4) { pc = 0x494030; continue; }
            pc = 0x494012; continue;
            }
            0x494012 => {
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,8); let r = c.and(a,0x40u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0x40u32,8); }
            if c.cond(4) { pc = 0x494030; continue; }
            pc = 0x49401c; continue;
            }
            0x49401c => {
            { let a = c.get(0,32); let r = c.and(a,0x1000u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x1000u32,32); }
            if c.cond(4) { pc = 0x494030; continue; }
            pc = 0x494028; continue;
            }
            0x494028 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xf),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x494059; continue; }
            pc = 0x494030; continue;
            }
            0x494030 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x494037);
            { let args = []; c.r[0] = host.call(m, 0x441710, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2060),32));
            c.set(1,32,c.r[6].wrapping_add(0x1ed8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x40),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[1].wrapping_add(0x180); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x494059; continue; }
            pc = 0x494053; continue;
            }
            0x494053 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x494059);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x494059; continue;
            }
            0x494059 => {
            { let a = rm(m,c.r[7].wrapping_add(0x134),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4941ef; continue; }
            pc = 0x494067; continue;
            }
            0x494067 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2060),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1dec),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x40),16));
            c.set(5,32,c.r[6].wrapping_add(0x1dec));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(1,8,u32::from(c.cond(5)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49408b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2060),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1d04),32));
            c.set(1,32,c.r[6].wrapping_add(0x1d04));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4940a6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[7].wrapping_add(0x40),16); c.sub(a,0x0u32,16); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1ed8),32));
            c.set(3,32,c.r[6].wrapping_add(0x1ed8));
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4940c0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2060),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2068),32));
            c.set(1,32,c.r[6].wrapping_add(0x2068));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4940db); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x42),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x494100; continue; }
            pc = 0x4940e8; continue;
            }
            0x4940e8 => {
            { let a = rm(m,c.r[7].wrapping_add(0x134),16); c.sub(a,0x12u32,16); }
            if c.cond(4) { pc = 0x494100; continue; }
            pc = 0x4940f2; continue;
            }
            0x4940f2 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x15c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x494100; continue; }
            pc = 0x4940fc; continue;
            }
            0x4940fc => {
            c.set(0,8,0x1u32);
            pc = 0x494102; continue;
            }
            0x494100 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x494102; continue;
            }
            0x494102 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c1c),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c1c));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x494112); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x494119);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let v = c.pop(m); c.set(5,32,v); }
            if c.cond(4) { pc = 0x494132; continue; }
            pc = 0x49411e; continue;
            }
            0x49411e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2060),32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x49412d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x494244; continue;
            }
            0x494132 => {
            c.set(1,32,c.r[6].wrapping_add(0x1d04));
            c.push(m, 0x49413d);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x494155; continue; }
            pc = 0x494141; continue;
            }
            0x494141 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2060),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x494150);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x494244; continue;
            }
            0x494155 => {
            c.set(1,32,c.r[6].wrapping_add(0x2068));
            c.push(m, 0x494160);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4941ab; continue; }
            pc = 0x494164; continue;
            }
            0x494164 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x49416d);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x2060),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x5u32); }
            c.set(0,16,rm(m,c.r[2],16));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x49419d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4941a6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x45c930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x494244; continue;
            }
            0x4941ab => {
            c.set(1,32,c.r[6].wrapping_add(0x1c1c));
            c.push(m, 0x4941b6);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x494244; continue; }
            pc = 0x4941be; continue;
            }
            0x4941be => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x4941c7);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x7u32); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(1,16,rm(m,c.r[0],16));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4941ed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x494244; continue;
            }
            0x4941ef => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1dec),32));
            c.set(1,32,c.r[6].wrapping_add(0x1dec));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x494200); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1d04),32));
            c.set(1,32,c.r[6].wrapping_add(0x1d04));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x494211); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1ed8),32));
            c.set(1,32,c.r[6].wrapping_add(0x1ed8));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x494222); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2068),32));
            c.set(1,32,c.r[6].wrapping_add(0x2068));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x494233); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c1c),32));
            c.set(1,32,c.r[6].wrapping_add(0x1c1c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x494244); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x494244; continue;
            }
            0x494244 => {
            c.set(3,32,c.r[6].wrapping_add(0x2150));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x494251);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4942ab; continue; }
            pc = 0x494255; continue;
            }
            0x494255 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x49425e);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x15c),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa0),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xdu32); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[1],16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,0x6999ecu32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x494294; continue; }
            pc = 0x49428f; continue;
            }
            0x49428f => {
            c.set(0,32,rm(m,0x69c98cu32,32));
            pc = 0x494294; continue;
            }
            0x494294 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x49429c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4942ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4942ab; continue;
            }
            0x4942ab => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn statistics_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x495c10;
    loop {
        match pc {
            0x495c10 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66083fu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,rm(m,0x6beee0u32,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(1,32));
            c.push(m,0x6ba528u32);
            c.push(m,0x6ba514u32);
            c.set(1,32,0x6dd524u32);
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x2u32); }
            c.push(m, 0x495c70);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x495c7e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,0x6beee0u32,32));
            c.push(m,c.get(2,32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,c.get(0,32));
            c.push(m,0x6ba530u32);
            c.push(m,0x6ba514u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x495ca7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            c.push(m,0x6dbb58u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x495cb5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,0x6ba538u32);
            c.push(m,0x6ba514u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x495ccf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x495cdd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4417d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0x2ab8));
            pc = 0x495ce3; continue;
            }
            0x495ce3 => {
            c.set(0,32,c.r[3].wrapping_add(c.r[5]));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x495cf8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x495cff); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x495d0b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x495ce3; continue; }
            pc = 0x495d17; continue;
            }
            0x495d17 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x495d31);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2e58),32));
            c.set(7,32,c.r[6].wrapping_add(0x2e58));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x495d42); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[6].wrapping_add(0x2f40));
            c.set(3,32,0x6u32);
            pc = 0x495d4d; continue;
            }
            0x495d4d => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x495d55);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.add(a,0xe8u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x495d4d; continue; }
            pc = 0x495d5e; continue;
            }
            0x495d5e => {
            c.set(5,32,c.r[6].wrapping_add(0x34b0));
            c.set(3,32,0x7u32);
            pc = 0x495d69; continue;
            }
            0x495d69 => {
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.r[7].wrapping_add(0xc0); wm(m,a,32,0xff000000u32); }
            c.set(0,32,rm(m,0x66e104u32,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[7].wrapping_add(0xc4); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x495d86); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x495d8f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.add(a,0x20u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x495d69; continue; }
            pc = 0x495d9b; continue;
            }
            0x495d9b => {
            c.set(2,32,rm(m,0x6dcf20u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dcf20u32);
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x495dae); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x2c),16));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x6ba548u32);
            c.push(m,0x6ba514u32);
            c.set(1,32,0x6dd524u32);
            { let a = c.r[6].wrapping_add(0xb2); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x495dd5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x495de7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc0),32));
            c.set(1,32,c.r[6].wrapping_add(0xc0));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x495df6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xb0); wm(m,a,16,0x2u32); }
            { let a = c.r[6].wrapping_add(0x3597); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0xb4); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0xb5); wm(m,a,8,0x6u32); }
            { let a = c.r[6].wrapping_add(0xb6); wm(m,a,8,0xbu32); }
            { let a = c.r[6].wrapping_add(0xb7); wm(m,a,8,0x10u32); }
            { let a = c.r[6].wrapping_add(0xb8); wm(m,a,8,0x13u32); }
            { let a = c.r[6].wrapping_add(0xb9); wm(m,a,8,0x16u32); }
            { let a = c.r[6].wrapping_add(0xba); wm(m,a,8,0x1cu32); }
            { let target = rm(m,c.r[2].wrapping_add(0x130),32); c.push(m, 0x495e41); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.push(m, 0x495e4f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x495e5d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x495e6e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn statistics_side_menu_show(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x495ff0;
    loop {
        match pc {
            0x495ff0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x496002);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0xac); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[7].wrapping_add(0xb0); wm(m,a,16,0x2u32); }
            { let a = c.r[7].wrapping_add(0x3590); wm(m,a,8,0x1u32); }
            { let a = c.r[7].wrapping_add(0x3591); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,c.r[7].wrapping_add(0x2e58));
            { let a = c.r[7].wrapping_add(0x3595); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,c.get(6,32));
            c.set(2,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[2].wrapping_add(0x44),32); c.push(m, 0x49603c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x2c),32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x64u32); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xb2),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x496077); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xb0),16));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[7]).wrapping_add(0xb4),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3597),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x496195; continue; }
            pc = 0x4960a5; continue;
            }
            0x4960a5 => {
            c.push(m,c.get(3,32));
            c.set(5,32,c.r[7].wrapping_add(0x2ab8));
            c.set(3,32,0x4u32);
            pc = 0x4960b1; continue;
            }
            0x4960b1 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4960bb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x4960c8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4960d2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.add(a,0xe8u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x4960b1; continue; }
            pc = 0x4960db; continue;
            }
            0x4960db => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(3,32,c.r[7].wrapping_add(0x34b0));
            c.set(2,32,c.r[1].wrapping_mul(8).wrapping_add(0x6cc830));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            pc = 0x4960f2; continue;
            }
            0x4960f2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4960fc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x496108); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x496111); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x68),32); c.push(m, 0x49611d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x496124);
            { let args = []; c.r[0] = host.call(m, 0x4b0540, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1bu32,32); }
            if c.cond(13) { pc = 0x496146; continue; }
            pc = 0x49612f; continue;
            }
            0x49612f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x49613d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            pc = 0x496157; continue;
            }
            0x496146 => {
            c.push(m,0x6ba554u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x496152);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6ba556u32);
            pc = 0x496157; continue;
            }
            0x496157 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            // Las dos ramas (0x49613D y 0x496146) dejan apilado el texto: `set_text` es thiscall con un argumento (`ret 4`).
            // El elevador no lo contó (la pila llega de dos bloques) y cada vuelta del bucle desplazaba `esp` 4 bytes; en la
            // segunda vuelta `append_child` recibía basura como pantalla y reservaba cientos de MiB (cierre al pulsar las monedas).
            { let target = rm(m,c.r[2].wrapping_add(0x130),32); c.push(m, 0x496161); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); c.sub(a,0x7u32,32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x4960f2; continue; }
            pc = 0x49618d; continue;
            }
            0x49618d => {
            { let a = c.r[7].wrapping_add(0x3597); wm(m,a,8,0x1u32); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x496195; continue;
            }
            0x496195 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x49619c);
            { let args = []; c.r[0] = host.call(m, 0x440e50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m, 0x4961a3);
            { let args = []; c.r[0] = host.call(m, 0x462280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xc0),32));
            c.set(1,32,c.r[7].wrapping_add(0xc0));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x4961b8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn statistics_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x496880;
    loop {
        match pc {
            0x496880 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xb0),16));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xb5),8));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xb4),8));
            c.set(6,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4968b3);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x94),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.push(m,0x1u32);
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[7].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[7].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4968f8);
            { let args = []; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xb0),16));
            c.set(1,32,c.r[0].wrapping_mul(8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.push(m, 0x49691c);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x496922);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,c.get(4,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x496943); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x4969b1; continue; }
            pc = 0x496947; continue;
            }
            0x496947 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,c.r[7].wrapping_add(0x2e58));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x496955; continue;
            }
            0x496955 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x49695e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x49696a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,c.get(4,32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x49698b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x496994); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x4969a0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x496955; continue; }
            pc = 0x4969b1; continue;
            }
            0x4969b1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(0,32));
            { let a = c.r[7].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[7].wrapping_add(0xa4); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x4969cf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn money_lender_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a28c0;
    loop {
        match pc {
            0x5a28c0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6650e2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5a2903);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xac),32));
            c.set(6,32,c.r[5].wrapping_add(0xac));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5a2914); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[5].wrapping_add(0x194));
            c.set(3,32,0x5u32);
            pc = 0x5a291f; continue;
            }
            0x5a291f => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a2927);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5a291f; continue; }
            pc = 0x5a2930; continue;
            }
            0x5a2930 => {
            c.set(3,32,c.r[5].wrapping_add(0x620));
            c.set(7,32,0x69b724u32);
            c.set(5,32,0xff000000u32);
            pc = 0x5a2940; continue;
            }
            0x5a2940 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,c.get(5,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x5a294e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a2957);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a2961);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,0x69b73cu32,32); }
            if c.cond(12) { pc = 0x5a2940; continue; }
            pc = 0x5a2975; continue;
            }
            0x5a2975 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(2,32));
            c.push(m,0x2u32);
            c.push(m,0x6c5224u32);
            c.push(m,0x6c5230u32);
            c.push(m, 0x5a2990);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x5a29b9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[2].wrapping_add(0x6e0); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x5a29da);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn money_lender_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a2a00;
    loop {
        match pc {
            0x5a2a00 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(7,32,c.get(1,32));
            c.push(m, 0x5a2a10);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x94),32));
            c.push(m,0x1u32);
            { let a = c.get(6,32); let r = c.neg(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[7].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[7].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x5a2a55);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x5a2a62);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5a2a68);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a2a86); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.r[7].wrapping_add(0xac));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x6u32); }
            pc = 0x5a2a94; continue;
            }
            0x5a2a94 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5a2a9d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a2aa9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a2acb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a2ad4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a2ae0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5a2a94; continue; }
            pc = 0x5a2af1; continue;
            }
            0x5a2af1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(0,32));
            { let a = c.r[7].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[7].wrapping_add(0xa4); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5a2b0f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn hanse_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a34c0;
    loop {
        match pc {
            0x5a34c0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x665152u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5a3503);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3].wrapping_add(0xac),32));
            c.set(6,32,c.r[3].wrapping_add(0xac));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5a3514); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[3].wrapping_add(0x194));
            c.set(5,32,0x6u32);
            pc = 0x5a351f; continue;
            }
            0x5a351f => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a3527);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x5a351f; continue; }
            pc = 0x5a3530; continue;
            }
            0x5a3530 => {
            { let a = c.get(3,32); let r = c.add(a,0x708u32,32); c.set(3,32,r); }
            c.set(5,32,0x69c18cu32);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x5a353d; continue;
            }
            0x5a353d => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,0xff000000u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x5a354f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a3558);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); c.sub(a,0xffffffe8u32,32); }
            if c.cond(14) { pc = 0x5a3563; continue; }
            pc = 0x5a355d; continue;
            }
            0x5a355d => {
            c.set(1,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            pc = 0x5a356a; continue;
            }
            0x5a3563 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x6b201c),32));
            c.push(m,c.get(2,32));
            pc = 0x5a356a; continue;
            }
            0x5a356a => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a3571);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.sub(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,0xffffffe4u32,32); }
            if c.cond(15) { pc = 0x5a353d; continue; }
            pc = 0x5a3585; continue;
            }
            0x5a3585 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(0,32));
            c.push(m,0x2u32);
            c.push(m,0x6c52c4u32);
            c.push(m,0x6c52d0u32);
            c.push(m, 0x5a35a0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x5a35c9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[2].wrapping_add(0x7e8); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x5a35ea);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn hanse_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a3610;
    loop {
        match pc {
            0x5a3610 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(7,32,c.get(1,32));
            c.push(m, 0x5a3620);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x94),32));
            c.push(m,0x1u32);
            { let a = c.get(6,32); let r = c.neg(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[7].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[7].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x5a3665);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x5a3672);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5a3678);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a3696); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.r[7].wrapping_add(0xac));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x7u32); }
            pc = 0x5a36a4; continue;
            }
            0x5a36a4 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a36b0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,c.get(4,32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x5a36d1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a36dd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5a36a4; continue; }
            pc = 0x5a36ee; continue;
            }
            0x5a36ee => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(2,32));
            { let a = c.r[7].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0xa4); wm(m,a,8,c.get(1,8)); }
            c.push(m, 0x5a370c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

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
            { let target = rm(m,c.r[0].wrapping_add(0x6c),32); c.push(m, 0x5a3d0c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
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
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
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
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn trading_office_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a4510;
    loop {
        match pc {
            0x5a4510 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x665204u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5a454f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3].wrapping_add(0xac),32));
            c.set(6,32,c.r[3].wrapping_add(0xac));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5a4560); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[3].wrapping_add(0x194));
            c.set(5,32,0x6u32);
            pc = 0x5a456b; continue;
            }
            0x5a456b => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a4573);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x5a456b; continue; }
            pc = 0x5a457c; continue;
            }
            0x5a457c => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(5,32,0xff000000u32);
            pc = 0x5a4583; continue;
            }
            0x5a4583 => {
            c.set(0,32,c.r[6].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[3].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.r[0].wrapping_add(0x16c); wm(m,a,32,c.get(5,32)); }
            c.set(7,32,c.r[0].wrapping_add(0xac));
            c.set(0,32,c.get(6,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x708));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x5a45b2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5a45bb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); c.sub(a,0x5u32,32); }
            if c.cond(7) { pc = 0x5a45ea; continue; }
            pc = 0x5a45c0; continue;
            }
            0x5a45c0 => {
            pc = m.r32(c.r[6].wrapping_mul(4).wrapping_add(0x5a4678)); continue;
            }
            0x5a45c7 => {
            c.set(2,32,rm(m,0x6b2cacu32,32));
            c.push(m,c.get(2,32));
            pc = 0x5a45f0; continue;
            }
            0x5a45d0 => {
            c.set(0,32,rm(m,0x6b2cbcu32,32));
            pc = 0x5a45ef; continue;
            }
            0x5a45d7 => {
            c.set(1,32,rm(m,0x6b2cd0u32,32));
            c.push(m,c.get(1,32));
            pc = 0x5a45f0; continue;
            }
            0x5a45e0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x69c964),32));
            c.push(m,c.get(2,32));
            pc = 0x5a45f0; continue;
            }
            0x5a45ea => {
            c.set(0,32,rm(m,0x69ddbcu32,32));
            pc = 0x5a45ef; continue;
            }
            0x5a45ef => {
            c.push(m,c.get(0,32));
            pc = 0x5a45f0; continue;
            }
            0x5a45f0 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5a45f7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x7u32,32); }
            if c.cond(12) { pc = 0x5a4583; continue; }
            pc = 0x5a45fd; continue;
            }
            0x5a45fd => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(1,32));
            c.push(m,0x2u32);
            c.push(m,0x6c5364u32);
            c.push(m,0x6c5370u32);
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x5a4618);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[2].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x40),32); c.push(m, 0x5a4641); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x18),16));
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[3].wrapping_add(0x804); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5a465e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn trading_office_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a4690;
    loop {
        match pc {
            0x5a4690 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5a46a0);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0x1u32);
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x5a46e9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x5a46f6);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5a46fc);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a471a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x80a),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5a47d3; continue; }
            pc = 0x5a4728; continue;
            }
            0x5a4728 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x5a472a; continue;
            }
            0x5a472a => {
            c.set(0,32,c.r[5].wrapping_mul(8));
            c.push(m,0x1u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0xac),32));
            c.set(7,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0xac));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a474f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); c.sub(a,0x6u32,32); }
            if c.cond(5) { pc = 0x5a4785; continue; }
            pc = 0x5a4754; continue;
            }
            0x5a4754 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x80c),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),8); c.and(a,0x40u32,8); }
            if c.cond(5) { pc = 0x5a4785; continue; }
            pc = 0x5a4772; continue;
            }
            0x5a4772 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x61c),32));
            c.set(1,32,c.r[6].wrapping_add(0x61c));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5a4783); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a478e; continue;
            }
            0x5a4785 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a478e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a478e; continue;
            }
            0x5a478e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[1]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a47b4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a47bd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a47c9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x7u32,32); }
            if c.cond(12) { pc = 0x5a472a; continue; }
            pc = 0x5a47d3; continue;
            }
            0x5a47d3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5a47f1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn church_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a5220;
    loop {
        match pc {
            0x5a5220 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x665272u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5a5263);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xac),32));
            c.set(6,32,c.r[5].wrapping_add(0xac));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5a5274); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[5].wrapping_add(0x194));
            c.set(3,32,0x3u32);
            pc = 0x5a527f; continue;
            }
            0x5a527f => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a5287);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5a527f; continue; }
            pc = 0x5a5290; continue;
            }
            0x5a5290 => {
            c.set(3,32,c.r[5].wrapping_add(0x450));
            c.set(7,32,0x69fea8u32);
            c.set(5,32,0xff000000u32);
            pc = 0x5a52a0; continue;
            }
            0x5a52a0 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,c.get(5,32)); }
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x5a52ae); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a52b7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a52c1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,0x69feb8u32,32); }
            if c.cond(12) { pc = 0x5a52a0; continue; }
            pc = 0x5a52d5; continue;
            }
            0x5a52d5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(2,32));
            c.push(m,0x2u32);
            c.push(m,0x6c5404u32);
            c.push(m,0x6c5410u32);
            c.push(m, 0x5a52f0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x5a5319); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[2].wrapping_add(0x4d0); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x5a533a);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn church_side_menu_render_buttons_to_cache(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a5360;
    loop {
        match pc {
            0x5a5360 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5a5370);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0x1u32);
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x5a53b5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x5a53c2);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5a53c8);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a53e6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4d5),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(7,32,c.r[6].wrapping_add(0xac));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x76c),32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,c.get(0,8)); }
            pc = 0x5a541a; continue;
            }
            0x5a541a => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5a5423); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5a5432; continue; }
            pc = 0x5a542b; continue;
            }
            0x5a542b => {
            { let a = rm(m,c.r[4].wrapping_add(0x14),32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x5a5460; continue; }
            pc = 0x5a5432; continue;
            }
            0x5a5432 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a543e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a5460); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a5460; continue;
            }
            0x5a5460 => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a5469); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a5475); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x5a541a; continue; }
            pc = 0x5a5489; continue;
            }
            0x5a5489 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5a54a7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

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
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5a55c2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
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

pub fn tavern_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a6030;
    loop {
        match pc {
            0x5a6030 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6652e2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5a606f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xac),32));
            c.set(6,32,c.r[5].wrapping_add(0xac));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5a6080); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[5].wrapping_add(0x194));
            c.set(3,32,0x9u32);
            pc = 0x5a608b; continue;
            }
            0x5a608b => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a6093);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5a608b; continue; }
            pc = 0x5a609c; continue;
            }
            0x5a609c => {
            c.set(7,32,c.r[5].wrapping_add(0x9c0));
            c.set(3,32,0xau32);
            pc = 0x5a60a7; continue;
            }
            0x5a60a7 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,0xff000000u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x5a60b9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a60c2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x20u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5a60a7; continue; }
            pc = 0x5a60ce; continue;
            }
            0x5a60ce => {
            c.set(6,32,0x1u32);
            pc = 0x5a60d3; continue;
            }
            0x5a60d3 => {
            c.set(0,32,rm(m,0x6b2ca0u32,32));
            { let a = c.get(6,32); c.sub(a,0x9u32,32); }
            if c.cond(4) { pc = 0x5a60e4; continue; }
            pc = 0x5a60dd; continue;
            }
            0x5a60dd => {
            c.set(0,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x69c9d4),32));
            pc = 0x5a60e4; continue;
            }
            0x5a60e4 => {
            c.set(1,32,c.r[6].wrapping_mul(8));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0xac));
            c.push(m, 0x5a60fd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0xau32,32); }
            if c.cond(12) { pc = 0x5a60d3; continue; }
            pc = 0x5a6103; continue;
            }
            0x5a6103 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(0,32));
            c.push(m,0x2u32);
            c.push(m,0x6c54a4u32);
            c.push(m,0x6c54b0u32);
            c.push(m, 0x5a611e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[1].wrapping_mul(8).wrapping_add(0x6dcd28));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x5a6147); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x18),16));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[5].wrapping_add(0xb00); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x5a6164);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tavern_side_menu_render_buttons_to_cache(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a6180;
    loop {
        match pc {
            0x5a6180 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(7,32,c.get(1,32));
            c.push(m, 0x5a6190);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x94),32));
            c.push(m,0x1u32);
            { let a = c.get(6,32); let r = c.neg(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[7].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[7].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x5a61d5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x5a61e2);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5a61e8);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a6206); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(6,32,c.r[7].wrapping_add(0xac));
            pc = 0x5a6214; continue;
            }
            0x5a6214 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0xb0c),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,c.get(6,32));
            if c.cond(4) { pc = 0x5a6270; continue; }
            pc = 0x5a6227; continue;
            }
            0x5a6227 => {
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a622c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a6238); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,c.get(4,32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x5a6259); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a6262); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a626e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a6278; continue;
            }
            0x5a6270 => {
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a6278); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a6278; continue;
            }
            0x5a6278 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xau32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x5a6214; continue; }
            pc = 0x5a628c; continue;
            }
            0x5a628c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(2,32));
            { let a = c.r[7].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0xa4); wm(m,a,8,c.get(1,8)); }
            c.push(m, 0x5a62aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn tavern_side_menu_on_show(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a6370;
    loop {
        match pc {
            0x5a6370 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x5a6382);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.r[7].wrapping_add(0xac));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[7].wrapping_add(0xb16); wm(m,a,8,0xffu32); }
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5a639a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x2c),32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x64u32); }
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xb08),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xb00),16));
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x30),32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x5a63ea); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb17),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5a6472; continue; }
            pc = 0x5a63ff; continue;
            }
            0x5a63ff => {
            c.push(m,c.get(3,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(3,32,c.r[7].wrapping_add(0x9c0));
            pc = 0x5a6408; continue;
            }
            0x5a6408 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5a6412);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0xb0c),8));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.get(6,32));
            c.push(m,0x0u32);
            if c.cond(4) { pc = 0x5a6455; continue; }
            pc = 0x5a6423; continue;
            }
            0x5a6423 => {
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a6429); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5a6432); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5a643e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5a6445);
            { let args = []; c.r[0] = host.call(m, 0x4b0540, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x5a645b; continue;
            }
            0x5a6455 => {
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a645b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5a645b; continue;
            }
            0x5a645b => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); c.sub(a,0xau32,32); }
            if c.cond(12) { pc = 0x5a6408; continue; }
            pc = 0x5a646a; continue;
            }
            0x5a646a => {
            { let a = c.r[7].wrapping_add(0xb17); wm(m,a,8,0x1u32); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x5a6472; continue;
            }
            0x5a6472 => {
            c.set(1,32,c.get(7,32));
            { let a = c.r[7].wrapping_add(0xb1c); wm(m,a,16,0xffffu32); }
            c.push(m, 0x5a6482);
            { let args = []; c.r[0] = host.call(m, 0x440e50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m, 0x5a6489);
            { let args = []; c.r[0] = host.call(m, 0x462280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m, 0x5a6497);
            { let args = []; c.r[0] = host.call(m, 0x466320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m, 0x5a64a4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x466430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5a64b3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m, 0x5a64c2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x2u32);
            c.push(m, 0x5a64d1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0x6u32);
            c.push(m, 0x5a64e0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0x0u32);
            c.push(m,0xau32);
            c.push(m, 0x5a64ef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x466490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[7].wrapping_add(0xb18); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1c5),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5a650f; continue; }
            pc = 0x5a6506; continue;
            }
            0x5a6506 => {
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m, 0x5a650f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4436e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5a650f; continue;
            }
            0x5a650f => {
            c.set(1,32,rm(m,0x6e5574u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x5a651d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[7].wrapping_add(0xb19); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_hall_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a8db0;
    loop {
        match pc {
            0x5a8db0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6653d2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5a8ded);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3].wrapping_add(0xac),32));
            c.set(6,32,c.r[3].wrapping_add(0xac));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5a8dfe); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[3].wrapping_add(0x194));
            c.set(5,32,0x7u32);
            pc = 0x5a8e09; continue;
            }
            0x5a8e09 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a8e11);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x5a8e09; continue; }
            pc = 0x5a8e1a; continue;
            }
            0x5a8e1a => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(5,32,0xff000000u32);
            pc = 0x5a8e21; continue;
            }
            0x5a8e21 => {
            c.set(0,32,c.r[6].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[3].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.r[0].wrapping_add(0x16c); wm(m,a,32,c.get(5,32)); }
            c.set(7,32,c.r[0].wrapping_add(0xac));
            c.set(0,32,c.get(6,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x7f0));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x5a8e50); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5a8e59);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); c.sub(a,0x2u32,32); }
            if c.cond(13) { pc = 0x5a8e67; continue; }
            pc = 0x5a8e5e; continue;
            }
            0x5a8e5e => {
            c.set(2,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x69ddb4),32));
            pc = 0x5a8e8d; continue;
            }
            0x5a8e67 => {
            { let a = c.get(6,32); c.sub(a,0x6u32,32); }
            if c.cond(13) { pc = 0x5a8e76; continue; }
            pc = 0x5a8e6c; continue;
            }
            0x5a8e6c => {
            c.set(0,32,rm(m,c.r[6].wrapping_mul(4).wrapping_add(0x69ddb8),32));
            c.push(m,c.get(0,32));
            pc = 0x5a8e8e; continue;
            }
            0x5a8e76 => {
            if c.cond(5) { pc = 0x5a8e87; continue; }
            pc = 0x5a8e78; continue;
            }
            0x5a8e78 => {
            c.set(1,32,rm(m,0x6a94f8u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[3].wrapping_add(0x61c));
            pc = 0x5a8e90; continue;
            }
            0x5a8e87 => {
            c.set(2,32,rm(m,0x6a9630u32,32));
            pc = 0x5a8e8d; continue;
            }
            0x5a8e8d => {
            c.push(m,c.get(2,32));
            pc = 0x5a8e8e; continue;
            }
            0x5a8e8e => {
            c.set(1,32,c.get(7,32));
            pc = 0x5a8e90; continue;
            }
            0x5a8e90 => {
            c.push(m, 0x5a8e95);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x5a8e21; continue; }
            pc = 0x5a8e9b; continue;
            }
            0x5a8e9b => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5a8eac);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_hall_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a8ed0;
    loop {
        match pc {
            0x5a8ed0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5a8ee0);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0x1u32);
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x5a8f29);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x5a8f36);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5a8f3c);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a8f5a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x5a8f5c; continue;
            }
            0x5a8f5c => {
            { let a = c.get(7,32); c.sub(a,0x7u32,32); }
            if c.cond(5) { pc = 0x5a8f71; continue; }
            pc = 0x5a8f61; continue;
            }
            0x5a8f61 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x8f5),8));
            c.set(1,8,rm(m,0x6de52eu32,8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5a8f93; continue; }
            pc = 0x5a8f71; continue;
            }
            0x5a8f71 => {
            c.set(1,32,c.r[7].wrapping_mul(8));
            c.push(m,0x1u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0xac),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0xac));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a8f93); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a8f93; continue;
            }
            0x5a8f93 => {
            c.set(1,32,c.r[7].wrapping_mul(8));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.get(4,32));
            c.set(5,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0xac));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[1]; wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a8fcd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5a8fda); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x5a8f5c; continue; }
            pc = 0x5a8fe4; continue;
            }
            0x5a8fe4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,c.get(1,8)); }
            c.push(m, 0x5a9002);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_hall_side_menu_refresh_button_states(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a9780;
    loop {
        match pc {
            0x5a9780 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6de4b0u32,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x8f5),8));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x5a99ce; continue; }
            pc = 0x5a979d; continue;
            }
            0x5a979d => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(6,32,c.r[5].wrapping_add(0xac));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x8u32); }
            pc = 0x5a97b7; continue;
            }
            0x5a97b1 => {
            c.set(7,32,rm(m,0x6de4b0u32,32));
            pc = 0x5a97b7; continue;
            }
            0x5a97b7 => {
            { let a = c.get(3,32); c.sub(a,0x7u32,32); }
            if c.cond(7) { pc = 0x5a99ad; continue; }
            pc = 0x5a97c0; continue;
            }
            0x5a97c0 => {
            pc = m.r32(c.r[3].wrapping_mul(4).wrapping_add(0x5a99d4)); continue;
            }
            0x5a97c7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x8f5),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(0,32,rm(m,0x6df270u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1010),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x5a99ad; continue; }
            pc = 0x5a97ec; continue;
            }
            0x5a97ec => {
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            pc = 0x5a97f2; continue;
            }
            0x5a97f2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,8,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6de530),8));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x5a981d; continue; }
            pc = 0x5a980b; continue;
            }
            0x5a980b => {
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0x7u32,8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            if c.cond(2) { pc = 0x5a97f2; continue; }
            pc = 0x5a9816; continue;
            }
            0x5a9816 => {
            c.push(m,0x0u32);
            pc = 0x5a99af; continue;
            }
            0x5a981d => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5a9826); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            pc = 0x5a99af; continue;
            }
            0x5a982d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x8f5),8));
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),8); c.and(a,0xe0u32,8); }
            if c.cond(4) { pc = 0x5a992c; continue; }
            pc = 0x5a9853; continue;
            }
            0x5a9853 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5a9864);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x39c),8); c.sub(a,0x4u32,8); }
            if c.cond(2) { pc = 0x5a98a5; continue; }
            pc = 0x5a986e; continue;
            }
            0x5a986e => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x8f5),8));
            c.set(2,16,rm(m,0x6dfc14u32,16));
            c.set(1,32,c.r[4].wrapping_add(0x11));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x1a));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x5a9893);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4da060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x12),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd748u32,16),16); }
            if c.cond(2) { pc = 0x5a99ad; continue; }
            pc = 0x5a98a5; continue;
            }
            0x5a98a5 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x8f5),8));
            c.set(1,8,rm(m,0x6de52eu32,8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5a992c; continue; }
            pc = 0x5a98b5; continue;
            }
            0x5a98b5 => {
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x5a98bf);
            { let args = []; c.r[0] = host.call(m, 0x4d9d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5a98f1; continue; }
            pc = 0x5a98c3; continue;
            }
            0x5a98c3 => {
            c.set(7,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x5a98d3);
            { let args = []; c.r[0] = host.call(m, 0x4d9d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,0xffffff00u32,32); }
            if c.cond(5) { pc = 0x5a98f1; continue; }
            pc = 0x5a98df; continue;
            }
            0x5a98df => {
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x5a98e9);
            { let args = []; c.r[0] = host.call(m, 0x4d9d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5a99ad; continue; }
            pc = 0x5a98f1; continue;
            }
            0x5a98f1 => {
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x5a98fb);
            { let args = []; c.r[0] = host.call(m, 0x4d9cd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5a992c; continue; }
            pc = 0x5a98ff; continue;
            }
            0x5a98ff => {
            c.set(7,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x5a990f);
            { let args = []; c.r[0] = host.call(m, 0x4d9cd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,0xffffff00u32,32); }
            if c.cond(5) { pc = 0x5a992c; continue; }
            pc = 0x5a991a; continue;
            }
            0x5a991a => {
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x5a9924);
            { let args = []; c.r[0] = host.call(m, 0x4d9d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5a99ad; continue; }
            pc = 0x5a992c; continue;
            }
            0x5a992c => {
            c.set(1,32,c.get(6,32));
            pc = 0x5a992e; continue;
            }
            0x5a992e => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5a9935); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a99b6; continue;
            }
            0x5a9937 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x8f5),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x6f1),8));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,c.get(6,32));
            if c.cond(5) { pc = 0x5a992e; continue; }
            pc = 0x5a9960; continue;
            }
            0x5a9960 => {
            c.push(m,0x1u32);
            pc = 0x5a99b1; continue;
            }
            0x5a9964 => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x8f5),8));
            c.set(0,8,rm(m,0x6de52eu32,8));
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5a99b6; continue; }
            pc = 0x5a9973; continue;
            }
            0x5a9973 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5a9984);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x37),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5a9996; continue; }
            pc = 0x5a998b; continue;
            }
            0x5a998b => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5a9994); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5a99b6; continue;
            }
            0x5a9996 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x6de52du32,8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,c.get(6,32));
            if c.cond(5) { pc = 0x5a992e; continue; }
            pc = 0x5a99a9; continue;
            }
            0x5a99a9 => {
            c.push(m,0x1u32);
            pc = 0x5a99b1; continue;
            }
            0x5a99ad => {
            c.push(m,0x1u32);
            pc = 0x5a99af; continue;
            }
            0x5a99af => {
            c.set(1,32,c.get(6,32));
            pc = 0x5a99b1; continue;
            }
            0x5a99b1 => {
            c.set(2,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5a99b6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5a99b6; continue;
            }
            0x5a99b6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5a97b1; continue; }
            pc = 0x5a99cc; continue;
            }
            0x5a99cc => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x5a99ce; continue;
            }
            0x5a99ce => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn barn_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a9e80;
    loop {
        match pc {
            0x5a9e80 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66548cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5a9ebb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xac),32));
            c.set(6,32,c.r[7].wrapping_add(0xac));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5a9ecc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[7].wrapping_add(0x16c); wm(m,a,32,0xff000000u32); }
            { let a = c.get(7,32); let r = c.add(a,0x198u32,32); c.set(7,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x5a9ee4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a9eed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6af4fcu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5a9efb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6af4fcu32,32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x130),32); c.push(m, 0x5a9f0b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5a9f1c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn barn_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a9f40;
    loop {
        match pc {
            0x5a9f40 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5a9f50);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0x1u32);
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x5a9f95);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x5a9fa2);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5a9fa8);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5a9fc6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1d8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xac),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(7,32,c.r[6].wrapping_add(0xac));
            c.set(1,8,u32::from(c.cond(4)));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5a9fe6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,32,c.get(4,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x5aa007); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5aa013); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,c.get(1,8)); }
            c.push(m, 0x5aa031);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn weapons_smith_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5aaa30;
    loop {
        match pc {
            0x5aaa30 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5aaa40);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0x1u32);
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x5aaa85);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x5aaa92);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5aaa98);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5aaab6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2c1),8));
            c.set(1,16,rm(m,0x6dfc14u32,16));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xac),32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5aaad9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(2,8,u32::from(c.cond(5)));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[6].wrapping_add(0xac));
            { let target = rm(m,c.r[7].wrapping_add(0x34),32); c.push(m, 0x5aaae8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2c1),8));
            c.set(7,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x6f1),8));
            c.set(7,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5aab41; continue; }
            pc = 0x5aab16; continue;
            }
            0x5aab16 => {
            c.set(1,16,c.get(1,8));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5aab28);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5aab3d; continue; }
            pc = 0x5aab2c; continue;
            }
            0x5aab2c => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,0x6de52du32,8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5aab41; continue; }
            pc = 0x5aab3d; continue;
            }
            0x5aab3d => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x5aab43; continue;
            }
            0x5aab41 => {
            c.set(0,8,0x1u32);
            pc = 0x5aab43; continue;
            }
            0x5aab43 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x194),32));
            c.set(1,32,c.r[6].wrapping_add(0x194));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5aab53); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.r[6].wrapping_add(0xac));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x2u32); }
            pc = 0x5aab61; continue;
            }
            0x5aab61 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5aab6d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,c.get(4,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x5aab8e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5aab97); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5aaba3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5aab61; continue; }
            pc = 0x5aabb4; continue;
            }
            0x5aabb4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5aabd2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_gate_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5ab6e0;
    loop {
        match pc {
            0x5ab6e0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6655a2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5ab723);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[3].wrapping_add(0xac),32));
            c.set(6,32,c.r[3].wrapping_add(0xac));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5ab734); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[3].wrapping_add(0x194));
            c.set(5,32,0x7u32);
            pc = 0x5ab73f; continue;
            }
            0x5ab73f => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ab747);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x5ab73f; continue; }
            pc = 0x5ab750; continue;
            }
            0x5ab750 => {
            c.set(5,32,c.r[3].wrapping_add(0x8f8));
            c.set(7,32,0x6a5da4u32);
            { let a = c.get(3,32); let r = c.add(a,0x7f0u32,32); c.set(3,32,r); }
            pc = 0x5ab761; continue;
            }
            0x5ab761 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,0xff000000u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x5ab773); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ab77c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ab786);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5]; wm(m,a,8,0x1u32); }
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,0x6a5dc4u32,32); }
            if c.cond(12) { pc = 0x5ab761; continue; }
            pc = 0x5ab79f; continue;
            }
            0x5ab79f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(2,32));
            c.push(m,0x2u32);
            c.push(m,0x6c5784u32);
            c.push(m,0x6c5790u32);
            c.push(m, 0x5ab7ba);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x5ab7e3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[2].wrapping_add(0x900); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x5ab804);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_gate_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5ab820;
    loop {
        match pc {
            0x5ab820 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5ab830);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            c.push(m,0x1u32);
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x5ab875);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x5ab882);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5ab888);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5ab8a6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ab8ad);
            { let args = []; c.r[0] = host.call(m, 0x5ac240, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0xac));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x5ab8b9; continue;
            }
            0x5ab8b9 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x8f8),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5ab901; continue; }
            pc = 0x5ab8c4; continue;
            }
            0x5ab8c4 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5ab8d0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,c.get(4,32));
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x5ab8f1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ab8fd); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x5ab901; continue;
            }
            0x5ab901 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x5ab8b9; continue; }
            pc = 0x5ab911; continue;
            }
            0x5ab911 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,c.get(1,8)); }
            c.push(m, 0x5ab92f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_gate_side_menu_show(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5ab9f0;
    loop {
        match pc {
            0x5ab9f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x5aba03);
            { let args = []; c.r[0] = host.call(m, 0x4b9730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5aba0e);
            { let args = []; c.r[0] = host.call(m, 0x5ac240, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xac),32));
            c.set(6,32,c.r[7].wrapping_add(0xac));
            c.set(1,32,c.get(6,32));
            { let a = c.r[7].wrapping_add(0x8f0); wm(m,a,8,0xffu32); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x5aba26); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x2c),32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(0,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x64u32); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8f4),32));
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x900),16));
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x30),32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x48),32); c.push(m, 0x5aba79); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8f2),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5abaff; continue; }
            pc = 0x5aba8e; continue;
            }
            0x5aba8e => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(3,32,c.r[7].wrapping_add(0x7f0));
            pc = 0x5aba96; continue;
            }
            0x5aba96 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5abaa0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b4e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x8f8),8));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.get(6,32));
            c.push(m,0x0u32);
            if c.cond(4) { pc = 0x5abae3; continue; }
            pc = 0x5abab1; continue;
            }
            0x5abab1 => {
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5abab7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5abac0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x5abacc); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5abad3);
            { let args = []; c.r[0] = host.call(m, 0x4b0540, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x5abae9; continue;
            }
            0x5abae3 => {
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5abae9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5abae9; continue;
            }
            0x5abae9 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x5aba96; continue; }
            pc = 0x5abaf8; continue;
            }
            0x5abaf8 => {
            { let a = c.r[7].wrapping_add(0x8f2); wm(m,a,8,0x1u32); }
            pc = 0x5abaff; continue;
            }
            0x5abaff => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5abb06);
            { let args = []; c.r[0] = host.call(m, 0x440e50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m, 0x5abb0d);
            { let args = []; c.r[0] = host.call(m, 0x462280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.push(m, 0x5abb1b);
            { let args = []; c.r[0] = host.call(m, 0x466320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc7e0u32,32));
            c.push(m,0xau32);
            c.push(m, 0x5abb28);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x466430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[7].wrapping_add(0x8f3); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6e55b8u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x5abb3d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x1c5),8));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5abb5a; continue; }
            pc = 0x5abb51; continue;
            }
            0x5abb51 => {
            c.push(m,0xau32);
            c.push(m,0x1u32);
            c.push(m, 0x5abb5a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4436e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5abb5a; continue;
            }
            0x5abb5a => {
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn market_hall_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5ac6e0;
    loop {
        match pc {
            0x5ac6e0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x665612u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5ac71f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xac),32));
            c.set(6,32,c.r[5].wrapping_add(0xac));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5ac730); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[5].wrapping_add(0x194));
            c.set(3,32,0x2u32);
            pc = 0x5ac73b; continue;
            }
            0x5ac73b => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ac743);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5ac73b; continue; }
            pc = 0x5ac74c; continue;
            }
            0x5ac74c => {
            c.set(7,32,0x69db7cu32);
            c.set(3,32,c.r[5].wrapping_add(0x368));
            pc = 0x5ac757; continue;
            }
            0x5ac757 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,0xff000000u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x5ac769); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ac772);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ac77c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,0x69db88u32,32); }
            if c.cond(12) { pc = 0x5ac757; continue; }
            pc = 0x5ac790; continue;
            }
            0x5ac790 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(2,32));
            c.push(m,0x2u32);
            c.push(m,0x6c5824u32);
            c.push(m,0x6c5830u32);
            c.push(m, 0x5ac7ab);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x5ac7d4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x18),16));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[5].wrapping_add(0x3c8); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x5ac7f1);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn market_hall_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5ac810;
    loop {
        match pc {
            0x5ac810 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(7,32,c.get(1,32));
            c.push(m, 0x5ac820);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x1c),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa4),8));
            c.set(2,32,rm(m,0x66d504u32,32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x94),32));
            c.push(m,0x1u32);
            { let a = c.get(6,32); let r = c.neg(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[7].wrapping_add(0x94); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[7].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x5ac865);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bba10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,0x6dce78u32);
            c.push(m, 0x5ac872);
            { let args = []; c.r[0] = host.call(m, 0x4c1b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x5ac878);
            { let args = []; c.r[0] = host.call(m, 0x4bb8f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(2,32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5ac896); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.r[7].wrapping_add(0xac));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x3u32); }
            pc = 0x5ac8a4; continue;
            }
            0x5ac8a4 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x5ac8ad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ac8b9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5ac8db); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5ac8e4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5ac8f0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5ac8a4; continue; }
            pc = 0x5ac901; continue;
            }
            0x5ac901 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(0,32));
            { let a = c.r[7].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[7].wrapping_add(0xa4); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5ac91f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn shipyard_side_menu_load_layout(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5ad590;
    loop {
        match pc {
            0x5ad590 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6656c2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,0x66de54u32,8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x66de5cu32);
            c.push(m, 0x5ad5d3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xac),32));
            c.set(6,32,c.r[5].wrapping_add(0xac));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x5ad5e4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[5].wrapping_add(0x194));
            c.set(3,32,0x5u32);
            pc = 0x5ad5ef; continue;
            }
            0x5ad5ef => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ad5f7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7a90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5ad5ef; continue; }
            pc = 0x5ad600; continue;
            }
            0x5ad600 => {
            c.set(3,32,c.r[5].wrapping_add(0x620));
            c.set(7,32,0x6a59b8u32);
            c.set(5,32,0xff404040u32);
            pc = 0x5ad610; continue;
            }
            0x5ad610 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0xc0); wm(m,a,32,0xff000000u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x5ad622); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ad62b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7c30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5ad635);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c7780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xc8); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xe8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,0x6a59d0u32,32); }
            if c.cond(12) { pc = 0x5ad610; continue; }
            pc = 0x5ad64f; continue;
            }
            0x5ad64f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,0x6dd524u32);
            c.push(m,c.get(2,32));
            c.push(m,0x2u32);
            c.push(m,0x6c58d4u32);
            c.push(m,0x6c58e0u32);
            c.push(m, 0x5ad66a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4be0b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28),32));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x6dcd28));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x5ad693); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[2].wrapping_add(0x6f8); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x5ad6b4);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn shipyard_side_menu_prerender(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5ad6d0;
    loop {
        match pc {
            0x5ad6d0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5ad6e0);
            { let args = []; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x6e8),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa4),8));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x66d504u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x76c),32));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(4,32));
            { let a = c.get(0,32); let n = 0x11u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            { let a = c.get(0,8); let r = c.and(a,0x1u32,8); c.set(0,8,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,0x0u32); }
            { let a = c.get(3,32); let r = c.neg(a,32); c.set(3,32,r); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6cbd94u32,32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x9c),32); c.push(m, 0x5ad75e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[6].wrapping_add(0xac));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x5ad76a; continue;
            }
            0x5ad76a => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x12),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5ad77c; continue; }
            pc = 0x5ad772; continue;
            }
            0x5ad772 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x5ad7ba; continue; }
            pc = 0x5ad777; continue;
            }
            0x5ad777 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(4) { pc = 0x5ad7ba; continue; }
            pc = 0x5ad77c; continue;
            }
            0x5ad77c => {
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ad788); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[1]; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6cbd94u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, 0x5ad7aa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x5ad7b6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x5ad7ba; continue;
            }
            0x5ad7ba => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xe8u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x5ad76a; continue; }
            pc = 0x5ad7ca; continue;
            }
            0x5ad7ca => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.push(m,c.get(2,32));
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,8,c.get(1,8)); }
            c.push(m, 0x5ad7e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "game_screen_create_side_menus_and_dialogs" => game_screen_create_side_menus_and_dialogs(m, host, regs),
        "game_screen_draw_at_origin" => game_screen_draw_at_origin(m, host, regs),
        "game_screen_apply_resolution_layout" => game_screen_apply_resolution_layout(m, host, regs),
        "game_screen_push_letter_ticker_line" => game_screen_push_letter_ticker_line(m, host, regs),
        "ship_list_window_load_layout" => ship_list_window_load_layout(m, host, regs),
        "ship_list_window_layout_controls" => ship_list_window_layout_controls(m, host, regs),
        "ship_list_window_place_row_buttons" => ship_list_window_place_row_buttons(m, host, regs),
        "build_side_menu_load_layout" => build_side_menu_load_layout(m, host, regs),
        "build_side_menu_show_buttons" => build_side_menu_show_buttons(m, host, regs),
        "build_side_menu_prerender" => build_side_menu_prerender(m, host, regs),
        "build_side_menu_update" => build_side_menu_update(m, host, regs),
        "personal_letters_side_menu_load_layout" => personal_letters_side_menu_load_layout(m, host, regs),
        "personal_letters_side_menu_prerender" => personal_letters_side_menu_prerender(m, host, regs),
        "ship_side_menu_hide" => ship_side_menu_hide(m, host, regs),
        "ship_side_menu_load_layout" => ship_side_menu_load_layout(m, host, regs),
        "ship_side_menu_prerender" => ship_side_menu_prerender(m, host, regs),
        "ship_side_menu_draw_route_list" => ship_side_menu_draw_route_list(m, host, regs),
        "ship_side_menu_update_mode_controls" => ship_side_menu_update_mode_controls(m, host, regs),
        "ship_side_menu_refresh_route_controls" => ship_side_menu_refresh_route_controls(m, host, regs),
        "ship_side_menu_update_crew_controls" => ship_side_menu_update_crew_controls(m, host, regs),
        "statistics_side_menu_load_layout" => statistics_side_menu_load_layout(m, host, regs),
        "statistics_side_menu_show" => statistics_side_menu_show(m, host, regs),
        "statistics_side_menu_prerender" => statistics_side_menu_prerender(m, host, regs),
        "money_lender_side_menu_load_layout" => money_lender_side_menu_load_layout(m, host, regs),
        "money_lender_side_menu_prerender" => money_lender_side_menu_prerender(m, host, regs),
        "hanse_side_menu_load_layout" => hanse_side_menu_load_layout(m, host, regs),
        "hanse_side_menu_prerender" => hanse_side_menu_prerender(m, host, regs),
        "hanse_side_menu_update" => hanse_side_menu_update(m, host, regs),
        "trading_office_side_menu_load_layout" => trading_office_side_menu_load_layout(m, host, regs),
        "trading_office_side_menu_prerender" => trading_office_side_menu_prerender(m, host, regs),
        "church_side_menu_load_layout" => church_side_menu_load_layout(m, host, regs),
        "church_side_menu_render_buttons_to_cache" => church_side_menu_render_buttons_to_cache(m, host, regs),
        "church_side_menu_on_show" => church_side_menu_on_show(m, host, regs),
        "tavern_side_menu_load_layout" => tavern_side_menu_load_layout(m, host, regs),
        "tavern_side_menu_render_buttons_to_cache" => tavern_side_menu_render_buttons_to_cache(m, host, regs),
        "tavern_side_menu_on_show" => tavern_side_menu_on_show(m, host, regs),
        "town_hall_side_menu_load_layout" => town_hall_side_menu_load_layout(m, host, regs),
        "town_hall_side_menu_prerender" => town_hall_side_menu_prerender(m, host, regs),
        "town_hall_side_menu_refresh_button_states" => town_hall_side_menu_refresh_button_states(m, host, regs),
        "barn_side_menu_load_layout" => barn_side_menu_load_layout(m, host, regs),
        "barn_side_menu_prerender" => barn_side_menu_prerender(m, host, regs),
        "weapons_smith_side_menu_prerender" => weapons_smith_side_menu_prerender(m, host, regs),
        "town_gate_side_menu_load_layout" => town_gate_side_menu_load_layout(m, host, regs),
        "town_gate_side_menu_prerender" => town_gate_side_menu_prerender(m, host, regs),
        "town_gate_side_menu_show" => town_gate_side_menu_show(m, host, regs),
        "market_hall_side_menu_load_layout" => market_hall_side_menu_load_layout(m, host, regs),
        "market_hall_side_menu_prerender" => market_hall_side_menu_prerender(m, host, regs),
        "shipyard_side_menu_load_layout" => shipyard_side_menu_load_layout(m, host, regs),
        "shipyard_side_menu_prerender" => shipyard_side_menu_prerender(m, host, regs),
        _ => return None,
    })
}
