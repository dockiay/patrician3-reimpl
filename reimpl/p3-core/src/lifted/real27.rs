//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn battle_publish_player_side_ships(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x58a240;
    loop {
        match pc {
            0x58a240 => {
            c.set(0,32,rm(m,0x6e59ccu32,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6e55d0),32));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x35a),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x58a275; continue; }
            pc = 0x58a26a; continue;
            }
            0x58a26a => {
            c.set(6,16,rm(m,c.r[5].wrapping_add(0x658),16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x58a280; continue;
            }
            0x58a275 => {
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x658),16));
            c.set(6,16,rm(m,c.r[5].wrapping_add(0x14),16));
            pc = 0x58a280; continue;
            }
            0x58a280 => {
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(3,32,c.get(6,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(7,32),32); c.set(3,32,r); }
            c.set(2,32,c.r[3].wrapping_add(c.r[3]));
            c.push(m,c.get(2,32));
            c.push(m, 0x58a299);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x58a2c0; continue; }
            pc = 0x58a2a2; continue;
            }
            0x58a2a2 => {
            c.set(2,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            pc = 0x58a2a9; continue;
            }
            0x58a2a9 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            c.set(7,32,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xfffffffc),32));
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.r[2].wrapping_add(0xfffffffe); wm(m,a,16,c.get(7,16)); }
            if c.cond(12) { pc = 0x58a2a9; continue; }
            pc = 0x58a2c0; continue;
            }
            0x58a2c0 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x58a2dd; continue; }
            pc = 0x58a2d4; continue;
            }
            0x58a2d4 => {
            c.set(0,8,rm(m,0x66f352u32,8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            pc = 0x58a2dd; continue;
            }
            0x58a2dd => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x58a2f0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(3,32); c.sub(a,0x1u32,32); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(5) { pc = 0x58a304; continue; }
            pc = 0x58a2f9; continue;
            }
            0x58a2f9 => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.push(m, 0x58a304);
            { let args = []; c.r[0] = host.call(m, 0x495150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x58a304; continue;
            }
            0x58a304 => {
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "battle_publish_player_side_ships" => battle_publish_player_side_ships(m, host, regs),
        _ => return None,
    })
}
