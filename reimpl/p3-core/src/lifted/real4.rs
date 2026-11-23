//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn multiplayer_screen_start_game(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43c450;
    loop {
        match pc {
            0x43c450 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65e2b0u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x43c481);
            { let args = []; c.r[0] = host.call(m, 0x4429d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb7cu32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x43c48f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4349a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4ca0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43c4ca; continue; }
            pc = 0x43c499; continue;
            }
            0x43c499 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x43c4b4; continue; }
            pc = 0x43c49c; continue;
            }
            0x43c49c => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x43c4dd; continue; }
            pc = 0x43c49f; continue;
            }
            0x43c49f => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = 0x6df2f0u32; wm(m,a,32,0x8u32); }
            { let a = c.r[0].wrapping_add(0xd); wm(m,a,8,0x2u32); }
            pc = 0x43c4dd; continue;
            }
            0x43c4b4 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = 0x6df2f0u32; wm(m,a,32,0x4u32); }
            { let a = c.r[1].wrapping_add(0xd); wm(m,a,8,0x1u32); }
            pc = 0x43c4dd; continue;
            }
            0x43c4ca => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = 0x6df2f0u32; wm(m,a,32,0x2u32); }
            { let a = c.r[2].wrapping_add(0xd); wm(m,a,8,c.get(3,8)); }
            pc = 0x43c4dd; continue;
            }
            0x43c4dd => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697824u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x43c4f1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x43c4f6);
            { let args = []; c.r[0] = host.call(m, 0x42da50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x43c596; continue;
            }
            0x43c4fe => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x43c506);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x9u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            c.push(m, 0x43c514);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x43c51c);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let a = c.r[1].wrapping_add(0x7cc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697840u32);
            c.push(m, 0x43c53a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x43c545);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x424d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x43c54f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x43c557);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.push(m,0xfu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[2].wrapping_add(0x7e4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43c56d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x43c575);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            { let a = c.r[1].wrapping_add(0x7fc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x43c58d);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x43c593u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x43c593 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            pc = 0x43c596; continue;
            }
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

pub fn std_string_copy_alloc(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x464530;
    loop {
        match pc {
            0x464530 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65ef30u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(0,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.or(a,0x1fu32,32); c.set(7,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            { let a = c.get(7,32); c.sub(a,0xfffffffdu32,32); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(6,32)); }
            if c.cond(6) { pc = 0x464565; continue; }
            pc = 0x464563; continue;
            }
            0x464563 => {
            c.set(7,32,c.get(0,32));
            pc = 0x464565; continue;
            }
            0x464565 => {
            c.set(0,32,c.r[7].wrapping_add(0x2));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x464575; continue; }
            pc = 0x464573; continue;
            }
            0x464573 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x464575; continue;
            }
            0x464575 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x46457b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x4645aa; continue;
            }
            0x464583 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x464592; continue; }
            pc = 0x464590; continue;
            }
            0x464590 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x464592; continue;
            }
            0x464592 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x464598);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,0x4645a4u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4645a4 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xffffffe8),32));
            pc = 0x4645aa; continue;
            }
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

pub fn choice_list_dialog_on_delete_answer(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x472470;
    loop {
        match pc {
            0x472470 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65f4a9u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1084),32));
            { let a = c.r[6].wrapping_add(0xadc); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,0x66bdc0u32,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x472560; continue; }
            pc = 0x4724b0; continue;
            }
            0x4724b0 => {
            c.push(m,0x6b97ccu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x4724bd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.push(m,0x6b97d4u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4724cf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(1,32,c.r[6].wrapping_add(0x938));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x4724e0);
            { let args = []; c.r[0] = host.call(m, 0x4d2ec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4724eb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f436, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            c.push(m, 0x4724fa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64f436, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x6u32); }
            c.push(m, 0x472506);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x5u32); }
            c.push(m, 0x472512);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            c.push(m, 0x47251e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x7u32); }
            c.push(m,c.get(2,32));
            c.push(m, 0x47252b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x653f49, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x472536; continue;
            }
            0x47252d => {
            c.set(0,32,0x472533u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x472533 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x472536; continue;
            }
            0x472536 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x4u32); }
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x472547); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x120),32); c.push(m, 0x472551); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x472560);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x472560; continue;
            }
            0x472560 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x472569); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x768),32));
            c.set(1,32,c.r[6].wrapping_add(0x768));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47257a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x680),32));
            c.set(1,32,c.r[6].wrapping_add(0x680));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x47258b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4a4),32));
            c.set(1,32,c.r[6].wrapping_add(0x4a4));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47259c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x850),32));
            c.set(1,32,c.r[6].wrapping_add(0x850));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x4725ad); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x938),32));
            c.set(1,32,c.r[6].wrapping_add(0x938));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x4725be); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
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
        "multiplayer_screen_start_game" => multiplayer_screen_start_game(m, host, regs),
        "std_string_copy_alloc" => std_string_copy_alloc(m, host, regs),
        "choice_list_dialog_on_delete_answer" => choice_list_dialog_on_delete_answer(m, host, regs),
        _ => return None,
    })
}
