//! Generado por `reimpl/tools/ui_rt_grp.py bld` con `tools/lift.py` (no editar a mano): grupo `bld`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_BLD: [(u32, super::core_table::CoreFn); 1] = [
    (0x5afd80, f_005afd80),
];
pub fn f_005afd80(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5afd80;
    loop {
        match pc {
            0x5afd80 => {
            c.set(0,32,rm(m,0x6cbb10u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x5a4),32));
            c.set(0,32,rm(m,0x66bdc0u32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5afe1b; continue; }
            pc = 0x5afd9c; continue;
            }
            0x5afd9c => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x5afda5);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5afdd7; continue; }
            pc = 0x5afdac; continue;
            }
            0x5afdac => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x28u32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            pc = 0x5afe00; continue;
            }
            0x5afdd7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x26u32); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            pc = 0x5afe00; continue;
            }
            0x5afe00 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x5afe0f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m, 0x5afe18);
            { let args = []; c.r[0] = host.call(m, 0x440f70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x5afe1b; continue;
            }
            0x5afe1b => {
            c.push(m,c.get(6,32));
            c.push(m, 0x5afe21);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
