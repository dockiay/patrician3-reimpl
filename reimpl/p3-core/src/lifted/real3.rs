//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn load_save_screen_load_game_file(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4335b0;
    loop {
        match pc {
            0x4335b0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65d870u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,0x6e6220u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            c.push(m, 0x4335e7);
            { let args = []; c.r[0] = host.call(m, 0x613290, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4335ff);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x547be0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            { let a = c.r[5].wrapping_add(0xf); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x168),8));
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[2].wrapping_add(0x1f); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x16c),8));
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[1].wrapping_add(0x24); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,0x6cbdf0u32,32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x164),8));
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[2].wrapping_add(0x20); wm(m,a,8,c.get(1,8)); }
            if c.cond(4) { pc = 0x433a57; continue; }
            pc = 0x433649; continue;
            }
            0x433649 => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x43366f; continue; }
            pc = 0x43365b; continue;
            }
            0x43365b => {
            { let a = rm(m,c.r[6].wrapping_add(0x11bc),32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x43366f; continue; }
            pc = 0x433664; continue;
            }
            0x433664 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x43366f);
            { let args = []; c.r[0] = host.call(m, 0x4429d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43366f; continue;
            }
            0x43366f => {
            c.set(0,32,rm(m,0x6cbdc8u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4336b2; continue; }
            pc = 0x43367a; continue;
            }
            0x43367a => {
            c.push(m,0x2cu32);
            c.push(m, 0x433681);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            if c.cond(4) { pc = 0x433698; continue; }
            pc = 0x43368f; continue;
            }
            0x43368f => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x433696);
            { let args = []; c.r[0] = host.call(m, 0x444ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43369a; continue;
            }
            0x433698 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x43369a; continue;
            }
            0x43369a => {
            { let a = 0x6cbdc8u32; wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x28),8));
            { let a = c.get(1,8); c.sub(a,0x1u32,8); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            if c.cond(4) { pc = 0x4336b2; continue; }
            pc = 0x4336ab; continue;
            }
            0x4336ab => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4336b2);
            { let args = []; c.r[0] = host.call(m, 0x444d30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4336b2; continue;
            }
            0x4336b2 => {
            c.set(1,32,rm(m,0x6dd8b4u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4336d7; continue; }
            pc = 0x4336bc; continue;
            }
            0x4336bc => {
            c.push(m, 0x4336c1);
            { let args = []; c.r[0] = host.call(m, 0x511080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6dd8b4u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4336d1; continue; }
            pc = 0x4336cb; continue;
            }
            0x4336cb => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x4336d1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4336d1; continue;
            }
            0x4336d1 => {
            { let a = 0x6dd8b4u32; wm(m,a,32,c.get(7,32)); }
            pc = 0x4336d7; continue;
            }
            0x4336d7 => {
            c.set(1,32,rm(m,0x6cc940u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4336f1; continue; }
            pc = 0x4336e1; continue;
            }
            0x4336e1 => {
            c.push(m, 0x4336e6);
            { let args = []; c.r[0] = host.call(m, 0x46d140, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.push(m, 0x4336f1);
            { let args = []; c.r[0] = host.call(m, 0x46d010, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4336f1; continue;
            }
            0x4336f1 => {
            c.push(m,0xcu32);
            c.push(m, 0x4336f8);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            if c.cond(4) { pc = 0x43370f; continue; }
            pc = 0x433706; continue;
            }
            0x433706 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x43370d);
            { let args = []; c.r[0] = host.call(m, 0x511000, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x433711; continue;
            }
            0x43370f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x433711; continue;
            }
            0x433711 => {
            { let a = 0x6dd8b4u32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11bc),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x0u32); }
            if c.cond(5) { pc = 0x433736; continue; }
            pc = 0x433724; continue;
            }
            0x433724 => {
            { let a = rm(m,c.r[6].wrapping_add(0x11c8),32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x433736; continue; }
            pc = 0x43372c; continue;
            }
            0x43372c => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,0x1u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            pc = 0x43373d; continue;
            }
            0x433736 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            pc = 0x43373d; continue;
            }
            0x43373d => {
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x433747);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5476e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[2].wrapping_add(0xd); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x168),8));
            { let a = c.r[2].wrapping_add(0x1f); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x16c),8));
            { let a = c.r[2].wrapping_add(0x24); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6cbdf0u32,32));
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x164),8));
            { let a = c.r[2].wrapping_add(0x20); wm(m,a,8,c.get(1,8)); }
            c.set(6,16,rm(m,0x6de4aau32,16));
            { let a = c.get(6,32); c.sub(a,0x14u32,32); }
            if c.cond(6) { pc = 0x4337b3; continue; }
            pc = 0x43379a; continue;
            }
            0x43379a => {
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4337a6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4337ae; continue; }
            pc = 0x4337ad; continue;
            }
            0x4337ad => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x4337ae; continue;
            }
            0x4337ae => {
            { let a = c.get(6,32); c.sub(a,0x14u32,32); }
            if c.cond(7) { pc = 0x43379a; continue; }
            pc = 0x4337b3; continue;
            }
            0x4337b3 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4337c6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x54b230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,0x6df2f0u32,32));
            { let a = c.get(1,32); let r = c.or(a,0x10u32,32); c.set(1,32,r); }
            { let a = 0x6df2f0u32; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = rm(m,c.r[1].wrapping_add(0x11bc),32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x433a57; continue; }
            pc = 0x4337e5; continue;
            }
            0x4337e5 => {
            c.set(6,32,0x14u32);
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(6,16),16); }
            if c.cond(6) { pc = 0x43381d; continue; }
            pc = 0x4337f3; continue;
            }
            0x4337f3 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4337fe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x43380f; continue; }
            pc = 0x433807; continue;
            }
            0x433807 => {
            { let a = c.get(4,8); c.and(a,c.get(4,8),8); }
            if c.cond(8) { pc = 0x4338a1; continue; }
            pc = 0x43380f; continue;
            }
            0x43380f => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x4337f3; continue; }
            pc = 0x43381d; continue;
            }
            0x43381d => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            pc = 0x433820; continue;
            }
            0x433820 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x433828);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6df31cu32,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            { let a = c.r[5].wrapping_add(0xffffffcc); wm(m,a,32,0xa8u32); }
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x433842);
            { let args = []; c.r[0] = host.call(m, 0x535760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[5].wrapping_add(0xffffffcc));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[5].wrapping_add(0xffffffcc); wm(m,a,32,0xc8u32); }
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,0x0u32); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x43386a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x43394d; continue; }
            pc = 0x433877; continue;
            }
            0x433877 => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6973e8u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x3u32); }
            c.push(m, 0x43388d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x433892);
            { let args = []; c.r[0] = host.call(m, 0x42da50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            pc = 0x433958; continue;
            }
            0x4338a1 => {
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(6,32)); }
            pc = 0x433820; continue;
            }
            0x4338a9 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x4338b1);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x9u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x5u32); }
            c.push(m, 0x4338bf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x4338c7);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let a = c.r[1].wrapping_add(0x7cc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697404u32);
            c.push(m, 0x4338e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x4338f0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x424d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x4338fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x433902);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.push(m,0xfu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[2].wrapping_add(0x7e4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x433918);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x433920);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            { let a = c.r[1].wrapping_add(0x7fc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x433938);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x43393eu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43393e => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            pc = 0x433958; continue;
            }
            0x43394d => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x433958);
            { let args = []; c.r[0] = host.call(m, 0x423720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x433958; continue;
            }
            0x433958 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x433963);
            { let args = []; c.r[0] = host.call(m, 0x4498d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.push(m, 0x43396e);
            { let args = []; c.r[0] = host.call(m, 0x46d010, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6e55d0u32);
            c.push(m, 0x433979);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x611e50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,0x6e59ccu32,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x4339ce; continue; }
            pc = 0x433982; continue;
            }
            0x433982 => {
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x433995);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4339a6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x58a4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4339b1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(0,32,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x4339ce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x521580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4339ce; continue;
            }
            0x4339ce => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4339d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(0,32,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,16,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddba4),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4339fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(0,32,c.get(2,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.set(0,16,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6ddba0),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x433a1b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x44f490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x433a2b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x433a38); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x433a48); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x433a55); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x433a5a; continue;
            }
            0x433a57 => {
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            pc = 0x433a5a; continue;
            }
            0x433a5a => {
            c.set(1,32,c.r[5].wrapping_add(0x8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x433a65);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xf),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x433a7b => {
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn side_menu_set_anchor_position(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x441750;
    loop {
        match pc {
            0x441750 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,0x6cbbc8u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            { let a = 0x6cbbdcu32; wm(m,a,32,c.get(0,32)); }
            { let a = 0x6cbbbcu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4]);
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6cbbc8u32);
            { let a = 0x6cbbe0u32; wm(m,a,32,c.get(6,32)); }
            { let target = rm(m,c.r[2].wrapping_add(0x40),32); c.push(m, 0x44177f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = 0x6cbd9cu32; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6cbd90u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = 0x6cbbc4u32; wm(m,a,32,0x0u32); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = 0x6cbbc0u32; wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(4) { pc = 0x4417c1; continue; }
            pc = 0x4417a9; continue;
            }
            0x4417a9 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x6cbbbcu32);
            { let target = rm(m,c.r[2].wrapping_add(0x68),32); c.push(m, 0x4417b3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbd90u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x130),32); c.push(m, 0x4417c1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4417c1; continue;
            }
            0x4417c1 => {
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn multiplayer_screen_update_hotseat_setup(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43d030;
    loop {
        match pc {
            0x43d030 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65e331u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x7cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            { let a = c.r[5].wrapping_add(0xffffffc8); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x43d060);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,c.get(2,32)); }
            c.set(1,8,0x2u32);
            pc = 0x43d069; continue;
            }
            0x43d069 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x204),8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x43d073; continue; }
            pc = 0x43d072; continue;
            }
            0x43d072 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x43d073; continue;
            }
            0x43d073 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43d069; continue; }
            pc = 0x43d079; continue;
            }
            0x43d079 => {
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,8,0x80u32);
            pc = 0x43d080; continue;
            }
            0x43d080 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x204),8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x43d092; continue; }
            pc = 0x43d089; continue;
            }
            0x43d089 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43d080; continue; }
            pc = 0x43d08f; continue;
            }
            0x43d08f => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x43d092; continue;
            }
            0x43d092 => {
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x43d0a6; continue; }
            pc = 0x43d097; continue;
            }
            0x43d097 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x1f4),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x43d0a6; continue; }
            pc = 0x43d0a2; continue;
            }
            0x43d0a2 => {
            c.set(0,8,0x1u32);
            pc = 0x43d0a8; continue;
            }
            0x43d0a6 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x43d0a8; continue;
            }
            0x43d0a8 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x13bc),32));
            c.set(1,32,c.r[6].wrapping_add(0x13bc));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43d0b8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x184));
            c.set(1,32,c.r[6].wrapping_add(0x1c14));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(1,32)); }
            pc = 0x43d0cc; continue;
            }
            0x43d0cc => {
            c.push(m, 0x43d0d1);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d478; continue; }
            pc = 0x43d0d9; continue;
            }
            0x43d0d9 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x43d0db; continue;
            }
            0x43d0db => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[3]).wrapping_add(0x204),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(8) { pc = 0x43d0ef; continue; }
            pc = 0x43d0e6; continue;
            }
            0x43d0e6 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43d0db; continue; }
            pc = 0x43d0ec; continue;
            }
            0x43d0ec => {
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            pc = 0x43d0ef; continue;
            }
            0x43d0ef => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1f4),8));
            if c.cond(4) { pc = 0x43d11a; continue; }
            pc = 0x43d100; continue;
            }
            0x43d100 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x43d11e; continue; }
            pc = 0x43d104; continue;
            }
            0x43d104 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x13bc),32));
            c.set(1,32,c.r[6].wrapping_add(0x13bc));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43d115); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43d1b8; continue;
            }
            0x43d11a => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d128; continue; }
            pc = 0x43d11e; continue;
            }
            0x43d11e => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffd8),32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x43d128; continue; }
            pc = 0x43d124; continue;
            }
            0x43d124 => {
            c.set(0,8,0x1u32);
            pc = 0x43d12a; continue;
            }
            0x43d128 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x43d12a; continue;
            }
            0x43d12a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x13bc),32));
            c.set(1,32,c.r[6].wrapping_add(0x13bc));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43d13a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43d147); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.push(m,0x1u32);
            { let a = c.get(1,32); let r = c.add(a,0x740u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43d15a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[3].wrapping_mul(8));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x2354),32));
            c.set(1,32,c.r[0].wrapping_add(0x2354));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43d180); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x1c14),32));
            c.set(1,32,c.r[0].wrapping_add(0x1c14));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43d197); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[3]).wrapping_add(0x204),8));
            { let a = c.get(1,8); let r = c.and(a,0x7fu32,8); c.set(1,8,r); }
            { let a = c.r[6].wrapping_add(c.r[3]).wrapping_add(0x204); wm(m,a,8,c.get(1,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[7]).wrapping_add(0x204),8));
            { let a = c.get(0,8); let r = c.or(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0x204); wm(m,a,8,c.get(0,8)); }
            pc = 0x43d1b8; continue;
            }
            0x43d1b8 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x5664),32));
            { let a = c.get(2,32); let r = c.or(a,0xffffffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4cb0); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x43d1fa; continue; }
            pc = 0x43d1d0; continue;
            }
            0x43d1d0 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x50),32));
            c.set(1,32,c.r[6].wrapping_add(0x54d4));
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(2,32)); }
            pc = 0x43d1df; continue;
            }
            0x43d1df => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffc4),32));
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x43d1e9; continue; }
            pc = 0x43d1e6; continue;
            }
            0x43d1e6 => {
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            pc = 0x43d1e9; continue;
            }
            0x43d1e9 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x5664),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x43d1df; continue; }
            pc = 0x43d1f7; continue;
            }
            0x43d1f7 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            pc = 0x43d1fa; continue;
            }
            0x43d1fa => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.r[6].wrapping_add(0x210); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d359; continue; }
            pc = 0x43d20e; continue;
            }
            0x43d20e => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1f4),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x43d364; continue; }
            pc = 0x43d21d; continue;
            }
            0x43d21d => {
            c.set(1,32,c.r[6].wrapping_add(0x4cd4));
            c.push(m,c.get(2,32));
            c.push(m, 0x43d229);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42e080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cd4),32));
            c.set(1,32,c.r[6].wrapping_add(0x4cd4));
            { let target = rm(m,c.r[0].wrapping_add(0x11c),32); c.push(m, 0x43d23b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x43d49c; continue; }
            pc = 0x43d244; continue;
            }
            0x43d244 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffff78));
            c.push(m, 0x43d24f);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x210),32));
            { let a = c.r[5].wrapping_add(0xffffff78); wm(m,a,32,0xa8u32); }
            { let a = c.r[5].wrapping_add(0xffffff7c); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,c.r[5].wrapping_add(0xffffff78));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54d4),32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[5].wrapping_add(0xffffff80); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43d280);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43d288);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43f2b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1788));
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1c4),8));
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4860); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = c.get(0,32); let r = c.add(a,0x20u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x43d2af);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            c.push(m, 0x43d2be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1cc),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1f4); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43d2e0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.push(m,0x1u32);
            { let a = c.get(1,32); let r = c.add(a,0x740u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43d2f3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[3].wrapping_mul(8));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x2354),32));
            c.set(1,32,c.r[0].wrapping_add(0x2354));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43d319); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x1c14),32));
            c.set(1,32,c.r[0].wrapping_add(0x1c14));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43d330); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0x204); wm(m,a,8,0x2u32); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[3]).wrapping_add(0x204),8));
            { let a = c.get(1,8); let r = c.and(a,0x7fu32,8); c.set(1,8,r); }
            { let a = c.r[6].wrapping_add(c.r[3]).wrapping_add(0x204); wm(m,a,8,c.get(1,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[7]).wrapping_add(0x204),8));
            { let a = c.get(0,8); let r = c.or(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(c.r[7]).wrapping_add(0x204); wm(m,a,8,c.get(0,8)); }
            pc = 0x43d359; continue;
            }
            0x43d359 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1f4),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d3bc; continue; }
            pc = 0x43d364; continue;
            }
            0x43d364 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            c.push(m,c.get(3,32));
            c.push(m, 0x43d373);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.add(a,0x20u32,32); c.set(3,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1788));
            c.push(m,c.get(3,32));
            c.push(m, 0x43d382);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1cc),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2b7c),32));
            c.set(1,32,c.r[6].wrapping_add(0x2b7c));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,c.get(0,8)); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43d39f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4cd1),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2a94),32));
            c.set(1,32,c.r[6].wrapping_add(0x2a94));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(2,8,u32::from(c.cond(4)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43d3ba); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43d41b; continue;
            }
            0x43d3bc => {
            c.push(m,0x6cbba4u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd4));
            c.push(m, 0x43d3c9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[5].wrapping_add(0xffffffd4));
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            c.push(m, 0x43d3df);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffd4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x43d3ed);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6cbba5u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x43d3fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x1788));
            c.push(m, 0x43d410);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x43d41b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43d41b; continue;
            }
            0x43d41b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d42d; continue; }
            pc = 0x43d425; continue;
            }
            0x43d425 => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            pc = 0x43d435; continue;
            }
            0x43d42d => {
            c.set(2,32,rm(m,0x6df2fcu32,32));
            c.set(1,32,rm(m,c.r[2],32));
            pc = 0x43d435; continue;
            }
            0x43d435 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1c4),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x43d44e; continue; }
            pc = 0x43d442; continue;
            }
            0x43d442 => {
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4860); wm(m,a,32,c.get(0,32)); }
            pc = 0x43d44e; continue;
            }
            0x43d44e => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffb4));
            c.set(1,32,0x6da5f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xffffffb8); wm(m,a,32,0x12cu32); }
            { let a = c.r[5].wrapping_add(0xffffffc0); wm(m,a,32,0x257u32); }
            { let a = c.r[5].wrapping_add(0xffffffb4); wm(m,a,32,0x0u32); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,0x15eu32); }
            c.push(m, 0x43d478);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43d478; continue;
            }
            0x43d478 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xe8u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,0x8u32,32); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x43d0cc; continue; }
            pc = 0x43d497; continue;
            }
            0x43d497 => {
            pc = 0x43d530; continue;
            }
            0x43d49c => {
            c.set(1,32,c.r[7].wrapping_mul(8));
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[0].wrapping_add(0x2354));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x2354),32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43d4c2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffc4),32));
            c.push(m,0x1u32);
            { let a = c.get(1,32); let r = c.add(a,0x1c14u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43d4d5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[3].wrapping_mul(8));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x1c14),32));
            c.set(1,32,c.r[0].wrapping_add(0x1c14));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43d4fb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x2354),32));
            c.set(1,32,c.r[0].wrapping_add(0x2354));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43d512); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[7].wrapping_add(c.r[6]).wrapping_add(0x204); wm(m,a,8,0x0u32); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x204),8));
            { let a = c.get(0,8); let r = c.or(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.r[3].wrapping_add(c.r[6]).wrapping_add(0x204); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x4cb0); wm(m,a,32,c.get(3,32)); }
            pc = 0x43d530; continue;
            }
            0x43d530 => {
            c.set(1,32,c.r[6].wrapping_add(0x11d4));
            c.push(m, 0x43d53b);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d56d; continue; }
            pc = 0x43d53f; continue;
            }
            0x43d53f => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x43d541; continue;
            }
            0x43d541 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[7]).wrapping_add(0x204),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(8) { pc = 0x43d554; continue; }
            pc = 0x43d54c; continue;
            }
            0x43d54c => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43d541; continue; }
            pc = 0x43d552; continue;
            }
            0x43d552 => {
            pc = 0x43d56d; continue;
            }
            0x43d554 => {
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x43d56d; continue; }
            pc = 0x43d559; continue;
            }
            0x43d559 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43d561);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43e8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d56d; continue; }
            pc = 0x43d565; continue;
            }
            0x43d565 => {
            { let a = rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x204),8); let r = c.or(a,0x2u32,8); { let a = c.r[7].wrapping_add(c.r[6]).wrapping_add(0x204); wm(m,a,8,r); } }
            pc = 0x43d56d; continue;
            }
            0x43d56d => {
            c.set(1,32,c.r[6].wrapping_add(0x12c8));
            c.push(m, 0x43d578);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d637; continue; }
            pc = 0x43d580; continue;
            }
            0x43d580 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x43d582; continue;
            }
            0x43d582 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[7]).wrapping_add(0x204),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(8) { pc = 0x43d598; continue; }
            pc = 0x43d58d; continue;
            }
            0x43d58d => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43d582; continue; }
            pc = 0x43d593; continue;
            }
            0x43d593 => {
            pc = 0x43d637; continue;
            }
            0x43d598 => {
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x43d637; continue; }
            pc = 0x43d5a1; continue;
            }
            0x43d5a1 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x5664),32));
            { let a = c.get(1,32); let r = c.or(a,0xffffffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4cb0); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x43d5d8; continue; }
            pc = 0x43d5b9; continue;
            }
            0x43d5b9 => {
            c.set(1,32,c.r[6].wrapping_add(0x54d4));
            pc = 0x43d5bf; continue;
            }
            0x43d5bf => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x1d4),32));
            { let a = rm(m,c.r[1],32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x43d5cd; continue; }
            pc = 0x43d5ca; continue;
            }
            0x43d5ca => {
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(0,32)); }
            pc = 0x43d5cd; continue;
            }
            0x43d5cd => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x43d5bf; continue; }
            pc = 0x43d5d5; continue;
            }
            0x43d5d5 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffd0),32));
            pc = 0x43d5d8; continue;
            }
            0x43d5d8 => {
            c.set(3,32,c.r[6].wrapping_add(0x4cd4));
            { let a = c.r[6].wrapping_add(0x210); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x43d5ec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42e080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x11c),32); c.push(m, 0x43d5f6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x43d637; continue; }
            pc = 0x43d5fb; continue;
            }
            0x43d5fb => {
            c.set(1,32,c.r[5].wrapping_add(0xffffff78));
            c.push(m, 0x43d606);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x210),32));
            { let a = c.r[5].wrapping_add(0xffffff78); wm(m,a,32,0xa8u32); }
            { let a = c.r[5].wrapping_add(0xffffff7c); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,c.r[5].wrapping_add(0xffffff78));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54d4),32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[5].wrapping_add(0xffffff80); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43d637);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43d637; continue;
            }
            0x43d637 => {
            c.set(1,32,c.r[6].wrapping_add(0xef8));
            c.push(m, 0x43d642);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x43d662; continue; }
            pc = 0x43d646; continue;
            }
            0x43d646 => {
            c.set(0,8,rm(m,0x6dcbdbu32,8));
            c.set(2,8,c.get(0,8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,8); let n = 0x1u32; let r = c.shr(a,n,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7eu32,8); c.set(2,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.or(a,c.get(0,8),8); c.set(2,8,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = 0x6dcbdbu32; wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x43d6b7; continue; }
            pc = 0x43d662; continue;
            }
            0x43d662 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d9c0; continue; }
            pc = 0x43d670; continue;
            }
            0x43d670 => {
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x43d688; continue; }
            pc = 0x43d67c; continue;
            }
            0x43d67c => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[0],32); c.push(m, 0x43d682); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = 0x6cbdc8u32; wm(m,a,32,c.get(7,32)); }
            pc = 0x43d688; continue;
            }
            0x43d688 => {
            c.set(1,32,rm(m,0x6dd8b4u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x43d69e; continue; }
            pc = 0x43d692; continue;
            }
            0x43d692 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,0x3u32);
            { let target = rm(m,c.r[2],32); c.push(m, 0x43d698); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = 0x6dd8b4u32; wm(m,a,32,c.get(7,32)); }
            pc = 0x43d69e; continue;
            }
            0x43d69e => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43d6a5);
            { let args = []; c.r[0] = host.call(m, 0x43fba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43d6a5; continue;
            }
            0x43d6a5 => {
            c.set(1,32,rm(m,0x66e0a4u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43d6b7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x43d6b7; continue;
            }
            0x43d6b7 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x43d6be);
            { let args = []; c.r[0] = host.call(m, 0x43edb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x2c64));
            c.push(m, 0x43d6c9);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d71d; continue; }
            pc = 0x43d6cd; continue;
            }
            0x43d6cd => {
            c.push(m,0x1u32);
            c.set(1,32,0x6d2f60u32);
            c.push(m, 0x43d6d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x49c0d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x5b9du32);
            c.push(m,0x6d2f60u32);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43d6ed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b90e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,0x6d2f60u32);
            c.push(m, 0x43d6f7);
            { let args = []; c.r[0] = host.call(m, 0x4b70c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x43d70e; continue; }
            pc = 0x43d6fc; continue;
            }
            0x43d6fc => {
            c.set(2,32,rm(m,0x6d1c7cu32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x468),32));
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(0,32)); }
            pc = 0x43d70e; continue;
            }
            0x43d70e => {
            c.push(m,0x6d2f60u32);
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43d71d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43d71d; continue;
            }
            0x43d71d => {
            c.set(1,32,c.r[6].wrapping_add(0x13bc));
            c.push(m, 0x43d728);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d960; continue; }
            pc = 0x43d730; continue;
            }
            0x43d730 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x43d732; continue;
            }
            0x43d732 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(c.r[3]).wrapping_add(0x204),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(8) { pc = 0x43d746; continue; }
            pc = 0x43d73d; continue;
            }
            0x43d73d => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43d732; continue; }
            pc = 0x43d743; continue;
            }
            0x43d743 => {
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            pc = 0x43d746; continue;
            }
            0x43d746 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x1f4),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d960; continue; }
            pc = 0x43d755; continue;
            }
            0x43d755 => {
            c.set(2,32,rm(m,0x69afa0u32,32));
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x43d769);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x69af9cu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x2u32); }
            c.push(m, 0x43d781);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43d793);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4ac700, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6d95a0u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x43d7a1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(5) { pc = 0x43d960; continue; }
            pc = 0x43d7aa; continue;
            }
            0x43d7aa => {
            c.set(0,32,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x204));
            { let a = c.r[3].wrapping_add(c.r[6]).wrapping_add(0x1f4); wm(m,a,8,0x0u32); }
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[3].wrapping_add(c.r[6]).wrapping_add(0x1fc); wm(m,a,8,0x0u32); }
            { let a = c.r[0]; wm(m,a,8,0x1u32); }
            { let a = c.r[3].wrapping_add(c.r[6]).wrapping_add(0x1c4); wm(m,a,8,0x28u32); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43d800; continue; }
            pc = 0x43d7d9; continue;
            }
            0x43d7d9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x1d4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x43d800; continue; }
            pc = 0x43d7ed; continue;
            }
            0x43d7ed => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x43d800; continue; }
            pc = 0x43d7f1; continue;
            }
            0x43d7f1 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x43d7fc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),8); let r = c.or(a,0x1u32,8); { let a = c.r[0].wrapping_add(0x8); wm(m,a,8,r); } }
            pc = 0x43d800; continue;
            }
            0x43d800 => {
            c.set(2,32,c.r[3].wrapping_mul(8));
            c.set(1,32,0x6da5f0u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x1d4); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[5].wrapping_add(0xffffffb4); wm(m,a,32,0x15eu32); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,0x320u32); }
            c.set(0,32,c.r[2].wrapping_mul(4).wrapping_add(0x64));
            { let a = c.r[5].wrapping_add(0xffffffb8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x1cu32,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffc0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(0xffffffb4));
            c.push(m,c.get(0,32));
            c.push(m, 0x43d840);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x43d842; continue;
            }
            0x43d842 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[7]).wrapping_add(0x204),8); c.and(a,0x2u32,8); }
            if c.cond(5) { pc = 0x43d852; continue; }
            pc = 0x43d84c; continue;
            }
            0x43d84c => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43d842; continue; }
            pc = 0x43d852; continue;
            }
            0x43d852 => {
            c.set(1,32,c.r[7].wrapping_mul(8));
            c.push(m,0x0u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.add(a,0x1c14u32,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43d874); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffc4),32));
            c.push(m,0x1u32);
            { let a = c.get(1,32); let r = c.add(a,0x2354u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43d887); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[3].wrapping_mul(8));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x2354),32));
            c.set(1,32,c.r[3].wrapping_add(0x2354));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43d8aa); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x1c14),32));
            c.set(1,32,c.r[3].wrapping_add(0x1c14));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43d8be); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffd0),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,8,0x0u32); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x204),8));
            { let a = c.get(0,8); let r = c.or(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.r[7].wrapping_add(c.r[6]).wrapping_add(0x204); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x43d8d8; continue;
            }
            0x43d8d8 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x204),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x43d8e3; continue; }
            pc = 0x43d8e2; continue;
            }
            0x43d8e2 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x43d8e3; continue;
            }
            0x43d8e3 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43d8d8; continue; }
            pc = 0x43d8e9; continue;
            }
            0x43d8e9 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x13bc),32));
            c.set(1,32,c.r[6].wrapping_add(0x13bc));
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            c.set(2,8,u32::from(c.cond(15)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43d8ff); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1788));
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1c4),8));
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4860); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x1a4));
            c.push(m,c.get(0,32));
            c.push(m, 0x43d927);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x184));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            c.push(m, 0x43d93a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x1cc),8));
            c.set(0,32,c.r[6].wrapping_add(0x2b7c));
            c.set(1,32,c.r[6].wrapping_add(0x2a94));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,c.get(2,8)); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdf0u32,32));
            c.push(m, 0x43d960);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x460680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x43d960; continue;
            }
            0x43d960 => {
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m, 0x43d96b);
            { let args = []; c.r[0] = host.call(m, 0x4c78b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43e1d0; continue; }
            pc = 0x43d973; continue;
            }
            0x43d973 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m, 0x43d97e);
            { let args = []; c.r[0] = host.call(m, 0x4429d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            c.push(m,0x0u32);
            c.push(m, 0x43d98b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4349a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbdc8u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x43dc12; continue; }
            pc = 0x43d998; continue;
            }
            0x43d998 => {
            c.push(m,0x2cu32);
            c.push(m, 0x43d99f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x3u32); }
            if c.cond(4) { pc = 0x43dbf5; continue; }
            pc = 0x43d9b4; continue;
            }
            0x43d9b4 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x43d9bb);
            { let args = []; c.r[0] = host.call(m, 0x444ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43dbf7; continue;
            }
            0x43d9c0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43d9df; continue; }
            pc = 0x43d9cb; continue;
            }
            0x43d9cb => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43d9d8; continue; }
            pc = 0x43d9ce; continue;
            }
            0x43d9ce => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x43d9e3; continue; }
            pc = 0x43d9d1; continue;
            }
            0x43d9d1 => {
            c.set(7,32,0x2u32);
            pc = 0x43d9e6; continue;
            }
            0x43d9d8 => {
            c.set(7,32,0x1u32);
            pc = 0x43d9e6; continue;
            }
            0x43d9df => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x43d9e6; continue;
            }
            0x43d9e3 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xffffffc4),32));
            pc = 0x43d9e6; continue;
            }
            0x43d9e6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x43d9e8; continue;
            }
            0x43d9e8 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x204),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(8) { pc = 0x43d9f9; continue; }
            pc = 0x43d9f3; continue;
            }
            0x43d9f3 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43d9e8; continue; }
            pc = 0x43d9f9; continue;
            }
            0x43d9f9 => {
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(0,32,c.r[6].wrapping_add(0x1a4));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x168u32,32); c.set(1,32,r); }
            c.push(m, 0x43da11);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6d1c7cu32,32));
            c.set(0,32,c.r[6].wrapping_add(0x184));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[2].wrapping_add(0x2e8));
            c.push(m, 0x43da29);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1cc),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[1].wrapping_add(0x46c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1c4),8));
            { let a = c.r[0].wrapping_add(0x468); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m, 0x43da60);
            { let args = []; c.r[0] = host.call(m, 0x49a2d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6da5f0u32);
            c.push(m, 0x43da6b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b9150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x43da77);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x497ee0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            c.set(7,32,c.r[6].wrapping_add(0x4c6c));
            c.push(m,0x20u32);
            { let a = c.get(1,32); let r = c.add(a,0x1b80u32,32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x43da92);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x11c),32); c.push(m, 0x43daa3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xfffffffeu32,32); }
            if c.cond(4) { pc = 0x43d69e; continue; }
            pc = 0x43daac; continue;
            }
            0x43daac => {
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.push(m,0x20u32);
            { let a = c.get(0,32); let r = c.add(a,0x1b80u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x43dabf);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6d1c7cu32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x168u32,32); c.set(1,32,r); }
            c.push(m, 0x43dad3);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x1788));
            c.push(m, 0x43dadf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6d1c7cu32,32));
            c.set(1,32,c.r[2].wrapping_add(0x2e8));
            c.push(m, 0x43daf0);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x1908));
            c.push(m, 0x43dafc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4caad0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x2b7c),32));
            c.set(1,32,c.r[6].wrapping_add(0x2b7c));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x46c),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x4cd1); wm(m,a,8,c.get(0,8)); }
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43db22); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x4cd1),8));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2a94),32));
            c.set(1,32,c.r[6].wrapping_add(0x2a94));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(2,8,u32::from(c.cond(4)));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43db3d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6d1c7cu32,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4cd1),8));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x468),32));
            { let a = c.r[6].wrapping_add(0x1cc); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,c.get(0,8));
            { let a = c.r[6].wrapping_add(0x4860); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1c4); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x214); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x204); wm(m,a,8,0x2u32); }
            c.push(m, 0x43db78);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ea60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x1f4); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x1fc); wm(m,a,8,0x1u32); }
            c.push(m, 0x43db8d);
            { let args = []; c.r[0] = host.call(m, 0x438ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10e0),32));
            c.set(1,32,c.r[6].wrapping_add(0x10e0));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x43db9e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1a88),32));
            c.set(1,32,c.r[6].wrapping_add(0x1a88));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x43dbb2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x13bc),32));
            c.set(7,32,c.r[6].wrapping_add(0x13bc));
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x43dbc8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[7],32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x43dbd1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4cb0),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x4ca8); wm(m,a,32,0x5u32); }
            { let a = c.r[6].wrapping_add(0x204); wm(m,a,8,0x82u32); }
            c.push(m, 0x43dbf0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43ea60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x43d6a5; continue;
            }
            0x43dbf5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x43dbf7; continue;
            }
            0x43dbf7 => {
            { let a = 0x6cbdc8u32; wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x28),8));
            { let a = c.get(1,8); c.sub(a,0x1u32,8); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            if c.cond(4) { pc = 0x43dc12; continue; }
            pc = 0x43dc0b; continue;
            }
            0x43dc0b => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x43dc12);
            { let args = []; c.r[0] = host.call(m, 0x444d30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43dc12; continue;
            }
            0x43dc12 => {
            c.set(0,16,rm(m,0x697880u32,16));
            c.set(2,32,rm(m,0x69787cu32,32));
            c.set(1,8,rm(m,0x697882u32,8));
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xa),8));
            c.set(0,32,0x3u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,0x14u32);
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,0x0u32); }
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43dc50);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.push(m,0x14u32);
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x43dc5f);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd8));
            c.push(m,c.get(1,32));
            c.push(m,0x697884u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x43dc6e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(3,32,c.r[6].wrapping_add(0x1d4));
            pc = 0x43dc80; continue;
            }
            0x43dc80 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[7]).wrapping_add(0x204),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x43dcc0; continue; }
            pc = 0x43dc8a; continue;
            }
            0x43dc8a => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(1,32)); }
            if c.cond(13) { pc = 0x43dc9b; continue; }
            pc = 0x43dc98; continue;
            }
            0x43dc98 => {
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(7,32)); }
            pc = 0x43dc9b; continue;
            }
            0x43dc9b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43dcc0; continue; }
            pc = 0x43dca5; continue;
            }
            0x43dca5 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,0x6de4a0u32);
            { let a = 0x6dfc14u32; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[3],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x43dcba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,0x0u32); }
            pc = 0x43dcc0; continue;
            }
            0x43dcc0 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x43dc80; continue; }
            pc = 0x43dcc9; continue;
            }
            0x43dcc9 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43dce3; continue; }
            pc = 0x43dcd3; continue;
            }
            0x43dcd3 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1d4),32));
            { let a = 0x6dfc14u32; wm(m,a,32,c.get(2,32)); }
            pc = 0x43dce3; continue;
            }
            0x43dce3 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xffffffc4),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.push(m,0x1u32);
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43dcf9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x54b230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,c.r[5].wrapping_add(0xffffff8c));
            c.push(m, 0x43dd01);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x43dd07);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffa0));
            c.push(m, 0x43dd12);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x43df6c; continue; }
            pc = 0x43dd20; continue;
            }
            0x43dd20 => {
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43dd2a);
            { let args = []; c.r[0] = host.call(m, 0x547f10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43dd34);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[6].wrapping_add(0x184));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            pc = 0x43dd3f; continue;
            }
            0x43dd3f => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[3]).wrapping_add(0x204),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x43df21; continue; }
            pc = 0x43dd4d; continue;
            }
            0x43dd4d => {
            c.set(1,8,rm(m,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x1c4),8));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0xffffffd0),8));
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[5].wrapping_add(0xffffff8c); wm(m,a,32,0xa7u32); }
            { let a = c.r[5].wrapping_add(0xffffff95); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[5].wrapping_add(0xffffff97); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            { let a = c.r[5].wrapping_add(0xffffff94); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x9),8));
            c.set(0,8,rm(m,c.r[3].wrapping_add(c.r[6]).wrapping_add(0x1cc),8));
            { let a = c.r[5].wrapping_add(0xffffff96); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[5].wrapping_add(0xffffff98); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x43dd84);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x16du32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[5].wrapping_add(0xffffff99); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x43dd94);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[5].wrapping_add(0xffffff9a); wm(m,a,16,c.get(0,16)); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x11),8));
            { let a = c.r[5].wrapping_add(0xffffff9c); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6df31cu32,32));
            { let a = c.get(0,32); c.sub(a,0xfffffffdu32,32); }
            { let a = c.r[5].wrapping_add(0xffffff90); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x43df92; continue; }
            pc = 0x43ddb5; continue;
            }
            0x43ddb5 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffff8c));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43ddc3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43ddcd);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            c.set(2,32,c.r[7].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m, 0x43dddc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x4u32); }
            c.push(m, 0x43ddec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,0x6dfc14u32,16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffa4); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.r[5].wrapping_add(0xffffffa6); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,0xaau32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x5u32); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x43de5d; continue; }
            pc = 0x43de11; continue;
            }
            0x43de11 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffa6),32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[7]),8));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[5].wrapping_add(0xffffffa8));
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
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x43de54; continue; }
            pc = 0x43de3f; continue;
            }
            0x43de3f => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffa0));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43de4d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[7].wrapping_add(0x1));
            { let a = c.r[5].wrapping_add(0xffffffa6); wm(m,a,16,c.get(2,16)); }
            pc = 0x43de54; continue;
            }
            0x43de54 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffff8),32),32); }
            if c.cond(12) { pc = 0x43de11; continue; }
            pc = 0x43de5d; continue;
            }
            0x43de5d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffa6),32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffa8));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[5].wrapping_add(0xffffffa0));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,8,0x0u32); }
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43de7c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffa6); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,0xabu32); }
            { let a = rm(m,c.r[0].wrapping_add(0xfffffff8),32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x43dedd; continue; }
            pc = 0x43de91; continue;
            }
            0x43de91 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffa6),32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[7]),8));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[5].wrapping_add(0xffffffa8));
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
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x43ded4; continue; }
            pc = 0x43debf; continue;
            }
            0x43debf => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffa0));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43decd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[7].wrapping_add(0x1));
            { let a = c.r[5].wrapping_add(0xffffffa6); wm(m,a,16,c.get(2,16)); }
            pc = 0x43ded4; continue;
            }
            0x43ded4 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffff8),32),32); }
            if c.cond(12) { pc = 0x43de91; continue; }
            pc = 0x43dedd; continue;
            }
            0x43dedd => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffa6),32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffa8));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[5].wrapping_add(0xffffffa0));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[1].wrapping_add(c.r[7]); wm(m,a,8,0x0u32); }
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43defc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43df06);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            c.push(m, 0x43df12);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43df21);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43df21; continue;
            }
            0x43df21 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,0x8u32,32); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x43dd3f; continue; }
            pc = 0x43df34; continue;
            }
            0x43df34 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffa0));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,0xc0u32); }
            c.push(m, 0x43df49);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffa0));
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,0xc6u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43df5e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x20c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x43e06b; continue; }
            pc = 0x43df6c; continue;
            }
            0x43df6c => {
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697888u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x6u32); }
            c.push(m, 0x43df85);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x43df8a);
            { let args = []; c.r[0] = host.call(m, 0x42da50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x43e040; continue;
            }
            0x43df92 => {
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
            0x43dfa8 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x43dfb0);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x9u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x8u32); }
            c.push(m, 0x43dfbe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x43dfc6);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6978a4u32);
            { let a = c.r[2].wrapping_add(0x7cc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43dfe4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x43dfef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x424d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x43dff9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x43e001);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0xfu32);
            { let a = c.r[1].wrapping_add(0x7e4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x43e017);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x43e01f);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x7u32); }
            { let a = c.r[2].wrapping_add(0x7fc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43e037);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x43e03du32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43e03d => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffc8),32));
            pc = 0x43e040; continue;
            }
            0x43e040 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffa0));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,0xc1u32); }
            c.push(m, 0x43e05c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43e066);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x43e148; continue;
            }
            0x43e06b => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffa0));
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,0xc1u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43e080);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x43e08a);
            { let args = []; c.r[0] = host.call(m, 0x546870, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6978c0u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x9u32); }
            c.push(m, 0x43e0a3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x43e0a8);
            { let args = []; c.r[0] = host.call(m, 0x42da50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x43e148; continue;
            }
            0x43e0b0 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x43e0b8);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x9u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xbu32); }
            c.push(m, 0x43e0c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x43e0ce);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6978dcu32);
            { let a = c.r[2].wrapping_add(0x7cc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43e0ec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x43e0f7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x424d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x43e101);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x43e109);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0xfu32);
            { let a = c.r[1].wrapping_add(0x7e4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x43e11f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x43e127);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xau32); }
            { let a = c.r[2].wrapping_add(0x7fc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43e13f);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x43e145u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43e145 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffc8),32));
            pc = 0x43e148; continue;
            }
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
            pc = 0x43e1d0; continue;
            }
            0x43e1d0 => {
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
            { let target = rm(m,c.r[2].wrapping_add(0x2c),32); c.push(m, 0x499235); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
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

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "load_save_screen_load_game_file" => load_save_screen_load_game_file(m, host, regs),
        "side_menu_set_anchor_position" => side_menu_set_anchor_position(m, host, regs),
        "multiplayer_screen_update_hotseat_setup" => multiplayer_screen_update_hotseat_setup(m, host, regs),
        "new_game_screen_process_input" => new_game_screen_process_input(m, host, regs),
        _ => return None,
    })
}
