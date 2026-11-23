//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn load_save_screen_load_game_from_cstring(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x433510;
    loop {
        match pc {
            0x433510 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d840u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11bc),32));
            c.set(2,32,0x3u32);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x433561; continue; }
            pc = 0x43353f; continue;
            }
            0x43353f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x11c0),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x43354f; continue; }
            pc = 0x433549; continue;
            }
            0x433549 => {
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(2,8)); }
            pc = 0x433565; continue;
            }
            0x43354f => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x6u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            pc = 0x433565; continue;
            }
            0x433561 => {
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(1,8)); }
            pc = 0x433565; continue;
            }
            0x433565 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x433581);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x433588);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4335b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.set(3,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x43359b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "load_save_screen_load_game_from_cstring" => load_save_screen_load_game_from_cstring(m, host, regs),
        _ => return None,
    })
}
