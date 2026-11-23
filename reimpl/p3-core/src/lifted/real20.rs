//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

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

pub fn personal_letters_side_menu_open_selected_letter(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x47f3b0;
    loop {
        match pc {
            0x47f3b0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65fae9u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x47cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(5,32,rm(m,c.r[3].wrapping_add(0xa0),32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x478));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m, 0x47f3ed);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1000u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x1a); wm(m,a,16,c.get(6,16)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x3fb00000u32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,c.get(6,16)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x23); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x21); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,8,0x0u32); }
            c.push(m, 0x47f437);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x486605; continue; }
            pc = 0x47f446; continue;
            }
            0x47f446 => {
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x486612; continue; }
            pc = 0x47f44e; continue;
            }
            0x47f44e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xd2),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x47f473; continue; }
            pc = 0x47f45d; continue;
            }
            0x47f45d => {
            c.set(7,16,rm(m,0x6dd736u32,16));
            c.set(2,32,c.get(5,32));
            pc = 0x47f466; continue;
            }
            0x47f466 => {
            { let a = rm(m,c.r[2],16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x47f4bf; continue; }
            pc = 0x47f46b; continue;
            }
            0x47f46b => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x14u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x47f466; continue; }
            pc = 0x47f473; continue;
            }
            0x47f473 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x560),32));
            c.set(1,32,c.r[3].wrapping_add(0x560));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47f484); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(0xb0); wm(m,a,32,0xffffffffu32); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xd2),16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6dd736u32,16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,16,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)),16));
            c.set(0,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x47f501; continue; }
            pc = 0x47f4b2; continue;
            }
            0x47f4b2 => {
            c.set(1,32,rm(m,0x6dd730u32,32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            pc = 0x47f503; continue;
            }
            0x47f4bf => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x118),32); c.push(m, 0x47f4cd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x560),32));
            c.set(1,32,c.r[3].wrapping_add(0x560));
            c.push(m,0x0u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x47f4de); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xd4),16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x47f4ef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x47c820, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x47f4f9);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x486632; continue;
            }
            0x47f501 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x47f503; continue;
            }
            0x47f503 => {
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x0u32); }
            c.set(0,8,rm(m,0x6b9cacu32,8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47f525; continue; }
            pc = 0x47f512; continue;
            }
            0x47f512 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6b9cac),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47f512; continue; }
            pc = 0x47f525; continue;
            }
            0x47f525 => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x47f56a; continue; }
            pc = 0x47f534; continue;
            }
            0x47f534 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9cb4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x47f54b);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x47f56a; continue; }
            pc = 0x47f55a; continue;
            }
            0x47f55a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47f55a; continue; }
            pc = 0x47f56a; continue;
            }
            0x47f56a => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2),16));
            { let a = c.get(1,32); let r = c.and(a,0xfu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[7],8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(0,32,c.r[4].wrapping_add(0x70));
            c.push(m,0x6b9cbcu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x47f58d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x47f5ac; continue; }
            pc = 0x47f59c; continue;
            }
            0x47f59c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47f59c; continue; }
            pc = 0x47f5ac; continue;
            }
            0x47f5ac => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x6c0198),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47f5bf; continue; }
            pc = 0x47f5bb; continue;
            }
            0x47f5bb => {
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x47f5c7; continue; }
            pc = 0x47f5bf; continue;
            }
            0x47f5bf => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[1].wrapping_add(0x5); wm(m,a,8,0x31u32); }
            pc = 0x47f5c7; continue;
            }
            0x47f5c7 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = rm(m,c.r[2].wrapping_add(0x6c0198),8); c.sub(a,0x1u32,8); }
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.get(0,32); c.sub(a,0x85u32,32); }
            if c.cond(7) { pc = 0x4865e5; continue; }
            pc = 0x47f5ea; continue;
            }
            0x47f5ea => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x48664c)); continue;
            }
            0x47f5f1 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0e80),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6b2e90u32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47f61d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x47f625 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(1,32));
            c.push(m,0x25u32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0e80),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6b2f20u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x47f65e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x47f666 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x47f6cb; continue; }
            pc = 0x47f671; continue;
            }
            0x47f671 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x160));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x47f6a4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(0,32,rm(m,0x6b3048u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0e38),32));
            c.set(1,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x47f6c3);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x47f6cb => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x47f6e4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b304cu32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0e38),32));
            c.set(2,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x47f704);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x47f70c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x5); wm(m,a,8,0x31u32); }
            c.set(0,32,rm(m,0x6b30dcu32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x3f400000u32); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,32,0x1u32); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a57c8),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xd2),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(2,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6b30d8u32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47f767);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x47f76f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x5); wm(m,a,8,0x31u32); }
            c.set(0,32,rm(m,0x6dd7a0u32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x3f400000u32); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(5,32,c.r[2].wrapping_add(c.r[0]));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x160));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x3),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x47f7c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x2),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x47f7d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6b317cu32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47f7ea);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x47f7f2 => {
            c.set(2,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(3,32,c.r[1].wrapping_add(c.r[2]));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x160));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x3),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x47f83b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x2),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x47f84c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b31e0u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x47f85e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x47f866 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,0x8000u32,16); }
            if c.cond(2) { pc = 0x47f89b; continue; }
            pc = 0x47f870; continue;
            }
            0x47f870 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x7fffu32,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47f88d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b33a0u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x47f8c0; continue;
            }
            0x47f89b => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x47f8b3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6b339cu32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x47f8c0; continue;
            }
            0x47f8c0 => {
            c.push(m, 0x47f8c5);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x47f8e4; continue; }
            pc = 0x47f8d4; continue;
            }
            0x47f8d4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47f8d4; continue; }
            pc = 0x47f8e4; continue;
            }
            0x47f8e4 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x47f900; continue; }
            pc = 0x47f8eb; continue;
            }
            0x47f8eb => {
            c.set(0,32,rm(m,0x6b33acu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x47f8fb);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x47f928; continue;
            }
            0x47f900 => {
            if c.cond(14) { pc = 0x47f911; continue; }
            pc = 0x47f902; continue;
            }
            0x47f902 => {
            c.set(2,32,rm(m,0x6b33a4u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x47f920; continue;
            }
            0x47f911 => {
            c.set(1,32,rm(m,0x6b33a8u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x47f920; continue;
            }
            0x47f920 => {
            c.push(m, 0x47f925);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x47f928; continue;
            }
            0x47f928 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47f944; continue; }
            pc = 0x47f934; continue;
            }
            0x47f934 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47f934; continue; }
            pc = 0x47f944; continue;
            }
            0x47f944 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x47f957);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x47f968);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,0x6b9cccu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x47f978);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x47f980 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x47f991);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x47f9a7; continue; }
            pc = 0x47f997; continue;
            }
            0x47f997 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x47f997; continue; }
            pc = 0x47f9a7; continue;
            }
            0x47f9a7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x47f9c1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x47f9d7; continue; }
            pc = 0x47f9c7; continue;
            }
            0x47f9c7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x47f9c7; continue; }
            pc = 0x47f9d7; continue;
            }
            0x47f9d7 => {
            c.set(1,32,rm(m,0x6b4a18u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47f9f3; continue; }
            pc = 0x47f9e3; continue;
            }
            0x47f9e3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47f9e3; continue; }
            pc = 0x47f9f3; continue;
            }
            0x47f9f3 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(0,16); c.sub(a,0x8u32,16); }
            if c.cond(5) { pc = 0x47fa05; continue; }
            pc = 0x47f9fd; continue;
            }
            0x47f9fd => {
            c.set(1,32,rm(m,0x6ae5c8u32,32));
            pc = 0x47fa11; continue;
            }
            0x47fa05 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6b4a0c),32));
            pc = 0x47fa11; continue;
            }
            0x47fa11 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47fa27; continue; }
            pc = 0x47fa17; continue;
            }
            0x47fa17 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47fa17; continue; }
            pc = 0x47fa27; continue;
            }
            0x47fa27 => {
            c.set(1,32,rm(m,0x6b4a1cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x486513; continue; }
            pc = 0x47fa37; continue;
            }
            0x47fa37 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47fa37; continue; }
            pc = 0x47fa47; continue;
            }
            0x47fa47 => {
            pc = 0x486513; continue;
            }
            0x47fa4c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[1].wrapping_add(0x5); wm(m,a,8,0x33u32); }
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x47faa0; continue; }
            pc = 0x47fa63; continue;
            }
            0x47fa63 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xa),8));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6add54),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6b70f4u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47fa9b);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x47fac2; continue;
            }
            0x47faa0 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,0x6adddcu32,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47fabf);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x47fac2; continue;
            }
            0x47fac2 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47fade; continue; }
            pc = 0x47face; continue;
            }
            0x47face => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47face; continue; }
            pc = 0x47fade; continue;
            }
            0x47fade => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(7) { pc = 0x47fb2b; continue; }
            pc = 0x47faf2; continue;
            }
            0x47faf2 => {
            { let a = c.get(2,32); let r = c.and(a,0xfeu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(2,32); c.sub(a,0x2u32,32); }
            c.push(m,c.get(1,32));
            if c.cond(13) { pc = 0x47fb67; continue; }
            pc = 0x47fb0a; continue;
            }
            0x47fb0a => {
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6add10),32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x47fb1c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(1,8)); }
            pc = 0x48651f; continue;
            }
            0x47fb2b => {
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.or(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); c.sub(a,0x2u32,32); }
            c.push(m,c.get(0,32));
            if c.cond(13) { pc = 0x47fb67; continue; }
            pc = 0x47fb46; continue;
            }
            0x47fb46 => {
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6add10),32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x47fb58);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(1,8)); }
            pc = 0x48651f; continue;
            }
            0x47fb67 => {
            c.set(2,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6add10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47fb87);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(1,8)); }
            pc = 0x48651f; continue;
            }
            0x47fb96 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x47fba7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,16,rm(m,c.r[0].wrapping_add(0xa),16));
            c.set(2,16,rm(m,0x6dd736u32,16));
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x37); wm(m,a,8,0x0u32); }
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            { let a = c.r[4].wrapping_add(0x36); wm(m,a,8,0x0u32); }
            if c.cond(3) { pc = 0x47fc0d; continue; }
            pc = 0x47fbc3; continue;
            }
            0x47fbc3 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,c.get(2,16));
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            pc = 0x47fbcc; continue;
            }
            0x47fbcc => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(7) { pc = 0x47fbec; continue; }
            pc = 0x47fbd7; continue;
            }
            0x47fbd7 => {
            c.set(3,32,rm(m,0x6dd730u32,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            pc = 0x47fbf2; continue;
            }
            0x47fbec => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            pc = 0x47fbf2; continue;
            }
            0x47fbf2 => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),8); c.sub(a,0xfu32,8); }
            if c.cond(4) { pc = 0x47fc03; continue; }
            pc = 0x47fbf8; continue;
            }
            0x47fbf8 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x47fbcc; continue; }
            pc = 0x47fc01; continue;
            }
            0x47fc01 => {
            pc = 0x47fc0d; continue;
            }
            0x47fc03 => {
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x47fc0d; continue; }
            pc = 0x47fc08; continue;
            }
            0x47fc08 => {
            { let a = c.r[4].wrapping_add(0x37); wm(m,a,8,0x1u32); }
            pc = 0x47fc0d; continue;
            }
            0x47fc0d => {
            c.set(1,32,c.get(5,32));
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x47fc52; continue; }
            pc = 0x47fc14; continue;
            }
            0x47fc14 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,c.get(2,16));
            pc = 0x47fc19; continue;
            }
            0x47fc19 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(7) { pc = 0x47fc33; continue; }
            pc = 0x47fc24; continue;
            }
            0x47fc24 => {
            c.set(3,32,rm(m,0x6dd730u32,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(0,32));
            pc = 0x47fc35; continue;
            }
            0x47fc33 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x47fc35; continue;
            }
            0x47fc35 => {
            { let a = rm(m,c.r[3].wrapping_add(0x4),8); c.sub(a,0x10u32,8); }
            if c.cond(4) { pc = 0x47fc46; continue; }
            pc = 0x47fc3b; continue;
            }
            0x47fc3b => {
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x6),16));
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x47fc19; continue; }
            pc = 0x47fc44; continue;
            }
            0x47fc44 => {
            pc = 0x47fc56; continue;
            }
            0x47fc46 => {
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x47fc56; continue; }
            pc = 0x47fc4b; continue;
            }
            0x47fc4b => {
            { let a = c.r[4].wrapping_add(0x36); wm(m,a,8,0x1u32); }
            pc = 0x47fc56; continue;
            }
            0x47fc52 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            pc = 0x47fc56; continue;
            }
            0x47fc56 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[0].wrapping_add(0x5); wm(m,a,8,0x31u32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x3f400000u32); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x47fc9e; continue; }
            pc = 0x47fc6d; continue;
            }
            0x47fc6d => {
            c.set(1,32,rm(m,0x6b39f0u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x47fc7f);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47fc9e; continue; }
            pc = 0x47fc8a; continue;
            }
            0x47fc8a => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x47fc8e; continue;
            }
            0x47fc8e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47fc8e; continue; }
            pc = 0x47fc9e; continue;
            }
            0x47fc9e => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x37),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47fce2; continue; }
            pc = 0x47fca6; continue;
            }
            0x47fca6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x47fce2; continue; }
            pc = 0x47fcb1; continue;
            }
            0x47fcb1 => {
            c.set(1,32,rm(m,0x6b38c0u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x47fcc3);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47fce2; continue; }
            pc = 0x47fcce; continue;
            }
            0x47fcce => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x47fcd2; continue;
            }
            0x47fcd2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47fcd2; continue; }
            pc = 0x47fce2; continue;
            }
            0x47fce2 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x36),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47fd21; continue; }
            pc = 0x47fcea; continue;
            }
            0x47fcea => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x47fd21; continue; }
            pc = 0x47fcf1; continue;
            }
            0x47fcf1 => {
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b3950u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x47fd02);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47fd21; continue; }
            pc = 0x47fd0d; continue;
            }
            0x47fd0d => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x47fd11; continue;
            }
            0x47fd11 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47fd11; continue; }
            pc = 0x47fd21; continue;
            }
            0x47fd21 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x37),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47fd65; continue; }
            pc = 0x47fd29; continue;
            }
            0x47fd29 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x47fd65; continue; }
            pc = 0x47fd34; continue;
            }
            0x47fd34 => {
            c.set(1,32,rm(m,0x6b38bcu32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x47fd46);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47fd65; continue; }
            pc = 0x47fd51; continue;
            }
            0x47fd51 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x47fd55; continue;
            }
            0x47fd55 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47fd55; continue; }
            pc = 0x47fd65; continue;
            }
            0x47fd65 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x36),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47fda9; continue; }
            pc = 0x47fd6d; continue;
            }
            0x47fd6d => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x47fda9; continue; }
            pc = 0x47fd74; continue;
            }
            0x47fd74 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(2,32,rm(m,0x6b39f4u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47fd8a);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47fda9; continue; }
            pc = 0x47fd95; continue;
            }
            0x47fd95 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x47fd99; continue;
            }
            0x47fd99 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47fd99; continue; }
            pc = 0x47fda9; continue;
            }
            0x47fda9 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x47fdb3 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x47fdcb; continue; }
            pc = 0x47fdbc; continue;
            }
            0x47fdbc => {
            c.set(0,32,0x1u32);
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0xe); wm(m,a,16,c.get(0,16)); }
            pc = 0x47fdef; continue;
            }
            0x47fdcb => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672d92),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x47fdef; continue; }
            pc = 0x47fde5; continue;
            }
            0x47fde5 => {
            { let a = c.r[7].wrapping_add(0xe); wm(m,a,16,0x1u32); }
            { let a = rm(m,c.r[7].wrapping_add(0xc),16); let r = c.inc(a,16); { let a = c.r[7].wrapping_add(0xc); wm(m,a,16,r); } }
            pc = 0x47fdef; continue;
            }
            0x47fdef => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b3aa0u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x78));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x47fe28);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x80),8));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47fe4a; continue; }
            pc = 0x47fe36; continue;
            }
            0x47fe36 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x47fe3a; continue;
            }
            0x47fe3a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47fe3a; continue; }
            pc = 0x47fe4a; continue;
            }
            0x47fe4a => {
            c.set(0,32,rm(m,0x6b3aa4u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x47fe6f; continue; }
            pc = 0x47fe57; continue;
            }
            0x47fe57 => {
            c.set(0,8,c.get(2,8));
            pc = 0x47fe59; continue;
            }
            0x47fe59 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6b3aa4u32,32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[1]),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47fe59; continue; }
            pc = 0x47fe6f; continue;
            }
            0x47fe6f => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x47fe79 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xa),8));
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x47fedf; continue; }
            pc = 0x47fe83; continue;
            }
            0x47fe83 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x47fec0; continue; }
            pc = 0x47fe86; continue;
            }
            0x47fe86 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x47ff03; continue; }
            pc = 0x47fe89; continue;
            }
            0x47fe89 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x47fe9c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6b3d78u32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x47febb);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x47ff03; continue;
            }
            0x47fec0 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6b3d74u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x47fefb; continue;
            }
            0x47fedf => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b3d70u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x47fefb; continue;
            }
            0x47fefb => {
            c.push(m, 0x47ff00);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x47ff03; continue;
            }
            0x47ff03 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48651f; continue; }
            pc = 0x47ff0e; continue;
            }
            0x47ff0e => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47ff2a; continue; }
            pc = 0x47ff1a; continue;
            }
            0x47ff1a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47ff1a; continue; }
            pc = 0x47ff2a; continue;
            }
            0x47ff2a => {
            c.set(1,32,rm(m,0x6b3d7cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x47ff46; continue; }
            pc = 0x47ff36; continue;
            }
            0x47ff36 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x47ff36; continue; }
            pc = 0x47ff46; continue;
            }
            0x47ff46 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x47ff50 => {
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xd),8));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            if c.cond(12) { pc = 0x47ffac; continue; }
            pc = 0x47ff63; continue;
            }
            0x47ff63 => {
            c.set(5,32,0xcu32);
            c.push(m,c.get(3,32));
            { let b = c.get(5,32); c.idiv32(b); }
            c.set(5,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(2,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0d04),32));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0e80),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b3e94u32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x7c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x47ffa4);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x47ffac => {
            c.set(3,32,0xcu32);
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(5,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(2,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0d04),32));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0e80),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b3e98u32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x78));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x47ffec);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x47fff4 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(2,32));
            c.push(m, 0x480004);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); c.sub(a,0x0u32,16); }
            c.set(3,32,c.get(0,32));
            if c.cond(4) { pc = 0x4800fe; continue; }
            pc = 0x480011; continue;
            }
            0x480011 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x4800fe; continue; }
            pc = 0x48001e; continue;
            }
            0x48001e => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,16); c.and(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x480058; continue; }
            pc = 0x480035; continue;
            }
            0x480035 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x480040);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b4180u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(2,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x480053);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x480080; continue;
            }
            0x480058 => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48006a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b4184u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(2,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x48007d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x480080; continue;
            }
            0x480080 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48009c; continue; }
            pc = 0x480088; continue;
            }
            0x480088 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x48008c; continue;
            }
            0x48008c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48008c; continue; }
            pc = 0x48009c; continue;
            }
            0x48009c => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,32,0x6c0bb0u32);
            pc = 0x4800a3; continue;
            }
            0x4800a3 => {
            c.set(0,32,0x1u32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4800e7; continue; }
            pc = 0x4800b4; continue;
            }
            0x4800b4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(0,32,rm(m,c.r[5],32));
            c.set(0,8,rm(m,c.r[0],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4800dd; continue; }
            pc = 0x4800c9; continue;
            }
            0x4800c9 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[5],32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[0]),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4800c9; continue; }
            pc = 0x4800dd; continue;
            }
            0x4800dd => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x2cu32); }
            pc = 0x4800e7; continue;
            }
            0x4800e7 => {
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,0x6c0bf0u32,32); }
            if c.cond(12) { pc = 0x4800a3; continue; }
            pc = 0x4800f3; continue;
            }
            0x4800f3 => {
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x4800fe => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x480135; continue; }
            pc = 0x48010c; continue;
            }
            0x48010c => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x48011a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b4178u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x48012d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x480135 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x480149);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b417cu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(2,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x48015c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x480164 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x480175);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x480187);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6b3f44u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x480199);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x4801a1 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x4801b6; continue; }
            pc = 0x4801ae; continue;
            }
            0x4801ae => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            pc = 0x4801c8; continue;
            }
            0x4801b6 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            pc = 0x4801c8; continue;
            }
            0x4801c8 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(1,32); let r = c.add(a,0x160u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x5u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0d00),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6b41f8u32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x78));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x480219);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x480221 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6b43d0),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x48024e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x480256 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x480266);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48027c; continue; }
            pc = 0x48026c; continue;
            }
            0x48026c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48026c; continue; }
            pc = 0x48027c; continue;
            }
            0x48027c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x480295);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4802ab; continue; }
            pc = 0x48029b; continue;
            }
            0x48029b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48029b; continue; }
            pc = 0x4802ab; continue;
            }
            0x4802ab => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[7].wrapping_add(0xa),8));
            { let a = c.get(3,32); c.sub(a,0x8u32,32); }
            if c.cond(13) { pc = 0x4802c1; continue; }
            pc = 0x4802b5; continue;
            }
            0x4802b5 => {
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6b48b8),32));
            pc = 0x4802cc; continue;
            }
            0x4802c1 => {
            c.set(1,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)).wrapping_add(0xffffffe8));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6ae5c8),32));
            pc = 0x4802cc; continue;
            }
            0x4802cc => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4802e2; continue; }
            pc = 0x4802d2; continue;
            }
            0x4802d2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4802d2; continue; }
            pc = 0x4802e2; continue;
            }
            0x4802e2 => {
            { let a = c.get(3,32); c.sub(a,0xfu32,32); }
            if c.cond(7) { pc = 0x48050a; continue; }
            pc = 0x4802eb; continue;
            }
            0x4802eb => {
            pc = m.r32(c.r[3].wrapping_mul(4).wrapping_add(0x486864)); continue;
            }
            0x4802f2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x6dda00),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x6dd8c0),32));
            c.set(0,32,rm(m,0x6b48bcu32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x480502; continue;
            }
            0x480318 => {
            c.set(1,32,rm(m,0x6b48c8u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x480336);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48050a; continue;
            }
            0x48033e => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x48034d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b48d4u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x48035f);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48050a; continue;
            }
            0x480367 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x480377);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x480388);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b48e0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4803a7);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x48050a; continue;
            }
            0x4803af => {
            c.set(0,32,rm(m,0x6b48ecu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4803bf);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x48050a; continue;
            }
            0x4803c7 => {
            c.set(2,32,rm(m,0x6ae5ccu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4803d8);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x48050a; continue;
            }
            0x4803e0 => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4803ef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6ae5d8u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x480401);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48050a; continue;
            }
            0x480409 => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x480418);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6ae5e4u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x48042a);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48050a; continue;
            }
            0x480432 => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x480441);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6ae5f0u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x480453);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48050a; continue;
            }
            0x48045b => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48046a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6ae5fcu32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x48047c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48050a; continue;
            }
            0x480484 => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x480493);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6ae608u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x480502; continue;
            }
            0x4804af => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4804be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.set(0,32,rm(m,0x6ae614u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x480502; continue;
            }
            0x4804d9 => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4804e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6ae620u32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x480502; continue;
            }
            0x480502 => {
            c.push(m, 0x480507);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48050a; continue;
            }
            0x48050a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480526; continue; }
            pc = 0x480516; continue;
            }
            0x480516 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x480516; continue; }
            pc = 0x480526; continue;
            }
            0x480526 => {
            { let a = c.get(3,32); c.sub(a,0x8u32,32); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            if c.cond(13) { pc = 0x48053c; continue; }
            pc = 0x480530; continue;
            }
            0x480530 => {
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6b48c0),32));
            pc = 0x480546; continue;
            }
            0x48053c => {
            c.set(1,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6ae570),32));
            pc = 0x480546; continue;
            }
            0x480546 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48055c; continue; }
            pc = 0x48054c; continue;
            }
            0x48054c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48054c; continue; }
            pc = 0x48055c; continue;
            }
            0x48055c => {
            c.set(1,32,rm(m,0x6b48f4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x486518; continue; }
            pc = 0x48056c; continue;
            }
            0x48056c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48056c; continue; }
            pc = 0x48057c; continue;
            }
            0x48057c => {
            pc = 0x486518; continue;
            }
            0x480581 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0xcu32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(5,16,rm(m,c.r[7].wrapping_add(0x2),16));
            c.set(1,32,0x18u32);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(3,32,c.get(2,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480625; continue; }
            pc = 0x4805b4; continue;
            }
            0x4805b4 => {
            c.set(2,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(2,32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x6dd8c0),32));
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4805ed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4805fe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b4bb0u32,32));
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.push(m,c.get(5,32));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(2,32,c.r[4].wrapping_add(0x88));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x48061d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x480625 => {
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x6dda00),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x6dd8c0),32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48065d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48066e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b4bb4u32,32));
            { let a = c.get(2,32); let r = c.and(a,0x1fu32,32); c.set(2,32,r); }
            c.push(m,c.get(5,32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[4].wrapping_add(0x88));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x48068c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x480694 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x480800; continue; }
            pc = 0x4806a3; continue;
            }
            0x4806a3 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4868a4)); continue;
            }
            0x4806aa => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(3) { pc = 0x48071a; continue; }
            pc = 0x4806b1; continue;
            }
            0x4806b1 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4806c1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(2,32); let n = 0xbu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4806d6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x3fffu32,32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4806f2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,32,rm(m,0x6b4d2cu32,32));
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0e38),32));
            c.set(2,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x480712);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x480800; continue;
            }
            0x48071a => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            { let a = c.get(1,32); let r = c.and(a,0x3fffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x480735);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,32,rm(m,0x6b4d28u32,32));
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0e38),32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x4807f8; continue;
            }
            0x480755 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(3) { pc = 0x4807c2; continue; }
            pc = 0x48075c; continue;
            }
            0x48075c => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x48076c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(0,32); let n = 0xbu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x480781);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0x3fffu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48079d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6b4d34u32,32));
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0e38),32));
            c.set(0,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4807bd);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x480800; continue;
            }
            0x4807c2 => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xf),8));
            { let a = c.get(2,32); let r = c.and(a,0x3fffu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4807dd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6b4d30u32,32));
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0e38),32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x4807f8; continue;
            }
            0x4807f8 => {
            c.push(m, 0x4807fd);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x480800; continue;
            }
            0x480800 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x48651f; continue; }
            pc = 0x48080e; continue;
            }
            0x48080e => {
            c.push(m, 0x480813);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48651f; continue; }
            pc = 0x48081b; continue;
            }
            0x48081b => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,0x3u32,16); }
            if c.cond(6) { pc = 0x48651f; continue; }
            pc = 0x480829; continue;
            }
            0x480829 => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(1,32); let r = c.and(a,0x7ffu32,32); c.set(1,32,r); }
            { let a = c.get(0,16); let n = 0x2u32; let r = c.shr(a,n,16); c.set(0,16,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x480844);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x44f490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x48651f; continue;
            }
            0x480849 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,0x6b9cd8u32);
            c.push(m,c.get(0,32));
            { let a = c.r[2].wrapping_add(0x5); wm(m,a,8,0x31u32); }
            c.push(m, 0x480860);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x480868 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4808d6; continue; }
            pc = 0x480875; continue;
            }
            0x480875 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x480885);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.push(m,0xau32);
            c.push(m,0xau32);
            c.push(m,0xau32);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x480894);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x48089c);
            { let args = []; c.r[0] = host.call(m, 0x4f6420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x1a),8));
            c.set(0,32,c.r[4].wrapping_add(0x78));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6b4f78),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4808b4);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x80),8));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4808d6; continue; }
            pc = 0x4808c2; continue;
            }
            0x4808c2 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x4808c6; continue;
            }
            0x4808c6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4808c6; continue; }
            pc = 0x4808d6; continue;
            }
            0x4808d6 => {
            c.set(1,32,rm(m,0x6de584u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4808f2; continue; }
            pc = 0x4808e0; continue;
            }
            0x4808e0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[7].wrapping_add(0x8));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4808f0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x533ee0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            pc = 0x4808f2; continue;
            }
            0x4808f2 => {
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = rm(m,c.r[7].wrapping_add(0xc),16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x480968; continue; }
            pc = 0x4808fe; continue;
            }
            0x4808fe => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0xau32); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[2].wrapping_add(c.r[6]); wm(m,a,8,0xau32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x5cu32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]); wm(m,a,8,0x63u32); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x480934);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x48093d);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x480945);
            { let args = []; c.r[0] = host.call(m, 0x4f6420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b4f9cu32,32));
            c.push(m,0x72u32);
            c.push(m,0x5cu32);
            c.push(m,0xau32);
            c.set(1,32,c.r[4].wrapping_add(0x78));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x48095c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            pc = 0x48096d; continue;
            }
            0x480968 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48096d; continue;
            }
            0x48096d => {
            c.push(m,0x8u32);
            c.push(m, 0x480974);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x48651f; continue; }
            pc = 0x480983; continue;
            }
            0x480983 => {
            { let a = rm(m,c.r[7].wrapping_add(0x4),8); c.sub(a,0x1eu32,8); }
            if c.cond(5) { pc = 0x48099d; continue; }
            pc = 0x480989; continue;
            }
            0x480989 => {
            c.set(1,32,rm(m,0x6b503cu32,32));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,0x1u32); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x48651f; continue;
            }
            0x48099d => {
            c.set(2,32,rm(m,0x6b5028u32,32));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,0x2u32); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,0x6b502cu32,32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            pc = 0x48651f; continue;
            }
            0x4809c5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x5); wm(m,a,8,0x31u32); }
            c.set(0,32,rm(m,0x6b50c8u32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x3f666666u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4809f2);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480a11; continue; }
            pc = 0x4809fd; continue;
            }
            0x4809fd => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x480a01; continue;
            }
            0x480a01 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x480a01; continue; }
            pc = 0x480a11; continue;
            }
            0x480a11 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x480a4f; continue; }
            pc = 0x480a1a; continue;
            }
            0x480a1a => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b50ccu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x480a30);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480a4f; continue; }
            pc = 0x480a3b; continue;
            }
            0x480a3b => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x480a3f; continue;
            }
            0x480a3f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x480a3f; continue; }
            pc = 0x480a4f; continue;
            }
            0x480a4f => {
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x480ad4; continue; }
            pc = 0x480a56; continue;
            }
            0x480a56 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x480ad4; continue; }
            pc = 0x480a5f; continue;
            }
            0x480a5f => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b50d0u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x480a75);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480a94; continue; }
            pc = 0x480a80; continue;
            }
            0x480a80 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x480a84; continue;
            }
            0x480a84 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x480a84; continue; }
            pc = 0x480a94; continue;
            }
            0x480a94 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x480a96; continue;
            }
            0x480a96 => {
            c.set(0,32,0x1u32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x480ace; continue; }
            pc = 0x480aa7; continue;
            }
            0x480aa7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(0,8,rm(m,c.r[2],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480ace; continue; }
            pc = 0x480abe; continue;
            }
            0x480abe => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[2],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x480abe; continue; }
            pc = 0x480ace; continue;
            }
            0x480ace => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x10u32,32); }
            if c.cond(12) { pc = 0x480a96; continue; }
            pc = 0x480ad4; continue;
            }
            0x480ad4 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x480ade => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x480aef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,c.get(5,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1a),8));
            c.push(m,c.get(2,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480b1c; continue; }
            pc = 0x480b01; continue;
            }
            0x480b01 => {
            c.push(m, 0x480b06);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b5298u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x480b17);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x480b3d; continue;
            }
            0x480b1c => {
            c.push(m, 0x480b21);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x480b29);
            { let args = []; c.r[0] = host.call(m, 0x4f6420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b529cu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x480b3a);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x480b3d; continue;
            }
            0x480b3d => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480b59; continue; }
            pc = 0x480b45; continue;
            }
            0x480b45 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x480b49; continue;
            }
            0x480b49 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x480b49; continue; }
            pc = 0x480b59; continue;
            }
            0x480b59 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x480b6a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x480b7c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6b52a4u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x480b8e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(13) { pc = 0x4844dc; continue; }
            pc = 0x480bb8; continue;
            }
            0x480bb8 => {
            c.push(m,0x4u32);
            c.push(m, 0x480bbf);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,0x100u32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[3].wrapping_add(0xb0); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x48651f; continue; }
            pc = 0x480bde; continue;
            }
            0x480bde => {
            c.set(1,32,rm(m,0x6b52a0u32,32));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,0x1u32); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x48651f; continue;
            }
            0x480bf2 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x480c03);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.set(5,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1a),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480c6e; continue; }
            pc = 0x480c14; continue;
            }
            0x480c14 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,32); let r = c.add(a,0x88b8u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            { let b = c.get(1,32); c.div32(b); }
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x480c3e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x480c4f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x480c57);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6b5420u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x78));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x480c69);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x480cce; continue;
            }
            0x480c6e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,32); let r = c.add(a,0x88b8u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            { let b = c.get(1,32); c.div32(b); }
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x480c98);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x480ca9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x480cb1);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x480cb9);
            { let args = []; c.r[0] = host.call(m, 0x4f6420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6b5424u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x7c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x480ccb);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            pc = 0x480cce; continue;
            }
            0x480cce => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480cea; continue; }
            pc = 0x480cd6; continue;
            }
            0x480cd6 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x480cda; continue;
            }
            0x480cda => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x480cda; continue; }
            pc = 0x480cea; continue;
            }
            0x480cea => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(2,32); let r = c.and(a,0xfu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0xeu32,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_mul(2).wrapping_add(0x672d94),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x480d08; continue; }
            pc = 0x480d05; continue;
            }
            0x480d05 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x480d08; continue;
            }
            0x480d08 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,0xcu32,32); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x2),16));
            if c.cond(13) { pc = 0x480d1e; continue; }
            pc = 0x480d13; continue;
            }
            0x480d13 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9cdcu32);
            pc = 0x480d2a; continue;
            }
            0x480d1e => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0xfffffff5u32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9ce8u32);
            pc = 0x480d2a; continue;
            }
            0x480d2a => {
            c.set(2,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(2,32));
            c.push(m, 0x480d34);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480d53; continue; }
            pc = 0x480d3f; continue;
            }
            0x480d3f => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x480d43; continue;
            }
            0x480d43 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x480d43; continue; }
            pc = 0x480d53; continue;
            }
            0x480d53 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x480d63);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x480d74);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6b5428u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x480d86);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xfu32,32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_mul(2).wrapping_add(0x672d78),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x2),16));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7],8));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x48651f; continue; }
            pc = 0x480dc8; continue;
            }
            0x480dc8 => {
            c.push(m,0x4u32);
            { let a = c.r[3].wrapping_add(0xb0); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x480dd5);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x48651f; continue; }
            pc = 0x480de4; continue;
            }
            0x480de4 => {
            c.set(2,32,rm(m,0x6b52a0u32,32));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,0x1u32); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            pc = 0x48651f; continue;
            }
            0x480df8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0x80000003u32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            if c.cond(9) { pc = 0x480e1e; continue; }
            pc = 0x480e19; continue;
            }
            0x480e19 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xfffffffcu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x480e1e; continue;
            }
            0x480e1e => {
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6b57fc),32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x480e30);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x480e38 => {
            c.set(2,32,rm(m,0x6b58d0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0e80),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x480e57);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480e76; continue; }
            pc = 0x480e62; continue;
            }
            0x480e62 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x480e66; continue;
            }
            0x480e66 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x480e66; continue; }
            pc = 0x480e76; continue;
            }
            0x480e76 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x480ee1; continue; }
            pc = 0x480e7f; continue;
            }
            0x480e7f => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(1,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x480ebe; continue; }
            pc = 0x480e8a; continue;
            }
            0x480e8a => {
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x4868b4)); continue;
            }
            0x480e91 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b58d4u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x480eb6; continue;
            }
            0x480ea4 => {
            c.set(2,32,rm(m,0x6b58e0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x480eb6; continue;
            }
            0x480eb6 => {
            c.push(m, 0x480ebb);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x480ebe; continue;
            }
            0x480ebe => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            c.set(2,32,0x3u32);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480ee6; continue; }
            pc = 0x480ecb; continue;
            }
            0x480ecb => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x480ecf; continue;
            }
            0x480ecf => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[3].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x480ecf; continue; }
            pc = 0x480edf; continue;
            }
            0x480edf => {
            pc = 0x480ee6; continue;
            }
            0x480ee1 => {
            c.set(2,32,0x2u32);
            pc = 0x480ee6; continue;
            }
            0x480ee6 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(0,16); c.sub(a,0x3u32,16); }
            if c.cond(7) { pc = 0x480f96; continue; }
            pc = 0x480ef4; continue;
            }
            0x480ef4 => {
            c.set(2,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6b58d0),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0e80),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x480f13);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480f32; continue; }
            pc = 0x480f1e; continue;
            }
            0x480f1e => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x480f22; continue;
            }
            0x480f22 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x480f22; continue; }
            pc = 0x480f32; continue;
            }
            0x480f32 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x480f96; continue; }
            pc = 0x480f3b; continue;
            }
            0x480f3b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(1,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x480f7a; continue; }
            pc = 0x480f46; continue;
            }
            0x480f46 => {
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x4868c4)); continue;
            }
            0x480f4d => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b58d4u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x480f72; continue;
            }
            0x480f60 => {
            c.set(2,32,rm(m,0x6b58e0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x480f72; continue;
            }
            0x480f72 => {
            c.push(m, 0x480f77);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x480f7a; continue;
            }
            0x480f7a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x480f96; continue; }
            pc = 0x480f82; continue;
            }
            0x480f82 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x480f86; continue;
            }
            0x480f86 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x480f86; continue; }
            pc = 0x480f96; continue;
            }
            0x480f96 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x2eu32); }
            { let a = c.r[4].wrapping_add(0x65); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x480fa5 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x480fbd; continue; }
            pc = 0x480fae; continue;
            }
            0x480fae => {
            c.set(0,32,0x1u32);
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0xe); wm(m,a,16,c.get(0,16)); }
            pc = 0x480fe1; continue;
            }
            0x480fbd => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672d92),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x480fe1; continue; }
            pc = 0x480fd7; continue;
            }
            0x480fd7 => {
            { let a = c.r[7].wrapping_add(0xe); wm(m,a,16,0x1u32); }
            { let a = rm(m,c.r[7].wrapping_add(0xc),16); let r = c.inc(a,16); { let a = c.r[7].wrapping_add(0xc); wm(m,a,16,r); } }
            pc = 0x480fe1; continue;
            }
            0x480fe1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0d00),32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,rm(m,0x6b597cu32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x78));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x481022);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x48102a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x48103b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(1,32,c.get(3,32));
            if c.cond(12) { pc = 0x4810bc; continue; }
            pc = 0x481046; continue;
            }
            0x481046 => {
            c.push(m,c.get(5,32));
            c.push(m, 0x48104c);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x481054);
            { let args = []; c.r[0] = host.call(m, 0x4f6420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6b5a84u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x481066);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481085; continue; }
            pc = 0x481071; continue;
            }
            0x481071 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x481075; continue;
            }
            0x481075 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481075; continue; }
            pc = 0x481085; continue;
            }
            0x481085 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6b5a88u32,32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4810b4);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x4810bc => {
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            c.push(m,c.get(5,32));
            c.push(m, 0x4810c4);
            { let args = []; c.r[0] = host.call(m, 0x4f6450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4810cc);
            { let args = []; c.r[0] = host.call(m, 0x4f6420, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b5a8cu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4810dd);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4810fc; continue; }
            pc = 0x4810e8; continue;
            }
            0x4810e8 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x4810ec; continue;
            }
            0x4810ec => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4810ec; continue; }
            pc = 0x4810fc; continue;
            }
            0x4810fc => {
            c.set(1,32,rm(m,0x6b5a90u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x481114);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x48111c => {
            c.set(0,32,c.r[4].wrapping_add(0x32));
            c.set(1,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(2,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x481139);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5311d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,0x6b5b10u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x481157);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x48115f => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x3f59999au32); }
            pc = 0x481167; continue;
            }
            0x481167 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[2].wrapping_add(0x5); wm(m,a,8,0x31u32); }
            pc = 0x48116f; continue;
            }
            0x48116f => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,0xf3cu32,32); }
            if c.cond(2) { pc = 0x4811a0; continue; }
            pc = 0x481179; continue;
            }
            0x481179 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x48117f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x64));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x48118f);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x481199);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x4811a0; continue;
            }
            0x4811a0 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x4811c0; continue; }
            pc = 0x4811a9; continue;
            }
            0x4811a9 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2]),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x4811a9; continue; }
            pc = 0x4811c0; continue;
            }
            0x4811c0 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x4811ca => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x4811db);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4811ed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(0,32,rm(m,0x6b5bacu32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x481205);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x48120d => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x48121e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b5e80u32,32));
            c.set(3,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48123c; continue; }
            pc = 0x48122c; continue;
            }
            0x48122c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48122c; continue; }
            pc = 0x48123c; continue;
            }
            0x48123c => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1a),8));
            c.set(1,32,rm(m,0x6b5e88u32,32));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x48124f; continue; }
            pc = 0x481249; continue;
            }
            0x481249 => {
            c.set(1,32,rm(m,0x6b5e84u32,32));
            pc = 0x48124f; continue;
            }
            0x48124f => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481265; continue; }
            pc = 0x481255; continue;
            }
            0x481255 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481255; continue; }
            pc = 0x481265; continue;
            }
            0x481265 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1c),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x481275);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48128b; continue; }
            pc = 0x48127b; continue;
            }
            0x48127b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48127b; continue; }
            pc = 0x48128b; continue;
            }
            0x48128b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x1d),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x4812a4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4812ba; continue; }
            pc = 0x4812aa; continue;
            }
            0x4812aa => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4812aa; continue; }
            pc = 0x4812ba; continue;
            }
            0x4812ba => {
            c.set(1,32,rm(m,0x6b5e8cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4812d6; continue; }
            pc = 0x4812c6; continue;
            }
            0x4812c6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4812c6; continue; }
            pc = 0x4812d6; continue;
            }
            0x4812d6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4812f8; continue; }
            pc = 0x4812e8; continue;
            }
            0x4812e8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4812e8; continue; }
            pc = 0x4812f8; continue;
            }
            0x4812f8 => {
            c.set(1,32,rm(m,0x6b5e90u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481314; continue; }
            pc = 0x481304; continue;
            }
            0x481304 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481304; continue; }
            pc = 0x481314; continue;
            }
            0x481314 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x481323);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x481339; continue; }
            pc = 0x481329; continue;
            }
            0x481329 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x481329; continue; }
            pc = 0x481339; continue;
            }
            0x481339 => {
            c.set(1,32,rm(m,0x6b5e94u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481355; continue; }
            pc = 0x481345; continue;
            }
            0x481345 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481345; continue; }
            pc = 0x481355; continue;
            }
            0x481355 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9cf4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x48136b);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x48138a; continue; }
            pc = 0x48137a; continue;
            }
            0x48137a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48137a; continue; }
            pc = 0x48138a; continue;
            }
            0x48138a => {
            c.set(1,32,rm(m,0x6b5e98u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4813a6; continue; }
            pc = 0x481396; continue;
            }
            0x481396 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481396; continue; }
            pc = 0x4813a6; continue;
            }
            0x4813a6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x4813b6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4813cc; continue; }
            pc = 0x4813bc; continue;
            }
            0x4813bc => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4813bc; continue; }
            pc = 0x4813cc; continue;
            }
            0x4813cc => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x4813e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4813fb; continue; }
            pc = 0x4813eb; continue;
            }
            0x4813eb => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4813eb; continue; }
            pc = 0x4813fb; continue;
            }
            0x4813fb => {
            c.set(1,32,rm(m,0x6b5e9cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481417; continue; }
            pc = 0x481407; continue;
            }
            0x481407 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481407; continue; }
            pc = 0x481417; continue;
            }
            0x481417 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9cf8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x48142d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x48144c; continue; }
            pc = 0x48143c; continue;
            }
            0x48143c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48143c; continue; }
            pc = 0x48144c; continue;
            }
            0x48144c => {
            c.set(1,32,rm(m,0x6b5ea0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481468; continue; }
            pc = 0x481458; continue;
            }
            0x481458 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481458; continue; }
            pc = 0x481468; continue;
            }
            0x481468 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x481472 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x481482);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b5e80u32,32));
            c.set(3,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4814a0; continue; }
            pc = 0x481490; continue;
            }
            0x481490 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481490; continue; }
            pc = 0x4814a0; continue;
            }
            0x4814a0 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1a),8));
            c.set(1,32,rm(m,0x6b5e88u32,32));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4814b3; continue; }
            pc = 0x4814ad; continue;
            }
            0x4814ad => {
            c.set(1,32,rm(m,0x6b5e84u32,32));
            pc = 0x4814b3; continue;
            }
            0x4814b3 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4814c9; continue; }
            pc = 0x4814b9; continue;
            }
            0x4814b9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4814b9; continue; }
            pc = 0x4814c9; continue;
            }
            0x4814c9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1c),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x4814d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4814ef; continue; }
            pc = 0x4814df; continue;
            }
            0x4814df => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4814df; continue; }
            pc = 0x4814ef; continue;
            }
            0x4814ef => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x1d),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x481508);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48151e; continue; }
            pc = 0x48150e; continue;
            }
            0x48150e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48150e; continue; }
            pc = 0x48151e; continue;
            }
            0x48151e => {
            c.set(1,32,rm(m,0x6b5eb0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48153a; continue; }
            pc = 0x48152a; continue;
            }
            0x48152a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48152a; continue; }
            pc = 0x48153a; continue;
            }
            0x48153a => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x481549);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48155f; continue; }
            pc = 0x48154f; continue;
            }
            0x48154f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48154f; continue; }
            pc = 0x48155f; continue;
            }
            0x48155f => {
            c.set(1,32,rm(m,0x6b5eb4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48157b; continue; }
            pc = 0x48156b; continue;
            }
            0x48156b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48156b; continue; }
            pc = 0x48157b; continue;
            }
            0x48157b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9cfcu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x481599);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4815b8; continue; }
            pc = 0x4815a8; continue;
            }
            0x4815a8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4815a8; continue; }
            pc = 0x4815b8; continue;
            }
            0x4815b8 => {
            c.set(1,32,rm(m,0x6b5eb8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4815d4; continue; }
            pc = 0x4815c4; continue;
            }
            0x4815c4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4815c4; continue; }
            pc = 0x4815d4; continue;
            }
            0x4815d4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9d00u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4815ea);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x481609; continue; }
            pc = 0x4815f9; continue;
            }
            0x4815f9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4815f9; continue; }
            pc = 0x481609; continue;
            }
            0x481609 => {
            c.set(1,32,rm(m,0x6b5ebcu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481625; continue; }
            pc = 0x481615; continue;
            }
            0x481615 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481615; continue; }
            pc = 0x481625; continue;
            }
            0x481625 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x48162f => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x48163f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b5e80u32,32));
            c.set(3,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48165d; continue; }
            pc = 0x48164d; continue;
            }
            0x48164d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48164d; continue; }
            pc = 0x48165d; continue;
            }
            0x48165d => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1a),8));
            c.set(1,32,rm(m,0x6b5e88u32,32));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x481670; continue; }
            pc = 0x48166a; continue;
            }
            0x48166a => {
            c.set(1,32,rm(m,0x6b5e84u32,32));
            pc = 0x481670; continue;
            }
            0x481670 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481686; continue; }
            pc = 0x481676; continue;
            }
            0x481676 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481676; continue; }
            pc = 0x481686; continue;
            }
            0x481686 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1c),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x481696);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4816ac; continue; }
            pc = 0x48169c; continue;
            }
            0x48169c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48169c; continue; }
            pc = 0x4816ac; continue;
            }
            0x4816ac => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x1d),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x4816c5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4816db; continue; }
            pc = 0x4816cb; continue;
            }
            0x4816cb => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4816cb; continue; }
            pc = 0x4816db; continue;
            }
            0x4816db => {
            c.set(1,32,rm(m,0x6b5ea4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4816f7; continue; }
            pc = 0x4816e7; continue;
            }
            0x4816e7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4816e7; continue; }
            pc = 0x4816f7; continue;
            }
            0x4816f7 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x481706);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48171c; continue; }
            pc = 0x48170c; continue;
            }
            0x48170c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48170c; continue; }
            pc = 0x48171c; continue;
            }
            0x48171c => {
            c.set(1,32,rm(m,0x6b5ea8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481738; continue; }
            pc = 0x481728; continue;
            }
            0x481728 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481728; continue; }
            pc = 0x481738; continue;
            }
            0x481738 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48175a; continue; }
            pc = 0x48174a; continue;
            }
            0x48174a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48174a; continue; }
            pc = 0x48175a; continue;
            }
            0x48175a => {
            c.set(1,32,rm(m,0x6b5eacu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481776; continue; }
            pc = 0x481766; continue;
            }
            0x481766 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481766; continue; }
            pc = 0x481776; continue;
            }
            0x481776 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x481780 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(0,8,u32::from(c.cond(5)));
            c.set(5,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48181c; continue; }
            pc = 0x48179c; continue;
            }
            0x48179c => {
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4817a7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4817b8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4817c9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4817da);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(0,32,c.r[2].wrapping_add(c.r[5].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x69ee48),32));
            c.set(2,32,rm(m,0x6c14c4u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6c0ef8),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x80));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x481814);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x48181c => {
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x4818a4; continue; }
            pc = 0x481824; continue;
            }
            0x481824 => {
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48182f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x481840);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x481851);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x481862);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(0,32,c.r[2].wrapping_add(c.r[5].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x69ee48),32));
            c.set(2,32,rm(m,0x6aa184u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6c0ef8),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x80));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x48189c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x4818a4 => {
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x48192c; continue; }
            pc = 0x4818ac; continue;
            }
            0x4818ac => {
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4818b7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4818c8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4818d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4818ea);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(0,32,c.r[2].wrapping_add(c.r[5].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x69ee48),32));
            c.set(2,32,rm(m,0x6c14c8u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6c0ef8),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x80));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x481924);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x48192c => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48193d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x48194e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x48195f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x481970);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(0,32,c.r[2].wrapping_add(c.r[5].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x69ee48),32));
            c.set(2,32,rm(m,0x6c14ccu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6c0ef8),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x84));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4819aa);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x4819b2 => {
            c.set(1,32,rm(m,0x6b6064u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4819ce; continue; }
            pc = 0x4819be; continue;
            }
            0x4819be => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4819be; continue; }
            pc = 0x4819ce; continue;
            }
            0x4819ce => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x8000u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(1,16); c.sub(a,0x8000u32,16); }
            c.set(1,32,0x6ddaa0u32);
            if c.cond(5) { pc = 0x481a0a; continue; }
            pc = 0x4819ec; continue;
            }
            0x4819ec => {
            c.push(m, 0x4819f1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x481a02);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b606cu32,32));
            pc = 0x481a26; continue;
            }
            0x481a0a => {
            c.push(m, 0x481a0f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x481a20);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b6068u32,32));
            pc = 0x481a26; continue;
            }
            0x481a26 => {
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x481a31);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481a50; continue; }
            pc = 0x481a3c; continue;
            }
            0x481a3c => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x481a40; continue;
            }
            0x481a40 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481a40; continue; }
            pc = 0x481a50; continue;
            }
            0x481a50 => {
            c.set(1,32,rm(m,0x6b6070u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481a6c; continue; }
            pc = 0x481a5c; continue;
            }
            0x481a5c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481a5c; continue; }
            pc = 0x481a6c; continue;
            }
            0x481a6c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x481a7c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x481a92; continue; }
            pc = 0x481a82; continue;
            }
            0x481a82 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x481a82; continue; }
            pc = 0x481a92; continue;
            }
            0x481a92 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x481ab0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x481ac6; continue; }
            pc = 0x481ab6; continue;
            }
            0x481ab6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x481ab6; continue; }
            pc = 0x481ac6; continue;
            }
            0x481ac6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x68));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6b6074u32,32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x481af3);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(2,8)); }
            pc = 0x48651f; continue;
            }
            0x481b02 => {
            c.set(1,32,rm(m,0x6b612cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481b1e; continue; }
            pc = 0x481b0e; continue;
            }
            0x481b0e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481b0e; continue; }
            pc = 0x481b1e; continue;
            }
            0x481b1e => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x481b2e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.set(1,8,rm(m,c.r[2],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x481b4a; continue; }
            pc = 0x481b3a; continue;
            }
            0x481b3a => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[2],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x481b3a; continue; }
            pc = 0x481b4a; continue;
            }
            0x481b4a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481b70; continue; }
            pc = 0x481b60; continue;
            }
            0x481b60 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481b60; continue; }
            pc = 0x481b70; continue;
            }
            0x481b70 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x3fu32); }
            c.set(1,32,rm(m,0x6b6130u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481b96; continue; }
            pc = 0x481b86; continue;
            }
            0x481b86 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481b86; continue; }
            pc = 0x481b96; continue;
            }
            0x481b96 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0e80),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481bb9; continue; }
            pc = 0x481ba9; continue;
            }
            0x481ba9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481ba9; continue; }
            pc = 0x481bb9; continue;
            }
            0x481bb9 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(1,32,rm(m,0x6b6138u32,32));
            { let a = c.get(0,8); c.sub(a,0x7fu32,8); }
            if c.cond(7) { pc = 0x481bcc; continue; }
            pc = 0x481bc6; continue;
            }
            0x481bc6 => {
            c.set(1,32,rm(m,0x6b6134u32,32));
            pc = 0x481bcc; continue;
            }
            0x481bcc => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481be2; continue; }
            pc = 0x481bd2; continue;
            }
            0x481bd2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481bd2; continue; }
            pc = 0x481be2; continue;
            }
            0x481be2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x481bf2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x481c08; continue; }
            pc = 0x481bf8; continue;
            }
            0x481bf8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x481bf8; continue; }
            pc = 0x481c08; continue;
            }
            0x481c08 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x481c21);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x481c37; continue; }
            pc = 0x481c27; continue;
            }
            0x481c27 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x481c27; continue; }
            pc = 0x481c37; continue;
            }
            0x481c37 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.set(1,32,rm(m,0x6aa188u32,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(8) { pc = 0x481c4a; continue; }
            pc = 0x481c44; continue;
            }
            0x481c44 => {
            c.set(1,32,rm(m,0x6b613cu32,32));
            pc = 0x481c4a; continue;
            }
            0x481c4a => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481c60; continue; }
            pc = 0x481c50; continue;
            }
            0x481c50 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481c50; continue; }
            pc = 0x481c60; continue;
            }
            0x481c60 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(0,16); let n = 0x2u32; let r = c.shr(a,n,16); c.set(0,16,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x481c73);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0x7fu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x481c85);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x481c96);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            pc = 0x48487f; continue;
            }
            0x481ca2 => {
            c.set(1,32,rm(m,0x6b61e8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481cbe; continue; }
            pc = 0x481cae; continue;
            }
            0x481cae => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481cae; continue; }
            pc = 0x481cbe; continue;
            }
            0x481cbe => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x481ccd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x481ce3; continue; }
            pc = 0x481cd3; continue;
            }
            0x481cd3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x481cd3; continue; }
            pc = 0x481ce3; continue;
            }
            0x481ce3 => {
            c.set(1,32,rm(m,0x6b61ecu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481cff; continue; }
            pc = 0x481cef; continue;
            }
            0x481cef => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481cef; continue; }
            pc = 0x481cff; continue;
            }
            0x481cff => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481d21; continue; }
            pc = 0x481d11; continue;
            }
            0x481d11 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481d11; continue; }
            pc = 0x481d21; continue;
            }
            0x481d21 => {
            c.set(1,32,rm(m,0x6b61f0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481d3d; continue; }
            pc = 0x481d2d; continue;
            }
            0x481d2d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481d2d; continue; }
            pc = 0x481d3d; continue;
            }
            0x481d3d => {
            c.set(1,32,rm(m,0x6b61f4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x481d55);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x481d5d => {
            c.set(1,32,rm(m,0x6b62a4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481d79; continue; }
            pc = 0x481d69; continue;
            }
            0x481d69 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481d69; continue; }
            pc = 0x481d79; continue;
            }
            0x481d79 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(0,32); let r = c.and(a,0x7u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481da5; continue; }
            pc = 0x481d95; continue;
            }
            0x481d95 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481d95; continue; }
            pc = 0x481da5; continue;
            }
            0x481da5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x481dbe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x481dd4; continue; }
            pc = 0x481dc4; continue;
            }
            0x481dc4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x481dc4; continue; }
            pc = 0x481dd4; continue;
            }
            0x481dd4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x481df2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x481e08; continue; }
            pc = 0x481df8; continue;
            }
            0x481df8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x481df8; continue; }
            pc = 0x481e08; continue;
            }
            0x481e08 => {
            c.set(1,32,rm(m,0x6b62a8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481e24; continue; }
            pc = 0x481e14; continue;
            }
            0x481e14 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481e14; continue; }
            pc = 0x481e24; continue;
            }
            0x481e24 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481e46; continue; }
            pc = 0x481e36; continue;
            }
            0x481e36 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481e36; continue; }
            pc = 0x481e46; continue;
            }
            0x481e46 => {
            c.set(1,32,rm(m,0x6b62acu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481e62; continue; }
            pc = 0x481e52; continue;
            }
            0x481e52 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481e52; continue; }
            pc = 0x481e62; continue;
            }
            0x481e62 => {
            c.set(1,32,rm(m,0x6b62b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x481e7a);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x481e82 => {
            c.set(1,32,rm(m,0x6b63b0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481e9e; continue; }
            pc = 0x481e8e; continue;
            }
            0x481e8e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481e8e; continue; }
            pc = 0x481e9e; continue;
            }
            0x481e9e => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0da0),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x481ed0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x481ee1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b63b4u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x78));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x481ef2);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x80),8));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x481f14; continue; }
            pc = 0x481f04; continue;
            }
            0x481f04 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481f04; continue; }
            pc = 0x481f14; continue;
            }
            0x481f14 => {
            c.set(2,32,rm(m,0x6b63b8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x481f33);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x481f3b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[4].wrapping_add(0x32));
            c.set(0,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(2,32));
            { let a = c.r[1].wrapping_add(0x5); wm(m,a,8,0x31u32); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x481f60);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5311d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x32),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d9c),32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x481f95);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(1,32,rm(m,0x6b6444u32,32));
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0e38),32));
            c.set(2,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x481fb5);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x481fbd => {
            c.set(1,32,rm(m,0x6b65e0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x481fd9; continue; }
            pc = 0x481fc9; continue;
            }
            0x481fc9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x481fc9; continue; }
            pc = 0x481fd9; continue;
            }
            0x481fd9 => {
            c.set(0,16,0x80u32);
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x8),16),16); }
            { let a = c.get(5,32); let r = c.sbb(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.push(m, 0x481ff7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x482009);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6b65e0),32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x48201c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x48203b; continue; }
            pc = 0x48202b; continue;
            }
            0x48202b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48202b; continue; }
            pc = 0x48203b; continue;
            }
            0x48203b => {
            c.set(1,32,rm(m,0x6b65ecu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482057; continue; }
            pc = 0x482047; continue;
            }
            0x482047 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482047; continue; }
            pc = 0x482057; continue;
            }
            0x482057 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6b65f0u32,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x482087);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x486518; continue;
            }
            0x48208f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[1].wrapping_add(0x5); wm(m,a,8,0x33u32); }
            c.set(1,32,rm(m,0x6b674cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4820b3; continue; }
            pc = 0x4820a3; continue;
            }
            0x4820a3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4820a3; continue; }
            pc = 0x4820b3; continue;
            }
            0x4820b3 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4820c3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c3048),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6b9d04u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4820ee);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x48210d; continue; }
            pc = 0x4820fd; continue;
            }
            0x4820fd => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4820fd; continue; }
            pc = 0x48210d; continue;
            }
            0x48210d => {
            c.set(1,32,rm(m,0x6b6750u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482129; continue; }
            pc = 0x482119; continue;
            }
            0x482119 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482119; continue; }
            pc = 0x482129; continue;
            }
            0x482129 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48214b; continue; }
            pc = 0x48213b; continue;
            }
            0x48213b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48213b; continue; }
            pc = 0x48214b; continue;
            }
            0x48214b => {
            c.set(1,32,rm(m,0x6b6754u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482167; continue; }
            pc = 0x482157; continue;
            }
            0x482157 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482157; continue; }
            pc = 0x482167; continue;
            }
            0x482167 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9d10u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x48217a);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x482199; continue; }
            pc = 0x482189; continue;
            }
            0x482189 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482189; continue; }
            pc = 0x482199; continue;
            }
            0x482199 => {
            c.set(1,32,rm(m,0x6b6758u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4821b5; continue; }
            pc = 0x4821a5; continue;
            }
            0x4821a5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4821a5; continue; }
            pc = 0x4821b5; continue;
            }
            0x4821b5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x486513; continue; }
            pc = 0x4821cb; continue;
            }
            0x4821cb => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4821cb; continue; }
            pc = 0x4821db; continue;
            }
            0x4821db => {
            pc = 0x486513; continue;
            }
            0x4821e0 => {
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x482218; continue; }
            pc = 0x4821e7; continue;
            }
            0x4821e7 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4821f8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x48220a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b6f48u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x482248; continue;
            }
            0x482218 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(2,32));
            c.push(m, 0x482229);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x48223b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b6f4cu32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x482248; continue;
            }
            0x482248 => {
            c.push(m, 0x48224d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x48226c; continue; }
            pc = 0x48225c; continue;
            }
            0x48225c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48225c; continue; }
            pc = 0x48226c; continue;
            }
            0x48226c => {
            c.set(1,32,rm(m,0x6b6f50u32,32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48651f; continue; }
            pc = 0x482281; continue;
            }
            0x482281 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482281; continue; }
            pc = 0x482291; continue;
            }
            0x482291 => {
            pc = 0x48651f; continue;
            }
            0x482296 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[0],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x4822dc; continue; }
            pc = 0x4822a3; continue;
            }
            0x4822a3 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4822a3; continue; }
            pc = 0x4822ac; continue;
            }
            0x4822ac => {
            { let a = c.get(1,32); c.sub(a,0x1000u32,32); }
            if c.cond(14) { pc = 0x4822dc; continue; }
            pc = 0x4822b4; continue;
            }
            0x4822b4 => {
            { let a = c.get(1,32); let r = c.add(a,0xau32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x4822bd);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4822cb);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4822d5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x4822dc; continue;
            }
            0x4822dc => {
            c.set(7,32,rm(m,c.r[7].wrapping_add(0xc),32));
            pc = 0x4822df; continue;
            }
            0x4822df => {
            c.set(2,8,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4822df; continue; }
            pc = 0x4822e6; continue;
            }
            0x4822e6 => {
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4822fc; continue; }
            pc = 0x4822ec; continue;
            }
            0x4822ec => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4822ec; continue; }
            pc = 0x4822fc; continue;
            }
            0x4822fc => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(1,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x482370; continue; }
            pc = 0x48230a; continue;
            }
            0x48230a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.r[2].wrapping_add(0xb0); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xb),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x482370; continue; }
            pc = 0x48231b; continue;
            }
            0x48231b => {
            c.set(0,16,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x16),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x482337);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x48236b; continue; }
            pc = 0x482344; continue;
            }
            0x482344 => {
            { let a = rm(m,c.r[4].wrapping_add(0x16),16); c.sub(a,c.get(1,16),16); }
            if c.cond(6) { pc = 0x482370; continue; }
            pc = 0x48234b; continue;
            }
            0x48234b => {
            pc = 0x482351; continue;
            }
            0x48234d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x482351; continue;
            }
            0x482351 => {
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(7,32)); }
            pc = 0x482354; continue;
            }
            0x482354 => {
            c.set(2,8,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x482354; continue; }
            pc = 0x48235b; continue;
            }
            0x48235b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x16),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x48234d; continue; }
            pc = 0x482369; continue;
            }
            0x482369 => {
            pc = 0x482370; continue;
            }
            0x48236b => {
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,c.get(1,16)); }
            pc = 0x482370; continue;
            }
            0x482370 => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0xe),8));
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0xf),8));
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xd),8));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651b; continue;
            }
            0x48238b => {
            c.set(0,32,rm(m,0x6b74ecu32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4823a9);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4823c8; continue; }
            pc = 0x4823b8; continue;
            }
            0x4823b8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4823b8; continue; }
            pc = 0x4823c8; continue;
            }
            0x4823c8 => {
            c.set(0,16,0x7fu32);
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0xc),16),16); }
            { let a = c.get(3,32); let r = c.sbb(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(3,32); let r = c.add(a,0x2u32,32); c.set(3,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4823e6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4823f8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x6b74ec),32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x48240b);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x48242a; continue; }
            pc = 0x48241a; continue;
            }
            0x48241a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48241a; continue; }
            pc = 0x48242a; continue;
            }
            0x48242a => {
            c.set(1,32,rm(m,0x6b74f8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482446; continue; }
            pc = 0x482436; continue;
            }
            0x482436 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482436; continue; }
            pc = 0x482446; continue;
            }
            0x482446 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x482455);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48246b; continue; }
            pc = 0x48245b; continue;
            }
            0x48245b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48245b; continue; }
            pc = 0x48246b; continue;
            }
            0x48246b => {
            c.set(1,32,rm(m,0x6b74fcu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482487; continue; }
            pc = 0x482477; continue;
            }
            0x482477 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482477; continue; }
            pc = 0x482487; continue;
            }
            0x482487 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4824aa; continue; }
            pc = 0x48249a; continue;
            }
            0x48249a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48249a; continue; }
            pc = 0x4824aa; continue;
            }
            0x4824aa => {
            c.set(1,32,rm(m,0x6b7500u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4824c6; continue; }
            pc = 0x4824b6; continue;
            }
            0x4824b6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4824b6; continue; }
            pc = 0x4824c6; continue;
            }
            0x4824c6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4824e9; continue; }
            pc = 0x4824d9; continue;
            }
            0x4824d9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4824d9; continue; }
            pc = 0x4824e9; continue;
            }
            0x4824e9 => {
            c.set(1,32,rm(m,0x6b7504u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482505; continue; }
            pc = 0x4824f5; continue;
            }
            0x4824f5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4824f5; continue; }
            pc = 0x482505; continue;
            }
            0x482505 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482527; continue; }
            pc = 0x482517; continue;
            }
            0x482517 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482517; continue; }
            pc = 0x482527; continue;
            }
            0x482527 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x482531 => {
            c.set(2,32,rm(m,0x6b7600u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x482550);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x48256f; continue; }
            pc = 0x48255f; continue;
            }
            0x48255f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48255f; continue; }
            pc = 0x48256f; continue;
            }
            0x48256f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x482580);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x482592);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6b7608u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4825a4);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4825c3; continue; }
            pc = 0x4825b3; continue;
            }
            0x4825b3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4825b3; continue; }
            pc = 0x4825c3; continue;
            }
            0x4825c3 => {
            c.set(1,32,rm(m,0x6b760cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4825df; continue; }
            pc = 0x4825cf; continue;
            }
            0x4825cf => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4825cf; continue; }
            pc = 0x4825df; continue;
            }
            0x4825df => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482602; continue; }
            pc = 0x4825f2; continue;
            }
            0x4825f2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4825f2; continue; }
            pc = 0x482602; continue;
            }
            0x482602 => {
            c.set(1,32,rm(m,0x6b7610u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48261e; continue; }
            pc = 0x48260e; continue;
            }
            0x48260e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48260e; continue; }
            pc = 0x48261e; continue;
            }
            0x48261e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9d14u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x482634);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x482653; continue; }
            pc = 0x482643; continue;
            }
            0x482643 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482643; continue; }
            pc = 0x482653; continue;
            }
            0x482653 => {
            c.set(1,32,rm(m,0x6b7614u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48266f; continue; }
            pc = 0x48265f; continue;
            }
            0x48265f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48265f; continue; }
            pc = 0x48266f; continue;
            }
            0x48266f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482691; continue; }
            pc = 0x482681; continue;
            }
            0x482681 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482681; continue; }
            pc = 0x482691; continue;
            }
            0x482691 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x48269b => {
            c.set(1,32,rm(m,0x6b76f8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4826bb);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x4826c3 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xb8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4826e0; continue; }
            pc = 0x4826cd; continue;
            }
            0x4826cd => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4826d3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[3].wrapping_add(0xb8); wm(m,a,32,0x0u32); }
            pc = 0x4826e0; continue;
            }
            0x4826e0 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4826e7);
            { let args = []; c.r[0] = host.call(m, 0x51ac60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[3].wrapping_add(0xb8); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,0x6b9d1cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4826fd);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x482705 => {
            c.set(1,32,c.r[4].wrapping_add(0x32));
            c.set(2,32,c.r[4].wrapping_add(0x38));
            c.push(m,c.get(1,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2),16));
            { let a = c.get(1,32); let r = c.and(a,0xfu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_mul(2).wrapping_add(0x672d78),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7],8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x9));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x48274b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5311d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6ac490),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x48276f);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x48278e; continue; }
            pc = 0x48277e; continue;
            }
            0x48277e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48277e; continue; }
            pc = 0x48278e; continue;
            }
            0x48278e => {
            { let a = rm(m,c.r[7].wrapping_add(0x9),8); c.sub(a,0x7fu32,8); }
            if c.cond(6) { pc = 0x4827a3; continue; }
            pc = 0x482794; continue;
            }
            0x482794 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.and(a,0x7u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0608),32));
            pc = 0x4827b0; continue;
            }
            0x4827a3 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(1,32); let r = c.and(a,0x7u32,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a0628),32));
            pc = 0x4827b0; continue;
            }
            0x4827b0 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4827c6; continue; }
            pc = 0x4827b6; continue;
            }
            0x4827b6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4827b6; continue; }
            pc = 0x4827c6; continue;
            }
            0x4827c6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x4827df);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4827f5; continue; }
            pc = 0x4827e5; continue;
            }
            0x4827e5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4827e5; continue; }
            pc = 0x4827f5; continue;
            }
            0x4827f5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x48280e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x482824; continue; }
            pc = 0x482814; continue;
            }
            0x482814 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x482814; continue; }
            pc = 0x482824; continue;
            }
            0x482824 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0xau32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]); wm(m,a,8,0xau32); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x482b48; continue; }
            pc = 0x482845; continue;
            }
            0x482845 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4829d8; continue; }
            pc = 0x48284c; continue;
            }
            0x48284c => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x482c31; continue; }
            pc = 0x482853; continue;
            }
            0x482853 => {
            c.set(1,32,rm(m,0x6ac4acu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48286f; continue; }
            pc = 0x48285f; continue;
            }
            0x48285f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48285f; continue; }
            pc = 0x48286f; continue;
            }
            0x48286f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x48287f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x482890);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6ac4b0u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4828a2);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4828c1; continue; }
            pc = 0x4828b1; continue;
            }
            0x4828b1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4828b1; continue; }
            pc = 0x4828c1; continue;
            }
            0x4828c1 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x482c31; continue; }
            pc = 0x4828cc; continue;
            }
            0x4828cc => {
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x482956; continue; }
            pc = 0x4828d4; continue;
            }
            0x4828d4 => {
            c.set(1,32,rm(m,0x6ac4b4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4828f0; continue; }
            pc = 0x4828e0; continue;
            }
            0x4828e0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4828e0; continue; }
            pc = 0x4828f0; continue;
            }
            0x4828f0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x32),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6ac4bcu32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x48292e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x7c),8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x482c31; continue; }
            pc = 0x482941; continue;
            }
            0x482941 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482941; continue; }
            pc = 0x482951; continue;
            }
            0x482951 => {
            pc = 0x482c31; continue;
            }
            0x482956 => {
            c.set(1,32,rm(m,0x6ac4b8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482972; continue; }
            pc = 0x482962; continue;
            }
            0x482962 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482962; continue; }
            pc = 0x482972; continue;
            }
            0x482972 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x32),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6ac4c0u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4829b0);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x7c),8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x482c31; continue; }
            pc = 0x4829c3; continue;
            }
            0x4829c3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4829c3; continue; }
            pc = 0x4829d3; continue;
            }
            0x4829d3 => {
            pc = 0x482c31; continue;
            }
            0x4829d8 => {
            c.set(2,32,rm(m,0x6ac49cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4829f6);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x482a15; continue; }
            pc = 0x482a05; continue;
            }
            0x482a05 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482a05; continue; }
            pc = 0x482a15; continue;
            }
            0x482a15 => {
            c.set(1,32,rm(m,0x6ac4a0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482a31; continue; }
            pc = 0x482a21; continue;
            }
            0x482a21 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482a21; continue; }
            pc = 0x482a31; continue;
            }
            0x482a31 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            { let a = c.get(0,8); c.sub(a,0x1u32,8); }
            if c.cond(6) { pc = 0x482abe; continue; }
            pc = 0x482a3c; continue;
            }
            0x482a3c => {
            c.set(1,32,rm(m,0x6ac4d4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482a58; continue; }
            pc = 0x482a48; continue;
            }
            0x482a48 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482a48; continue; }
            pc = 0x482a58; continue;
            }
            0x482a58 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x32),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6ac4d8u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x482a96);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x7c),8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x482c31; continue; }
            pc = 0x482aa9; continue;
            }
            0x482aa9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482aa9; continue; }
            pc = 0x482ab9; continue;
            }
            0x482ab9 => {
            pc = 0x482c31; continue;
            }
            0x482abe => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x482c31; continue; }
            pc = 0x482ac6; continue;
            }
            0x482ac6 => {
            c.set(1,32,rm(m,0x6ac4a4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482ae2; continue; }
            pc = 0x482ad2; continue;
            }
            0x482ad2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482ad2; continue; }
            pc = 0x482ae2; continue;
            }
            0x482ae2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x32),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6ac4a8u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x482b20);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x7c),8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x482c31; continue; }
            pc = 0x482b33; continue;
            }
            0x482b33 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482b33; continue; }
            pc = 0x482b43; continue;
            }
            0x482b43 => {
            pc = 0x482c31; continue;
            }
            0x482b48 => {
            c.set(2,32,rm(m,0x6ac4dcu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x482b66);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x482b85; continue; }
            pc = 0x482b75; continue;
            }
            0x482b75 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482b75; continue; }
            pc = 0x482b85; continue;
            }
            0x482b85 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x482c2a; continue; }
            pc = 0x482b90; continue;
            }
            0x482b90 => {
            c.set(2,32,rm(m,0x6ac4c8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x482bae);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x482bcd; continue; }
            pc = 0x482bbd; continue;
            }
            0x482bbd => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482bbd; continue; }
            pc = 0x482bcd; continue;
            }
            0x482bcd => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x32),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6ac4ccu32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x482c0b);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x7c),8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x482c2a; continue; }
            pc = 0x482c1a; continue;
            }
            0x482c1a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482c1a; continue; }
            pc = 0x482c2a; continue;
            }
            0x482c2a => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(0,8)); }
            pc = 0x482c31; continue;
            }
            0x482c31 => {
            c.set(1,32,rm(m,0x6ac4d0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482c4d; continue; }
            pc = 0x482c3d; continue;
            }
            0x482c3d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482c3d; continue; }
            pc = 0x482c4d; continue;
            }
            0x482c4d => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x482c57 => {
            c.set(1,32,rm(m,0x6b77ccu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482c73; continue; }
            pc = 0x482c63; continue;
            }
            0x482c63 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482c63; continue; }
            pc = 0x482c73; continue;
            }
            0x482c73 => {
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,0x7fu32,16); }
            c.set(1,32,rm(m,0x6b77d4u32,32));
            if c.cond(7) { pc = 0x482c86; continue; }
            pc = 0x482c80; continue;
            }
            0x482c80 => {
            c.set(1,32,rm(m,0x6b77d0u32,32));
            pc = 0x482c86; continue;
            }
            0x482c86 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482c9c; continue; }
            pc = 0x482c8c; continue;
            }
            0x482c8c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482c8c; continue; }
            pc = 0x482c9c; continue;
            }
            0x482c9c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x482cad);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x482cc3; continue; }
            pc = 0x482cb3; continue;
            }
            0x482cb3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x482cb3; continue; }
            pc = 0x482cc3; continue;
            }
            0x482cc3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x482cdd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x482cf3; continue; }
            pc = 0x482ce3; continue;
            }
            0x482ce3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x482ce3; continue; }
            pc = 0x482cf3; continue;
            }
            0x482cf3 => {
            c.set(1,32,rm(m,0x6b77d8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482d0f; continue; }
            pc = 0x482cff; continue;
            }
            0x482cff => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482cff; continue; }
            pc = 0x482d0f; continue;
            }
            0x482d0f => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9d28u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x482d22);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x482d41; continue; }
            pc = 0x482d31; continue;
            }
            0x482d31 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482d31; continue; }
            pc = 0x482d41; continue;
            }
            0x482d41 => {
            c.set(1,32,rm(m,0x6b77dcu32,32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482d62; continue; }
            pc = 0x482d52; continue;
            }
            0x482d52 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482d52; continue; }
            pc = 0x482d62; continue;
            }
            0x482d62 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48651f; continue; }
            pc = 0x482d78; continue;
            }
            0x482d78 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482d78; continue; }
            pc = 0x482d88; continue;
            }
            0x482d88 => {
            pc = 0x48651f; continue;
            }
            0x482d8d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x5cu32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,0x6c3054u32);
            { let a = c.r[1].wrapping_add(c.r[6]); wm(m,a,8,0x6cu32); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(2,32); let n = 0xfu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482dcb; continue; }
            pc = 0x482dbb; continue;
            }
            0x482dbb => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482dbb; continue; }
            pc = 0x482dcb; continue;
            }
            0x482dcb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x482ddb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x482df1; continue; }
            pc = 0x482de1; continue;
            }
            0x482de1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x482de1; continue; }
            pc = 0x482df1; continue;
            }
            0x482df1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x482e0f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x482e25; continue; }
            pc = 0x482e15; continue;
            }
            0x482e15 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x482e15; continue; }
            pc = 0x482e25; continue;
            }
            0x482e25 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(7) { pc = 0x483038; continue; }
            pc = 0x482e34; continue;
            }
            0x482e34 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4868d4)); continue;
            }
            0x482e3b => {
            c.set(1,32,rm(m,0x6b7cbcu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482e57; continue; }
            pc = 0x482e47; continue;
            }
            0x482e47 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482e47; continue; }
            pc = 0x482e57; continue;
            }
            0x482e57 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dd8c0),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482e7a; continue; }
            pc = 0x482e6a; continue;
            }
            0x482e6a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482e6a; continue; }
            pc = 0x482e7a; continue;
            }
            0x482e7a => {
            c.set(1,32,rm(m,0x6b7cdcu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482e96; continue; }
            pc = 0x482e86; continue;
            }
            0x482e86 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482e86; continue; }
            pc = 0x482e96; continue;
            }
            0x482e96 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482eb9; continue; }
            pc = 0x482ea9; continue;
            }
            0x482ea9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482ea9; continue; }
            pc = 0x482eb9; continue;
            }
            0x482eb9 => {
            c.set(1,32,rm(m,0x6b7ce0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483054; continue; }
            pc = 0x482ec9; continue;
            }
            0x482ec9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482ec9; continue; }
            pc = 0x482ed9; continue;
            }
            0x482ed9 => {
            pc = 0x483054; continue;
            }
            0x482ede => {
            c.set(1,32,rm(m,0x6b7cc0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482efa; continue; }
            pc = 0x482eea; continue;
            }
            0x482eea => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482eea; continue; }
            pc = 0x482efa; continue;
            }
            0x482efa => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482f1d; continue; }
            pc = 0x482f0d; continue;
            }
            0x482f0d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482f0d; continue; }
            pc = 0x482f1d; continue;
            }
            0x482f1d => {
            c.set(1,32,rm(m,0x6b7ce4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483054; continue; }
            pc = 0x482f2d; continue;
            }
            0x482f2d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482f2d; continue; }
            pc = 0x482f3d; continue;
            }
            0x482f3d => {
            pc = 0x483054; continue;
            }
            0x482f42 => {
            c.set(1,32,rm(m,0x6b7cc4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483054; continue; }
            pc = 0x482f52; continue;
            }
            0x482f52 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482f52; continue; }
            pc = 0x482f62; continue;
            }
            0x482f62 => {
            pc = 0x483054; continue;
            }
            0x482f67 => {
            c.set(1,32,rm(m,0x6b7cc8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x482f83; continue; }
            pc = 0x482f73; continue;
            }
            0x482f73 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482f73; continue; }
            pc = 0x482f83; continue;
            }
            0x482f83 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x482f93);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x482fa9; continue; }
            pc = 0x482f99; continue;
            }
            0x482f99 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x482f99; continue; }
            pc = 0x482fa9; continue;
            }
            0x482fa9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x482fc7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x482fdd; continue; }
            pc = 0x482fcd; continue;
            }
            0x482fcd => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x482fcd; continue; }
            pc = 0x482fdd; continue;
            }
            0x482fdd => {
            c.set(1,32,rm(m,0x6b7ce8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483054; continue; }
            pc = 0x482fe9; continue;
            }
            0x482fe9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x482fe9; continue; }
            pc = 0x482ff9; continue;
            }
            0x482ff9 => {
            pc = 0x483054; continue;
            }
            0x482ffb => {
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6b7cbc),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483054; continue; }
            pc = 0x483008; continue;
            }
            0x483008 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483008; continue; }
            pc = 0x483018; continue;
            }
            0x483018 => {
            pc = 0x483054; continue;
            }
            0x48301a => {
            c.set(1,32,rm(m,0x6ae648u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483054; continue; }
            pc = 0x483026; continue;
            }
            0x483026 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483026; continue; }
            pc = 0x483036; continue;
            }
            0x483036 => {
            pc = 0x483054; continue;
            }
            0x483038 => {
            c.set(1,32,rm(m,0x6b7cc4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483054; continue; }
            pc = 0x483044; continue;
            }
            0x483044 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483044; continue; }
            pc = 0x483054; continue;
            }
            0x483054 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            if c.cond(12) { pc = 0x48306a; continue; }
            pc = 0x48305f; continue;
            }
            0x48305f => {
            c.set(1,32,rm(m,0x6b7cf0u32,32));
            { let a = c.get(0,32); c.sub(a,0x7u32,32); }
            if c.cond(14) { pc = 0x483070; continue; }
            pc = 0x48306a; continue;
            }
            0x48306a => {
            c.set(1,32,rm(m,0x6b7cecu32,32));
            pc = 0x483070; continue;
            }
            0x483070 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x486513; continue; }
            pc = 0x48307a; continue;
            }
            0x48307a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48307a; continue; }
            pc = 0x48308a; continue;
            }
            0x48308a => {
            pc = 0x486513; continue;
            }
            0x48308f => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48309f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4830b5; continue; }
            pc = 0x4830a5; continue;
            }
            0x4830a5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4830a5; continue; }
            pc = 0x4830b5; continue;
            }
            0x4830b5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x4830ce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4830e4; continue; }
            pc = 0x4830d4; continue;
            }
            0x4830d4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4830d4; continue; }
            pc = 0x4830e4; continue;
            }
            0x4830e4 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(0,8); c.sub(a,0x8u32,8); }
            if c.cond(3) { pc = 0x4830f9; continue; }
            pc = 0x4830eb; continue;
            }
            0x4830eb => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6b8080),32));
            pc = 0x483107; continue;
            }
            0x4830f9 => {
            c.set(1,32,rm(m,0x6ae634u32,32));
            if c.cond(4) { pc = 0x483107; continue; }
            pc = 0x483101; continue;
            }
            0x483101 => {
            c.set(1,32,rm(m,0x6ae638u32,32));
            pc = 0x483107; continue;
            }
            0x483107 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48311d; continue; }
            pc = 0x48310d; continue;
            }
            0x48310d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48310d; continue; }
            pc = 0x48311d; continue;
            }
            0x48311d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(0,32); c.sub(a,0x7u32,32); }
            if c.cond(7) { pc = 0x48328d; continue; }
            pc = 0x48312b; continue;
            }
            0x48312b => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4868f8)); continue;
            }
            0x483132 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dd8c0),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483154; continue; }
            pc = 0x483144; continue;
            }
            0x483144 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483144; continue; }
            pc = 0x483154; continue;
            }
            0x483154 => {
            c.set(1,32,rm(m,0x6b80a0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483170; continue; }
            pc = 0x483160; continue;
            }
            0x483160 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483160; continue; }
            pc = 0x483170; continue;
            }
            0x483170 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483192; continue; }
            pc = 0x483182; continue;
            }
            0x483182 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483182; continue; }
            pc = 0x483192; continue;
            }
            0x483192 => {
            c.set(1,32,rm(m,0x6b80a4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48328d; continue; }
            pc = 0x4831a2; continue;
            }
            0x4831a2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4831a2; continue; }
            pc = 0x4831b2; continue;
            }
            0x4831b2 => {
            pc = 0x48328d; continue;
            }
            0x4831b7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4831d9; continue; }
            pc = 0x4831c9; continue;
            }
            0x4831c9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4831c9; continue; }
            pc = 0x4831d9; continue;
            }
            0x4831d9 => {
            c.set(1,32,rm(m,0x6b80a8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48328d; continue; }
            pc = 0x4831e9; continue;
            }
            0x4831e9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4831e9; continue; }
            pc = 0x4831f9; continue;
            }
            0x4831f9 => {
            pc = 0x48328d; continue;
            }
            0x4831fe => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x48320e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483224; continue; }
            pc = 0x483214; continue;
            }
            0x483214 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483214; continue; }
            pc = 0x483224; continue;
            }
            0x483224 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x48323d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483253; continue; }
            pc = 0x483243; continue;
            }
            0x483243 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483243; continue; }
            pc = 0x483253; continue;
            }
            0x483253 => {
            c.set(1,32,rm(m,0x6b80acu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48328d; continue; }
            pc = 0x48325f; continue;
            }
            0x48325f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48325f; continue; }
            pc = 0x48326f; continue;
            }
            0x48326f => {
            pc = 0x48328d; continue;
            }
            0x483271 => {
            c.set(1,32,rm(m,0x6b80b0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48328d; continue; }
            pc = 0x48327d; continue;
            }
            0x48327d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48327d; continue; }
            pc = 0x48328d; continue;
            }
            0x48328d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x483349; continue; }
            pc = 0x48329b; continue;
            }
            0x48329b => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x4832ec; continue; }
            pc = 0x48329e; continue;
            }
            0x48329e => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x486518; continue; }
            pc = 0x4832a5; continue;
            }
            0x4832a5 => {
            c.set(1,32,rm(m,0x6ae63cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4832c1; continue; }
            pc = 0x4832b1; continue;
            }
            0x4832b1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4832b1; continue; }
            pc = 0x4832c1; continue;
            }
            0x4832c1 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6ae644u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4832e4);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x486518; continue;
            }
            0x4832ec => {
            c.set(1,32,rm(m,0x6ae63cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483308; continue; }
            pc = 0x4832f8; continue;
            }
            0x4832f8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4832f8; continue; }
            pc = 0x483308; continue;
            }
            0x483308 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,0x6b9d2cu32);
            if c.cond(5) { pc = 0x483321; continue; }
            pc = 0x48331c; continue;
            }
            0x48331c => {
            c.set(1,32,0x6b9d30u32);
            pc = 0x483321; continue;
            }
            0x483321 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(2,32,rm(m,0x6ae640u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x483341);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x486518; continue;
            }
            0x483349 => {
            c.set(1,32,rm(m,0x6b80b4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483365; continue; }
            pc = 0x483355; continue;
            }
            0x483355 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483355; continue; }
            pc = 0x483365; continue;
            }
            0x483365 => {
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(3,16); let r = c.xor(a,c.get(3,16),16); c.set(3,16,r); }
            { let a = c.get(0,8); let r = c.add(a,0xau32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(1,8); let r = c.and(a,0xfu32,8); c.set(1,8,r); }
            c.set(3,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,c.get(1,8)); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x2),16));
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(3,16); c.sub(a,rm(m,c.r[2].wrapping_mul(2).wrapping_add(0x672d94),16),16); }
            c.set(2,32,c.r[2].wrapping_mul(2).wrapping_add(0x672d94));
            if c.cond(6) { pc = 0x4833c3; continue; }
            pc = 0x4833a0; continue;
            }
            0x4833a0 => {
            c.set(3,8,rm(m,c.r[2],8));
            { let a = c.get(0,8); let r = c.sub(a,c.get(3,8),8); c.set(0,8,r); }
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0xcu32,8); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,c.get(1,8)); }
            if c.cond(2) { pc = 0x4833c3; continue; }
            pc = 0x4833b3; continue;
            }
            0x4833b3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x32),32));
            { let a = c.get(1,8); let r = c.add(a,0xf4u32,8); c.set(1,8,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,32,c.get(0,32)); }
            pc = 0x4833c3; continue;
            }
            0x4833c3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6b80b8u32,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4833e7);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x486518; continue;
            }
            0x4833ef => {
            { let a = rm(m,c.r[7].wrapping_add(0x9),8); c.sub(a,0x7fu32,8); }
            if c.cond(6) { pc = 0x483404; continue; }
            pc = 0x4833f5; continue;
            }
            0x4833f5 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(1,32); let r = c.and(a,0x7u32,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a0608),32));
            pc = 0x483411; continue;
            }
            0x483404 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(2,32); let r = c.and(a,0x7u32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a0628),32));
            pc = 0x483411; continue;
            }
            0x483411 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483427; continue; }
            pc = 0x483417; continue;
            }
            0x483417 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483417; continue; }
            pc = 0x483427; continue;
            }
            0x483427 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x483440);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483456; continue; }
            pc = 0x483446; continue;
            }
            0x483446 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483446; continue; }
            pc = 0x483456; continue;
            }
            0x483456 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x48346f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483485; continue; }
            pc = 0x483475; continue;
            }
            0x483475 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483475; continue; }
            pc = 0x483485; continue;
            }
            0x483485 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0xau32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]); wm(m,a,8,0xau32); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x48353f; continue; }
            pc = 0x4834a6; continue;
            }
            0x4834a6 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x483537; continue; }
            pc = 0x4834ad; continue;
            }
            0x4834ad => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x486518; continue; }
            pc = 0x4834b4; continue;
            }
            0x4834b4 => {
            { let a = rm(m,c.r[7].wrapping_add(0xd),8); c.sub(a,0x1u32,8); }
            if c.cond(6) { pc = 0x4834d5; continue; }
            pc = 0x4834ba; continue;
            }
            0x4834ba => {
            c.set(1,32,rm(m,0x6ac7ccu32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x4834ee; continue;
            }
            0x4834d5 => {
            c.set(2,32,rm(m,0x6ac7d0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x4834ee; continue;
            }
            0x4834ee => {
            c.push(m, 0x4834f3);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x483512; continue; }
            pc = 0x483502; continue;
            }
            0x483502 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483502; continue; }
            pc = 0x483512; continue;
            }
            0x483512 => {
            c.set(1,32,rm(m,0x6ac7d4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x486513; continue; }
            pc = 0x483522; continue;
            }
            0x483522 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483522; continue; }
            pc = 0x483532; continue;
            }
            0x483532 => {
            pc = 0x486513; continue;
            }
            0x483537 => {
            c.set(1,32,rm(m,0x6add4cu32,32));
            pc = 0x483545; continue;
            }
            0x48353f => {
            c.set(1,32,rm(m,0x6ac7c8u32,32));
            pc = 0x483545; continue;
            }
            0x483545 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x48355e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x486518; continue;
            }
            0x483566 => {
            c.set(1,32,rm(m,0x6b8194u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483582; continue; }
            pc = 0x483572; continue;
            }
            0x483572 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483572; continue; }
            pc = 0x483582; continue;
            }
            0x483582 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x483593);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4835a9; continue; }
            pc = 0x483599; continue;
            }
            0x483599 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483599; continue; }
            pc = 0x4835a9; continue;
            }
            0x4835a9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x4835c3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4835d9; continue; }
            pc = 0x4835c9; continue;
            }
            0x4835c9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4835c9; continue; }
            pc = 0x4835d9; continue;
            }
            0x4835d9 => {
            c.set(1,32,rm(m,0x6b8198u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4835f5; continue; }
            pc = 0x4835e5; continue;
            }
            0x4835e5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4835e5; continue; }
            pc = 0x4835f5; continue;
            }
            0x4835f5 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x483604);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48361a; continue; }
            pc = 0x48360a; continue;
            }
            0x48360a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48360a; continue; }
            pc = 0x48361a; continue;
            }
            0x48361a => {
            c.set(1,32,rm(m,0x6b819cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483636; continue; }
            pc = 0x483626; continue;
            }
            0x483626 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483626; continue; }
            pc = 0x483636; continue;
            }
            0x483636 => {
            c.set(1,32,rm(m,0x6b81a0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x48364e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x483656 => {
            c.set(1,32,rm(m,0x6b8288u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483672; continue; }
            pc = 0x483662; continue;
            }
            0x483662 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483662; continue; }
            pc = 0x483672; continue;
            }
            0x483672 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x483682);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483698; continue; }
            pc = 0x483688; continue;
            }
            0x483688 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483688; continue; }
            pc = 0x483698; continue;
            }
            0x483698 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4836b6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4836cc; continue; }
            pc = 0x4836bc; continue;
            }
            0x4836bc => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4836bc; continue; }
            pc = 0x4836cc; continue;
            }
            0x4836cc => {
            c.set(1,32,rm(m,0x6b828cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4836e8; continue; }
            pc = 0x4836d8; continue;
            }
            0x4836d8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4836d8; continue; }
            pc = 0x4836e8; continue;
            }
            0x4836e8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x4836f9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48370f; continue; }
            pc = 0x4836ff; continue;
            }
            0x4836ff => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4836ff; continue; }
            pc = 0x48370f; continue;
            }
            0x48370f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x483729);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48373f; continue; }
            pc = 0x48372f; continue;
            }
            0x48372f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48372f; continue; }
            pc = 0x48373f; continue;
            }
            0x48373f => {
            c.set(1,32,rm(m,0x6b8290u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48375b; continue; }
            pc = 0x48374b; continue;
            }
            0x48374b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48374b; continue; }
            pc = 0x48375b; continue;
            }
            0x48375b => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x48376a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483780; continue; }
            pc = 0x483770; continue;
            }
            0x483770 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483770; continue; }
            pc = 0x483780; continue;
            }
            0x483780 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x48378a => {
            c.set(1,32,rm(m,0x6b8324u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4837a6; continue; }
            pc = 0x483796; continue;
            }
            0x483796 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483796; continue; }
            pc = 0x4837a6; continue;
            }
            0x4837a6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x4837b7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4837cd; continue; }
            pc = 0x4837bd; continue;
            }
            0x4837bd => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4837bd; continue; }
            pc = 0x4837cd; continue;
            }
            0x4837cd => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x4837e7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4837fd; continue; }
            pc = 0x4837ed; continue;
            }
            0x4837ed => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4837ed; continue; }
            pc = 0x4837fd; continue;
            }
            0x4837fd => {
            c.set(1,32,rm(m,0x6b8328u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483819; continue; }
            pc = 0x483809; continue;
            }
            0x483809 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483809; continue; }
            pc = 0x483819; continue;
            }
            0x483819 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x48382a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48383c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,0x6b9d34u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x48384c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x483854 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x483864);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48387a; continue; }
            pc = 0x48386a; continue;
            }
            0x48386a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48386a; continue; }
            pc = 0x48387a; continue;
            }
            0x48387a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x483898);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4838ae; continue; }
            pc = 0x48389e; continue;
            }
            0x48389e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48389e; continue; }
            pc = 0x4838ae; continue;
            }
            0x4838ae => {
            c.set(1,32,rm(m,0x6b83d0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4838ca; continue; }
            pc = 0x4838ba; continue;
            }
            0x4838ba => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4838ba; continue; }
            pc = 0x4838ca; continue;
            }
            0x4838ca => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4838d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4838ef; continue; }
            pc = 0x4838df; continue;
            }
            0x4838df => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4838df; continue; }
            pc = 0x4838ef; continue;
            }
            0x4838ef => {
            c.set(1,32,rm(m,0x6b83d4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48390b; continue; }
            pc = 0x4838fb; continue;
            }
            0x4838fb => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4838fb; continue; }
            pc = 0x48390b; continue;
            }
            0x48390b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x48391c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48392e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,0x6b9d3cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x48393e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x483946 => {
            c.set(1,32,rm(m,0x6b845cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483962; continue; }
            pc = 0x483952; continue;
            }
            0x483952 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483952; continue; }
            pc = 0x483962; continue;
            }
            0x483962 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x483973);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483989; continue; }
            pc = 0x483979; continue;
            }
            0x483979 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483979; continue; }
            pc = 0x483989; continue;
            }
            0x483989 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x4839a3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4839b9; continue; }
            pc = 0x4839a9; continue;
            }
            0x4839a9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4839a9; continue; }
            pc = 0x4839b9; continue;
            }
            0x4839b9 => {
            c.set(1,32,rm(m,0x6b8460u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4839d5; continue; }
            pc = 0x4839c5; continue;
            }
            0x4839c5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4839c5; continue; }
            pc = 0x4839d5; continue;
            }
            0x4839d5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dd8c0),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4839f7; continue; }
            pc = 0x4839e7; continue;
            }
            0x4839e7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4839e7; continue; }
            pc = 0x4839f7; continue;
            }
            0x4839f7 => {
            c.set(1,32,rm(m,0x6b8464u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483a13; continue; }
            pc = 0x483a03; continue;
            }
            0x483a03 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483a03; continue; }
            pc = 0x483a13; continue;
            }
            0x483a13 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483a35; continue; }
            pc = 0x483a25; continue;
            }
            0x483a25 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483a25; continue; }
            pc = 0x483a35; continue;
            }
            0x483a35 => {
            c.set(1,32,rm(m,0x6b8468u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483a51; continue; }
            pc = 0x483a41; continue;
            }
            0x483a41 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483a41; continue; }
            pc = 0x483a51; continue;
            }
            0x483a51 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x483a62);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x483a74);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,0x6b9d44u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x483a84);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x483a8c => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6b8748),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483aaf; continue; }
            pc = 0x483a9f; continue;
            }
            0x483a9f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483a9f; continue; }
            pc = 0x483aaf; continue;
            }
            0x483aaf => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x483abf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483ad5; continue; }
            pc = 0x483ac5; continue;
            }
            0x483ac5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483ac5; continue; }
            pc = 0x483ad5; continue;
            }
            0x483ad5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x483af3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483b09; continue; }
            pc = 0x483af9; continue;
            }
            0x483af9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483af9; continue; }
            pc = 0x483b09; continue;
            }
            0x483b09 => {
            { let a = rm(m,c.r[7].wrapping_add(0xa),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x483b58; continue; }
            pc = 0x483b0f; continue;
            }
            0x483b0f => {
            c.set(1,32,rm(m,0x6b8750u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483b2b; continue; }
            pc = 0x483b1b; continue;
            }
            0x483b1b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483b1b; continue; }
            pc = 0x483b2b; continue;
            }
            0x483b2b => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x483b3a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483b50; continue; }
            pc = 0x483b40; continue;
            }
            0x483b40 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483b40; continue; }
            pc = 0x483b50; continue;
            }
            0x483b50 => {
            c.set(1,32,rm(m,0x6b8754u32,32));
            pc = 0x483b5e; continue;
            }
            0x483b58 => {
            c.set(1,32,rm(m,0x6b8758u32,32));
            pc = 0x483b5e; continue;
            }
            0x483b5e => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483b74; continue; }
            pc = 0x483b64; continue;
            }
            0x483b64 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483b64; continue; }
            pc = 0x483b74; continue;
            }
            0x483b74 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x483b83);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x483b95);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x483ba7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b875cu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x483bb8);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x483bc0 => {
            { let a = rm(m,c.r[7].wrapping_add(0xf),8); c.and(a,0x80u32,8); }
            c.set(1,32,rm(m,0x6b8d4cu32,32));
            if c.cond(5) { pc = 0x483bd2; continue; }
            pc = 0x483bcc; continue;
            }
            0x483bcc => {
            c.set(1,32,rm(m,0x6b8d48u32,32));
            pc = 0x483bd2; continue;
            }
            0x483bd2 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483be8; continue; }
            pc = 0x483bd8; continue;
            }
            0x483bd8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483bd8; continue; }
            pc = 0x483be8; continue;
            }
            0x483be8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x483bf8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483c0e; continue; }
            pc = 0x483bfe; continue;
            }
            0x483bfe => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483bfe; continue; }
            pc = 0x483c0e; continue;
            }
            0x483c0e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x483c2c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483c42; continue; }
            pc = 0x483c32; continue;
            }
            0x483c32 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483c32; continue; }
            pc = 0x483c42; continue;
            }
            0x483c42 => {
            c.set(1,32,rm(m,0x6b8d50u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483c5e; continue; }
            pc = 0x483c4e; continue;
            }
            0x483c4e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483c4e; continue; }
            pc = 0x483c5e; continue;
            }
            0x483c5e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.push(m,c.get(0,32));
            if c.cond(4) { pc = 0x483c88; continue; }
            pc = 0x483c7a; continue;
            }
            0x483c7a => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.push(m,0x6b9d4cu32);
            pc = 0x483c94; continue;
            }
            0x483c88 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.push(m,0x6b9d54u32);
            pc = 0x483c94; continue;
            }
            0x483c94 => {
            c.push(m,c.get(2,32));
            c.push(m, 0x483c9a);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x483cb9; continue; }
            pc = 0x483ca9; continue;
            }
            0x483ca9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483ca9; continue; }
            pc = 0x483cb9; continue;
            }
            0x483cb9 => {
            c.set(1,32,rm(m,0x6b8d54u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483cd5; continue; }
            pc = 0x483cc5; continue;
            }
            0x483cc5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483cc5; continue; }
            pc = 0x483cd5; continue;
            }
            0x483cd5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x483d5e; continue; }
            pc = 0x483ce8; continue;
            }
            0x483ce8 => {
            c.set(1,32,rm(m,0x6b8d58u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483d04; continue; }
            pc = 0x483cf4; continue;
            }
            0x483cf4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483cf4; continue; }
            pc = 0x483d04; continue;
            }
            0x483d04 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x68));
            c.push(m,0x6b9d5cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x483d23);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x483d42; continue; }
            pc = 0x483d32; continue;
            }
            0x483d32 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483d32; continue; }
            pc = 0x483d42; continue;
            }
            0x483d42 => {
            c.set(1,32,rm(m,0x6b8d5cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483d5e; continue; }
            pc = 0x483d4e; continue;
            }
            0x483d4e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483d4e; continue; }
            pc = 0x483d5e; continue;
            }
            0x483d5e => {
            c.set(1,32,rm(m,0x6b8d60u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483d7a; continue; }
            pc = 0x483d6a; continue;
            }
            0x483d6a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483d6a; continue; }
            pc = 0x483d7a; continue;
            }
            0x483d7a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483d9c; continue; }
            pc = 0x483d8c; continue;
            }
            0x483d8c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483d8c; continue; }
            pc = 0x483d9c; continue;
            }
            0x483d9c => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651b; continue;
            }
            0x483da9 => {
            { let a = rm(m,c.r[7].wrapping_add(0xf),8); c.and(a,0x80u32,8); }
            c.set(1,32,rm(m,0x6b8e5cu32,32));
            if c.cond(5) { pc = 0x483dbb; continue; }
            pc = 0x483db5; continue;
            }
            0x483db5 => {
            c.set(1,32,rm(m,0x6b8e58u32,32));
            pc = 0x483dbb; continue;
            }
            0x483dbb => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483dd1; continue; }
            pc = 0x483dc1; continue;
            }
            0x483dc1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483dc1; continue; }
            pc = 0x483dd1; continue;
            }
            0x483dd1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x483de1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483df7; continue; }
            pc = 0x483de7; continue;
            }
            0x483de7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483de7; continue; }
            pc = 0x483df7; continue;
            }
            0x483df7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x483e15);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x483e2b; continue; }
            pc = 0x483e1b; continue;
            }
            0x483e1b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x483e1b; continue; }
            pc = 0x483e2b; continue;
            }
            0x483e2b => {
            c.set(1,32,rm(m,0x6b8e60u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483e47; continue; }
            pc = 0x483e37; continue;
            }
            0x483e37 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483e37; continue; }
            pc = 0x483e47; continue;
            }
            0x483e47 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.push(m,c.get(0,32));
            if c.cond(4) { pc = 0x483e71; continue; }
            pc = 0x483e63; continue;
            }
            0x483e63 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(1,32));
            c.push(m,0x6b9d60u32);
            pc = 0x483e7d; continue;
            }
            0x483e71 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(1,32));
            c.push(m,0x6b9d68u32);
            pc = 0x483e7d; continue;
            }
            0x483e7d => {
            c.push(m,c.get(2,32));
            c.push(m, 0x483e83);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x483ea2; continue; }
            pc = 0x483e92; continue;
            }
            0x483e92 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483e92; continue; }
            pc = 0x483ea2; continue;
            }
            0x483ea2 => {
            c.set(1,32,rm(m,0x6b8e64u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483ebe; continue; }
            pc = 0x483eae; continue;
            }
            0x483eae => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483eae; continue; }
            pc = 0x483ebe; continue;
            }
            0x483ebe => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x483ee0; continue; }
            pc = 0x483ed0; continue;
            }
            0x483ed0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483ed0; continue; }
            pc = 0x483ee0; continue;
            }
            0x483ee0 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651b; continue;
            }
            0x483eed => {
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,0x7fu32,16); }
            if c.cond(6) { pc = 0x483f25; continue; }
            pc = 0x483ef4; continue;
            }
            0x483ef4 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x483f05);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x483f17);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b8fbcu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x483f55; continue;
            }
            0x483f25 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(2,32));
            c.push(m, 0x483f36);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x483f48);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b8fb8u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x483f55; continue;
            }
            0x483f55 => {
            c.push(m, 0x483f5a);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x483f79; continue; }
            pc = 0x483f69; continue;
            }
            0x483f69 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483f69; continue; }
            pc = 0x483f79; continue;
            }
            0x483f79 => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x483f9a; continue; }
            pc = 0x483f8d; continue;
            }
            0x483f8d => {
            c.set(1,32,rm(m,0x6b8fc4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            pc = 0x483fa0; continue;
            }
            0x483f9a => {
            c.set(1,32,rm(m,0x6b8fc0u32,32));
            pc = 0x483fa0; continue;
            }
            0x483fa0 => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x483faf);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x483fce; continue; }
            pc = 0x483fbe; continue;
            }
            0x483fbe => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483fbe; continue; }
            pc = 0x483fce; continue;
            }
            0x483fce => {
            { let a = rm(m,c.r[7].wrapping_add(0xe),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x486513; continue; }
            pc = 0x483fd9; continue;
            }
            0x483fd9 => {
            c.set(1,32,rm(m,0x6b8fc8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x486513; continue; }
            pc = 0x483fe9; continue;
            }
            0x483fe9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x483fe9; continue; }
            pc = 0x483ff9; continue;
            }
            0x483ff9 => {
            pc = 0x486513; continue;
            }
            0x483ffe => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[1].wrapping_add(0x5); wm(m,a,8,0x31u32); }
            c.set(1,32,rm(m,0x6b9008u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484022; continue; }
            pc = 0x484012; continue;
            }
            0x484012 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484012; continue; }
            pc = 0x484022; continue;
            }
            0x484022 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484044; continue; }
            pc = 0x484034; continue;
            }
            0x484034 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484034; continue; }
            pc = 0x484044; continue;
            }
            0x484044 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x484057);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x484068);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x484078);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a171c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x74));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6b900cu32,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4840a3);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x4840ab => {
            c.set(1,32,rm(m,0x6b9088u32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x3f400000u32); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4840cf; continue; }
            pc = 0x4840bf; continue;
            }
            0x4840bf => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4840bf; continue; }
            pc = 0x4840cf; continue;
            }
            0x4840cf => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4840f1; continue; }
            pc = 0x4840e1; continue;
            }
            0x4840e1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4840e1; continue; }
            pc = 0x4840f1; continue;
            }
            0x4840f1 => {
            c.set(1,32,rm(m,0x6b908cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48410d; continue; }
            pc = 0x4840fd; continue;
            }
            0x4840fd => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4840fd; continue; }
            pc = 0x48410d; continue;
            }
            0x48410d => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x484117 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x5); wm(m,a,8,0x31u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(0,32,rm(m,0x6b9168u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x48413c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x484144 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(0x5); wm(m,a,8,0x31u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b916cu32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x484170);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x484178 => {
            c.set(1,32,rm(m,0x6b9224u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484194; continue; }
            pc = 0x484184; continue;
            }
            0x484184 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484184; continue; }
            pc = 0x484194; continue;
            }
            0x484194 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(3,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(3,32));
            c.push(m,0x6b9d70u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4841b5);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4841d4; continue; }
            pc = 0x4841c4; continue;
            }
            0x4841c4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4841c4; continue; }
            pc = 0x4841d4; continue;
            }
            0x4841d4 => {
            c.set(1,32,rm(m,0x6b9228u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4841f0; continue; }
            pc = 0x4841e0; continue;
            }
            0x4841e0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4841e0; continue; }
            pc = 0x4841f0; continue;
            }
            0x4841f0 => {
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x48421d; continue; }
            pc = 0x4841f8; continue;
            }
            0x4841f8 => {
            c.set(1,32,rm(m,0x6b9234u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48429f; continue; }
            pc = 0x484208; continue;
            }
            0x484208 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484208; continue; }
            pc = 0x484218; continue;
            }
            0x484218 => {
            pc = 0x48429f; continue;
            }
            0x48421d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x48422d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x484243; continue; }
            pc = 0x484233; continue;
            }
            0x484233 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x484233; continue; }
            pc = 0x484243; continue;
            }
            0x484243 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x484261);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x484277; continue; }
            pc = 0x484267; continue;
            }
            0x484267 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x484267; continue; }
            pc = 0x484277; continue;
            }
            0x484277 => {
            { let a = rm(m,c.r[7].wrapping_add(0x9),8); c.and(a,0x80u32,8); }
            c.set(1,32,rm(m,0x6b922cu32,32));
            if c.cond(4) { pc = 0x484289; continue; }
            pc = 0x484283; continue;
            }
            0x484283 => {
            c.set(1,32,rm(m,0x6b9230u32,32));
            pc = 0x484289; continue;
            }
            0x484289 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48429f; continue; }
            pc = 0x48428f; continue;
            }
            0x48428f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48428f; continue; }
            pc = 0x48429f; continue;
            }
            0x48429f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9d74u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4842b5);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4842d4; continue; }
            pc = 0x4842c4; continue;
            }
            0x4842c4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4842c4; continue; }
            pc = 0x4842d4; continue;
            }
            0x4842d4 => {
            c.set(1,32,rm(m,0x6b9238u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4842f0; continue; }
            pc = 0x4842e0; continue;
            }
            0x4842e0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4842e0; continue; }
            pc = 0x4842f0; continue;
            }
            0x4842f0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(1,32); let r = c.imul(a,c.get(3,32),32); c.set(1,32,r); }
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[4].wrapping_add(0x68));
            c.push(m,0x6b9d78u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x484315);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x484334; continue; }
            pc = 0x484324; continue;
            }
            0x484324 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484324; continue; }
            pc = 0x484334; continue;
            }
            0x484334 => {
            c.set(1,32,rm(m,0x6b923cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484350; continue; }
            pc = 0x484340; continue;
            }
            0x484340 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484340; continue; }
            pc = 0x484350; continue;
            }
            0x484350 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x48435a => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x48436a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1a),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4843e3; continue; }
            pc = 0x484373; continue;
            }
            0x484373 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.push(m,c.get(1,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(2,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x484391);
            { let args = []; c.r[0] = host.call(m, 0x636620, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x4ac); wm(m,a,32,0x0u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d00),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xe4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xe8),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6a99fcu32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6a99f4u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x94));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4843da);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x8c));
            pc = 0x484451; continue;
            }
            0x4843e3 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.push(m,c.get(2,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x484401);
            { let args = []; c.r[0] = host.call(m, 0x636620, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[0],32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.r[4].wrapping_add(0x4ac); wm(m,a,32,0x1u32); }
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0d00),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xe4),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xe8),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6a9a00u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6a99f4u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x94));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x48444a);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x94));
            pc = 0x484451; continue;
            }
            0x484451 => {
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x494); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x484464);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x7fu32,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672d78),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7],8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.set(5,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0xd));
            { let a = c.get(5,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(0,8); c.sub(a,0xfeu32,8); }
            if c.cond(4) { pc = 0x4844dc; continue; }
            pc = 0x484497; continue;
            }
            0x484497 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(7) { pc = 0x4844dc; continue; }
            pc = 0x4844a2; continue;
            }
            0x4844a2 => {
            c.push(m,0x4u32);
            c.push(m, 0x4844a9);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,0x100u32,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[3].wrapping_add(0xb0); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x48651f; continue; }
            pc = 0x4844c8; continue;
            }
            0x4844c8 => {
            c.set(1,32,rm(m,0x6a9a04u32,32));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,0x1u32); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x48651f; continue;
            }
            0x4844dc => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x4844e9 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4844fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.set(5,32,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1a),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(1,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            if c.cond(4) { pc = 0x484586; continue; }
            pc = 0x484517; continue;
            }
            0x484517 => {
            c.set(2,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(2,32));
            c.push(m, 0x484521);
            { let args = []; c.r[0] = host.call(m, 0x636620, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x4b0); wm(m,a,32,0x2u32); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d00),32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xe4),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xe8),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6a99fcu32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6a99f8u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x94));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x48456a);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x60));
            { let a = c.r[4].wrapping_add(0x494); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x484581);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x48651f; continue;
            }
            0x484586 => {
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m, 0x484590);
            { let args = []; c.r[0] = host.call(m, 0x636620, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x4b0); wm(m,a,32,0x3u32); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0d00),32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xe4),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xe8),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6a9a00u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6a99f8u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x94));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4845d9);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x38u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x58));
            { let a = c.r[4].wrapping_add(0x494); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4845f0);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x48651f; continue;
            }
            0x4845f5 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x484605);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x484648; continue; }
            pc = 0x48460c; continue;
            }
            0x48460c => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6a99fcu32,32));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x74));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a9a08),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x484640);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x484648 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.set(0,32,rm(m,0x6a9a00u32,32));
            c.push(m,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x74));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a9a08),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x48467b);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x484683 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6aadd8u32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4846b0);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x3f600000u32); }
            pc = 0x48651f; continue;
            }
            0x4846c0 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6aaddcu32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4846ed);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x4846f5 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(7,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1]).wrapping_add(0x2));
            c.set(2,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6aadd8),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x484736);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x48473e => {
            c.set(2,32,rm(m,0x6aade4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x48475c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x484764 => {
            c.set(1,32,rm(m,0x6b612cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484780; continue; }
            pc = 0x484770; continue;
            }
            0x484770 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484770; continue; }
            pc = 0x484780; continue;
            }
            0x484780 => {
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x484790);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xe8),32));
            c.set(1,8,rm(m,c.r[2],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4847ac; continue; }
            pc = 0x48479c; continue;
            }
            0x48479c => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[2],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48479c; continue; }
            pc = 0x4847ac; continue;
            }
            0x4847ac => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xe4),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4847d2; continue; }
            pc = 0x4847c2; continue;
            }
            0x4847c2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4847c2; continue; }
            pc = 0x4847d2; continue;
            }
            0x4847d2 => {
            c.set(1,32,rm(m,0x6b6130u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4847ee; continue; }
            pc = 0x4847de; continue;
            }
            0x4847de => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4847de; continue; }
            pc = 0x4847ee; continue;
            }
            0x4847ee => {
            c.set(1,32,rm(m,0x6aa18cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48480a; continue; }
            pc = 0x4847fa; continue;
            }
            0x4847fa => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4847fa; continue; }
            pc = 0x48480a; continue;
            }
            0x48480a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48482c; continue; }
            pc = 0x48481c; continue;
            }
            0x48481c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48481c; continue; }
            pc = 0x48482c; continue;
            }
            0x48482c => {
            c.set(1,32,rm(m,0x6aa190u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484848; continue; }
            pc = 0x484838; continue;
            }
            0x484838 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484838; continue; }
            pc = 0x484848; continue;
            }
            0x484848 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x484858);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0x7fu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48486a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x48487b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            pc = 0x48487f; continue;
            }
            0x48487f => {
            c.set(1,32,rm(m,0x6b6140u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x484891);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x484899 => {
            c.set(1,32,rm(m,0x6ab238u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4848b5; continue; }
            pc = 0x4848a5; continue;
            }
            0x4848a5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4848a5; continue; }
            pc = 0x4848b5; continue;
            }
            0x4848b5 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xd),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x484970; continue; }
            pc = 0x4848cc; continue;
            }
            0x4848cc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x484952; continue; }
            pc = 0x4848d7; continue;
            }
            0x4848d7 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x486918)); continue;
            }
            0x4848de => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(0,32,rm(m,0x6ab24cu32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x48494a; continue;
            }
            0x4848ef => {
            c.set(2,32,rm(m,0x6ab23cu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x484900);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x484952; continue;
            }
            0x484905 => {
            c.set(1,32,rm(m,0x6ab240u32,32));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x484916);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x484952; continue;
            }
            0x48491b => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(6) { pc = 0x484938; continue; }
            pc = 0x484925; continue;
            }
            0x484925 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6ab244u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x48494a; continue;
            }
            0x484938 => {
            c.set(2,32,rm(m,0x6ab248u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x48494a; continue;
            }
            0x48494a => {
            c.push(m, 0x48494f);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x484952; continue;
            }
            0x484952 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484993; continue; }
            pc = 0x48495e; continue;
            }
            0x48495e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48495e; continue; }
            pc = 0x48496e; continue;
            }
            0x48496e => {
            pc = 0x484993; continue;
            }
            0x484970 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ab250),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484993; continue; }
            pc = 0x484983; continue;
            }
            0x484983 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484983; continue; }
            pc = 0x484993; continue;
            }
            0x484993 => {
            c.set(2,32,rm(m,0x6ab260u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4849b1);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(1,8)); }
            pc = 0x48651f; continue;
            }
            0x4849c0 => {
            c.set(1,32,rm(m,0x6ab32cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4849dc; continue; }
            pc = 0x4849cc; continue;
            }
            0x4849cc => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4849cc; continue; }
            pc = 0x4849dc; continue;
            }
            0x4849dc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4849fe; continue; }
            pc = 0x4849ee; continue;
            }
            0x4849ee => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4849ee; continue; }
            pc = 0x4849fe; continue;
            }
            0x4849fe => {
            c.set(1,32,rm(m,0x6ab330u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484a1a; continue; }
            pc = 0x484a0a; continue;
            }
            0x484a0a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484a0a; continue; }
            pc = 0x484a1a; continue;
            }
            0x484a1a => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9d7cu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x484a2d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x484a4c; continue; }
            pc = 0x484a3c; continue;
            }
            0x484a3c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484a3c; continue; }
            pc = 0x484a4c; continue;
            }
            0x484a4c => {
            c.set(1,32,rm(m,0x6ab334u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484a68; continue; }
            pc = 0x484a58; continue;
            }
            0x484a58 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484a58; continue; }
            pc = 0x484a68; continue;
            }
            0x484a68 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(0,32); let r = c.and(a,0x7fffffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x6b9d80u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x484a80);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x484a9f; continue; }
            pc = 0x484a8f; continue;
            }
            0x484a8f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484a8f; continue; }
            pc = 0x484a9f; continue;
            }
            0x484a9f => {
            c.set(1,32,rm(m,0x6ab338u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484abb; continue; }
            pc = 0x484aab; continue;
            }
            0x484aab => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484aab; continue; }
            pc = 0x484abb; continue;
            }
            0x484abb => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(9) { pc = 0x484ade; continue; }
            pc = 0x484ac2; continue;
            }
            0x484ac2 => {
            c.set(1,32,rm(m,0x6ab33cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484ade; continue; }
            pc = 0x484ace; continue;
            }
            0x484ace => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484ace; continue; }
            pc = 0x484ade; continue;
            }
            0x484ade => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x484ae8 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(0,8); c.sub(a,0x80u32,8); }
            if c.cond(2) { pc = 0x484b57; continue; }
            pc = 0x484afb; continue;
            }
            0x484afb => {
            c.set(1,32,rm(m,0x6ab638u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484b17; continue; }
            pc = 0x484b07; continue;
            }
            0x484b07 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484b07; continue; }
            pc = 0x484b17; continue;
            }
            0x484b17 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0c18),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484b39; continue; }
            pc = 0x484b29; continue;
            }
            0x484b29 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484b29; continue; }
            pc = 0x484b39; continue;
            }
            0x484b39 => {
            c.set(1,32,rm(m,0x6ab63cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484bb1; continue; }
            pc = 0x484b45; continue;
            }
            0x484b45 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484b45; continue; }
            pc = 0x484b55; continue;
            }
            0x484b55 => {
            pc = 0x484bb1; continue;
            }
            0x484b57 => {
            c.set(1,32,rm(m,0x6ab630u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484b73; continue; }
            pc = 0x484b63; continue;
            }
            0x484b63 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484b63; continue; }
            pc = 0x484b73; continue;
            }
            0x484b73 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0c54),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484b95; continue; }
            pc = 0x484b85; continue;
            }
            0x484b85 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484b85; continue; }
            pc = 0x484b95; continue;
            }
            0x484b95 => {
            c.set(1,32,rm(m,0x6ab634u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484bb1; continue; }
            pc = 0x484ba1; continue;
            }
            0x484ba1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484ba1; continue; }
            pc = 0x484bb1; continue;
            }
            0x484bb1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x484bc1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x484bd7; continue; }
            pc = 0x484bc7; continue;
            }
            0x484bc7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x484bc7; continue; }
            pc = 0x484bd7; continue;
            }
            0x484bd7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x484bf0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x484c06; continue; }
            pc = 0x484bf6; continue;
            }
            0x484bf6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x484bf6; continue; }
            pc = 0x484c06; continue;
            }
            0x484c06 => {
            c.set(1,32,rm(m,0x6ab640u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484c22; continue; }
            pc = 0x484c12; continue;
            }
            0x484c12 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484c12; continue; }
            pc = 0x484c22; continue;
            }
            0x484c22 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484c44; continue; }
            pc = 0x484c34; continue;
            }
            0x484c34 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484c34; continue; }
            pc = 0x484c44; continue;
            }
            0x484c44 => {
            c.set(1,32,rm(m,0x6ab644u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484c60; continue; }
            pc = 0x484c50; continue;
            }
            0x484c50 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484c50; continue; }
            pc = 0x484c60; continue;
            }
            0x484c60 => {
            { let a = rm(m,c.r[7].wrapping_add(0x9),8); c.sub(a,0x80u32,8); }
            if c.cond(2) { pc = 0x484cc2; continue; }
            pc = 0x484c66; continue;
            }
            0x484c66 => {
            c.set(1,32,rm(m,0x6ab650u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484c82; continue; }
            pc = 0x484c72; continue;
            }
            0x484c72 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484c72; continue; }
            pc = 0x484c82; continue;
            }
            0x484c82 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x69ee48),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484ca4; continue; }
            pc = 0x484c94; continue;
            }
            0x484c94 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484c94; continue; }
            pc = 0x484ca4; continue;
            }
            0x484ca4 => {
            c.set(1,32,rm(m,0x6ab654u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484d1c; continue; }
            pc = 0x484cb0; continue;
            }
            0x484cb0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484cb0; continue; }
            pc = 0x484cc0; continue;
            }
            0x484cc0 => {
            pc = 0x484d1c; continue;
            }
            0x484cc2 => {
            c.set(1,32,rm(m,0x6ab648u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484cde; continue; }
            pc = 0x484cce; continue;
            }
            0x484cce => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484cce; continue; }
            pc = 0x484cde; continue;
            }
            0x484cde => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x69ee48),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484d00; continue; }
            pc = 0x484cf0; continue;
            }
            0x484cf0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484cf0; continue; }
            pc = 0x484d00; continue;
            }
            0x484d00 => {
            c.set(1,32,rm(m,0x6ab64cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484d1c; continue; }
            pc = 0x484d0c; continue;
            }
            0x484d0c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484d0c; continue; }
            pc = 0x484d1c; continue;
            }
            0x484d1c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x484d2c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x484d42; continue; }
            pc = 0x484d32; continue;
            }
            0x484d32 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x484d32; continue; }
            pc = 0x484d42; continue;
            }
            0x484d42 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x484d5b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48651f; continue; }
            pc = 0x484d65; continue;
            }
            0x484d65 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x484d65; continue; }
            pc = 0x484d75; continue;
            }
            0x484d75 => {
            pc = 0x48651f; continue;
            }
            0x484d7a => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xb8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x484d97; continue; }
            pc = 0x484d84; continue;
            }
            0x484d84 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x484d8a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bb490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[3].wrapping_add(0xb8); wm(m,a,32,0x0u32); }
            pc = 0x484d97; continue;
            }
            0x484d97 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x484d9e);
            { let args = []; c.r[0] = host.call(m, 0x51ac60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[3].wrapping_add(0xb8); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xb8),32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x68));
            c.push(m,0x6b9d84u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x484dc1);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x484dc9 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x484de0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x484df1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xa),8));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x2u32,32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ab630),32));
            c.push(m,c.get(1,32));
            c.push(m,0x6b9d90u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x484e12);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x484e31; continue; }
            pc = 0x484e21; continue;
            }
            0x484e21 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484e21; continue; }
            pc = 0x484e31; continue;
            }
            0x484e31 => {
            { let a = rm(m,c.r[7].wrapping_add(0x4),8); c.sub(a,0x6fu32,8); }
            if c.cond(5) { pc = 0x484fa1; continue; }
            pc = 0x484e3b; continue;
            }
            0x484e3b => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.and(a,0x7fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x484f32; continue; }
            pc = 0x484e4a; continue;
            }
            0x484e4a => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x486928)); continue;
            }
            0x484e51 => {
            c.set(2,32,rm(m,0x6aba78u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x484e6f);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x484f37; continue;
            }
            0x484e77 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6aba7cu32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x484ea2);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x484f37; continue;
            }
            0x484eaa => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x484eba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x484ecb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6aba80u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x484edc);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x484f37; continue;
            }
            0x484ee1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,32); c.sub(a,0x28u32,32); }
            if c.cond(3) { pc = 0x484ef4; continue; }
            pc = 0x484eeb; continue;
            }
            0x484eeb => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddaa8),32));
            pc = 0x484ef9; continue;
            }
            0x484ef4 => {
            c.set(0,32,rm(m,0x6ddaa0u32,32));
            pc = 0x484ef9; continue;
            }
            0x484ef9 => {
            c.set(2,32,rm(m,0x6aba84u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x484f0b);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x484f37; continue;
            }
            0x484f10 => {
            c.set(0,32,rm(m,0x6aba88u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x484f2d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x484f37; continue;
            }
            0x484f32 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x484f37; continue;
            }
            0x484f37 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484f53; continue; }
            pc = 0x484f43; continue;
            }
            0x484f43 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484f43; continue; }
            pc = 0x484f53; continue;
            }
            0x484f53 => {
            c.set(1,32,rm(m,0x6aba8cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484f6f; continue; }
            pc = 0x484f5f; continue;
            }
            0x484f5f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484f5f; continue; }
            pc = 0x484f6f; continue;
            }
            0x484f6f => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,8); let r = c.and(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x80u32,8); }
            if c.cond(5) { pc = 0x4850f0; continue; }
            pc = 0x484f7c; continue;
            }
            0x484f7c => {
            c.set(1,32,rm(m,0x6aba90u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4850f0; continue; }
            pc = 0x484f8c; continue;
            }
            0x484f8c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484f8c; continue; }
            pc = 0x484f9c; continue;
            }
            0x484f9c => {
            pc = 0x4850f0; continue;
            }
            0x484fa1 => {
            c.set(1,32,rm(m,0x6aba94u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x484fbd; continue; }
            pc = 0x484fad; continue;
            }
            0x484fad => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x484fad; continue; }
            pc = 0x484fbd; continue;
            }
            0x484fbd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x4850b3; continue; }
            pc = 0x484fcb; continue;
            }
            0x484fcb => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x48693c)); continue;
            }
            0x484fd2 => {
            c.set(2,32,rm(m,0x6aba98u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x484ff0);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4850b8; continue;
            }
            0x484ff8 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6aba9cu32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x485023);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x4850b8; continue;
            }
            0x48502b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48503b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x48504c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6abaa0u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x48505d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x4850b8; continue;
            }
            0x485062 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,32); c.sub(a,0x28u32,32); }
            if c.cond(3) { pc = 0x485075; continue; }
            pc = 0x48506c; continue;
            }
            0x48506c => {
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddaa8),32));
            pc = 0x48507a; continue;
            }
            0x485075 => {
            c.set(0,32,rm(m,0x6ddaa0u32,32));
            pc = 0x48507a; continue;
            }
            0x48507a => {
            c.set(2,32,rm(m,0x6abaa4u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x48508c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4850b8; continue;
            }
            0x485091 => {
            c.set(0,32,rm(m,0x6abaa8u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4850ae);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4850b8; continue;
            }
            0x4850b3 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x4850b8; continue;
            }
            0x4850b8 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4850d4; continue; }
            pc = 0x4850c4; continue;
            }
            0x4850c4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4850c4; continue; }
            pc = 0x4850d4; continue;
            }
            0x4850d4 => {
            c.set(1,32,rm(m,0x6abaacu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4850f0; continue; }
            pc = 0x4850e0; continue;
            }
            0x4850e0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4850e0; continue; }
            pc = 0x4850f0; continue;
            }
            0x4850f0 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x2u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ab64c),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485116; continue; }
            pc = 0x485106; continue;
            }
            0x485106 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485106; continue; }
            pc = 0x485116; continue;
            }
            0x485116 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x485126);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48513c; continue; }
            pc = 0x48512c; continue;
            }
            0x48512c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48512c; continue; }
            pc = 0x48513c; continue;
            }
            0x48513c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x485155);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48516b; continue; }
            pc = 0x48515b; continue;
            }
            0x48515b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48515b; continue; }
            pc = 0x48516b; continue;
            }
            0x48516b => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x485175 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x485185);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48519b; continue; }
            pc = 0x48518b; continue;
            }
            0x48518b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48518b; continue; }
            pc = 0x48519b; continue;
            }
            0x48519b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4851b9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4851cf; continue; }
            pc = 0x4851bf; continue;
            }
            0x4851bf => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4851bf; continue; }
            pc = 0x4851cf; continue;
            }
            0x4851cf => {
            c.set(1,32,rm(m,0x6abb78u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4851eb; continue; }
            pc = 0x4851db; continue;
            }
            0x4851db => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4851db; continue; }
            pc = 0x4851eb; continue;
            }
            0x4851eb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48520d; continue; }
            pc = 0x4851fd; continue;
            }
            0x4851fd => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4851fd; continue; }
            pc = 0x48520d; continue;
            }
            0x48520d => {
            c.set(1,32,rm(m,0x6abb7cu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485229; continue; }
            pc = 0x485219; continue;
            }
            0x485219 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485219; continue; }
            pc = 0x485229; continue;
            }
            0x485229 => {
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,32,0x0u32); }
            { let a = c.get(7,32); let r = c.add(a,0x8u32,32); c.set(7,32,r); }
            c.set(3,32,0x3u32);
            pc = 0x485239; continue;
            }
            0x485239 => {
            { let a = rm(m,c.r[7],16); c.sub(a,0x1800u32,16); }
            if c.cond(3) { pc = 0x4852c5; continue; }
            pc = 0x485244; continue;
            }
            0x485244 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x32),32));
            c.set(1,16,c.get(0,16));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,16); c.and(a,c.get(1,16),16); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x485271; continue; }
            pc = 0x485255; continue;
            }
            0x485255 => {
            c.set(1,32,rm(m,0x6abb80u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485271; continue; }
            pc = 0x485261; continue;
            }
            0x485261 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485261; continue; }
            pc = 0x485271; continue;
            }
            0x485271 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            c.push(m,c.get(1,32));
            c.set(1,8,rm(m,c.r[7],8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0dec),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x70));
            c.push(m,0x6b9d9cu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4852a6);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4852c5; continue; }
            pc = 0x4852b5; continue;
            }
            0x4852b5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4852b5; continue; }
            pc = 0x4852c5; continue;
            }
            0x4852c5 => {
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x485239; continue; }
            pc = 0x4852cf; continue;
            }
            0x4852cf => {
            { let a = rm(m,c.r[4].wrapping_add(0x32),16); c.sub(a,0x0u32,16); }
            c.set(1,32,rm(m,0x6abb84u32,32));
            if c.cond(5) { pc = 0x4852e3; continue; }
            pc = 0x4852dd; continue;
            }
            0x4852dd => {
            c.set(1,32,rm(m,0x6abb88u32,32));
            pc = 0x4852e3; continue;
            }
            0x4852e3 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4852f9; continue; }
            pc = 0x4852e9; continue;
            }
            0x4852e9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4852e9; continue; }
            pc = 0x4852f9; continue;
            }
            0x4852f9 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x485303 => {
            { let a = rm(m,c.r[7].wrapping_add(0x9),8); c.and(a,0x80u32,8); }
            if c.cond(5) { pc = 0x48531a; continue; }
            pc = 0x485309; continue;
            }
            0x485309 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xa),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0628),32));
            pc = 0x48532a; continue;
            }
            0x48531a => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a0608),32));
            pc = 0x48532a; continue;
            }
            0x48532a => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485340; continue; }
            pc = 0x485330; continue;
            }
            0x485330 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485330; continue; }
            pc = 0x485340; continue;
            }
            0x485340 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x485359);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48536f; continue; }
            pc = 0x48535f; continue;
            }
            0x48535f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48535f; continue; }
            pc = 0x48536f; continue;
            }
            0x48536f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48538d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4853a3; continue; }
            pc = 0x485393; continue;
            }
            0x485393 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x485393; continue; }
            pc = 0x4853a3; continue;
            }
            0x4853a3 => {
            c.set(2,32,rm(m,0x6abd0cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4853c1);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4853e0; continue; }
            pc = 0x4853d0; continue;
            }
            0x4853d0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4853d0; continue; }
            pc = 0x4853e0; continue;
            }
            0x4853e0 => {
            { let a = rm(m,c.r[7].wrapping_add(0xa),16); c.sub(a,0xffu32,16); }
            c.set(1,32,rm(m,0x6abd10u32,32));
            if c.cond(7) { pc = 0x4853f4; continue; }
            pc = 0x4853ee; continue;
            }
            0x4853ee => {
            c.set(1,32,rm(m,0x6abd1cu32,32));
            pc = 0x4853f4; continue;
            }
            0x4853f4 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48540a; continue; }
            pc = 0x4853fa; continue;
            }
            0x4853fa => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4853fa; continue; }
            pc = 0x48540a; continue;
            }
            0x48540a => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(0,16); c.sub(a,0x8000u32,16); }
            { let a = c.get(5,32); let r = c.sbb(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(5,32); let r = c.neg(a,32); c.set(5,32,r); }
            c.push(m, 0x485426);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x485437);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[5].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x69ef2c),32));
            c.set(1,32,rm(m,0x6abd14u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6c0ee8),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x78));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x485472);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x80),8));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x485494; continue; }
            pc = 0x485484; continue;
            }
            0x485484 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485484; continue; }
            pc = 0x485494; continue;
            }
            0x485494 => {
            c.set(1,32,rm(m,0x6abd18u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4854b0; continue; }
            pc = 0x4854a0; continue;
            }
            0x4854a0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4854a0; continue; }
            pc = 0x4854b0; continue;
            }
            0x4854b0 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x4854ba => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x4854ca);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4854e0; continue; }
            pc = 0x4854d0; continue;
            }
            0x4854d0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4854d0; continue; }
            pc = 0x4854e0; continue;
            }
            0x4854e0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4854fe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x485514; continue; }
            pc = 0x485504; continue;
            }
            0x485504 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x485504; continue; }
            pc = 0x485514; continue;
            }
            0x485514 => {
            c.set(1,32,rm(m,0x6abdb4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485530; continue; }
            pc = 0x485520; continue;
            }
            0x485520 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485520; continue; }
            pc = 0x485530; continue;
            }
            0x485530 => {
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,32,0x0u32); }
            { let a = c.get(7,32); let r = c.add(a,0x8u32,32); c.set(7,32,r); }
            c.set(3,32,0x3u32);
            pc = 0x485540; continue;
            }
            0x485540 => {
            { let a = rm(m,c.r[7],16); c.sub(a,0x1800u32,16); }
            if c.cond(3) { pc = 0x4855cc; continue; }
            pc = 0x48554b; continue;
            }
            0x48554b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x32),32));
            c.set(1,16,c.get(0,16));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,16); c.and(a,c.get(1,16),16); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x485578; continue; }
            pc = 0x48555c; continue;
            }
            0x48555c => {
            c.set(1,32,rm(m,0x6abdb8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485578; continue; }
            pc = 0x485568; continue;
            }
            0x485568 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485568; continue; }
            pc = 0x485578; continue;
            }
            0x485578 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            c.push(m,c.get(1,32));
            c.set(1,8,rm(m,c.r[7],8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6c0dec),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x70));
            c.push(m,0x6b9da8u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x4855ad);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4855cc; continue; }
            pc = 0x4855bc; continue;
            }
            0x4855bc => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4855bc; continue; }
            pc = 0x4855cc; continue;
            }
            0x4855cc => {
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x485540; continue; }
            pc = 0x4855d6; continue;
            }
            0x4855d6 => {
            { let a = rm(m,c.r[4].wrapping_add(0x32),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x4855fa; continue; }
            pc = 0x4855de; continue;
            }
            0x4855de => {
            c.set(1,32,rm(m,0x6abdbcu32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4855fa; continue; }
            pc = 0x4855ea; continue;
            }
            0x4855ea => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4855ea; continue; }
            pc = 0x4855fa; continue;
            }
            0x4855fa => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x485604 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(9) { pc = 0x48563b; continue; }
            pc = 0x48560b; continue;
            }
            0x48560b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x48561b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48562c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6ae924u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x485668; continue;
            }
            0x48563b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48564b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x48565c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6ae920u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x485668; continue;
            }
            0x485668 => {
            c.push(m, 0x48566d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x48568c; continue; }
            pc = 0x48567c; continue;
            }
            0x48567c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48567c; continue; }
            pc = 0x48568c; continue;
            }
            0x48568c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.get(0,32); let r = c.add(a,0xffffff8au32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x9u32,32); }
            if c.cond(7) { pc = 0x48651f; continue; }
            pc = 0x48569d; continue;
            }
            0x48569d => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x486964),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x486950)); continue;
            }
            0x4856ac => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a57c8),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6ae92cu32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4856d6);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x4856de => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x4856f5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x485706);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a57c8),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6ae928u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x78));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x485739);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x485741 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x485751);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x485762);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a57c8),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6ae934u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x485795);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x48579d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a57c8),32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6ae930u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4857d0);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x4857d8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x48582a; continue; }
            pc = 0x4857e2; continue;
            }
            0x4857e2 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x485804; continue; }
            pc = 0x4857e5; continue;
            }
            0x4857e5 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x48651f; continue; }
            pc = 0x4857ec; continue;
            }
            0x4857ec => {
            c.set(0,32,rm(m,0x6addd8u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4857fc);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x485804 => {
            c.set(1,32,rm(m,0x6addd4u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x485822);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x48582a => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x9),8));
            { let a = c.get(0,8); c.sub(a,0x5u32,8); }
            if c.cond(2) { pc = 0x48593d; continue; }
            pc = 0x485835; continue;
            }
            0x485835 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(9) { pc = 0x48586c; continue; }
            pc = 0x48583c; continue;
            }
            0x48583c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x48584c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48585d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6addc4u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x485899; continue;
            }
            0x48586c => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48587c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x48588d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6addc0u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x485899; continue;
            }
            0x485899 => {
            c.push(m, 0x48589e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4858bd; continue; }
            pc = 0x4858ad; continue;
            }
            0x4858ad => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4858ad; continue; }
            pc = 0x4858bd; continue;
            }
            0x4858bd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6addb4),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4858e3);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x485902; continue; }
            pc = 0x4858f2; continue;
            }
            0x4858f2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4858f2; continue; }
            pc = 0x485902; continue;
            }
            0x485902 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x485912);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x485923);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6addd0u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x485935);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x48593d => {
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x48598d; continue; }
            pc = 0x48594c; continue;
            }
            0x48594c => {
            c.set(1,32,rm(m,0x6add90u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485968; continue; }
            pc = 0x485958; continue;
            }
            0x485958 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485958; continue; }
            pc = 0x485968; continue;
            }
            0x485968 => {
            c.set(1,32,rm(m,0x6adda8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485aa8; continue; }
            pc = 0x485978; continue;
            }
            0x485978 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485978; continue; }
            pc = 0x485988; continue;
            }
            0x485988 => {
            pc = 0x485aa8; continue;
            }
            0x48598d => {
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6add84),32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4859ac);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4859cb; continue; }
            pc = 0x4859bb; continue;
            }
            0x4859bb => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4859bb; continue; }
            pc = 0x4859cb; continue;
            }
            0x4859cb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x9),8));
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x485a8c; continue; }
            pc = 0x4859d9; continue;
            }
            0x4859d9 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x486970)); continue;
            }
            0x4859e0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6add98u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x485a0b);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x485a8c; continue;
            }
            0x485a10 => {
            c.set(2,32,rm(m,0x6add9cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x485a84; continue;
            }
            0x485a2b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x485a48);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(0,32,rm(m,0x6adda0u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x485a66);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x485a8c; continue;
            }
            0x485a6b => {
            c.set(1,32,rm(m,0x6adda4u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x485a84; continue;
            }
            0x485a84 => {
            c.push(m, 0x485a89);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x485a8c; continue;
            }
            0x485a8c => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485aa8; continue; }
            pc = 0x485a98; continue;
            }
            0x485a98 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485a98; continue; }
            pc = 0x485aa8; continue;
            }
            0x485aa8 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6addac),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485acb; continue; }
            pc = 0x485abb; continue;
            }
            0x485abb => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485abb; continue; }
            pc = 0x485acb; continue;
            }
            0x485acb => {
            c.set(0,32,rm(m,0x6addbcu32,32));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x485adb);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x485ae3 => {
            c.set(1,32,rm(m,0x6add70u32,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x2),16));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485b03; continue; }
            pc = 0x485af3; continue;
            }
            0x485af3 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[3].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485af3; continue; }
            pc = 0x485b03; continue;
            }
            0x485b03 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x9),8));
            { let a = c.get(1,8); let r = c.and(a,0x7fu32,8); c.set(1,8,r); }
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(2) { pc = 0x485b1b; continue; }
            pc = 0x485b10; continue;
            }
            0x485b10 => {
            if c.cond(5) { pc = 0x485b1c; continue; }
            pc = 0x485b12; continue;
            }
            0x485b12 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(3) { pc = 0x485b1c; continue; }
            pc = 0x485b1b; continue;
            }
            0x485b1b => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x485b1c; continue;
            }
            0x485b1c => {
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0d00),32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6add74u32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x74));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x485b53);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x7c),8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x485b72; continue; }
            pc = 0x485b62; continue;
            }
            0x485b62 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485b62; continue; }
            pc = 0x485b72; continue;
            }
            0x485b72 => {
            { let a = rm(m,c.r[7].wrapping_add(0xb),8); c.and(a,0xf0u32,8); }
            if c.cond(5) { pc = 0x485ba3; continue; }
            pc = 0x485b78; continue;
            }
            0x485b78 => {
            c.set(0,32,rm(m,0x6a96d0u32,32));
            c.set(1,32,rm(m,0x6a96acu32,32));
            c.set(2,32,rm(m,0x6add78u32,32));
            c.push(m,c.get(0,32));
            c.push(m,0x6ce6cau32);
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x485b9b);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x485c5c; continue;
            }
            0x485ba3 => {
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            { let a = c.r[4].wrapping_add(0x494); wm(m,a,32,0x4u32); }
            { let a = c.get(0,32); let r = c.and(a,0xfu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x6u32,32); }
            if c.cond(7) { pc = 0x485c0e; continue; }
            pc = 0x485bc3; continue;
            }
            0x485bc3 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x486984)); continue;
            }
            0x485bca => {
            c.set(2,32,rm(m,0x6a95bcu32,32));
            pc = 0x485c04; continue;
            }
            0x485bd2 => {
            c.set(2,32,rm(m,0x6a95b8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6c0bb0),32));
            c.set(0,32,c.r[4].wrapping_add(0x40));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x485bf0);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x485c0e; continue;
            }
            0x485bf5 => {
            c.set(1,32,rm(m,0x6a95c0u32,32));
            c.push(m,c.get(1,32));
            pc = 0x485c05; continue;
            }
            0x485bfe => {
            c.set(2,32,rm(m,0x6a95c4u32,32));
            pc = 0x485c04; continue;
            }
            0x485c04 => {
            c.push(m,c.get(2,32));
            pc = 0x485c05; continue;
            }
            0x485c05 => {
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x485c0e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x485c0e; continue;
            }
            0x485c0e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,0x6a95b4u32);
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,0x6add78u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x485c45);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x494); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x485c5c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x485c5c; continue;
            }
            0x485c5c => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485c78; continue; }
            pc = 0x485c68; continue;
            }
            0x485c68 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485c68; continue; }
            pc = 0x485c78; continue;
            }
            0x485c78 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(9) { pc = 0x485cc7; continue; }
            pc = 0x485c84; continue;
            }
            0x485c84 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x485c8f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x485ca0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6add80u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x69ee48),32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x485cbf);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x485cc7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x485cd2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x485ce3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6add7cu32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x69ee48),32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x485d02);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x485d0a => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd748u32,16),16); }
            if c.cond(3) { pc = 0x485dd8; continue; }
            pc = 0x485d1b; continue;
            }
            0x485d1b => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x485d2b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x6),16); c.sub(a,0x1u32,16); }
            if c.cond(5) { pc = 0x485d7f; continue; }
            pc = 0x485d32; continue;
            }
            0x485d32 => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x11),8));
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(2,16); c.sub(a,c.get(1,16),16); }
            if c.cond(2) { pc = 0x485d7f; continue; }
            pc = 0x485d43; continue;
            }
            0x485d43 => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x10),8));
            { let a = c.get(2,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x485d7f; continue; }
            pc = 0x485d4d; continue;
            }
            0x485d4d => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x485d7f; continue; }
            pc = 0x485d54; continue;
            }
            0x485d54 => {
            c.push(m,0x8u32);
            { let a = c.r[4].wrapping_add(0x1a); wm(m,a,16,0x2u32); }
            c.push(m, 0x485d62);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6aebfcu32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,0x6aec00u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x485dd8; continue;
            }
            0x485d7f => {
            c.set(1,32,c.r[4].wrapping_add(0x464));
            c.push(m, 0x485d8b);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x464); wm(m,a,32,0x77u32); }
            { let a = c.r[4].wrapping_add(0x468); wm(m,a,32,0xffffu32); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xd2),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[4].wrapping_add(0x464));
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)),16));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x470); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x474); wm(m,a,32,0x1u32); }
            c.push(m, 0x485dd8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x485dd8; continue;
            }
            0x485dd8 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xf),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x485de8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x485df9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(1,32,0x6c3054u32);
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x6c));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(2,32));
            c.push(m,0x6b9db4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x485e1e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x485e3d; continue; }
            pc = 0x485e2d; continue;
            }
            0x485e2d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485e2d; continue; }
            pc = 0x485e3d; continue;
            }
            0x485e3d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.and(a,0x3c0u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(0,32); let n = 0xau32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x485e70);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.push(m,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0x2fu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a0628),32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xd2),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,8,rm(m,c.r[5].wrapping_add(c.r[1].wrapping_mul(4)),8));
            c.set(1,32,c.r[4].wrapping_add(0x74));
            { let a = c.get(2,32); let r = c.and(a,0x2u32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6aec04),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x485ea4);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x7c),8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x485ec3; continue; }
            pc = 0x485eb3; continue;
            }
            0x485eb3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485eb3; continue; }
            pc = 0x485ec3; continue;
            }
            0x485ec3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xd2),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,8,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)),8));
            { let a = c.get(1,32); let r = c.and(a,0x2u32,32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6aec08),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485ef3; continue; }
            pc = 0x485ee3; continue;
            }
            0x485ee3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485ee3; continue; }
            pc = 0x485ef3; continue;
            }
            0x485ef3 => {
            c.set(1,32,rm(m,0x6aec14u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485f0f; continue; }
            pc = 0x485eff; continue;
            }
            0x485eff => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485eff; continue; }
            pc = 0x485f0f; continue;
            }
            0x485f0f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485f31; continue; }
            pc = 0x485f21; continue;
            }
            0x485f21 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485f21; continue; }
            pc = 0x485f31; continue;
            }
            0x485f31 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x485f3b => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(9) { pc = 0x485f6c; continue; }
            pc = 0x485f48; continue;
            }
            0x485f48 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x485f53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6aed88u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x485f64);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x485f6c => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x485f77);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6aed8cu32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x485f89);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x485f91 => {
            c.set(0,8,rm(m,0x6b9dc4u32,8));
            c.set(1,32,0x6b9dc4u32);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x485faf; continue; }
            pc = 0x485f9f; continue;
            }
            0x485f9f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x485f9f; continue; }
            pc = 0x485faf; continue;
            }
            0x485faf => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,16); let n = 0x7u32; let r = c.shr(a,n,16); c.set(0,16,r); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x485fcc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x485fde);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x36),32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0c18),32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6b1d6c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x486009);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x486028; continue; }
            pc = 0x486018; continue;
            }
            0x486018 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x486018; continue; }
            pc = 0x486028; continue;
            }
            0x486028 => {
            c.set(2,32,rm(m,0x6b1d74u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x486046);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x48604e => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x48605e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6ddaa0u32);
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x1d),8));
            c.push(m,c.get(2,32));
            c.push(m, 0x486070);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1c),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x486081);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.push(m,0x6b9dccu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x486091);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4860b0; continue; }
            pc = 0x4860a0; continue;
            }
            0x4860a0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4860a0; continue; }
            pc = 0x4860b0; continue;
            }
            0x4860b0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xa),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6b23a0u32,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x6c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4860db);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4860fa; continue; }
            pc = 0x4860ea; continue;
            }
            0x4860ea => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4860ea; continue; }
            pc = 0x4860fa; continue;
            }
            0x4860fa => {
            c.set(1,32,rm(m,0x6b23a4u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x486116; continue; }
            pc = 0x486106; continue;
            }
            0x486106 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x486106; continue; }
            pc = 0x486116; continue;
            }
            0x486116 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0xcu32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b23a8u32,32));
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x48613d);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x48615c; continue; }
            pc = 0x48614c; continue;
            }
            0x48614c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48614c; continue; }
            pc = 0x48615c; continue;
            }
            0x48615c => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            { let a = c.get(0,8); let n = 0x7u32; let r = c.shr(a,n,8); c.set(0,8,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x9),8));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48617b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x48618c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x36),32));
            c.set(2,32,0x6a0644u32);
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x6c));
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,c.get(0,32));
            c.push(m,0x6b9dd8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4861b0);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(2,8)); }
            pc = 0x48651f; continue;
            }
            0x4861bf => {
            c.set(2,32,rm(m,0x6b19e8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4861dd);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4861fc; continue; }
            pc = 0x4861ec; continue;
            }
            0x4861ec => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4861ec; continue; }
            pc = 0x4861fc; continue;
            }
            0x4861fc => {
            c.set(2,32,rm(m,0x6b19ecu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x48621a);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x486222 => {
            c.set(1,32,rm(m,0x6b19f0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48623e; continue; }
            pc = 0x48622e; continue;
            }
            0x48622e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48622e; continue; }
            pc = 0x48623e; continue;
            }
            0x48623e => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,8); let n = 0x7u32; let r = c.shr(a,n,8); c.set(0,8,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            c.set(5,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,32,c.get(5,32)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.push(m, 0x486264);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xc),16));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x486276);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(1,32,c.r[5].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0628),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x70));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0c54),32));
            c.set(0,32,rm(m,c.r[5].wrapping_mul(4).wrapping_add(0x6b19fc),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4862b7);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x7c),8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4862d6; continue; }
            pc = 0x4862c6; continue;
            }
            0x4862c6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4862c6; continue; }
            pc = 0x4862d6; continue;
            }
            0x4862d6 => {
            c.set(2,32,rm(m,0x6b19f4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,c.r[4].wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4862f4);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x70),8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x486313; continue; }
            pc = 0x486303; continue;
            }
            0x486303 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x486303; continue; }
            pc = 0x486313; continue;
            }
            0x486313 => {
            c.set(1,32,rm(m,0x6b19f8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48632f; continue; }
            pc = 0x48631f; continue;
            }
            0x48631f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48631f; continue; }
            pc = 0x48632f; continue;
            }
            0x48632f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x486351; continue; }
            pc = 0x486341; continue;
            }
            0x486341 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x486341; continue; }
            pc = 0x486351; continue;
            }
            0x486351 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x48651f; continue;
            }
            0x48635b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6b1e04u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x48637f);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x48651f; continue;
            }
            0x486387 => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(2,32); c.and(a,0x8000u32,32); }
            if c.cond(6) { pc = 0x4863a0; continue; }
            pc = 0x486393; continue;
            }
            0x486393 => {
            c.set(1,32,rm(m,0x6ab638u32,32));
            c.set(2,32,0x38u32);
            pc = 0x4863ab; continue;
            }
            0x4863a0 => {
            c.set(1,32,rm(m,0x6ab630u32,32));
            c.set(2,32,0x47u32);
            pc = 0x4863ab; continue;
            }
            0x4863ab => {
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,32,c.get(2,32)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4863c9; continue; }
            pc = 0x4863b5; continue;
            }
            0x4863b5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4863b5; continue; }
            pc = 0x4863c5; continue;
            }
            0x4863c5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x32),32));
            pc = 0x4863c9; continue;
            }
            0x4863c9 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0b38),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4863f4; continue; }
            pc = 0x4863e4; continue;
            }
            0x4863e4 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4863e4; continue; }
            pc = 0x4863f4; continue;
            }
            0x4863f4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x9),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let n = 0xfu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a0628),32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48642d; continue; }
            pc = 0x48641d; continue;
            }
            0x48641d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48641d; continue; }
            pc = 0x48642d; continue;
            }
            0x48642d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xb),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m, 0x486446);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x48645c; continue; }
            pc = 0x48644c; continue;
            }
            0x48644c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x48644c; continue; }
            pc = 0x48645c; continue;
            }
            0x48645c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xa),8));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x48647a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x486490; continue; }
            pc = 0x486480; continue;
            }
            0x486480 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x486480; continue; }
            pc = 0x486490; continue;
            }
            0x486490 => {
            c.set(1,32,rm(m,0x6aafc0u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4864ac; continue; }
            pc = 0x48649c; continue;
            }
            0x48649c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48649c; continue; }
            pc = 0x4864ac; continue;
            }
            0x4864ac => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xd),8));
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0da0),32));
            c.set(1,32,rm(m,0x6aafc4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0x3fu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x70));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4864d8);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x78),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,32,c.r[4].wrapping_add(0x64));
            if c.cond(4) { pc = 0x4864f7; continue; }
            pc = 0x4864e7; continue;
            }
            0x4864e7 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4864e7; continue; }
            pc = 0x4864f7; continue;
            }
            0x4864f7 => {
            c.set(1,32,rm(m,0x6aafc8u32,32));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x486513; continue; }
            pc = 0x486503; continue;
            }
            0x486503 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x486503; continue; }
            pc = 0x486513; continue;
            }
            0x486513 => {
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,0x0u32); }
            pc = 0x486518; continue;
            }
            0x486518 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x5),8));
            pc = 0x48651b; continue;
            }
            0x48651b => {
            { let a = c.r[4].wrapping_add(0x1f); wm(m,a,8,c.get(0,8)); }
            pc = 0x48651f; continue;
            }
            0x48651f => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x64),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x48653b; continue; }
            pc = 0x486527; continue;
            }
            0x486527 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            pc = 0x48652b; continue;
            }
            0x48652b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x48652b; continue; }
            pc = 0x48653b; continue;
            }
            0x48653b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,0x0u32); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x4d6c50u32); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xd2),16));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)),16));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.r[4].wrapping_add(0x47c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x478); wm(m,a,32,0x6eu32); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xd2),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),16));
            c.set(1,32,c.r[4].wrapping_add(0x478));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x484); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4865ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m, 0x4865b2);
            { let args = []; c.r[0] = host.call(m, 0x462280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb10u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m, 0x4865c5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4865cf);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x486632; continue; }
            pc = 0x4865da; continue;
            }
            0x4865da => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4865e0);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x486632; continue;
            }
            0x4865e5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4865ef);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x486632; continue; }
            pc = 0x4865fa; continue;
            }
            0x4865fa => {
            c.push(m,c.get(0,32));
            c.push(m, 0x486600);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x486632; continue;
            }
            0x486605 => {
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x486612; continue; }
            pc = 0x486609; continue;
            }
            0x486609 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x48660f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x486612; continue;
            }
            0x486612 => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x486620); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x560),32));
            { let a = c.get(3,32); let r = c.add(a,0x560u32,32); c.set(3,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(3,32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x486632); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x486632; continue;
            }
            0x486632 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x488u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "choice_list_dialog_on_delete_answer" => choice_list_dialog_on_delete_answer(m, host, regs),
        "personal_letters_side_menu_open_selected_letter" => personal_letters_side_menu_open_selected_letter(m, host, regs),
        _ => return None,
    })
}
