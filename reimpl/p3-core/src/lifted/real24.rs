//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn game_main_window_run(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x463d20;
    loop {
        match pc {
            0x463d20 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65ef0du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x54u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1e4u32);
            c.set(5,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,0x1u32); }
            c.push(m, 0x463d4d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x463d67; continue; }
            pc = 0x463d5e; continue;
            }
            0x463d5e => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x463d65);
            { let args = []; c.r[0] = host.call(m, 0x4419a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x463d69; continue;
            }
            0x463d67 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x463d69; continue;
            }
            0x463d69 => {
            c.set(1,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0xffffffffu32); }
            { let a = 0x6cbdb8u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x463d7d);
            { let args = []; c.r[0] = host.call(m, 0x441cd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x463d88);
            { let args = []; c.r[0] = host.call(m, 0x441e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x698a28u32);
            c.push(m, 0x463d9a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x44),32));
            c.set(1,32,c.r[5].wrapping_add(0x44));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x463da3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x464230; continue; }
            pc = 0x463dab; continue;
            }
            0x463dab => {
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x463db5);
            { let args = []; c.r[0] = host.call(m, 0x54a310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.r[5].wrapping_add(0x44));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x463dbf);
            { let args = []; c.r[0] = host.call(m, 0x42f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x2fc),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x463df1; continue; }
            pc = 0x463dc9; continue;
            }
            0x463dc9 => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x463dd3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffbu32,32); }
            if c.cond(4) { pc = 0x464245; continue; }
            pc = 0x463ddc; continue;
            }
            0x463ddc => {
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x463dec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x463e7a; continue;
            }
            0x463df1 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x463e34; continue; }
            pc = 0x463df6; continue;
            }
            0x463df6 => {
            c.set(1,32,c.r[5].wrapping_add(0x308));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x463e06);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,0x1u32); }
            c.set(1,32,c.r[0].wrapping_add(0x2e8));
            c.push(m, 0x463e23);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x463e34);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x463e34; continue;
            }
            0x463e34 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x304),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x300),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x2fc),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x463e54);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4381e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x463e62); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(4) { pc = 0x463e7a; continue; }
            pc = 0x463e67; continue;
            }
            0x463e67 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x463e71); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffbu32,32); }
            if c.cond(4) { pc = 0x464245; continue; }
            pc = 0x463e7a; continue;
            }
            0x463e7a => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,0x2u32);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(6,32)); }
            c.push(m,0x6cbb4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,8,0x3u32); }
            c.push(m, 0x463ea3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6cbb4cu32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x463eb1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x14u32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x463ebe);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0xac44u32); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,0x10u32); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(6,32)); }
            c.push(m,0x6dcb70u32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,0x4u32); }
            c.push(m, 0x463f0b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6dcb74u32);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x463f19);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dcb8cu32,32));
            c.set(0,32,rm(m,0x6dcb90u32,32));
            c.set(1,32,0x5u32);
            c.set(6,32,0x6dcb78u32);
            c.set(7,32,c.r[4].wrapping_add(0x24));
            c.rep_movsd(m);
            c.set(1,32,rm(m,0x6dcb94u32,32));
            c.set(6,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcb98u32,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcb9cu32,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dcba0u32,32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcba4u32,32));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcba8u32,32));
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dcbacu32,32));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,0x6dcbb0u32,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6dcbb4u32,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x168),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x164),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(1,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x46405c; continue; }
            pc = 0x463fbb; continue;
            }
            0x463fbb => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x463fd1; continue; }
            pc = 0x463fbf; continue;
            }
            0x463fbf => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x320u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x258u32); }
            pc = 0x463ffd; continue;
            }
            0x463fd1 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x463fe8; continue; }
            pc = 0x463fd6; continue;
            }
            0x463fd6 => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x400u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x300u32); }
            pc = 0x463ffd; continue;
            }
            0x463fe8 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x463ffd; continue; }
            pc = 0x463fed; continue;
            }
            0x463fed => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x500u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x400u32); }
            pc = 0x463ffd; continue;
            }
            0x463ffd => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x164),32));
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x464011; continue; }
            pc = 0x464007; continue;
            }
            0x464007 => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x10u32); }
            pc = 0x46401e; continue;
            }
            0x464011 => {
            { let a = c.get(6,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x46401e; continue; }
            pc = 0x464016; continue;
            }
            0x464016 => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x20u32); }
            pc = 0x46401e; continue;
            }
            0x46401e => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x1c),32));
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x46402c);
            { let args = []; c.r[0] = host.call(m, 0x4bccc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x46405c; continue; }
            pc = 0x464033; continue;
            }
            0x464033 => {
            c.push(m,c.get(3,32));
            c.push(m,0x698a40u32);
            c.push(m,0x698a68u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x464045);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x651b5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[2].wrapping_add(0x168); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[0].wrapping_add(0x164); wm(m,a,32,c.get(3,32)); }
            pc = 0x46405c; continue;
            }
            0x46405c => {
            c.set(6,32,c.r[5].wrapping_add(0x44));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x464066);
            { let args = []; c.r[0] = host.call(m, 0x42f990, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x46406d);
            { let args = []; c.r[0] = host.call(m, 0x6534f6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x168),8));
            { let a = c.r[0].wrapping_add(0x1f); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x164),8));
            { let a = c.r[0].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x180),8));
            { let a = c.r[0].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x184),8));
            { let a = c.r[0].wrapping_add(0x25); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x178),8));
            { let a = c.r[0].wrapping_add(0x22); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x174),8));
            { let a = c.r[0].wrapping_add(0x23); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x170),8));
            { let a = c.r[0].wrapping_add(0x21); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x16c),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x24); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x21),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x464124);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442d90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x23),8));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x46413b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x442e90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x188),32));
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[1].wrapping_add(0x1dd); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x464164);
            { let args = []; c.r[0] = host.call(m, 0x429a40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x4c),32); c.push(m, 0x46417a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xd4),32); c.push(m, 0x464188); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x464193);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x120),32); c.push(m, 0x4641a1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffbu32,32); }
            if c.cond(5) { pc = 0x4641aa; continue; }
            pc = 0x4641a6; continue;
            }
            0x4641a6 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(3,8)); }
            pc = 0x4641aa; continue;
            }
            0x4641aa => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x4641b5);
            { let args = []; c.r[0] = host.call(m, 0x427960, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x4641c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4641d7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(0x2fc); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0x5u32); }
            c.push(m, 0x4641ee);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4641ff);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x13),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x463dab; continue; }
            pc = 0x464209; continue;
            }
            0x464209 => {
            { let a = rm(m,0x6cbb94u32,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x464230; continue; }
            pc = 0x464211; continue;
            }
            0x464211 => {
            c.push(m,c.get(3,32));
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x698ab8u32);
            c.push(m, 0x464225);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m, 0x464230);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x440310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x464230; continue;
            }
            0x464230 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x64),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x60u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x464245 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x46424c);
            { let args = []; c.r[0] = host.call(m, 0x42f990, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x464230; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "game_main_window_run" => game_main_window_run(m, host, regs),
        _ => return None,
    })
}
