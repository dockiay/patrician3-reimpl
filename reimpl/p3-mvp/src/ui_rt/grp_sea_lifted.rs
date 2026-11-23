//! Generado por `reimpl/tools/ui_rt_grp.py sea` con `tools/lift.py` (no editar a mano): grupo `sea`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_SEA: [(u32, super::core_table::CoreFn); 44] = [
    (0x4d4dd0, f_004d4dd0),
    (0x4d5ee0, f_004d5ee0),
    (0x4f3980, f_004f3980),
    (0x4f39a0, f_004f39a0),
    (0x4f39c0, f_004f39c0),
    (0x4f39e0, f_004f39e0),
    (0x4f3a40, f_004f3a40),
    (0x4f3ad0, f_004f3ad0),
    (0x5029f0, f_005029f0),
    (0x503230, f_00503230),
    (0x5038f0, f_005038f0),
    (0x503930, f_00503930),
    (0x5039f0, f_005039f0),
    (0x505b60, f_00505b60),
    (0x505c30, f_00505c30),
    (0x5062f0, f_005062f0),
    (0x509030, f_00509030),
    (0x50a2b0, f_0050a2b0),
    (0x50a4e0, f_0050a4e0),
    (0x50aae0, f_0050aae0),
    (0x50ab50, f_0050ab50),
    (0x50b030, f_0050b030),
    (0x50cb80, f_0050cb80),
    (0x50cc90, f_0050cc90),
    (0x50d2b0, f_0050d2b0),
    (0x50e280, f_0050e280),
    (0x512fc0, f_00512fc0),
    (0x513a40, f_00513a40),
    (0x5144a0, f_005144a0),
    (0x514530, f_00514530),
    (0x514920, f_00514920),
    (0x514b80, f_00514b80),
    (0x515d00, f_00515d00),
    (0x515d20, f_00515d20),
    (0x515d30, f_00515d30),
    (0x515d90, f_00515d90),
    (0x515ea0, f_00515ea0),
    (0x515f00, f_00515f00),
    (0x517170, f_00517170),
    (0x5176c0, f_005176c0),
    (0x518460, f_00518460),
    (0x518750, f_00518750),
    (0x519b50, f_00519b50),
    (0x515da0, f_00515da0),
];
pub fn f_004d4dd0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d4dd0;
    loop {
        match pc {
            0x4d4dd0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2),16));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            c.set(6,32,c.get(7,32));
            if c.cond(3) { pc = 0x4d4e6f; continue; }
            pc = 0x4d4de9; continue;
            }
            0x4d4de9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4d4e17; continue; }
            pc = 0x4d4df1; continue;
            }
            0x4d4df1 => {
            c.push(m,0xffffu32);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x4d4e01);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x509030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(0,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(6,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4d4df1; continue; }
            pc = 0x4d4e17; continue;
            }
            0x4d4e17 => {
            c.set(1,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,c.get(7,32));
            c.set(1,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(6,16,rm(m,c.r[2],16));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4d4e57; continue; }
            pc = 0x4d4e33; continue;
            }
            0x4d4e33 => {
            { let a = c.r[2].wrapping_add(0x3); wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1]),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(6,16,rm(m,c.r[2],16));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4d4e33; continue; }
            pc = 0x4d4e57; continue;
            }
            0x4d4e57 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,16,rm(m,c.r[5],16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[5]; wm(m,a,16,c.get(7,16)); }
            pc = 0x4d4e6f; continue;
            }
            0x4d4e6f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004d5ee0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d5ee0;
    loop {
        match pc {
            0x4d5ee0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6629b4u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x4d5f04);
            { let args = []; c.r[0] = host.call(m, 0x54c3f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,0x1u32);
            c.push(m,0x6c0180u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4d5f31);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4d5f3f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6c0190u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4d5f4d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4d5f68);
            { let args = []; c.r[0] = host.call(m, 0x6389ee, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4d5fc7; continue; }
            pc = 0x4d5f71; continue;
            }
            0x4d5f71 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4d5f86);
            { let args = []; c.r[0] = host.call(m, 0x6389ee, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4d5fc3; continue; }
            pc = 0x4d5f8f; continue;
            }
            0x4d5f8f => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4d5f9c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4d5fad);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4d5fc3 => {
            { let a = c.r[4].wrapping_add(0xf); wm(m,a,8,c.get(3,8)); }
            pc = 0x4d5fc7; continue;
            }
            0x4d5fc7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0xdcu32);
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); let r = c.and(a,0x3fffffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let b = c.get(1,32); c.div32(b); }
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4d6056; continue; }
            pc = 0x4d5fe4; continue;
            }
            0x4d5fe4 => {
            c.set(6,32,c.get(0,32));
            c.set(2,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4d5ff8);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4d6056; continue; }
            pc = 0x4d6001; continue;
            }
            0x4d6001 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x4d6010);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x54c480, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[4].wrapping_add(0x18),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4d604b; continue; }
            pc = 0x4d6016; continue;
            }
            0x4d6016 => {
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            c.set(2,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,16,c.get(3,16)); }
            c.set(0,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x4d6056; continue; }
            pc = 0x4d602c; continue;
            }
            0x4d602c => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(0x1));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(4)); wm(m,a,16,c.get(1,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4d602c; continue; }
            pc = 0x4d6049; continue;
            }
            0x4d6049 => {
            pc = 0x4d6056; continue;
            }
            0x4d604b => {
            c.push(m,c.get(7,32));
            c.push(m, 0x4d6051);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4d6056; continue;
            }
            0x4d6056 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            c.push(m,c.get(5,32));
            if c.cond(4) { pc = 0x4d6069; continue; }
            pc = 0x4d6060; continue;
            }
            0x4d6060 => {
            c.set(1,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(1,32));
            pc = 0x4d606a; continue;
            }
            0x4d6069 => {
            c.push(m,c.get(3,32));
            pc = 0x4d606a; continue;
            }
            0x4d606a => {
            c.push(m, 0x4d606f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6389f4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4d607f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4d6090);
            { let args = []; c.r[0] = host.call(m, 0x54c400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3980(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3980;
    loop {
        match pc {
            0x4f3980 => {
            c.set(0,32,c.get(1,32));
            c.set(1,8,0xffu32);
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[0].wrapping_add(0x5); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0xffffu32); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f39a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f39a0;
    loop {
        match pc {
            0x4f39a0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1],32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4f39bd; continue; }
            pc = 0x4f39a7; continue;
            }
            0x4f39a7 => {
            { let a = c.r[1]; wm(m,a,32,0x0u32); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4f39b4);
            { let args = []; c.r[0] = host.call(m, 0x4f39a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x4f39ba);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4f39bd; continue;
            }
            0x4f39bd => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f39c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f39c0;
    loop {
        match pc {
            0x4f39c0 => {
            c.set(1,32,rm(m,c.r[1],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4f39cf; continue; }
            pc = 0x4f39c6; continue;
            }
            0x4f39c6 => {
            c.push(m, 0x4f39cb);
            { let args = []; c.r[0] = host.call(m, 0x4f39c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,0x5u32,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4f39cf => {
            c.set(0,32,0x5u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f39e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f39e0;
    loop {
        match pc {
            0x4f39e0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x4),8));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x5),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x6),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,8); let n = 0x8u32; let r = c.sar(a,n,8); c.set(2,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x6),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4f3a2c; continue; }
            pc = 0x4f3a05; continue;
            }
            0x4f3a05 => {
            { let a = c.r[0]; wm(m,a,8,0x7fu32); }
            c.set(1,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x4),8));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x5),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x6),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,8); let n = 0x8u32; let r = c.sar(a,n,8); c.set(2,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x6),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4f3a05; continue; }
            pc = 0x4f3a2c; continue;
            }
            0x4f3a2c => {
            { let a = c.r[0]; wm(m,a,8,0x0u32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3a40(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3a40;
    loop {
        match pc {
            0x4f3a40 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[7].wrapping_add(0x5); wm(m,a,8,c.get(1,8)); }
            c.set(0,16,rm(m,c.r[6],8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,16,c.get(2,16)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,0x7fu32,8); }
            if c.cond(5) { pc = 0x4f3abd; continue; }
            pc = 0x4f3a6c; continue;
            }
            0x4f3a6c => {
            c.push(m,c.get(3,32));
            c.set(3,8,0xffu32);
            pc = 0x4f3a6f; continue;
            }
            0x4f3a6f => {
            c.push(m,0x8u32);
            c.push(m, 0x4f3a76);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4f3a91; continue; }
            pc = 0x4f3a7d; continue;
            }
            0x4f3a7d => {
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[0].wrapping_add(0x5); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0xffffu32); }
            pc = 0x4f3a93; continue;
            }
            0x4f3a91 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4f3a93; continue;
            }
            0x4f3a93 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.set(7,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(0x5); wm(m,a,8,c.get(2,8)); }
            c.set(1,16,rm(m,c.r[6],8));
            c.set(2,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,c.get(2,16)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,0x7fu32,8); }
            if c.cond(4) { pc = 0x4f3a6f; continue; }
            pc = 0x4f3abc; continue;
            }
            0x4f3abc => {
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x4f3abd; continue;
            }
            0x4f3abd => {
            c.set(0,32,c.r[6].wrapping_add(0x1));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004f3ad0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4f3ad0;
    loop {
        match pc {
            0x4f3ad0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x8),8));
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x4),8));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,8,c.get(2,8)); }
            c.set(2,16,rm(m,c.r[4].wrapping_add(0xc),16));
            { let a = c.r[1].wrapping_add(0x5); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[1].wrapping_add(0x6); wm(m,a,16,c.get(2,16)); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005029f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5029f0;
    loop {
        match pc {
            0x5029f0 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x502a04);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x12u32); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x10),16));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x8),16));
            { let a = c.r[0].wrapping_add(0xa); wm(m,a,16,0xffffu32); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,0x13u32); }
            c.push(m, 0x502a4b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x502a72; continue; }
            pc = 0x502a55; continue;
            }
            0x502a55 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x94),32));
            c.set(0,32,rm(m,0x66d504u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x502a72; continue; }
            pc = 0x502a64; continue;
            }
            0x502a64 => {
            c.push(m,0x0u32);
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x502a72);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4885a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x502a72; continue;
            }
            0x502a72 => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x502aca; continue; }
            pc = 0x502a82; continue;
            }
            0x502a82 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x502a92);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x502aca; continue; }
            pc = 0x502a99; continue;
            }
            0x502a99 => {
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); c.sub(a,0x9c40u32,32); }
            if c.cond(14) { pc = 0x502ac3; continue; }
            pc = 0x502aa3; continue;
            }
            0x502aa3 => {
            { let a = c.get(1,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x186a0u32,32); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x502ab8; continue; }
            pc = 0x502ab1; continue;
            }
            0x502ab1 => {
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,0x186a0u32); }
            pc = 0x502ab8; continue;
            }
            0x502ab8 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x502aca; continue;
            }
            0x502ac3 => {
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,0x2710u32); }
            pc = 0x502aca; continue;
            }
            0x502aca => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(3) { pc = 0x502b17; continue; }
            pc = 0x502adf; continue;
            }
            0x502adf => {
            c.set(2,32,0x7530u32);
            pc = 0x502ae4; continue;
            }
            0x502ae4 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.r[0].wrapping_add(0x1e); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[0].wrapping_add(0x22); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[0].wrapping_add(0x134); wm(m,a,16,0x13u32); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x502ae4; continue; }
            pc = 0x502b17; continue;
            }
            0x502b17 => {
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00503230(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x503230;
    loop {
        match pc {
            0x503230 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662fc2u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x98u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x50325e);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x10),16));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0xb0); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x40u32); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x132),16));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd72cu32,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x3),8));
            c.set(7,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,8,c.get(1,8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,8); let r = c.and(a,0x4u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x4u32,8); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x1c),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(5) { pc = 0x5036d7; continue; }
            pc = 0x5032d3; continue;
            }
            0x5032d3 => {
            { let a = c.get(1,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x50334d; continue; }
            pc = 0x5032d8; continue;
            }
            0x5032d8 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x76c),8); c.and(a,0x40u32,8); }
            if c.cond(5) { pc = 0x5032fb; continue; }
            pc = 0x5032f4; continue;
            }
            0x5032f4 => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[7].wrapping_add(0x3); wm(m,a,8,c.get(1,8)); }
            pc = 0x50334d; continue;
            }
            0x5032fb => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xa),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x50334d; continue; }
            pc = 0x503311; continue;
            }
            0x503311 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x18),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x503337; continue; }
            pc = 0x503331; continue;
            }
            0x503331 => {
            { let a = c.r[3].wrapping_add(0x12); wm(m,a,16,0x4u32); }
            pc = 0x503337; continue;
            }
            0x503337 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x6),16));
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x503311; continue; }
            pc = 0x50334d; continue;
            }
            0x50334d => {
            { let a = rm(m,c.r[3].wrapping_add(0x12),16); c.sub(a,0x4u32,16); }
            if c.cond(5) { pc = 0x503386; continue; }
            pc = 0x503354; continue;
            }
            0x503354 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x1c),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x503370);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x52acd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x503380; continue; }
            pc = 0x503374; continue;
            }
            0x503374 => {
            { let a = c.r[6].wrapping_add(0x136); wm(m,a,8,0x2u32); }
            pc = 0x5038c0; continue;
            }
            0x503380 => {
            { let a = c.r[3].wrapping_add(0x12); wm(m,a,16,0x0u32); }
            pc = 0x503386; continue;
            }
            0x503386 => {
            c.set(1,32,0x6dd728u32);
            c.push(m, 0x503390);
            { let args = []; c.r[0] = host.call(m, 0x4d4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.push(m,0xdcu32);
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            c.set(0,32,c.r[5].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd72cu32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5033c3);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x44),16));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2),8));
            { let a = c.r[0].wrapping_add(0x3); wm(m,a,8,0x0u32); }
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            c.set(0,16,rm(m,c.r[3],8));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5033ed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[3].wrapping_add(0xa),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x50354d; continue; }
            pc = 0x50340b; continue;
            }
            0x50340b => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,8,rm(m,c.r[3],8));
            c.set(5,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(5,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            c.set(2,8,rm(m,c.r[5],8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x50352f; continue; }
            pc = 0x503426; continue;
            }
            0x503426 => {
            c.set(0,32,c.r[5].wrapping_add(0x54));
            c.push(m,0x60u32);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x50));
            c.push(m,c.get(0,32));
            c.push(m, 0x503436);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x44),16));
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x132),16));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x42),16));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0x132); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(1,16)); }
            if c.cond(2) { pc = 0x503466; continue; }
            pc = 0x50345e; continue;
            }
            0x50345e => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.r[5].wrapping_add(0x42); wm(m,a,16,c.get(1,16)); }
            pc = 0x503466; continue;
            }
            0x503466 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(1,32,0x6dd728u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x503476);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d5200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x10),16));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x50348e; continue; }
            pc = 0x503480; continue;
            }
            0x503480 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x40),16));
            { let a = c.r[5].wrapping_add(0x132); wm(m,a,16,c.get(1,16)); }
            pc = 0x5034aa; continue;
            }
            0x50348e => {
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x14),16));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x42),16));
            { let a = c.r[5].wrapping_add(0x132); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x42),16),16); }
            if c.cond(5) { pc = 0x5034aa; continue; }
            pc = 0x5034a4; continue;
            }
            0x5034a4 => {
            { let a = c.r[5].wrapping_add(0x42); wm(m,a,16,0xffffu32); }
            pc = 0x5034aa; continue;
            }
            0x5034aa => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[1].wrapping_add(0x7c));
            c.set(1,32,c.r[5].wrapping_add(0x54));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x5034ba; continue;
            }
            0x5034ba => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0x48),32));
            c.set(2,32,rm(m,c.r[2],32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5034e0; continue; }
            pc = 0x5034c8; continue;
            }
            0x5034c8 => {
            { let a = rm(m,c.r[0].wrapping_add(0xffffffa0),32); c.sub(a,0x0u32,32); }
            if c.cond(15) { pc = 0x5034d8; continue; }
            pc = 0x5034ce; continue;
            }
            0x5034ce => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            pc = 0x5034e0; continue;
            }
            0x5034d8 => {
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x5034e0; continue;
            }
            0x5034e0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,0x18u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x5034ba; continue; }
            pc = 0x5034f4; continue;
            }
            0x5034f4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x503516; continue; }
            pc = 0x5034fc; continue;
            }
            0x5034fc => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x124),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0x124); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            pc = 0x503516; continue;
            }
            0x503516 => {
            { let a = rm(m,c.r[5].wrapping_add(0x3f),8); c.sub(a,0x2u32,8); }
            if c.cond(13) { pc = 0x50352f; continue; }
            pc = 0x50351c; continue;
            }
            0x50351c => {
            c.set(0,16,0x500u32);
            { let a = c.get(0,16); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0x3e),16),16); c.set(0,16,r); }
            { let a = rm(m,c.r[4].wrapping_add(0x18),16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x50352f; continue; }
            pc = 0x50352b; continue;
            }
            0x50352b => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x50352f; continue;
            }
            0x50352f => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x6),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x50340b; continue; }
            pc = 0x503549; continue;
            }
            0x503549 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x50354d; continue;
            }
            0x50354d => {
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            c.set(1,32,0x6dd728u32);
            c.push(m, 0x50355a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d4dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x50355a; continue;
            }
            0x50355a => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[3].wrapping_add(0x10),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5035d2; continue; }
            pc = 0x503576; continue;
            }
            0x503576 => {
            c.set(2,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x42),16));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x7u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x39),8));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2d); wm(m,a,8,c.get(1,8)); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x15e),16));
            { let a = c.get(6,8); let r = c.or(a,0x80u32,8); c.set(6,8,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,16,c.get(2,16)); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2),8));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x3),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[3],8));
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x5035d2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5035d2; continue;
            }
            0x5035d2 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(1,32,c.get(7,32));
            c.set(6,16,rm(m,c.r[7].wrapping_add(0x132),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x5035e6);
            { let args = []; c.r[0] = host.call(m, 0x519ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x503866; continue; }
            pc = 0x5035ee; continue;
            }
            0x5035ee => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x132),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd72cu32,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),16));
            { let a = c.r[7].wrapping_add(0x132); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x5037e2; continue; }
            pc = 0x50361d; continue;
            }
            0x50361d => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[3].wrapping_add(0xa),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5035ee; continue; }
            pc = 0x503633; continue;
            }
            0x503633 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,8,rm(m,c.r[3],8));
            c.set(6,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x50380f; continue; }
            pc = 0x50364e; continue;
            }
            0x50364e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x10),16));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x503677; continue; }
            pc = 0x503658; continue;
            }
            0x503658 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x132),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd728u32);
            c.push(m, 0x50366e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d4dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[6].wrapping_add(0x132); wm(m,a,16,0xffffu32); }
            pc = 0x503677; continue;
            }
            0x503677 => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x132),16));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6dd728u32);
            { let a = c.r[6].wrapping_add(0x132); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x503690);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d5ab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x10),16));
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5036ac; continue; }
            pc = 0x5036a3; continue;
            }
            0x5036a3 => {
            { let a = c.r[6].wrapping_add(0x132); wm(m,a,16,0xffffu32); }
            pc = 0x5036ac; continue;
            }
            0x5036ac => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x50380f; continue; }
            pc = 0x5036b4; continue;
            }
            0x5036b4 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x503633; continue; }
            pc = 0x5036ce; continue;
            }
            0x5036ce => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x5035ee; continue;
            }
            0x5036d7 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x5036e2);
            { let args = []; c.r[0] = host.call(m, 0x519ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x503890; continue; }
            pc = 0x5036ea; continue;
            }
            0x5036ea => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[3].wrapping_add(0xa),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x50355a; continue; }
            pc = 0x503704; continue;
            }
            0x503704 => {
            pc = 0x50370a; continue;
            }
            0x503706 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x50370a; continue;
            }
            0x50370a => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,8,rm(m,c.r[3],8));
            c.set(7,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5037a6; continue; }
            pc = 0x503725; continue;
            }
            0x503725 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x10),16));
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x50374e; continue; }
            pc = 0x50372f; continue;
            }
            0x50372f => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x0u32);
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x132),16));
            c.set(1,32,0x6dd728u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x503745);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d4dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[7].wrapping_add(0x132); wm(m,a,16,0xffffu32); }
            pc = 0x50374e; continue;
            }
            0x50374e => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x132),16));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6dd728u32);
            { let a = c.r[7].wrapping_add(0x132); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x503767);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d5ab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x10),16));
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x50377f; continue; }
            pc = 0x503776; continue;
            }
            0x503776 => {
            { let a = c.r[7].wrapping_add(0x132); wm(m,a,16,0xffffu32); }
            pc = 0x50377f; continue;
            }
            0x50377f => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5037a2; continue; }
            pc = 0x503783; continue;
            }
            0x503783 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x503706; continue; }
            pc = 0x50379d; continue;
            }
            0x50379d => {
            pc = 0x50355a; continue;
            }
            0x5037a2 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5037a6; continue;
            }
            0x5037a6 => {
            c.set(0,32,0x1u32);
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x132),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd72cu32,32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x2),8));
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x13b); wm(m,a,8,c.get(0,8)); }
            pc = 0x5038c0; continue;
            }
            0x5037e2 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x136),8));
            { let a = c.get(1,8); let r = c.and(a,0xfcu32,8); c.set(1,8,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x3d),8); let r = c.or(a,0x8u32,8); { let a = c.r[7].wrapping_add(0x3d); wm(m,a,8,r); } }
            c.set(2,16,rm(m,c.r[7],8));
            { let a = c.r[7].wrapping_add(0x136); wm(m,a,8,c.get(1,8)); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x10),16));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x39),8));
            c.push(m,c.get(0,32));
            c.push(m,0x25u32);
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            pc = 0x5038b5; continue;
            }
            0x50380f => {
            c.set(0,32,0x1u32);
            { let a = c.r[3].wrapping_add(0x12); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0x134); wm(m,a,16,c.get(0,16)); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x18),16));
            { let a = c.r[7].wrapping_add(0x138); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x132),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd72cu32,32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x2),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(0x38); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[7].wrapping_add(0x13b); wm(m,a,8,c.get(0,8)); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x10),16));
            c.set(2,8,c.get(0,8));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x503864);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5172a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x5038c0; continue;
            }
            0x503866 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x136),8));
            { let a = c.get(1,8); let r = c.and(a,0xfcu32,8); c.set(1,8,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x3d),8); let r = c.or(a,0x8u32,8); { let a = c.r[7].wrapping_add(0x3d); wm(m,a,8,r); } }
            c.set(2,16,rm(m,c.r[7],8));
            { let a = c.r[7].wrapping_add(0x136); wm(m,a,8,c.get(1,8)); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x10),16));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x39),8));
            c.push(m,c.get(0,32));
            c.push(m,0x22u32);
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            pc = 0x5038b5; continue;
            }
            0x503890 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x136),8));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.get(0,8); let r = c.and(a,0xfcu32,8); c.set(0,8,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x3d),8); let r = c.or(a,0x8u32,8); { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,r); } }
            c.set(2,16,rm(m,c.r[6],8));
            { let a = c.r[6].wrapping_add(0x136); wm(m,a,8,c.get(0,8)); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x10),16));
            c.push(m,c.get(0,32));
            c.push(m,0x22u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(5,32));
            pc = 0x5038b5; continue;
            }
            0x5038b5 => {
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5038c0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x548ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            pc = 0x5038c0; continue;
            }
            0x5038c0 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0xb0); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5038d4);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xa8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0xa4u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005038f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5038f0;
    loop {
        match pc {
            0x5038f0 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xa),16));
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x503926; continue; }
            pc = 0x503908; continue;
            }
            0x503908 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            c.push(m,c.get(7,32));
            pc = 0x503910; continue;
            }
            0x503910 => {
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(7,16,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x6),16));
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x503910; continue; }
            pc = 0x503924; continue;
            }
            0x503924 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x503926; continue;
            }
            0x503926 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00503930(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x503930;
    loop {
        match pc {
            0x503930 => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5039e3; continue; }
            pc = 0x503949; continue;
            }
            0x503949 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(5,32,0x80u32);
            pc = 0x503953; continue;
            }
            0x503953 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(6,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(0,32,0x9u32);
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x503981; continue; }
            pc = 0x50396f; continue;
            }
            0x50396f => {
            { let a = rm(m,c.r[6].wrapping_add(0x3c),8); let r = c.or(a,0x20u32,8); { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,r); } }
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,16,c.get(3,16)); }
            pc = 0x503981; continue;
            }
            0x503981 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x138),16));
            { let a = c.get(0,16); c.sub(a,0x20u32,16); }
            if c.cond(3) { pc = 0x5039c6; continue; }
            pc = 0x50398e; continue;
            }
            0x50398e => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,16); c.sub(a,c.get(3,16),16); }
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,16,c.get(0,16)); }
            if c.cond(5) { pc = 0x5039b9; continue; }
            pc = 0x50399b; continue;
            }
            0x50399b => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5039a2);
            { let args = []; c.r[0] = host.call(m, 0x5170f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x3c),16); let r = c.or(a,c.get(5,16),16); { let a = c.r[6].wrapping_add(0x3c); wm(m,a,16,r); } }
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,0x6e6220u32);
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5039b7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6135c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5039c6; continue;
            }
            0x5039b9 => {
            { let a = c.get(0,16); c.sub(a,0x20u32,16); }
            if c.cond(5) { pc = 0x5039c6; continue; }
            pc = 0x5039bf; continue;
            }
            0x5039bf => {
            { let a = rm(m,c.r[6].wrapping_add(0x3c),16); let r = c.or(a,c.get(5,16),16); { let a = c.r[6].wrapping_add(0x3c); wm(m,a,16,r); } }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            pc = 0x5039c6; continue;
            }
            0x5039c6 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x503953; continue; }
            pc = 0x5039e0; continue;
            }
            0x5039e0 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x5039e3; continue;
            }
            0x5039e3 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005039f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5039f0;
    loop {
        match pc {
            0x5039f0 => {
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x1c),16));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x503ab3; continue; }
            pc = 0x503a06; continue;
            }
            0x503a06 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x12),16));
            { let a = c.get(0,16); c.sub(a,0x9u32,16); }
            if c.cond(4) { pc = 0x503a1a; continue; }
            pc = 0x503a10; continue;
            }
            0x503a10 => {
            { let a = c.get(0,16); c.sub(a,0xau32,16); }
            if c.cond(5) { pc = 0x503ab3; continue; }
            pc = 0x503a1a; continue;
            }
            0x503a1a => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x2),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd748u32,16),16); }
            if c.cond(3) { pc = 0x503ab3; continue; }
            pc = 0x503a2b; continue;
            }
            0x503a2b => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(6,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x503ab3; continue; }
            pc = 0x503a38; continue;
            }
            0x503a38 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x503a48);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(7,16); c.sub(a,0x34u32,16); }
            if c.cond(4) { pc = 0x503a5e; continue; }
            pc = 0x503a52; continue;
            }
            0x503a52 => {
            { let a = c.get(7,16); c.sub(a,0x23u32,16); }
            if c.cond(4) { pc = 0x503a5e; continue; }
            pc = 0x503a58; continue;
            }
            0x503a58 => {
            { let a = c.get(7,16); c.sub(a,0x24u32,16); }
            if c.cond(5) { pc = 0x503ab3; continue; }
            pc = 0x503a5e; continue;
            }
            0x503a5e => {
            { let a = rm(m,c.r[0].wrapping_add(0x15),8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x503ab3; continue; }
            pc = 0x503a64; continue;
            }
            0x503a64 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(3,32));
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x14),8));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x39),8));
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(5) { pc = 0x503ab3; continue; }
            pc = 0x503a86; continue;
            }
            0x503a86 => {
            c.set(0,32,0x1u32);
            { let a = rm(m,c.r[1].wrapping_add(0x134),16); c.sub(a,c.get(0,16),16); }
            if c.cond(7) { pc = 0x503ab3; continue; }
            pc = 0x503a94; continue;
            }
            0x503a94 => {
            { let a = c.get(7,16); c.sub(a,0x34u32,16); }
            if c.cond(5) { pc = 0x503ab5; continue; }
            pc = 0x503a9a; continue;
            }
            0x503a9a => {
            c.set(2,32,rm(m,0x672cc8u32,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x120),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x503ab5; continue; }
            pc = 0x503ab3; continue;
            }
            0x503ab3 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x503ab5; continue;
            }
            0x503ab5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00505b60(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x505b60;
    loop {
        match pc {
            0x505b60 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,c.get(1,32));
            c.set(3,32,c.get(7,32));
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m, 0x505b78);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.r[6].wrapping_add(0xea));
            c.set(1,32,c.get(2,32));
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(4) { pc = 0x505bae; continue; }
            pc = 0x505b88; continue;
            }
            0x505b88 => {
            c.set(5,16,rm(m,c.r[6].wrapping_add(0xf4),16));
            pc = 0x505b8f; continue;
            }
            0x505b8f => {
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x505bae; continue; }
            pc = 0x505b94; continue;
            }
            0x505b94 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x6));
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(5) { pc = 0x505b8f; continue; }
            pc = 0x505bae; continue;
            }
            0x505bae => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x6),16));
            { let a = c.r[1]; wm(m,a,16,c.get(5,16)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(5,32,0xffffu32);
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0x6); wm(m,a,16,c.get(5,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2],16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xf4),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x505c10; continue; }
            pc = 0x505bde; continue;
            }
            0x505bde => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x50),16); c.sub(a,c.get(7,16),16); }
            if c.cond(5) { pc = 0x505bf3; continue; }
            pc = 0x505bef; continue;
            }
            0x505bef => {
            { let a = c.r[1].wrapping_add(0x50); wm(m,a,16,c.get(5,16)); }
            pc = 0x505bf3; continue;
            }
            0x505bf3 => {
            { let a = rm(m,c.r[1].wrapping_add(0x52),16); c.sub(a,c.get(7,16),16); }
            if c.cond(5) { pc = 0x505bfd; continue; }
            pc = 0x505bf9; continue;
            }
            0x505bf9 => {
            { let a = c.r[1].wrapping_add(0x52); wm(m,a,16,c.get(5,16)); }
            pc = 0x505bfd; continue;
            }
            0x505bfd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x6),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xf4),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x505bde; continue; }
            pc = 0x505c10; continue;
            }
            0x505c10 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x505c1c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x44f940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00505c30(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x505c30;
    loop {
        match pc {
            0x505c30 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xea),16));
            c.set(7,32,c.get(3,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x505c51);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x50ce50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m, 0x505c61);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xea),16));
            { let a = c.r[1].wrapping_add(c.r[7]).wrapping_add(0x6); wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(c.r[7]).wrapping_add(0x39); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0xea); wm(m,a,16,c.get(3,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005062f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5062f0;
    loop {
        match pc {
            0x5062f0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0xf6),16),16); }
            if c.cond(2) { pc = 0x50630b; continue; }
            pc = 0x506306; continue;
            }
            0x506306 => {
            c.push(m, 0x50630b);
            { let args = []; c.r[0] = host.call(m, 0x509ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x50630b; continue;
            }
            0x50630b => {
            c.set(5,16,rm(m,c.r[7].wrapping_add(0xe4),16));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(0,32,c.get(5,32));
            c.push(m,0x2u32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x8),16));
            { let a = c.r[7].wrapping_add(0xe4); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(6,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,16,rm(m,c.r[0],16));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[0]; wm(m,a,16,c.get(5,16)); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xa); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,c.get(6,32));
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.r[6]; wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xe),8));
            { let a = c.r[6].wrapping_add(0x1); wm(m,a,8,c.get(2,8)); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,16,c.get(2,16)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x37),8));
            { let a = c.r[6].wrapping_add(0x1e); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x38),8));
            { let a = c.r[6].wrapping_add(0x1f); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(4,8); let r = c.or(a,0x4u32,8); c.set(4,8,r); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.r[3].wrapping_add(c.r[2]).wrapping_add(0x8); wm(m,a,16,c.get(5,16)); }
            c.push(m, 0x506399);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x501680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(c.r[0]).wrapping_add(0x134),16));
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,0x100u32); }
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,c.get(1,16)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00509030(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x509030;
    loop {
        match pc {
            0x509030 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0xf4),16); c.sub(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x50906e; continue; }
            pc = 0x50903e; continue;
            }
            0x50903e => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,16,rm(m,c.r[4].wrapping_add(0x10),16));
            c.push(m,c.get(7,32));
            c.set(7,16,rm(m,c.r[4].wrapping_add(0x10),16));
            { let a = c.get(0,32); let r = c.add(a,0x132u32,32); c.set(0,32,r); }
            pc = 0x509050; continue;
            }
            0x509050 => {
            { let a = rm(m,c.r[0],16); c.sub(a,c.get(7,16),16); }
            if c.cond(5) { pc = 0x509058; continue; }
            pc = 0x509055; continue;
            }
            0x509055 => {
            { let a = c.r[0]; wm(m,a,16,c.get(6,16)); }
            pc = 0x509058; continue;
            }
            0x509058 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(3,16,rm(m,c.r[1].wrapping_add(0xf4),16));
            { let a = c.get(0,32); let r = c.add(a,0x180u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x509050; continue; }
            pc = 0x50906b; continue;
            }
            0x50906b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x50906e; continue;
            }
            0x50906e => {
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050a2b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50a2b0;
    loop {
        match pc {
            0x50a2b0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(3,32,rm(m,0x6dfc14u32,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xf6),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x50a2d6; continue; }
            pc = 0x50a2ce; continue;
            }
            0x50a2ce => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x50a2d6 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xf4),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x50a329; continue; }
            pc = 0x50a2f3; continue;
            }
            0x50a2f3 => {
            c.push(m,c.get(6,32));
            pc = 0x50a2f4; continue;
            }
            0x50a2f4 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x50a315; continue; }
            pc = 0x50a307; continue;
            }
            0x50a307 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x50a313);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x518460, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            pc = 0x50a315; continue;
            }
            0x50a315 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xf4),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x50a2f4; continue; }
            pc = 0x50a328; continue;
            }
            0x50a328 => {
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x50a329; continue;
            }
            0x50a329 => {
            c.set(0,32,c.get(5,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050a4e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50a4e0;
    loop {
        match pc {
            0x50a4e0 => {
            c.set(2,32,c.r[1].wrapping_add(0xec));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,16,rm(m,c.r[2],16));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x50a524; continue; }
            pc = 0x50a4fb; continue;
            }
            0x50a4fb => {
            c.set(3,16,rm(m,c.r[1].wrapping_add(0xf6),16));
            pc = 0x50a502; continue;
            }
            0x50a502 => {
            { let a = c.get(6,16); c.sub(a,c.get(3,16),16); }
            if c.cond(3) { pc = 0x50a56f; continue; }
            pc = 0x50a507; continue;
            }
            0x50a507 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),16));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8));
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x50a502; continue; }
            pc = 0x50a524; continue;
            }
            0x50a524 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.r[2]; wm(m,a,16,c.get(0,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,0xffffu32); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(0xf4),16),16); }
            if c.cond(3) { pc = 0x50a567; continue; }
            pc = 0x50a54a; continue;
            }
            0x50a54a => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x50a555);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x10),16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x50a567);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x44f940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x50a567; continue;
            }
            0x50a567 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x50a56f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050aae0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50aae0;
    loop {
        match pc {
            0x50aae0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.get(1,32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xec),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.r[0].wrapping_add(c.r[7]).wrapping_add(0x8); wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0xec); wm(m,a,16,c.get(1,16)); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x10),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[7]));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xf4),16),16); }
            if c.cond(2) { pc = 0x50ab27; continue; }
            pc = 0x50ab22; continue;
            }
            0x50ab22 => {
            c.push(m, 0x50ab27);
            { let args = []; c.r[0] = host.call(m, 0x5017f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x50ab27; continue;
            }
            0x50ab27 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,0xffffu32);
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x10),16));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x50ab3e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x50ce50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050ab50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50ab50;
    loop {
        match pc {
            0x50ab50 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6dd7a8u32,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(6,16,rm(m,c.r[1].wrapping_add(0xec),16));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,16,c.get(6,16)); }
            c.set(6,16,rm(m,c.r[4].wrapping_add(0xc),16));
            { let a = c.r[1].wrapping_add(0xec); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xa),16));
            { let a = c.r[2].wrapping_add(0x12); wm(m,a,16,c.get(6,16)); }
            { let a = c.r[2].wrapping_add(0x16); wm(m,a,16,0x0u32); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xf4),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x50abd6; continue; }
            pc = 0x50ab98; continue;
            }
            0x50ab98 => {
            c.set(2,32,0x20u32);
            c.push(m,c.get(3,32));
            pc = 0x50ab9e; continue;
            }
            0x50ab9e => {
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = rm(m,c.r[0].wrapping_add(0x3c),16); let r = c.or(a,c.get(2,16),16); { let a = c.r[0].wrapping_add(0x3c); wm(m,a,16,r); } }
            { let a = c.r[0].wrapping_add(0x134); wm(m,a,16,c.get(6,16)); }
            { let a = c.r[0].wrapping_add(0x138); wm(m,a,16,0x0u32); }
            c.set(0,32,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[1].wrapping_add(0xf4),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x50ab9e; continue; }
            pc = 0x50abd5; continue;
            }
            0x50abd5 => {
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x50abd6; continue;
            }
            0x50abd6 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050b030(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50b030;
    loop {
        match pc {
            0x50b030 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(5,16,rm(m,c.r[0].wrapping_add(0xf4),16));
            c.set(6,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x3a),16));
            { let a = c.get(3,16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x50b06a; continue; }
            pc = 0x50b05a; continue;
            }
            0x50b05a => {
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,0xffffu32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x50b06a => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0x16u32,16); }
            if c.cond(4) { pc = 0x50b093; continue; }
            pc = 0x50b077; continue;
            }
            0x50b077 => {
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x50b093; continue; }
            pc = 0x50b07d; continue;
            }
            0x50b07d => {
            { let a = c.get(0,16); c.sub(a,0x10u32,16); }
            if c.cond(4) { pc = 0x50b093; continue; }
            pc = 0x50b083; continue;
            }
            0x50b083 => {
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,0xffffu32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x50b093 => {
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x15c),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x50b0fd; continue; }
            pc = 0x50b0ab; continue;
            }
            0x50b0ab => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.get(2,32); c.sub(a,rm(m,0x6de4b0u32,32),32); }
            if c.cond(3) { pc = 0x50b0d0; continue; }
            pc = 0x50b0b8; continue;
            }
            0x50b0b8 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,0xffffu32); }
            c.push(m, 0x50b0c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x516840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x50b0d0 => {
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x50b0fd; continue; }
            pc = 0x50b0d4; continue;
            }
            0x50b0d4 => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(2,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x50b12a; continue; }
            pc = 0x50b0e1; continue;
            }
            0x50b0e1 => {
            { let a = c.get(2,16); c.sub(a,0x10u32,16); }
            if c.cond(4) { pc = 0x50b12a; continue; }
            pc = 0x50b0e7; continue;
            }
            0x50b0e7 => {
            { let a = c.get(2,16); c.sub(a,0x12u32,16); }
            if c.cond(4) { pc = 0x50b130; continue; }
            pc = 0x50b0ed; continue;
            }
            0x50b0ed => {
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,0xffffu32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x50b0fd => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(2,16); c.sub(a,0x12u32,16); }
            if c.cond(4) { pc = 0x50b130; continue; }
            pc = 0x50b10a; continue;
            }
            0x50b10a => {
            { let a = c.get(2,16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x50b11a; continue; }
            pc = 0x50b110; continue;
            }
            0x50b110 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x15c),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x50b12a; continue; }
            pc = 0x50b11a; continue;
            }
            0x50b11a => {
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,0xffffu32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x50b12a => {
            { let a = c.get(2,16); c.sub(a,0x12u32,16); }
            if c.cond(5) { pc = 0x50b14c; continue; }
            pc = 0x50b130; continue;
            }
            0x50b130 => {
            { let a = rm(m,c.r[0].wrapping_add(0xa),16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x50b14c; continue; }
            pc = 0x50b136; continue;
            }
            0x50b136 => {
            { let a = rm(m,c.r[0].wrapping_add(0xc),16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x50b14c; continue; }
            pc = 0x50b13c; continue;
            }
            0x50b13c => {
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,0xffffu32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x50b14c => {
            c.set(5,32,sx(rm(m,c.r[6].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(7,32,sx(rm(m,c.r[0].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(7,32); let r = c.sub(a,c.get(5,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6dd89cu32,32));
            c.set(7,32,0xffffu32);
            { let a = c.get(1,32); let r = c.and(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x50b197; continue; }
            pc = 0x50b17b; continue;
            }
            0x50b17b => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0x16u32,16); }
            if c.cond(4) { pc = 0x50b221; continue; }
            pc = 0x50b189; continue;
            }
            0x50b189 => {
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,c.get(7,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x50b197 => {
            { let a = c.get(2,32); c.sub(a,0x190u32,32); }
            if c.cond(13) { pc = 0x50b1bc; continue; }
            pc = 0x50b19f; continue;
            }
            0x50b19f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x50b1ae);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x50b250, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,c.get(7,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x50b1bc => {
            { let a = rm(m,c.r[6].wrapping_add(0xe),8); c.sub(a,0x1u32,8); }
            if c.cond(6) { pc = 0x50b1ed; continue; }
            pc = 0x50b1c2; continue;
            }
            0x50b1c2 => {
            c.set(1,32,0x673568u32);
            pc = 0x50b1c7; continue;
            }
            0x50b1c7 => {
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[1].wrapping_add(0xfffffffc),16),16); }
            if c.cond(12) { pc = 0x50b1e2; continue; }
            pc = 0x50b1cd; continue;
            }
            0x50b1cd => {
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[1],16),16); }
            if c.cond(15) { pc = 0x50b1e2; continue; }
            pc = 0x50b1d2; continue;
            }
            0x50b1d2 => {
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[1].wrapping_add(0xfffffffe),16),16); }
            if c.cond(12) { pc = 0x50b1e2; continue; }
            pc = 0x50b1dc; continue;
            }
            0x50b1dc => {
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[1].wrapping_add(0x2),16),16); }
            if c.cond(14) { pc = 0x50b22b; continue; }
            pc = 0x50b1e2; continue;
            }
            0x50b1e2 => {
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x673580u32,32); }
            if c.cond(12) { pc = 0x50b1c7; continue; }
            pc = 0x50b1ed; continue;
            }
            0x50b1ed => {
            { let a = rm(m,0x6de4b4u32,8); c.and(a,0x7u32,8); }
            if c.cond(5) { pc = 0x50b221; continue; }
            pc = 0x50b1f6; continue;
            }
            0x50b1f6 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x2c),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x20),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x28),32));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x1c),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0xffu32);
            c.push(m, 0x50b221);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5172a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x50b221; continue;
            }
            0x50b221 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x50b22b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,c.get(7,16)); }
            c.push(m, 0x50b23b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x516840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050cb80(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50cb80;
    loop {
        match pc {
            0x50cb80 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(5,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[5]),8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x50cc87; continue; }
            pc = 0x50cba8; continue;
            }
            0x50cba8 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x500));
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x50cbbf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x20),8));
            c.set(6,32,c.get(0,32));
            { let a = c.get(3,8); c.sub(a,0xdu32,8); }
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,16,0x5u32); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[5]),8));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            if c.cond(3) { pc = 0x50cbf8; continue; }
            pc = 0x50cbd9; continue;
            }
            0x50cbd9 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x50cbe9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.r[6].wrapping_add(0x9); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[6].wrapping_add(0xa); wm(m,a,8,c.get(0,8)); }
            pc = 0x50cc03; continue;
            }
            0x50cbf8 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.r[6].wrapping_add(0xa); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x9); wm(m,a,8,c.get(1,8)); }
            pc = 0x50cc03; continue;
            }
            0x50cc03 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x8),8));
            c.push(m,c.get(2,32));
            c.push(m, 0x50cc13);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.r[6].wrapping_add(0xb); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x11); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(3,16)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xf4),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[5]));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x50cc6f; continue; }
            pc = 0x50cc4e; continue;
            }
            0x50cc4e => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            pc = 0x50cc56; continue;
            }
            0x50cc56 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x15e),16));
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x50cc6f => {
            c.set(1,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x50cc83; continue; }
            pc = 0x50cc78; continue;
            }
            0x50cc78 => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            pc = 0x50cc56; continue;
            }
            0x50cc83 => {
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(3,16)); }
            pc = 0x50cc87; continue;
            }
            0x50cc87 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050cc90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50cc90;
    loop {
        match pc {
            0x50cc90 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,32); c.sub(a,rm(m,0x6de4b0u32,32),32); }
            if c.cond(3) { pc = 0x50ce3c; continue; }
            pc = 0x50cca4; continue;
            }
            0x50cca4 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(5,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x994),16));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x994));
            { let a = c.get(5,16); c.sub(a,rm(m,c.r[7].wrapping_add(0xf4),16),16); }
            if c.cond(3) { pc = 0x50ce3c; continue; }
            pc = 0x50ccce; continue;
            }
            0x50ccce => {
            { let a = c.r[0]; wm(m,a,16,0xffffu32); }
            c.set(0,32,c.get(5,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xf4),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe8),16));
            c.set(1,32,c.r[7].wrapping_add(0xe8));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x50cd22; continue; }
            pc = 0x50cd00; continue;
            }
            0x50cd00 => {
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(4) { pc = 0x50ce27; continue; }
            pc = 0x50cd09; continue;
            }
            0x50cd09 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x6));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x6),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x50cd00; continue; }
            pc = 0x50cd22; continue;
            }
            0x50cd22 => {
            c.set(1,32,0xffffu32);
            pc = 0x50cd27; continue;
            }
            0x50cd27 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x24),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x1); wm(m,a,8,c.get(3,8)); }
            if c.cond(4) { pc = 0x50cd5e; continue; }
            pc = 0x50cd33; continue;
            }
            0x50cd33 => {
            { let a = rm(m,c.r[6].wrapping_add(0x36),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x50cd5b; continue; }
            pc = 0x50cd38; continue;
            }
            0x50cd38 => {
            { let a = c.r[6].wrapping_add(0x36); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x50cd4a; continue; }
            pc = 0x50cd41; continue;
            }
            0x50cd41 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x50cd47);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x50cd4a; continue;
            }
            0x50cd4a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x24),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x50cd53);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,0xffffu32);
            pc = 0x50cd5b; continue;
            }
            0x50cd5b => {
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            pc = 0x50cd5e; continue;
            }
            0x50cd5e => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.r[6].wrapping_add(0x3a); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[6].wrapping_add(0x52); wm(m,a,16,c.get(1,16)); }
            c.push(m,0x60u32);
            c.set(1,32,c.r[6].wrapping_add(0x54));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x37); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x39); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x50cd98);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x60u32);
            c.set(2,32,c.r[6].wrapping_add(0xb4));
            c.push(m,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x50cda7);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x114); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x118); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,16,0x14u32); }
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x13b); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x15c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x15d); wm(m,a,8,0xffu32); }
            c.push(m, 0x50cde4);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffu32); }
            c.push(m, 0x50cdf9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5062f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0xf6),16),16); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(3) { pc = 0x50ce3c; continue; }
            pc = 0x50ce08; continue;
            }
            0x50ce08 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x14),8); let r = c.or(a,0x10u32,8); { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x14); wm(m,a,8,r); } }
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x14));
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x50ce27 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            c.set(1,32,0xffffu32);
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,16,c.get(1,16)); }
            pc = 0x50cd27; continue;
            }
            0x50ce3c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,16); let r = c.or(a,0xffffu32,16); c.set(0,16,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050d2b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50d2b0;
    loop {
        match pc {
            0x50d2b0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x48u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,8,0x9u32);
            c.set(6,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x42); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x56); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(3,8,0xfcu32);
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(2,8,0x4u32);
            c.set(1,8,0x7u32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,c.get(3,8)); }
            c.set(5,32,c.r[6].wrapping_add(0x50));
            { let a = c.r[4].wrapping_add(0x46); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x4a); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0xf7u32); }
            { let a = c.r[4].wrapping_add(0x39); wm(m,a,8,0xf6u32); }
            { let a = c.r[4].wrapping_add(0x3a); wm(m,a,8,0xf7u32); }
            { let a = c.r[4].wrapping_add(0x3b); wm(m,a,8,0xf9u32); }
            { let a = c.r[4].wrapping_add(0x3d); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x3e); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x3f); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x41); wm(m,a,8,0xau32); }
            { let a = c.r[4].wrapping_add(0x43); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x45); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x47); wm(m,a,8,0xf9u32); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x49); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x4b); wm(m,a,8,0xf9u32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,8,0xf7u32); }
            { let a = c.r[4].wrapping_add(0x4d); wm(m,a,8,0xf6u32); }
            { let a = c.r[4].wrapping_add(0x4e); wm(m,a,8,0xf7u32); }
            { let a = c.r[4].wrapping_add(0x4f); wm(m,a,8,0xf9u32); }
            { let a = c.r[4].wrapping_add(0x51); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x52); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x53); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x55); wm(m,a,8,0xau32); }
            { let a = c.r[4].wrapping_add(0x57); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            pc = 0x50d371; continue;
            }
            0x50d371 => {
            c.set(1,16,rm(m,c.r[5],16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[7].wrapping_add(0xf4),16),16); }
            if c.cond(3) { pc = 0x50d422; continue; }
            pc = 0x50d382; continue;
            }
            0x50d382 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(2,16); c.sub(a,0x12u32,16); }
            if c.cond(5) { pc = 0x50d3b8; continue; }
            pc = 0x50d3a0; continue;
            }
            0x50d3a0 => {
            { let a = rm(m,c.r[0].wrapping_add(0x138),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x50d3b8; continue; }
            pc = 0x50d3aa; continue;
            }
            0x50d3aa => {
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x50d3b6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x452f60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x50d41c; continue;
            }
            0x50d3b8 => {
            { let a = c.get(2,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x50d3c4; continue; }
            pc = 0x50d3be; continue;
            }
            0x50d3be => {
            { let a = c.get(2,16); c.sub(a,0x12u32,16); }
            if c.cond(5) { pc = 0x50d41c; continue; }
            pc = 0x50d3c4; continue;
            }
            0x50d3c4 => {
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(5,32,sx(rm(m,c.r[6].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,32,sx(rm(m,c.r[6].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,sx(rm(m,c.r[0].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            c.set(5,16,rm(m,c.r[7].wrapping_add(0xf4),16));
            { let a = rm(m,c.r[0].wrapping_add(0xa),16); c.sub(a,c.get(5,16),16); }
            if c.cond(2) { pc = 0x50d3ef; continue; }
            pc = 0x50d3e9; continue;
            }
            0x50d3e9 => {
            { let a = rm(m,c.r[0].wrapping_add(0xc),16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x50d40c; continue; }
            pc = 0x50d3ef; continue;
            }
            0x50d3ef => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(5,32,c.get(2,32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(2,32),32); c.set(5,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xfc),16));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(13) { pc = 0x50d467; continue; }
            pc = 0x50d40c; continue;
            }
            0x50d40c => {
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x50d418);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x452f60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x50d41c; continue;
            }
            0x50d41c => {
            { let a = c.r[5]; wm(m,a,16,0xffffu32); }
            pc = 0x50d422; continue;
            }
            0x50d422 => {
            { let a = c.r[4].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x30); wm(m,a,32,0x80000000u32); }
            pc = 0x50d42a; continue;
            }
            0x50d42a => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x2u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); c.sub(a,0x2u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            if c.cond(12) { pc = 0x50d371; continue; }
            pc = 0x50d43b; continue;
            }
            0x50d43b => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x50d475; continue; }
            pc = 0x50d447; continue;
            }
            0x50d447 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x52),16));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,0x80000000u32);
            { let a = c.r[2]; wm(m,a,16,c.get(1,16)); }
            { let a = c.r[6].wrapping_add(0x52); wm(m,a,16,0xffffu32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            pc = 0x50d481; continue;
            }
            0x50d467 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x30); wm(m,a,32,0x7fffffffu32); }
            pc = 0x50d42a; continue;
            }
            0x50d475 => {
            { let a = c.get(3,32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(4) { pc = 0x50d9d5; continue; }
            pc = 0x50d481; continue;
            }
            0x50d481 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            c.set(5,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(1,32,sx(rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x38),8),8)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x48),8),8)&0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xfe),16));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1]));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xf4),16));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x50d6ef; continue; }
            pc = 0x50d4d3; continue;
            }
            0x50d4d3 => {
            pc = 0x50d4d9; continue;
            }
            0x50d4d5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x50d4d9; continue;
            }
            0x50d4d9 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(7,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x50d6eb; continue; }
            pc = 0x50d4f6; continue;
            }
            0x50d4f6 => {
            c.set(2,8,rm(m,c.r[7],8));
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x50d6d0; continue; }
            pc = 0x50d502; continue;
            }
            0x50d502 => {
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x20),32),32); }
            if c.cond(12) { pc = 0x50d6d0; continue; }
            pc = 0x50d510; continue;
            }
            0x50d510 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x24),32),32); }
            if c.cond(15) { pc = 0x50d6d0; continue; }
            pc = 0x50d51a; continue;
            }
            0x50d51a => {
            { let a = rm(m,c.r[7].wrapping_add(0x134),16); c.sub(a,0x15u32,16); }
            if c.cond(5) { pc = 0x50d55a; continue; }
            pc = 0x50d524; continue;
            }
            0x50d524 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x39),8));
            { let a = c.get(0,8); c.sub(a,0x10u32,8); }
            if c.cond(3) { pc = 0x50d6d0; continue; }
            pc = 0x50d52f; continue;
            }
            0x50d52f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2a); wm(m,a,16,0xffu32); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x39),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x9));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x26),16));
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            c.push(m,c.get(2,32));
            pc = 0x50d5cc; continue;
            }
            0x50d55a => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x52),16));
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x50d6d0; continue; }
            pc = 0x50d568; continue;
            }
            0x50d568 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(2,16,rm(m,c.r[2],16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x50d6d0; continue; }
            pc = 0x50d57d; continue;
            }
            0x50d57d => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x134),16));
            { let a = c.get(2,16); c.sub(a,0x12u32,16); }
            if c.cond(5) { pc = 0x50d659; continue; }
            pc = 0x50d58e; continue;
            }
            0x50d58e => {
            { let a = rm(m,c.r[7].wrapping_add(0x138),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x50d6d0; continue; }
            pc = 0x50d59c; continue;
            }
            0x50d59c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(13) { pc = 0x50d603; continue; }
            pc = 0x50d5a7; continue;
            }
            0x50d5a7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x7fffffffu32); }
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.r[1]; wm(m,a,16,c.get(5,16)); }
            { let a = c.r[6].wrapping_add(0x52); wm(m,a,16,c.get(0,16)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x22),16));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x1e),16));
            c.push(m,c.get(0,32));
            pc = 0x50d5cc; continue;
            }
            0x50d5cc => {
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.push(m, 0x50d5d8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x50d6d0; continue; }
            pc = 0x50d5e4; continue;
            }
            0x50d5e4 => {
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x2u32);
            c.push(m, 0x50d5fe);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x459790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x50d6d0; continue;
            }
            0x50d603 => {
            { let a = c.get(3,32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(13) { pc = 0x50d6d0; continue; }
            pc = 0x50d60f; continue;
            }
            0x50d60f => {
            { let a = c.r[6].wrapping_add(0x52); wm(m,a,16,c.get(5,16)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x22),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x1e),16));
            c.set(3,32,0x7fffffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x50d631);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x50d6d0; continue; }
            pc = 0x50d63d; continue;
            }
            0x50d63d => {
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,c.get(5,32));
            c.push(m,0x2u32);
            c.push(m, 0x50d657);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x459790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x50d6d0; continue;
            }
            0x50d659 => {
            { let a = c.get(2,16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x50d6d0; continue; }
            pc = 0x50d65f; continue;
            }
            0x50d65f => {
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(3,32,c.get(1,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(3,32); let r = c.imul(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(c.r[2]).wrapping_add(0x38),8),8)&0xffffffffu32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(c.r[2]).wrapping_add(0x48),8),8)&0xffffffffu32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x1c),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x50d6d0; continue; }
            pc = 0x50d6b8; continue;
            }
            0x50d6b8 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0x52); wm(m,a,16,c.get(5,16)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(15) { pc = 0x50d6c8; continue; }
            pc = 0x50d6c4; continue;
            }
            0x50d6c4 => {
            c.set(3,32,c.get(0,32));
            pc = 0x50d6cc; continue;
            }
            0x50d6c8 => {
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            pc = 0x50d6cc; continue;
            }
            0x50d6cc => {
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            pc = 0x50d6d0; continue;
            }
            0x50d6d0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xf4),16));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x50d4d5; continue; }
            pc = 0x50d6eb; continue;
            }
            0x50d6eb => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x50d6ef; continue;
            }
            0x50d6ef => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xfe),16));
            c.set(5,16,rm(m,c.r[6].wrapping_add(0xc),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xf4),16));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            if c.cond(13) { pc = 0x50d92b; continue; }
            pc = 0x50d71d; continue;
            }
            0x50d71d => {
            pc = 0x50d723; continue;
            }
            0x50d71f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x50d723; continue;
            }
            0x50d723 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(7,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,rm(m,c.r[2].wrapping_add(0x4),32),32); c.set(7,32,r); }
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x50d927; continue; }
            pc = 0x50d73d; continue;
            }
            0x50d73d => {
            c.set(0,8,rm(m,c.r[7],8));
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x50d908; continue; }
            pc = 0x50d749; continue;
            }
            0x50d749 => {
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x20),32),32); }
            if c.cond(12) { pc = 0x50d908; continue; }
            pc = 0x50d757; continue;
            }
            0x50d757 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x24),32),32); }
            if c.cond(15) { pc = 0x50d908; continue; }
            pc = 0x50d761; continue;
            }
            0x50d761 => {
            { let a = rm(m,c.r[7].wrapping_add(0x134),16); c.sub(a,0x15u32,16); }
            if c.cond(5) { pc = 0x50d7a1; continue; }
            pc = 0x50d76b; continue;
            }
            0x50d76b => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x39),8));
            { let a = c.get(0,8); c.sub(a,0x10u32,8); }
            if c.cond(3) { pc = 0x50d908; continue; }
            pc = 0x50d776; continue;
            }
            0x50d776 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x2a); wm(m,a,16,0xffu32); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x39),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x9));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x26),16));
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            c.push(m,c.get(2,32));
            pc = 0x50d802; continue;
            }
            0x50d7a1 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x52),16));
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x50d908; continue; }
            pc = 0x50d7af; continue;
            }
            0x50d7af => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(2,16,rm(m,c.r[2],16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x50d908; continue; }
            pc = 0x50d7c4; continue;
            }
            0x50d7c4 => {
            { let a = rm(m,c.r[7].wrapping_add(0x134),16); c.sub(a,0x12u32,16); }
            if c.cond(5) { pc = 0x50d88f; continue; }
            pc = 0x50d7d2; continue;
            }
            0x50d7d2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(13) { pc = 0x50d839; continue; }
            pc = 0x50d7dd; continue;
            }
            0x50d7dd => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x7fffffffu32); }
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.r[1]; wm(m,a,16,c.get(5,16)); }
            { let a = c.r[6].wrapping_add(0x52); wm(m,a,16,c.get(0,16)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x22),16));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x1e),16));
            c.push(m,c.get(0,32));
            pc = 0x50d802; continue;
            }
            0x50d802 => {
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.push(m, 0x50d80e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x50d908; continue; }
            pc = 0x50d81a; continue;
            }
            0x50d81a => {
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,0x2u32);
            c.push(m, 0x50d834);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x459790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x50d908; continue;
            }
            0x50d839 => {
            { let a = c.get(3,32); c.sub(a,0x7fffffffu32,32); }
            if c.cond(13) { pc = 0x50d908; continue; }
            pc = 0x50d845; continue;
            }
            0x50d845 => {
            { let a = c.r[6].wrapping_add(0x52); wm(m,a,16,c.get(5,16)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x22),16));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x1e),16));
            c.set(3,32,0x7fffffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x50d867);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x50d908; continue; }
            pc = 0x50d873; continue;
            }
            0x50d873 => {
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[7].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,c.get(5,32));
            c.push(m,0x2u32);
            c.push(m, 0x50d88d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x459790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x50d908; continue;
            }
            0x50d88f => {
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            c.set(5,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.imul(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(c.r[2]).wrapping_add(0x38),8),8)&0xffffffffu32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,sx(rm(m,c.r[4].wrapping_add(c.r[2]).wrapping_add(0x48),8),8)&0xffffffffu32);
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x50d908; continue; }
            pc = 0x50d8e2; continue;
            }
            0x50d8e2 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(15) { pc = 0x50d8f7; continue; }
            pc = 0x50d8ea; continue;
            }
            0x50d8ea => {
            c.set(3,32,c.get(0,32));
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.r[6].wrapping_add(0x52); wm(m,a,16,c.get(0,16)); }
            pc = 0x50d904; continue;
            }
            0x50d8f7 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x52); wm(m,a,16,c.get(1,16)); }
            pc = 0x50d904; continue;
            }
            0x50d904 => {
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            pc = 0x50d908; continue;
            }
            0x50d908 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[7].wrapping_add(0xc),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0xf4),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x50d71f; continue; }
            pc = 0x50d927; continue;
            }
            0x50d927 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x50d92b; continue;
            }
            0x50d92b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x50d99c; continue; }
            pc = 0x50d935; continue;
            }
            0x50d935 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(5,32,0x2u32);
            pc = 0x50d93e; continue;
            }
            0x50d93e => {
            c.set(0,16,rm(m,c.r[3],16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0xf4),16),16); }
            if c.cond(3) { pc = 0x50d996; continue; }
            pc = 0x50d94a; continue;
            }
            0x50d94a => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(6,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x22),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x1e),16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x50d971);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x46f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x60),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x50d996; continue; }
            pc = 0x50d979; continue;
            }
            0x50d979 => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(2,16,rm(m,c.r[3],16));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,c.get(2,32));
            c.push(m,0x2u32);
            c.push(m, 0x50d996);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x459790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x50d996; continue;
            }
            0x50d996 => {
            { let a = c.get(3,32); let r = c.add(a,0x2u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x50d93e; continue; }
            pc = 0x50d99c; continue;
            }
            0x50d99c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(2,32,0x2u32);
            pc = 0x50d9a5; continue;
            }
            0x50d9a5 => {
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0xf4),16),16); }
            if c.cond(3) { pc = 0x50d9cf; continue; }
            pc = 0x50d9b1; continue;
            }
            0x50d9b1 => {
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x134),16); c.sub(a,0x15u32,16); }
            if c.cond(5) { pc = 0x50d9cf; continue; }
            pc = 0x50d9ca; continue;
            }
            0x50d9ca => {
            { let a = c.r[1]; wm(m,a,16,0xffffu32); }
            pc = 0x50d9cf; continue;
            }
            0x50d9cf => {
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x50d9a5; continue; }
            pc = 0x50d9d5; continue;
            }
            0x50d9d5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050e280(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50e280;
    loop {
        match pc {
            0x50e280 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,8,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbc1),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x50e2b3; continue; }
            pc = 0x50e2a1; continue;
            }
            0x50e2a1 => {
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(6) { pc = 0x50e2ac; continue; }
            pc = 0x50e2a5; continue;
            }
            0x50e2a5 => {
            c.set(7,32,0xffff0000u32);
            pc = 0x50e2b3; continue;
            }
            0x50e2ac => {
            if c.cond(3) { pc = 0x50e2b3; continue; }
            pc = 0x50e2ae; continue;
            }
            0x50e2ae => {
            c.set(7,32,0x10000u32);
            pc = 0x50e2b3; continue;
            }
            0x50e2b3 => {
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(2) { pc = 0x50e2ca; continue; }
            pc = 0x50e2b7; continue;
            }
            0x50e2b7 => {
            { let a = c.get(0,8); c.sub(a,0x7u32,8); }
            if c.cond(4) { pc = 0x50e2ca; continue; }
            pc = 0x50e2bb; continue;
            }
            0x50e2bb => {
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(6) { pc = 0x50e2cf; continue; }
            pc = 0x50e2bf; continue;
            }
            0x50e2bf => {
            { let a = c.get(0,8); c.sub(a,0x6u32,8); }
            if c.cond(3) { pc = 0x50e2cf; continue; }
            pc = 0x50e2c3; continue;
            }
            0x50e2c3 => {
            c.set(6,32,0x10000u32);
            pc = 0x50e2cf; continue;
            }
            0x50e2ca => {
            c.set(6,32,0xffff0000u32);
            pc = 0x50e2cf; continue;
            }
            0x50e2cf => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[1].wrapping_add(0xf6),16),16); }
            if c.cond(3) { pc = 0x50e339; continue; }
            pc = 0x50e2dc; continue;
            }
            0x50e2dc => {
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xf4),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x50e339; continue; }
            pc = 0x50e2fe; continue;
            }
            0x50e2fe => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            pc = 0x50e301; continue;
            }
            0x50e301 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1c),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x20),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x20));
            { let a = c.get(3,32); let r = c.add(a,c.get(6,32),32); c.set(3,32,r); }
            { let a = c.r[2]; wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x6),16));
            c.set(0,32,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[1].wrapping_add(0xf4),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x50e301; continue; }
            pc = 0x50e339; continue;
            }
            0x50e339 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00512fc0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x512fc0;
    loop {
        match pc {
            0x512fc0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xda),16));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.set(2,32,c.r[0].wrapping_add(0xffffffff));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x512fe1; continue; }
            pc = 0x512fdb; continue;
            }
            0x512fdb => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x512fe1 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xe4),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x512ff1; continue; }
            pc = 0x512feb; continue;
            }
            0x512feb => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x512ff1 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xd8),16));
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xcc),32));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)),16); let r = c.dec(a,16); { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(2)); wm(m,a,16,r); } }
            c.push(m,c.get(7,32));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            pc = 0x51300f; continue;
            }
            0x51300f => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xcc),32));
            c.set(3,8,rm(m,c.r[6],8));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xb8),32));
            { let a = c.r[7].wrapping_add(c.r[2]); wm(m,a,8,c.get(3,8)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xcc),32));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)),16); let r = c.inc(a,16); { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(2)); wm(m,a,16,r); } }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xffffffff),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x51300f; continue; }
            pc = 0x51303b; continue;
            }
            0x51303b => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xcc),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(6,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xb8),32));
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,0x0u32); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00513a40(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x513a40;
    loop {
        match pc {
            0x513a40 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x663140u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x513a81; continue; }
            pc = 0x513a60; continue;
            }
            0x513a60 => {
            c.push(m, 0x513a65);
            { let args = []; c.r[0] = host.call(m, 0x4f9930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[7],32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x513a7b; continue; }
            pc = 0x513a6b; continue;
            }
            0x513a6b => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x513a72);
            { let args = []; c.r[0] = host.call(m, 0x4f9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x513a78);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x513a7b; continue;
            }
            0x513a7b => {
            { let a = c.r[7]; wm(m,a,32,0x0u32); }
            pc = 0x513a81; continue;
            }
            0x513a81 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x513a98; continue; }
            pc = 0x513a88; continue;
            }
            0x513a88 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x513a8e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            pc = 0x513a98; continue;
            }
            0x513a98 => {
            c.set(0,32,0x10u32);
            c.push(m,0x80u32);
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x513aaf);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,0x80u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x513ac2);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x80u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x513ad5);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(1,32,c.r[7].wrapping_add(0xc));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x513ae4);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(2,32,c.r[7].wrapping_add(0x16));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x513af3);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2),8));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[7].wrapping_add(0xe); wm(m,a,8,c.get(0,8)); }
            c.push(m,0x2u32);
            c.set(2,32,c.r[7].wrapping_add(0x10));
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            { let a = c.r[7].wrapping_add(0xf); wm(m,a,8,c.get(1,8)); }
            c.push(m, 0x513b0f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x40u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.set(0,32,c.r[7].wrapping_add(0x12));
            c.push(m,0x2u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x513b21);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2),8));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[7].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[7].wrapping_add(0x15); wm(m,a,8,c.get(2,8)); }
            if c.cond(4) { pc = 0x513b8b; continue; }
            pc = 0x513b3c; continue;
            }
            0x513b3c => {
            c.push(m,0x6cu32);
            c.push(m, 0x513b43);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x513b5f; continue; }
            pc = 0x513b56; continue;
            }
            0x513b56 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x513b5d);
            { let args = []; c.r[0] = host.call(m, 0x4f95c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x513b61; continue;
            }
            0x513b5f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x513b61; continue;
            }
            0x513b61 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(0,32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x513b78);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4f9770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x513b8b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005144a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5144a0;
    loop {
        match pc {
            0x5144a0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xe),8));
            { let a = rm(m,c.r[4].wrapping_add(0xc),16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x514526; continue; }
            pc = 0x5144b0; continue;
            }
            0x5144b0 => {
            { let a = rm(m,c.r[6],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x514526; continue; }
            pc = 0x5144b5; continue;
            }
            0x5144b5 => {
            c.set(2,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x13),8));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x12),16));
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x673658),8));
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x5144fa; continue; }
            pc = 0x5144de; continue;
            }
            0x5144de => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.push(m,c.get(7,32));
            pc = 0x5144e5; continue;
            }
            0x5144e5 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x6),16));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x5144e5; continue; }
            pc = 0x5144f9; continue;
            }
            0x5144f9 => {
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x5144fa; continue;
            }
            0x5144fa => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x514526; continue; }
            pc = 0x5144fe; continue;
            }
            0x5144fe => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,8,rm(m,0x6de4a4u32,8));
            { let a = c.get(0,8); let r = c.add(a,c.get(3,8),8); c.set(0,8,r); }
            c.set(2,32,c.r[1].wrapping_mul(8).wrapping_add(0x64));
            { let a = c.get(0,8); let r = c.and(a,0x3u32,8); c.set(0,8,r); }
            c.push(m,c.get(2,32));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x514521);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x516170, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(15) { pc = 0x5144fe; continue; }
            pc = 0x514526; continue;
            }
            0x514526 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00514530(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x514530;
    loop {
        match pc {
            0x514530 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x663155u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x514776; continue; }
            pc = 0x514568; continue;
            }
            0x514568 => {
            c.push(m,0x6cu32);
            c.push(m, 0x51456f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x514587; continue; }
            pc = 0x51457e; continue;
            }
            0x51457e => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x514585);
            { let args = []; c.r[0] = host.call(m, 0x4f95c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x514589; continue;
            }
            0x514587 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x514589; continue;
            }
            0x514589 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xf),8));
            c.set(2,32,0xfeu32);
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x5145a8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f9b40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0x8),16); c.sub(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x514776; continue; }
            pc = 0x5145b2; continue;
            }
            0x5145b2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x6); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[3].wrapping_mul(8)));
            c.set(1,8,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,0x24u32,8); }
            if c.cond(5) { pc = 0x514619; continue; }
            pc = 0x5145c9; continue;
            }
            0x5145c9 => {
            { let a = c.r[0].wrapping_add(0x7); wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(0,32,c.r[2].wrapping_add(c.r[3].wrapping_mul(8)));
            c.push(m,0x24u32);
            c.set(1,32,c.r[0].wrapping_add(0x1));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5145e5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f9c80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x514608; continue; }
            pc = 0x5145e9; continue;
            }
            0x5145e9 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.push(m,0x24u32);
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x1),8));
            c.set(0,32,c.r[2].wrapping_add(c.r[3].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x0u32);
            c.set(2,8,rm(m,c.r[0],8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            c.push(m, 0x514608);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4fb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x514608; continue;
            }
            0x514608 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x4),16));
            c.set(3,32,c.get(1,32));
            pc = 0x514768; continue;
            }
            0x514619 => {
            { let a = c.get(1,8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x514727; continue; }
            pc = 0x514622; continue;
            }
            0x514622 => {
            { let a = c.get(1,8); c.sub(a,0x31u32,8); }
            if c.cond(4) { pc = 0x514727; continue; }
            pc = 0x51462b; continue;
            }
            0x51462b => {
            { let a = c.get(1,8); c.sub(a,0x1bu32,8); }
            if c.cond(5) { pc = 0x514768; continue; }
            pc = 0x514634; continue;
            }
            0x514634 => {
            { let a = c.get(3,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x514644; continue; }
            pc = 0x514639; continue;
            }
            0x514639 => {
            c.set(2,8,rm(m,0x67792fu32,8));
            { let a = c.r[0].wrapping_add(0x7); wm(m,a,8,c.get(2,8)); }
            pc = 0x51464d; continue;
            }
            0x514644 => {
            c.set(1,8,rm(m,0x677929u32,8));
            { let a = c.r[0].wrapping_add(0x7); wm(m,a,8,c.get(1,8)); }
            pc = 0x51464d; continue;
            }
            0x51464d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,8,c.get(0,8));
            c.push(m,0x0u32);
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x5c),8));
            { let a = c.get(2,8); let r = c.add(a,c.get(3,8),8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x7u32,8); c.set(2,8,r); }
            { let a = c.get(1,8); let r = c.sub(a,c.get(2,8),8); c.set(1,8,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,8); let r = c.add(a,0x4u32,8); c.set(1,8,r); }
            { let a = c.r[2].wrapping_add(c.r[3].wrapping_mul(8)); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x5d),8));
            { let a = c.get(0,8); let r = c.add(a,c.get(3,8),8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x7u32,8); c.set(0,8,r); }
            { let a = c.get(2,8); let r = c.sub(a,c.get(0,8),8); c.set(2,8,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(2,8); let r = c.add(a,0x4u32,8); c.set(2,8,r); }
            { let a = c.r[0].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x1); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x7),8));
            c.set(0,32,c.r[1].wrapping_add(c.r[3].wrapping_mul(8)));
            c.push(m,c.get(2,32));
            c.push(m,0x1bu32);
            c.set(1,32,c.r[0].wrapping_add(0x1));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x51469b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f9c80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5146d1; continue; }
            pc = 0x51469f; continue;
            }
            0x51469f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x7),8));
            c.set(0,32,c.r[2].wrapping_add(c.r[3].wrapping_mul(8)));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            c.push(m,0x1bu32);
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m, 0x5146c0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4fb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x4),16));
            c.set(3,32,c.get(0,32));
            pc = 0x514768; continue;
            }
            0x5146d1 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(5,32,c.get(3,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x4),16));
            c.set(2,32,c.r[7].wrapping_add(0x4));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(5,16); c.sub(a,c.get(0,16),16); }
            c.set(3,16,rm(m,c.r[1].wrapping_add(c.r[7]).wrapping_add(0x4),16));
            if c.cond(4) { pc = 0x51470d; continue; }
            pc = 0x5146f4; continue;
            }
            0x5146f4 => {
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x8),16),16); }
            if c.cond(3) { pc = 0x514768; continue; }
            pc = 0x5146fa; continue;
            }
            0x5146fa => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),16));
            { let a = c.get(5,16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x5146f4; continue; }
            pc = 0x51470d; continue;
            }
            0x51470d => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[7]).wrapping_add(0x4),16));
            { let a = c.r[2]; wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.r[2].wrapping_add(c.r[1]).wrapping_add(0x4); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0xa); wm(m,a,16,c.get(5,16)); }
            pc = 0x514768; continue;
            }
            0x514727 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.r[0].wrapping_add(0x1));
            c.push(m,0x30u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x514739);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f9c80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x514763; continue; }
            pc = 0x514740; continue;
            }
            0x514740 => {
            c.set(0,32,c.r[2].wrapping_add(c.r[3].wrapping_mul(8)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[0],8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            c.push(m, 0x514755);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fbdb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x4),16));
            c.set(3,32,c.get(1,32));
            pc = 0x514768; continue;
            }
            0x514763 => {
            { let a = c.r[2].wrapping_add(c.r[3].wrapping_mul(8)).wrapping_add(0x3); wm(m,a,8,0x1bu32); }
            pc = 0x514768; continue;
            }
            0x514768 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x5145b2; continue; }
            pc = 0x514776; continue;
            }
            0x514776 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x10),16));
            c.set(5,32,c.r[6].wrapping_add(0x10));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(2) { pc = 0x51484d; continue; }
            pc = 0x51478a; continue;
            }
            0x51478a => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x5147b7; continue; }
            pc = 0x514797; continue;
            }
            0x514797 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_mul(8).wrapping_add(0x64));
            c.push(m,c.get(1,32));
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5147b2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x516170, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x5147b7; continue;
            }
            0x5147b7 => {
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x12),16));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            c.set(7,32,c.get(3,32));
            c.push(m,c.get(3,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.push(m,c.get(5,32));
            c.set(7,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,16,0xffffu32); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xe),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6ddbb0),16));
            { let a = c.r[7].wrapping_add(0x1e); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xe),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,0x6dd7a0u32);
            c.set(0,16,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6ddbb4),16));
            { let a = c.r[7].wrapping_add(0x22); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x514818);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5062f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x51484d; continue; }
            pc = 0x514820; continue;
            }
            0x514820 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x14),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(0,16,rm(m,c.r[5],16));
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,c.get(3,16)); }
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,0xffffu32);
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,16,c.get(0,16)); }
            pc = 0x51484d; continue;
            }
            0x51484d => {
            c.set(0,16,rm(m,c.r[5],16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x514904; continue; }
            pc = 0x51485e; continue;
            }
            0x51485e => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x514904; continue; }
            pc = 0x51486a; continue;
            }
            0x51486a => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            if c.cond(3) { pc = 0x514904; continue; }
            pc = 0x51488a; continue;
            }
            0x51488a => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.r[6].wrapping_add(0x12); wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xa),16));
            { let a = c.r[7].wrapping_add(0x6); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[3].wrapping_add(0xa); wm(m,a,16,c.get(1,16)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,16,c.get(1,16)); }
            c.set(0,8,rm(m,0x6de52eu32,8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x5148d6);
            { let args = []; c.r[0] = host.call(m, 0x52bb00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[2].wrapping_mul(4).wrapping_add(0x32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5148f7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x51a4e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x12),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x51488a; continue; }
            pc = 0x514904; continue;
            }
            0x514904 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00514920(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x514920;
    loop {
        match pc {
            0x514920 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x514932);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x10),16));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x5149d6; continue; }
            pc = 0x514948; continue;
            }
            0x514948 => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x514958; continue; }
            pc = 0x514952; continue;
            }
            0x514952 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x514958);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x494a80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x514958; continue;
            }
            0x514958 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x5149d6; continue; }
            pc = 0x51497b; continue;
            }
            0x51497b => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.get(1,32));
            c.set(2,32,0xffffu32);
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.r[6].wrapping_add(0xa); wm(m,a,16,c.get(3,16)); }
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,c.get(2,16)); }
            if c.cond(14) { pc = 0x5149b8; continue; }
            pc = 0x5149a5; continue;
            }
            0x5149a5 => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x12),16));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[7].wrapping_add(0x12); wm(m,a,16,c.get(1,16)); }
            pc = 0x5149c9; continue;
            }
            0x5149b8 => {
            { let a = c.get(1,32); let r = c.and(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,c.get(2,16)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x5149c9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x507fb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5149c9; continue;
            }
            0x5149c9 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x51497b; continue; }
            pc = 0x5149d6; continue;
            }
            0x5149d6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6dd7a0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x5149e7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50a3d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,16); c.and(a,c.get(6,16),16); }
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,16,0xffffu32); }
            if c.cond(6) { pc = 0x514a26; continue; }
            pc = 0x5149f8; continue;
            }
            0x5149f8 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(3,8,0xc8u32);
            pc = 0x5149fd; continue;
            }
            0x5149fd => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(1,8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x514a13; continue; }
            pc = 0x514a0e; continue;
            }
            0x514a0e => {
            { let a = c.get(1,8); c.sub(a,0x31u32,8); }
            if c.cond(5) { pc = 0x514a1d; continue; }
            pc = 0x514a13; continue;
            }
            0x514a13 => {
            { let a = rm(m,c.r[0].wrapping_add(0x6),8); c.sub(a,c.get(3,8),8); }
            if c.cond(3) { pc = 0x514a1d; continue; }
            pc = 0x514a18; continue;
            }
            0x514a18 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x514a1d; continue;
            }
            0x514a1d => {
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(6,16),16); }
            if c.cond(2) { pc = 0x5149fd; continue; }
            pc = 0x514a26; continue;
            }
            0x514a26 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x514b70; continue; }
            pc = 0x514a32; continue;
            }
            0x514a32 => {
            { let a = c.r[7].wrapping_add(0x15); wm(m,a,8,0x1u32); }
            c.set(0,32,rm(m,0x6df31cu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x514a8c; continue; }
            pc = 0x514a3f; continue;
            }
            0x514a3f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xb4u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x514a4d; continue;
            }
            0x514a4d => {
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dd7ac),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x514a5e; continue; }
            pc = 0x514a58; continue;
            }
            0x514a58 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            if c.cond(2) { pc = 0x514a4d; continue; }
            pc = 0x514a5e; continue;
            }
            0x514a5e => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x14),8));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x514a70);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x514a79);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x514a8c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x514a8c; continue;
            }
            0x514a8c => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x514ad4; continue; }
            pc = 0x514a97; continue;
            }
            0x514a97 => {
            c.set(3,8,0xffu32);
            pc = 0x514a99; continue;
            }
            0x514a99 => {
            c.set(6,32,c.get(5,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m, 0x514aac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[0].wrapping_add(0xe1),8));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x514aca; continue; }
            pc = 0x514ab9; continue;
            }
            0x514ab9 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x514ac4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0xe1); wm(m,a,8,c.get(3,8)); }
            pc = 0x514aca; continue;
            }
            0x514aca => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x514a99; continue; }
            pc = 0x514ad4; continue;
            }
            0x514ad4 => {
            c.set(6,32,rm(m,c.r[7],32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x514af0; continue; }
            pc = 0x514ada; continue;
            }
            0x514ada => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x514ae1);
            { let args = []; c.r[0] = host.call(m, 0x4f9650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x514ae7);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[7]; wm(m,a,32,0x0u32); }
            pc = 0x514af0; continue;
            }
            0x514af0 => {
            c.set(5,32,0xffffu32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x514af7; continue;
            }
            0x514af7 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dd7ac),32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x514b0f; continue; }
            pc = 0x514b08; continue;
            }
            0x514b08 => {
            c.set(5,32,c.get(0,32));
            c.set(0,32,0x5u32);
            pc = 0x514b0f; continue;
            }
            0x514b0f => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,16); c.sub(a,0x5u32,16); }
            if c.cond(2) { pc = 0x514af7; continue; }
            pc = 0x514b16; continue;
            }
            0x514b16 => {
            c.set(1,32,rm(m,0x6dd744u32,32));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd748u32,16),16); }
            if c.cond(3) { pc = 0x514b70; continue; }
            pc = 0x514b25; continue;
            }
            0x514b25 => {
            c.set(6,32,0x32u32);
            c.set(3,8,0x2u32);
            pc = 0x514b2c; continue;
            }
            0x514b2c => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x514b3d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x6),16); c.sub(a,c.get(6,16),16); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x4),16));
            if c.cond(5) { pc = 0x514b67; continue; }
            pc = 0x514b47; continue;
            }
            0x514b47 => {
            { let a = rm(m,c.r[0].wrapping_add(0xc),8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x514b67; continue; }
            pc = 0x514b4d; continue;
            }
            0x514b4d => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x10),8));
            { let a = c.get(2,16); c.sub(a,c.get(5,16),16); }
            if c.cond(5) { pc = 0x514b67; continue; }
            pc = 0x514b57; continue;
            }
            0x514b57 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x11),8));
            { let a = c.get(2,8); c.sub(a,rm(m,c.r[7].wrapping_add(0xe),8),8); }
            if c.cond(5) { pc = 0x514b67; continue; }
            pc = 0x514b5f; continue;
            }
            0x514b5f => {
            { let a = c.r[0].wrapping_add(0x12); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,0xffffu32);
            pc = 0x514b67; continue;
            }
            0x514b67 => {
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd748u32,16),16); }
            if c.cond(2) { pc = 0x514b2c; continue; }
            pc = 0x514b70; continue;
            }
            0x514b70 => {
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

pub fn f_00514b80(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x514b80;
    loop {
        match pc {
            0x514b80 => {
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(5,32));
            c.set(3,32,c.get(1,32));
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x514bbd);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x514bcd; continue; }
            pc = 0x514bc7; continue;
            }
            0x514bc7 => {
            c.push(m,c.get(5,32));
            c.push(m, 0x514bcd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x494a80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x514bcd; continue;
            }
            0x514bcd => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x514d03; continue; }
            pc = 0x514bde; continue;
            }
            0x514bde => {
            pc = 0x514be4; continue;
            }
            0x514be0 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            pc = 0x514be4; continue;
            }
            0x514be4 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,0xffffu32); }
            { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,c.get(2,16)); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x12),16));
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[3].wrapping_add(0x12); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,0x7fffffffu32);
            c.set(7,32,0x673a18u32);
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(5,32,c.r[6].wrapping_add(0x54));
            pc = 0x514c24; continue;
            }
            0x514c24 => {
            c.fild32(m,c.r[5]);
            c.fstp32(m,c.r[4].wrapping_add(0x14));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x14),4));
            { let v = c.ld(m,c.r[7],4); c.fop('*',v); }
            c.push(m, 0x514c36);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[5]; wm(m,a,32,0x0u32); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,0x673a78u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x514c24; continue; }
            pc = 0x514c55; continue;
            }
            0x514c55 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x514c68; continue; }
            pc = 0x514c61; continue;
            }
            0x514c61 => {
            { let a = c.r[6].wrapping_add(0x15d); wm(m,a,8,0xffu32); }
            pc = 0x514c68; continue;
            }
            0x514c68 => {
            { let a = rm(m,c.r[6].wrapping_add(0xf),8); c.sub(a,0x2u32,8); }
            if c.cond(3) { pc = 0x514c77; continue; }
            pc = 0x514c6e; continue;
            }
            0x514c6e => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x514c77);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51a750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x514c77; continue;
            }
            0x514c77 => {
            c.set(2,16,rm(m,0x6dd892u32,16));
            { let a = rm(m,c.r[6].wrapping_add(0x42),16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x514cea; continue; }
            pc = 0x514c84; continue;
            }
            0x514c84 => {
            c.set(0,32,rm(m,0x6df31cu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x514cea; continue; }
            pc = 0x514c8d; continue;
            }
            0x514c8d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x12u32); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x10),16));
            c.set(1,32,0x32u32);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x514cea; continue; }
            pc = 0x514cc8; continue;
            }
            0x514cc8 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x42),16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x514cea; continue; }
            pc = 0x514cdb; continue;
            }
            0x514cdb => {
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x514cea);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x514cea; continue;
            }
            0x514cea => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xa),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x514be0; continue; }
            pc = 0x514cff; continue;
            }
            0x514cff => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x514d03; continue;
            }
            0x514d03 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x514d0e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50a3d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],rm(m,c.r[4].wrapping_add(0x10),32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.get(3,32));
            { let a = rm(m,c.r[3].wrapping_add(0x16),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[3].wrapping_add(0x16); wm(m,a,16,r); } }
            c.push(m, 0x514d2c);
            { let args = []; c.r[0] = host.call(m, 0x515da0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00515d00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x515d00;
    loop {
        match pc {
            0x515d00 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x15),8));
            c.set(0,32,0x1u32);
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x515d1b; continue; }
            pc = 0x515d0c; continue;
            }
            0x515d0c => {
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x10),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(2) { pc = 0x515d1b; continue; }
            pc = 0x515d19; continue;
            }
            0x515d19 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x515d1b; continue;
            }
            0x515d1b => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00515d20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x515d20;
    loop {
        match pc {
            0x515d20 => {
            c.set(1,8,rm(m,c.r[1].wrapping_add(0xf),8));
            c.set(0,32,0xfeu32);
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00515d30(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x515d30;
    loop {
        match pc {
            0x515d30 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x515d89; continue; }
            pc = 0x515d3e; continue;
            }
            0x515d3e => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x6),8));
            { let a = c.get(0,8); c.sub(a,0xc8u32,8); }
            if c.cond(3) { pc = 0x515d89; continue; }
            pc = 0x515d4b; continue;
            }
            0x515d4b => {
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x3),8));
            { let a = c.get(1,8); c.sub(a,0x30u32,8); }
            if c.cond(5) { pc = 0x515d5a; continue; }
            pc = 0x515d53; continue;
            }
            0x515d53 => {
            c.set(1,32,0x2fda0u32);
            pc = 0x515d6b; continue;
            }
            0x515d5a => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,8); c.sub(a,0x31u32,8); }
            c.set(2,8,u32::from(c.cond(5)));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0x77a10u32,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            pc = 0x515d6b; continue;
            }
            0x515d6b => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,0xc8u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x515d89 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00515d90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x515d90;
    loop {
        match pc {
            0x515d90 => {
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00515ea0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x515ea0;
    loop {
        match pc {
            0x515ea0 => {
            c.set(2,16,rm(m,0x6dd894u32,16));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(7,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x515efc; continue; }
            pc = 0x515eb1; continue;
            }
            0x515eb1 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(1,32); let r = c.add(a,0xcu32,32); c.set(1,32,r); }
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x515ede; continue; }
            pc = 0x515ebd; continue;
            }
            0x515ebd => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            pc = 0x515ec4; continue;
            }
            0x515ec4 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x515ec4; continue; }
            pc = 0x515edd; continue;
            }
            0x515edd => {
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x515ede; continue;
            }
            0x515ede => {
            { let a = c.r[1]; wm(m,a,16,c.get(7,16)); }
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(0,32,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2]).wrapping_add(0x4); wm(m,a,16,0xffffu32); }
            pc = 0x515efc; continue;
            }
            0x515efc => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00515f00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x515f00;
    loop {
        match pc {
            0x515f00 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(1,32); let r = c.add(a,0xcu32,32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,16,rm(m,0x6dd894u32,16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x515f65; continue; }
            pc = 0x515f15; continue;
            }
            0x515f15 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            pc = 0x515f1f; continue;
            }
            0x515f1f => {
            { let a = c.get(2,16); c.sub(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x515f42; continue; }
            pc = 0x515f24; continue;
            }
            0x515f24 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x515f1f; continue; }
            pc = 0x515f3d; continue;
            }
            0x515f3d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x515f42 => {
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4),16));
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.r[0].wrapping_add(c.r[1]).wrapping_add(0x4); wm(m,a,16,0xffffu32); }
            pc = 0x515f65; continue;
            }
            0x515f65 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00517170(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x517170;
    loop {
        match pc {
            0x517170 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x39),8));
            c.set(6,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x517194; continue; }
            pc = 0x517180; continue;
            }
            0x517180 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x37),8));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x517194; continue; }
            pc = 0x517189; continue;
            }
            0x517189 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x38),8));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            pc = 0x517194; continue;
            }
            0x517194 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x6de4b8),8));
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,c.r[2].wrapping_add(c.r[6].wrapping_mul(4)));
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(2,16,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6ddbb0),16));
            { let a = c.r[1].wrapping_add(0x1e); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x6de4b8),8));
            c.set(0,32,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,16,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbb4),16));
            { let a = rm(m,c.r[1].wrapping_add(0x3d),8); let r = c.or(a,0x20u32,8); { let a = c.r[1].wrapping_add(0x3d); wm(m,a,8,r); } }
            { let a = c.r[1].wrapping_add(0x22); wm(m,a,16,c.get(2,16)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005176c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5176c0;
    loop {
        match pc {
            0x5176c0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x24),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x517768; continue; }
            pc = 0x5176cf; continue;
            }
            0x5176cf => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x24),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x517768; continue; }
            pc = 0x5176de; continue;
            }
            0x5176de => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x1c),32));
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x20),32));
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x28),32));
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x2c),32));
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x30),32));
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x34),16));
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2),16));
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,16,c.get(0,16)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x36),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x51775e; continue; }
            pc = 0x517713; continue;
            }
            0x517713 => {
            c.push(m,0x8u32);
            { let a = c.r[6].wrapping_add(0x36); wm(m,a,8,0x1u32); }
            c.push(m, 0x51771e);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x24),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x517739);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x24),32));
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x24),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x517756);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x51775e => {
            { let a = c.r[6].wrapping_add(0x36); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x24),32));
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            pc = 0x517768; continue;
            }
            0x517768 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00518460(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x518460;
    loop {
        match pc {
            0x518460 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            c.set(7,32,0xfffffec4u32);
            c.set(2,32,c.r[1].wrapping_add(0x13c));
            { let a = c.get(7,32); let r = c.sub(a,c.get(1,32),32); c.set(7,32,r); }
            pc = 0x518473; continue;
            }
            0x518473 => {
            c.set(0,8,rm(m,c.r[2],8));
            { let a = c.get(0,8); c.sub(a,0x6u32,8); }
            if c.cond(3) { pc = 0x51848a; continue; }
            pc = 0x518479; continue;
            }
            0x518479 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_mul(2).wrapping_add(0x672cb4),16));
            { let a = c.get(6,32); let r = c.sub(a,c.get(3,32),32); c.set(6,32,r); }
            pc = 0x51848a; continue;
            }
            0x51848a => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(0,32,c.r[7].wrapping_add(c.r[2]));
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x518473; continue; }
            pc = 0x518493; continue;
            }
            0x518493 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x154),16));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xe),8));
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x40),16));
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[2].wrapping_add(0x673664),8));
            c.set(6,16,c.get(2,8));
            { let a = c.get(1,16); c.sub(a,c.get(6,16),16); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(6) { pc = 0x5184dd; continue; }
            pc = 0x5184c4; continue;
            }
            0x5184c4 => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            pc = 0x5184dd; continue;
            }
            0x5184dd => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = rm(m,c.r[4].wrapping_add(0x4),32); c.div32(b); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00518750(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x518750;
    loop {
        match pc {
            0x518750 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.set(2,16,rm(m,0x6dd894u32,16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            c.push(m,c.get(6,32));
            if c.cond(2) { pc = 0x51877b; continue; }
            pc = 0x518761; continue;
            }
            0x518761 => {
            { let a = rm(m,c.r[1].wrapping_add(0x50),16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x518777; continue; }
            pc = 0x518767; continue;
            }
            0x518767 => {
            { let a = rm(m,c.r[1].wrapping_add(0x52),16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x518777; continue; }
            pc = 0x51876d; continue;
            }
            0x51876d => {
            { let a = rm(m,c.r[1].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x5187ce; continue; }
            pc = 0x518777; continue;
            }
            0x518777 => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x51877b => {
            c.set(6,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(6,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x518777; continue; }
            pc = 0x518798; continue;
            }
            0x518798 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = rm(m,c.r[0].wrapping_add(0x50),16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x518777; continue; }
            pc = 0x5187b3; continue;
            }
            0x5187b3 => {
            { let a = rm(m,c.r[0].wrapping_add(0x52),16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x518777; continue; }
            pc = 0x5187b9; continue;
            }
            0x5187b9 => {
            c.set(2,8,rm(m,c.r[6],8));
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(2,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x518777; continue; }
            pc = 0x5187c1; continue;
            }
            0x5187c1 => {
            { let a = rm(m,c.r[6].wrapping_add(0x12),16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x518777; continue; }
            pc = 0x5187c8; continue;
            }
            0x5187c8 => {
            { let a = rm(m,c.r[6].wrapping_add(0x14),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x518777; continue; }
            pc = 0x5187ce; continue;
            }
            0x5187ce => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x22),16));
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x1e),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5187dd);
            { let args = []; c.r[0] = host.call(m, 0x517260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(2,8,u32::from(c.cond(12)));
            c.set(0,8,c.get(2,8));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00519b50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x519b50;
    loop {
        match pc {
            0x519b50 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x38),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x13a),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[6].wrapping_add(0x13b); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,c.get(6,32));
            if c.cond(5) { pc = 0x519b7f; continue; }
            pc = 0x519b68; continue;
            }
            0x519b68 => {
            c.push(m, 0x519b6d);
            { let args = []; c.r[0] = host.call(m, 0x5193f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x13b),8));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x519b7d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x519920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x519b7f => {
            c.push(m, 0x519b84);
            { let args = []; c.r[0] = host.call(m, 0x5196b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x13b),8));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x519b94);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x519920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00515da0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x515da0;
    loop {
        match pc {
            0x515da0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x16),16));
            c.set(1,16,rm(m,0x672a2eu32,16));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x20u32,32); }
            if c.cond(14) { pc = 0x515dc7; continue; }
            pc = 0x515dc2; continue;
            }
            0x515dc2 => {
            c.set(0,32,0x20u32);
            pc = 0x515dc7; continue;
            }
            0x515dc7 => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x10),16));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(2) { pc = 0x515e98; continue; }
            pc = 0x515dd8; continue;
            }
            0x515dd8 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(3,8,0x1bu32);
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x8),16));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x515e03; continue; }
            pc = 0x515de4; continue;
            }
            0x515de4 => {
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x4),32));
            pc = 0x515de7; continue;
            }
            0x515de7 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x3),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x515df6; continue; }
            pc = 0x515ded; continue;
            }
            0x515ded => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x515e98; continue; }
            pc = 0x515df6; continue;
            }
            0x515df6 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x4),16));
            c.set(1,32,c.get(5,32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x515de7; continue; }
            pc = 0x515e03; continue;
            }
            0x515e03 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x515e0a);
            { let args = []; c.r[0] = host.call(m, 0x513bf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x3); wm(m,a,8,c.get(3,8)); }
            c.set(0,8,rm(m,0x6de4b4u32,8));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x515e22; continue; }
            pc = 0x515e18; continue;
            }
            0x515e18 => {
            c.set(0,8,rm(m,0x677929u32,8));
            { let a = c.r[6].wrapping_add(0x7); wm(m,a,8,c.get(0,8)); }
            pc = 0x515e2b; continue;
            }
            0x515e22 => {
            c.set(1,8,rm(m,0x67792fu32,8));
            { let a = c.r[6].wrapping_add(0x7); wm(m,a,8,c.get(1,8)); }
            pc = 0x515e2b; continue;
            }
            0x515e2b => {
            { let a = rm(m,c.r[7],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x515e91; continue; }
            pc = 0x515e30; continue;
            }
            0x515e30 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(2,8,rm(m,0x6de4b4u32,8));
            { let a = c.get(2,8); let r = c.and(a,0x3u32,8); c.set(2,8,r); }
            c.set(3,32,c.r[6].wrapping_add(0x1));
            c.set(1,8,rm(m,c.r[0],8));
            c.push(m,0x0u32);
            { let a = c.get(1,8); let r = c.sub(a,c.get(2,8),8); c.set(1,8,r); }
            c.push(m,0x2au32);
            { let a = c.get(1,8); let r = c.add(a,0xfu32,8); c.set(1,8,r); }
            { let a = c.r[6]; wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(0,8,rm(m,0x6de4a4u32,8));
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x1),8));
            { let a = c.get(0,8); let r = c.and(a,0x3u32,8); c.set(0,8,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,8); let r = c.sub(a,c.get(0,8),8); c.set(1,8,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x7),8));
            { let a = c.get(1,8); let r = c.add(a,0xfu32,8); c.set(1,8,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[3]; wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x515e6f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f9c80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x515e98; continue; }
            pc = 0x515e73; continue;
            }
            0x515e73 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[3],8));
            c.push(m,c.get(0,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,0x2au32);
            c.set(2,8,rm(m,c.r[6],8));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(2,32));
            c.push(m, 0x515e8c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4fb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x515e91 => {
            { let a = c.r[6]; wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x1); wm(m,a,8,0xffu32); }
            pc = 0x515e98; continue;
            }
            0x515e98 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
